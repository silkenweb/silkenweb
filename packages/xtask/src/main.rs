use clap::{Parser, Subcommand};
use itertools::Itertools;
use scopeguard::defer;
use xshell::{cmd, mkdir_p, pushd};
use xtask_base::{
    build_readme, ci_nightly, clippy, generate_open_source_files, run, target_os, CommonCmds,
    TargetOs, WorkflowResult,
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
    /// Run TodoMVC with `trunk`
    TodomvcRun,
    /// Run the TodoMVC Cypress tests
    TodomvcCypress {
        #[clap(long)]
        gui: bool,
    },
    BuildAllExamples,
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
                }
            }
            Commands::TestFeatures => test_features()?,
            Commands::WasmPackTest => wasm_pack_test()?,
            Commands::TodomvcRun => {
                let _dir = pushd("examples/todomvc")?;
                cmd!("trunk serve --open").run()?;
            }
            Commands::TodomvcCypress { gui } => {
                cypress("install", if gui { "open" } else { "run" }, None)?;
            }
            Commands::BuildAllExamples => build_all_examples()?,
            Commands::GithubActions { full } => {
                let reuse = (!full).then(|| "--reuse");

                cmd!("docker build . -t silkenweb-github-actions").run()?;
                cmd!(
                    "act -P ubuntu-latest=silkenweb-github-actions:latest --use-gitignore {reuse...}"
                )
                .run()?;
            }
            Commands::Common(cmds) => cmds.run::<Commands>(workspace)?,
        }

        Ok(())
    });
}

fn build_all_examples() -> WorkflowResult<()> {
    let dest_dir = "target/examples";
    mkdir_p(&dest_dir)?;

    for example in ["animation"] {
        let example_dir = format!("examples/{example}");

        {
            let _dir = pushd(&example_dir);
            cmd!("trunk build --release --public-url silkenweb/").run()?;
        }

        cmd!("cp -R {example_dir}/dist/ {dest_dir}/{example}").run()?;
    }

    Ok(())
}

fn test_features() -> WorkflowResult<()> {
    for features in ["client-side-render", "server-side-render", "hydration"]
        .into_iter()
        .powerset()
    {
        clippy(None, &features)?;

        let features = features.join(",");

        cmd!("cargo test --package silkenweb --features {features}").run()?;
    }

    Ok(())
}

fn ci_browser() -> WorkflowResult<()> {
    match target_os() {
        TargetOs::Windows => cypress("ci", "run", Some("edge"))?,
        TargetOs::Linux => {
            wasm_pack_test()?;
            cypress("ci", "run", Some("firefox"))?
        }
        _ => (),
    };

    Ok(())
}

fn ci_stable(fast: bool, toolchain: Option<String>) -> WorkflowResult<()> {
    build_readme(".", true)?;
    generate_open_source_files(2021, true)?;
    xtask_base::ci_stable(fast, toolchain.as_deref(), &[])?;
    test_features()?;
    Ok(())
}

fn cypress(npm_install_cmd: &str, cypress_cmd: &str, browser: Option<&str>) -> WorkflowResult<()> {
    let _dir = pushd("examples/todomvc")?;
    cmd!("trunk build").run()?;
    let trunk = duct::cmd("trunk", ["serve", "--no-autoreload", "--ignore=."]).start()?;
    defer! { let _ = trunk.kill(); };

    let _dir = pushd("e2e")?;
    cmd!("npm {npm_install_cmd}").run()?;

    if let Some(browser) = browser {
        cmd!("npx cypress {cypress_cmd} --browser {browser}").run()?;
    } else {
        cmd!("npx cypress {cypress_cmd}").run()?;
    }

    Ok(())
}

fn wasm_pack_test() -> WorkflowResult<()> {
    let _dir = pushd("packages/silkenweb")?;
    cmd!("wasm-pack test --headless --firefox").run()?;
    cmd!("wasm-pack test --headless --firefox --features hydration").run()?;
    Ok(())
}
