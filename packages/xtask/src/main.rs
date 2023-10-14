use std::{
    env,
    ffi::OsStr,
    fmt::Write,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use itertools::Itertools;
use scopeguard::defer;
use xshell::{cmd, Shell};
use xtask_base::{
    build_readme, ci_nightly, clippy, generate_open_source_files, run, CommonCmds, WorkflowResult,
};

#[derive(Parser)]
enum Commands {
    /// Generate all derived files. Will overwrite existing content.
    Codegen {
        /// If set, just check the file contents are up to date.
        #[clap(long)]
        check: bool,
    },
    /// Run CI checks
    Ci {
        #[clap(subcommand)]
        command: Option<CiCommand>,
    },
    TestFeatures,
    WasmPackTest,
    TrunkBuild,
    /// Run TodoMVC with `trunk`
    TodomvcRun,
    /// Run the TodoMVC Cypress tests
    TodomvcCypress {
        #[clap(long)]
        gui: bool,
    },
    BuildWebsite,
    GithubActions {
        #[clap(long)]
        full: bool,
    },
    FmtAll,
    #[clap(flatten)]
    Common(CommonCmds),
}

#[derive(Subcommand, PartialEq, Eq)]
enum CiCommand {
    Stable {
        #[clap(long)]
        fast: bool,
        toolchain: Option<String>,
    },
    Nightly {
        toolchain: Option<String>,
    },
    Browser,
}

fn main() {
    run(|workspace| {
        let sh = Shell::new()?;

        match Commands::parse() {
            Commands::Codegen { check } => {
                build_readme(".", check)?;
                generate_open_source_files(2021, check)?;
            }
            Commands::Ci { command } => {
                if let Some(command) = command {
                    match command {
                        CiCommand::Stable { fast, toolchain } => ci_stable(fast, toolchain)?,
                        CiCommand::Nightly { toolchain } => ci_nightly(toolchain.as_deref())?,
                        CiCommand::Browser => ci_browser()?,
                    }
                } else {
                    ci_stable(false, None)?;
                    ci_nightly(Some("nightly"))?;
                    ci_browser()?;
                    wasm_pack_test()?;
                }
            }
            Commands::TestFeatures => test_features()?,
            Commands::WasmPackTest => wasm_pack_test()?,
            Commands::TrunkBuild => trunk_build()?,
            Commands::TodomvcRun => {
                let _dir = sh.push_dir("examples/todomvc");
                cmd!(sh, "trunk serve --open").run()?;
            }
            Commands::TodomvcCypress { gui } => {
                cypress("install", if gui { "open" } else { "run" }, None)?;
            }
            Commands::BuildWebsite => build_website()?,
            Commands::GithubActions { full } => {
                let reuse = (!full).then_some("--reuse");

                cmd!(sh, "docker build . -t silkenweb-github-actions").run()?;
                cmd!(sh,
                    "act -P ubuntu-latest=silkenweb-github-actions:latest --use-gitignore {reuse...}"
                )
                .run()?;
            }
            Commands::FmtAll => foreach_workspace(|| {
                let sh = Shell::new()?;
                cmd!(sh, "cargo +nightly fmt --all").run()?;
                Ok(())
            })?,
            Commands::Common(cmds) => cmds.run::<Commands>(workspace)?,
        }

        Ok(())
    });
}

fn foreach_workspace(f: impl Fn() -> WorkflowResult<()>) -> WorkflowResult<()> {
    for dir in [".", "examples/ssr-full"] {
        let previous_dir = env::current_dir()?;
        env::set_current_dir(dir)?;
        f()?;
        env::set_current_dir(previous_dir)?;
    }

    Ok(())
}

fn build_website() -> WorkflowResult<()> {
    let sh = Shell::new()?;
    let dest_dir = "target/website";
    sh.remove_path(dest_dir)?;
    let examples_dest_dir = format!("{dest_dir}/examples");
    sh.create_dir(&examples_dest_dir)?;
    let mut redirects = String::new();

    for example in browser_examples()? {
        let examples_dir: PathBuf = [Path::new("examples"), &example].iter().collect();

        {
            let _dir = sh.push_dir(&examples_dir);
            cmd!(sh, "trunk build --release --public-url examples/{example}").run()?;
        }

        cmd!(
            sh,
            "cp -R {examples_dir}/dist/ {examples_dest_dir}/{example}"
        )
        .run()?;

        {
            let example = example.display();
            writeln!(
                &mut redirects,
                "/examples/{example}/* /examples/{example}/index.html 200"
            )?;
        }
    }

    sh.write_file(format!("{dest_dir}/_redirects"), redirects)?;

    Ok(())
}

fn ci_browser() -> WorkflowResult<()> {
    cypress("ci", "run", None)?;
    trunk_build()
}

fn ci_stable(fast: bool, toolchain: Option<String>) -> WorkflowResult<()> {
    build_readme(".", true)?;
    generate_open_source_files(2021, true)?;
    foreach_workspace(|| xtask_base::ci_stable(fast, toolchain.as_deref(), &[]))?;
    test_features()
}

fn test_features() -> WorkflowResult<()> {
    let sh = Shell::new()?;

    for features in ["declarative-shadow-dom"].into_iter().powerset() {
        if !features.is_empty() {
            clippy(None, &features)?;

            let features = features.join(",");

            cmd!(sh, "cargo test --package silkenweb --features {features}").run()?;
        }
    }

    Ok(())
}

fn cypress(npm_install_cmd: &str, cypress_cmd: &str, browser: Option<&str>) -> WorkflowResult<()> {
    let sh = Shell::new()?;
    let _dir = sh.push_dir("examples/todomvc");
    cmd!(sh, "trunk build").run()?;
    let dir = env::current_dir()?;
    env::set_current_dir(sh.current_dir())?;
    let trunk = duct::cmd("trunk", ["serve", "--no-autoreload", "--ignore=."]).start()?;
    env::set_current_dir(dir)?;
    defer! { let _ = trunk.kill(); };

    let _dir = sh.push_dir("e2e");
    cmd!(sh, "npm {npm_install_cmd}").run()?;

    if let Some(browser) = browser {
        cmd!(sh, "npx cypress {cypress_cmd} --browser {browser}").run()?;
    } else {
        cmd!(sh, "npx cypress {cypress_cmd}").run()?;
    }

    Ok(())
}

fn wasm_pack_test() -> WorkflowResult<()> {
    let sh = Shell::new()?;
    let _dir = sh.push_dir("packages/silkenweb");
    cmd!(sh, "wasm-pack test --headless --firefox").run()?;
    Ok(())
}

fn browser_examples() -> WorkflowResult<Vec<PathBuf>> {
    let sh = Shell::new()?;
    let _dir = sh.push_dir("examples");
    let examples = sh.read_dir(".")?;
    let non_browser = ["htmx-axum"];
    let non_browser: Vec<_> = non_browser.into_iter().map(OsStr::new).collect();
    let mut browser_examples = Vec::new();

    for example in examples {
        if let Some(example) = example.file_name() {
            if !non_browser.contains(&example) {
                for file in sh.read_dir(example)? {
                    if file.extension() == Some(OsStr::new("html")) {
                        browser_examples.push(example.into());
                        break;
                    }
                }
            }
        }
    }

    Ok(browser_examples)
}

fn trunk_build() -> WorkflowResult<()> {
    let sh = Shell::new()?;
    let _dir = sh.push_dir("examples");

    for example in browser_examples()? {
        let _dir = sh.push_dir(example);
        cmd!(sh, "trunk build").run()?;
    }

    Ok(())
}
