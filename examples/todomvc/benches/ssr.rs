use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use silkenweb::task::server::render_now_sync;
use silkenweb_examples_todomvc::{model::TodoApp, view::TodoAppView};

pub fn ssr(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_n");

    for n in [0, 10, 100, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, n| {
            b.iter(|| {
                let node =
                    TodoAppView::new(TodoApp::with_todos((0..*n).map(|n| format!("Todo #{n}"))))
                        .render()
                        .freeze();

                render_now_sync();
                let rendered = node.to_string();

                black_box(rendered);
            })
        });
    }
}

criterion_group!(benches, ssr);
criterion_main!(benches);
