use std::path::Path;

use log::LevelFilter;
use silkenweb::{document::Document, dom::Dry, router, task};
use ssr_full_app::app;
use xshell::Shell;
use xtask_wasm::{
    anyhow::Result,
    clap::{self, Parser},
    default_dist_dir, WasmOpt,
};

#[derive(clap::Parser)]
enum Workflow {
    Build(xtask_wasm::Dist),
    Serve(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    match Workflow::parse() {
        Workflow::Build(build) => {
            let release = build.release;
            let dist = build
                .app_name("ssr_example_pre_rendered_client")
                .run("ssr_example_pre_rendered_client")?;

            if release {
                WasmOpt::level(1).shrink(2).optimize(&dist)?;
            }

            generate_pages(&dist)?;
        }
        Workflow::Serve(server) => {
            server.arg("build").start(default_dist_dir(false))?;
        }
    }

    Ok(())
}

fn generate_pages(dist_dir: &Path) -> xshell::Result<()> {
    task::sync_scope(|| {
        let (head, body) = app::<Dry>();
        Dry::mount_in_head("head", head);
        let body = body.freeze();
        let sh = Shell::new()?;

        for page in ["index", "page_1", "page_2"] {
            router::set_url_path(format!("{page}.html").as_str());
            task::server::render_now_sync();

            let page_html = format!(
                include_str!("../../app/page.tmpl.html"),
                init_script = r#"
                import init from "/ssr_example_pre_rendered_client.js";
                init(new URL('ssr_example_pre_rendered_client_bg.wasm', import.meta.url));
            "#,
                head_html = Dry::head_inner_html(),
                body_html = body
            );
            let page_path = Path::new(dist_dir).join(page).with_extension("html");

            sh.write_file(page_path, page_html)?;
        }

        Ok(())
    })
}
