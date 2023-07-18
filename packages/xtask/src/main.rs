use std::{
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
                        CiCommand::Stable { fast, toolchain } => ci_stable(&sh, fast, toolchain)?,
                        CiCommand::Nightly { toolchain } => ci_nightly(toolchain.as_deref())?,
                        CiCommand::Browser => ci_browser(&sh)?,
                    }
                } else {
                    ci_stable(&sh, false, None)?;
                    ci_nightly(Some("nightly"))?;
                    ci_browser(&sh)?;
                    wasm_pack_test(&sh)?;
                }
            }
            Commands::TestFeatures => test_features(&sh)?,
            Commands::WasmPackTest => wasm_pack_test(&sh)?,
            Commands::TrunkBuild => trunk_build(&sh)?,
            Commands::TodomvcRun => {
                let _dir = sh.push_dir("examples/todomvc");
                cmd!(sh, "trunk serve --open").run()?;
            }
            Commands::TodomvcCypress { gui } => {
                cypress(&sh, "install", if gui { "open" } else { "run" }, None)?;
            }
            Commands::BuildWebsite => build_website(&sh)?,
            Commands::GithubActions { full } => {
                let reuse = (!full).then_some("--reuse");

                cmd!(sh, "docker build . -t silkenweb-github-actions").run()?;
                cmd!(sh,
                    "act -P ubuntu-latest=silkenweb-github-actions:latest --use-gitignore {reuse...}"
                )
                .run()?;
            }
            Commands::Common(cmds) => cmds.run::<Commands>(workspace)?,
        }

        Ok(())
    });
}

fn build_website(sh: &Shell) -> WorkflowResult<()> {
    let dest_dir = "target/website";
    sh.remove_path(dest_dir)?;
    let examples_dest_dir = format!("{dest_dir}/examples");
    sh.create_dir(&examples_dest_dir)?;
    let mut redirects = String::new();

    for example in browser_examples(sh)? {
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

fn ci_browser(sh: &Shell) -> WorkflowResult<()> {
    cypress(sh, "ci", "run", None)?;
    trunk_build(sh)
}

fn ci_stable(sh: &Shell, fast: bool, toolchain: Option<String>) -> WorkflowResult<()> {
    build_readme(".", true)?;
    generate_open_source_files(2021, true)?;
    xtask_base::ci_stable(fast, toolchain.as_deref(), &[])?;
    test_features(sh)
}

fn test_features(sh: &Shell) -> WorkflowResult<()> {
    for features in ["declarative-shadow-dom"].into_iter().powerset() {
        if !features.is_empty() {
            clippy(None, &features)?;

            let features = features.join(",");

            cmd!(sh, "cargo test --package silkenweb --features {features}").run()?;
        }
    }

    Ok(())
}

fn cypress(
    sh: &Shell,
    npm_install_cmd: &str,
    cypress_cmd: &str,
    browser: Option<&str>,
) -> WorkflowResult<()> {
    let _dir = sh.push_dir("examples/todomvc");
    cmd!(sh, "trunk build").run()?;
    let trunk = duct::cmd("trunk", ["serve", "--no-autoreload", "--ignore=."]).start()?;
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

fn wasm_pack_test(sh: &Shell) -> WorkflowResult<()> {
    let _dir = sh.push_dir("packages/silkenweb");
    cmd!(sh, "wasm-pack test --headless --firefox").run()?;
    Ok(())
}

fn browser_examples(sh: &Shell) -> WorkflowResult<Vec<PathBuf>> {
    let _dir = sh.push_dir("examples");
    let examples = sh.read_dir(".")?;
    let non_browser = ["htmx-axum"];
    let non_browser: Vec<_> = non_browser
        .into_iter()
        .map(|x| Some(OsStr::new(x)))
        .collect();
    let mut browser_examples = Vec::new();

    for example in examples {
        if !non_browser.contains(&example.file_name()) {
            for file in sh.read_dir(&example)? {
                if file.extension() == Some(OsStr::new("html")) {
                    browser_examples.push(example);
                    break;
                }
            }
        }
    }

    Ok(browser_examples)
}

fn trunk_build(sh: &Shell) -> WorkflowResult<()> {
    let _dir = sh.push_dir("examples");

    for example in browser_examples(sh)? {
        let _dir = sh.push_dir(example);
        cmd!(sh, "trunk build").run()?;
    }

    Ok(())
}
