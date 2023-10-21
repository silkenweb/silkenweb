use std::{
    ffi::OsStr,
    fs::{self},
    path::PathBuf,
};

use clap::Parser;
use duct::cmd;
use itertools::Itertools;
use scopeguard::defer;
use xtask_base::{
    build_readme,
    ci::{Tasks, CI},
    generate_open_source_files,
    github::actions::{self, action, install, push, rust_toolchain, script, Platform, Rust, Step},
    in_workspace, CommonCmds, WorkflowResult,
};

// TODO: Remove xshell dependencies

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
    dbg!();
    in_workspace(|workspace| {
        dbg!();
        let web_tests = || web_tests(Platform::current());
        dbg!();
        type Cmds = Commands;
        dbg!();

        match Cmds::parse() {
            Cmds::TestFeatures => test_features(tests(Platform::current())).execute()?,
            Cmds::WasmPackTest => wasm_pack_test(web_tests()).execute()?,
            Cmds::TrunkBuild => trunk_build(web_tests())?.execute()?,
            Cmds::TodomvcCypress { gui } => {
                dbg!();
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
    rust_toolchain("1.71").minimal().default()
}
fn tests(platform: Platform) -> Tasks {
    Tasks::new("tests", platform, stable_rust().clippy())
}

fn web_tests(platform: Platform) -> Tasks {
    Tasks::new("web-tests", platform, stable_rust().wasm())
        .step(action("actions/setup-node@v3").with("node-version", "18"))
        .cmd("npm", ["--version"])
        .cmd("npx", ["--version"])
        .step(install("wasm-pack", "0.12.1"))
        .step(trunk())
}

fn trunk() -> Step {
    install("trunk", "0.17.2")
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
    let tasks = Tasks::new(
        "build-website",
        Platform::UbuntuLatest,
        stable_rust().wasm(),
    )
    .step(trunk());

    let dest_dir = "target/website";
    let redirects_file = format!("{dest_dir}/_redirects");
    let mut tasks = tasks
        .cmd("mkdir", ["-p", dest_dir])
        .cmd("touch", [&redirects_file]);

    for example_dir in browser_example_dirs()? {
        let example_dir = example_dir.to_str().expect("invalid path name");

        tasks = tasks
            .run(
                actions::cmd("trunk", ["build", "--release", "--public-url", example_dir])
                    .in_directory(example_dir),
            )
            .run(
                actions::cmd(
                    "cp",
                    [
                        "-R",
                        &format!("{example_dir}/dist/"),
                        &format!("{dest_dir}/{example_dir}"),
                    ],
                )
                .in_directory(example_dir),
            )
            .step(
                actions::cmd(
                    "echo",
                    [
                        &format!("/{example_dir}/* /{example_dir}/index.html 200"),
                        ">>",
                        &redirects_file,
                    ],
                )
                .in_directory(example_dir),
            );
    }

    let tasks = tasks.step(
        action("nwtgck/actions-netlify@v2.0")
            .with("publish-dir", "'target/website'")
            .with("production-deploy", "true")
            .env("NETLIFY_AUTH_TOKEN", "${{ secrets.NETLIFY_AUTH_TOKEN }}")
            .env("NETLIFY_SITE_ID", "${{ secrets.NETLIFY_SITE_ID }}"),
    );

    Ok(tasks)
}

const WORKSPACE_SUB_DIRS: &[&str] = &["examples/ssr-full", "examples/tauri"];

fn ci() -> WorkflowResult<CI> {
    let mut ci = CI::new().job(
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
    );

    for platform in Platform::latest() {
        ci.add_job(ci_native(platform));
        ci.add_job(ci_browser(platform)?);
    }

    Ok(ci)
}

fn pre_tauri_build() -> actions::Run {
    actions::cmd("mkdir", ["-p", "examples/tauri/frontend/dist"])
}

fn install_gtk() -> actions::Run {
    script([
        vec!["sudo", "apt-get", "update"],
        vec!["sudo", "apt-get", "install", "-y", "webkit2gtk-4.0"],
    ])
}

fn ci_browser(platform: Platform) -> WorkflowResult<Tasks> {
    let mut tasks = web_tests(platform).cmd("cargo", ["xtask", "todomvc-cypress"]);
    tasks = playwright(tasks);
    trunk_build(tasks)
}

fn ci_native(platform: Platform) -> Tasks {
    let mut tasks = tests(platform);

    if platform == Platform::UbuntuLatest {
        tasks.add_step(install_gtk());
    }

    tasks = tasks.run(pre_tauri_build()).tests(WORKSPACE_SUB_DIRS);
    test_features(tasks)
}

fn test_features(mut tasks: Tasks) -> Tasks {
    for features in ["declarative-shadow-dom"].into_iter().powerset() {
        if !features.is_empty() {
            // TODO: We need something like the cmd macro in xshell
            let mut args = vec!["clippy", "--features"];
            args.extend(features.clone());
            args.extend(["--all-targets", "--", "-D", "warnings", "-D", "clippy::all"]);
            tasks.add_cmd("cargo", args);
            tasks.add_cmd(
                "cargo",
                ["test", "--package", "silkenweb", "--features"]
                    .into_iter()
                    .chain(features),
            );
        }
    }

    tasks
}

fn cypress(cypress_cmd: &str, browser: Option<&str>) -> WorkflowResult<()> {
    dbg!();
    cmd("npm", ["--version"]).run()?;
    dbg!();
    cmd("npx", ["--version"]).run()?;
    dbg!();
    let dir = "examples/todomvc";
    cmd("trunk", ["build"]).dir(dir).run()?;
    dbg!();
    let trunk = cmd("trunk", ["serve", "--no-autoreload", "--ignore=."])
        .dir(dir)
        .start()?;
    dbg!();
    defer! { let _ = trunk.kill(); };
    dbg!();

    let dir = format!("{dir}/e2e");
    dbg!();
    cmd("npm", ["--version"]).run()?;
    dbg!();
    cmd("npx", ["--version"]).run()?;
    dbg!();

    cmd("npm", ["ci"]).dir(&dir).run()?;
    dbg!();

    cmd("npm", ["--version"]).run()?;
    dbg!();
    cmd("npx", ["--version"]).run()?;
    dbg!();
    if let Some(browser) = browser {
        dbg!();
        cmd("npx", ["cypress", cypress_cmd, "--browser", browser])
            .dir(&dir)
            .run()?;
    } else {
        dbg!();
        cmd("npx", ["cypress", cypress_cmd]).dir(&dir).run()?;
    }
    dbg!();

    Ok(())
}

fn playwright(tasks: Tasks) -> Tasks {
    let dir = "examples/todomvc/playwright";
    tasks
        .run(actions::cmd("npm", ["ci"]).in_directory(dir))
        .step(actions::cmd("npx", ["playwright", "install", "--with-deps"]).in_directory(dir))
        .run(actions::cmd("npx", ["playwright", "install"]).in_directory(dir))
        .run(actions::cmd("npx", ["playwright", "test"]).in_directory(dir))
}

fn wasm_pack_test(tasks: Tasks) -> Tasks {
    let dir = "packages/silkenweb";
    tasks.run(actions::cmd("wasm-pack", ["test", "--headless", "--firefox"]).in_directory(dir))
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
        tasks.add_run(
            actions::cmd("trunk", ["build"])
                .in_directory(example_dir.to_str().expect("Invalid path name")),
        );
    }

    Ok(tasks)
}
