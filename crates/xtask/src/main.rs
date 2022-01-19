use clap::Parser;
use xshell::{cmd, pushd};
use xtask_base::{
    build_readme, ci, generate_open_source_files, run, CommonCmds, Toolchain, WorkflowResult,
};

#[derive(Parser)]
enum Commands {
    Codegen {
        #[clap(long)]
        check: bool,
    },
    Ci {
        #[clap(long)]
        fast: bool,
        toolchain: Option<Toolchain>,
    },
    TodomvcRun,
    TodomvcCypress,
    #[clap(flatten)]
    Common(CommonCmds),
}

fn main() {
    run(|workspace| {
        match Commands::parse() {
            Commands::Codegen { check } => {
                build_readme(".", check)?;
                generate_open_source_files(2021, check)?;
            }
            Commands::Ci { fast, toolchain } => {
                build_readme(".", true)?;
                generate_open_source_files(2021, true)?;
                ci(fast, toolchain)?;

                if !fast {
                    cypress("ci")?;
                }

                if toolchain.map_or(true, |tc| tc == Toolchain::Stable) {
                    wasm_pack_test()?;
                }
            }
            Commands::TodomvcRun => {
                let _dir = pushd("examples/todomvc")?;
                cmd!("trunk serve --open").run()?;
            }
            Commands::TodomvcCypress => {
                cypress("install")?;
            }
            Commands::Common(cmds) => cmds.run::<Commands>(workspace)?,
        }

        Ok(())
    });
}

fn cypress(install_cmd: &str) -> WorkflowResult<()> {
    let _dir = pushd("examples/todomvc/e2e")?;
    cmd!("npm {install_cmd}").run()?;
    cmd!("npm test").run()?;
    Ok(())
}

fn wasm_pack_test() -> WorkflowResult<()> {
    let _dir = pushd("crates/silkenweb")?;
    cmd!("wasm-pack test --headless --firefox").run()?;
    Ok(())
}
