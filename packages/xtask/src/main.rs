use std::{
    ffi::OsStr,
    fs::{self},
    path::PathBuf,
};

use clap::Parser;
use clonelet::clone;
use itertools::Itertools;
use scopeguard::defer;
use xtask_base::{
    build_readme,
    ci::{Tasks, CI},
    cmd, generate_open_source_files,
    github::actions::{self, action, install, push, rust_toolchain, script, Platform, Rust, Step},
    in_workspace, CommonCmds, WorkflowResult,
};

#[derive(Parser)]
enum Commands {
    TestFeatures,
    WasmPackTest,
    TrunkBuild,
    /// Run the TodoMVC Cypress tests
    TodomvcCypress {
        #[clap(long)]
        gui: bool,
    },
    TodomvcPlaywright,
    #[clap(flatten)]
    Common(CommonCmds),
}

fn main() {
    in_workspace(|workspace| {
        let web_tests = || web_tests(Platform::current());
        type Cmds = Commands;

        match Cmds::parse() {
            Cmds::TestFeatures => test_features(tests(Platform::current())).execute()?,
            Cmds::WasmPackTest => wasm_pack_test(web_tests()).execute()?,
            Cmds::TrunkBuild => trunk_build(web_tests())?.execute()?,
            Cmds::TodomvcCypress { gui } => {
                cmd!("trunk build").dir(TODOMVC_DIR).run()?;
                cypress(if gui { "open" } else { "run" }, None)?;
            }
            Cmds::TodomvcPlaywright => playwright(web_tests()).execute()?,
            Cmds::Common(cmds) => cmds.sub_command::<Cmds>(
                workspace,
                WORKSPACE_SUB_DIRS.iter().copied(),
                ci()?,
                codegen,
            )?,
        }

        Ok(())
    });
}

fn stable_rust() -> Rust {
    rust_toolchain(RUSTC_VERSION).minimal().default()
}

fn tests(platform: Platform) -> Tasks {
    Tasks::new("tests", platform, stable_rust().clippy())
}

fn web_tests(platform: Platform) -> Tasks {
    Tasks::new("web-tests", platform, stable_rust().wasm())
        .step(action("actions/setup-node@v3").with("node-version", "18"))
        .step(install("wasm-pack", WASM_PACK_VERSION))
        .step(trunk())
}

fn trunk() -> Step {
    install("trunk", TRUNK_VERSION)
}

fn codegen(check: bool) -> WorkflowResult<()> {
    build_readme(".", check)?;
    generate_open_source_files(2021, check)?;
    CI::named("website")
        .on(push().branch("main"))
        .job(deploy_website()?)
        .write(check)?;

    Ok(())
}

fn deploy_website() -> WorkflowResult<Tasks> {
    let dest_dir = "target/website";
    let redirects_file = format!("{dest_dir}/_redirects");

    let mut tasks = Tasks::new(
        "build-website",
        Platform::UbuntuLatest,
        stable_rust().wasm(),
    )
    .step(trunk())
    .run(cmd!("mkdir -p {dest_dir}"))
    .run(cmd!("touch {redirects_file}"));

    for example_dir in browser_example_dirs()? {
        let example_dir = example_dir.to_str().expect("invalid path name");

        tasks = tasks
            .run(cmd!("trunk build --release --public-url {example_dir}").dir(example_dir))
            .run(cmd!("cp -R {example_dir}/dist/ {dest_dir}/{example_dir}").dir(example_dir))
            .step(
                cmd!(
                    "echo /{example_dir}'/\\* /'{example_dir}'/index.html 200' >> {redirects_file}"
                )
                .dir(example_dir),
            );
    }

    Ok(tasks.step(
        action("nwtgck/actions-netlify@v2.0")
            .with("publish-dir", "'target/website'")
            .with("production-deploy", "true")
            .env("NETLIFY_AUTH_TOKEN", "${{ secrets.NETLIFY_AUTH_TOKEN }}")
            .env("NETLIFY_SITE_ID", "${{ secrets.NETLIFY_SITE_ID }}"),
    ))
}

fn ci() -> WorkflowResult<CI> {
    let mut ci = CI::new()
        .job(
            Tasks::new(
                "lints",
                Platform::UbuntuLatest,
                rust_toolchain("nightly-2023-10-14")
                    .minimal()
                    .default()
                    .rustfmt(),
            )
            .step(install_gtk())
            .run(pre_tauri_build())
            .lints("0.1.43", WORKSPACE_SUB_DIRS),
        )
        .standard_release_tests(RUSTC_VERSION, &[]);

    for platform in Platform::latest() {
        ci.add_job(ci_native(platform));
        ci.add_job(ci_browser(platform)?);
    }

    Ok(ci)
}

fn pre_tauri_build() -> actions::Run {
    cmd!("mkdir -p examples/tauri/frontend/dist")
}

fn install_gtk() -> actions::Run {
    script([
        vec!["sudo", "apt-get", "update"],
        vec!["sudo", "apt-get", "install", "-y", "webkit2gtk-4.0"],
    ])
}

fn ci_browser(platform: Platform) -> WorkflowResult<Tasks> {
    let tasks = web_tests(platform).run(cmd!("trunk build").dir(TODOMVC_DIR));

    if platform == Platform::WindowsLatest {
        Ok(tasks.step(
            action("cypress-io/github-action@v5")
                .with("working-directory", "examples/todomvc/e2e")
                .with("start", "npm start")
                .with("wait-on", "'http://localhost:8080'"),
        ))
    } else {
        tasks
            .run(cmd!("cargo xtask todomvc-cypress"))
            .apply(playwright)
            .apply(trunk_build)
    }
}

fn ci_native(platform: Platform) -> Tasks {
    tests(platform)
        .step_when(platform == Platform::UbuntuLatest, install_gtk())
        .run(pre_tauri_build())
        .tests(WORKSPACE_SUB_DIRS)
        .apply(test_features)
}

fn test_features(mut tasks: Tasks) -> Tasks {
    for features in ["declarative-shadow-dom"].into_iter().powerset() {
        if !features.is_empty() {
            tasks.add_run({
                clone!(features);
                cmd!( "cargo clippy --features {features...} --all-targets -- -D warnings -D clippy::all" )
            });
            tasks.add_run(cmd!(
                "cargo test --package silkenweb --features {features...}"
            ));
        }
    }

    tasks
}

fn cypress(cypress_cmd: &str, browser: Option<&str>) -> WorkflowResult<()> {
    let trunk = duct::cmd("trunk", ["serve", "--no-autoreload", "--ignore=."])
        .dir(TODOMVC_DIR)
        .start()?;
    defer! { let _ = trunk.kill(); };

    let dir = format!("{TODOMVC_DIR}/e2e");
    cmd!("npm ci").dir(&dir).run()?;

    if let Some(browser) = browser {
        cmd!("npx cypress {cypress_cmd} --browser {browser}")
            .dir(&dir)
            .run()?;
    } else {
        cmd!("npx cypress {cypress_cmd}").dir(&dir).run()?;
    }

    Ok(())
}

fn playwright(tasks: Tasks) -> Tasks {
    let dir = "examples/todomvc/playwright";
    tasks
        .run(cmd!("npm ci").dir(dir))
        .step(cmd!("npx playwright install --with-deps").dir(dir))
        .run(cmd!("npx playwright install").dir(dir))
        .run(cmd!("npx playwright test").dir(dir))
}

fn wasm_pack_test(tasks: Tasks) -> Tasks {
    let dir = "packages/silkenweb";
    tasks.run(cmd!("wasm-pack test --headless --firefox").dir(dir))
}

fn browser_example_dirs() -> WorkflowResult<Vec<PathBuf>> {
    let non_browser = ["htmx-axum"].map(OsStr::new).map(Some);
    let mut browser_examples = Vec::new();

    for example in fs::read_dir("examples")? {
        let example = example?.path();

        if !non_browser.contains(&example.file_name()) {
            for file in fs::read_dir(&example)? {
                let file: PathBuf = file?.file_name().into();

                if file.extension() == Some(OsStr::new("html")) {
                    browser_examples.push(example);
                    break;
                }
            }
        }
    }

    browser_examples.sort();

    Ok(browser_examples)
}

fn trunk_build(mut tasks: Tasks) -> WorkflowResult<Tasks> {
    for example_dir in browser_example_dirs()? {
        tasks.add_run(cmd!("trunk build").dir(example_dir.to_str().expect("Invalid path name")));
    }

    Ok(tasks)
}

const RUSTC_VERSION: &str = "1.73";
const WASM_PACK_VERSION: &str = "0.12.1";
const TRUNK_VERSION: &str = "0.17.2";

const TODOMVC_DIR: &str = "examples/todomvc";
const WORKSPACE_SUB_DIRS: &[&str] = &["examples/ssr-full", "examples/tauri"];
