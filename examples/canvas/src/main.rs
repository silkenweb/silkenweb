use std::cell::RefCell;

use silkenweb::{
    clone,
    elements::html::canvas,
    mount,
    prelude::{Element, ElementEvents, Mutable},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn main() {
    // Accumulate the changes to draw.
    let lines = Mutable::new(Lines::default());
    let drawing = Mutable::new(false);

    mount(
        "app",
        canvas()
            .on_mousedown({
                clone!(lines, drawing);
                move |ev, _| {
                    drawing.set(true);
                    lines.lock_mut().move_to(Point {
                        x: ev.offset_x(),
                        y: ev.offset_y(),
                    });
                }
            })
            .on_mouseup({
                clone!(drawing);
                move |_, _| drawing.set(false)
            })
            .on_mousemove({
                clone!(lines, drawing);
                move |ev, _| {
                    if drawing.get() {
                        lines.lock_mut().line_to(Point {
                            x: ev.offset_x(),
                            y: ev.offset_y(),
                        });
                    }
                }
            })
            .effect_signal(lines.signal_ref(|_| ()), move |canvas, _| {
                lines.lock_mut().draw(canvas);
            }),
    );
}

#[derive(Default)]
struct Lines(RefCell<Vec<Line>>);

impl Lines {
    fn move_to(&mut self, p: Point) {
        self.0.borrow_mut().push(Line::new(p))
    }

    fn line_to(&mut self, p: Point) {
        if let Some(current) = self.0.borrow_mut().last_mut() {
            current.push(p)
        }
    }

    fn draw(&self, c: &HtmlCanvasElement) {
        let ctx = c
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        ctx.begin_path();

        for line in self.0.borrow().iter() {
            line.draw(&ctx);
        }

        ctx.stroke();
        self.clear_processed();
    }

    fn clear_processed(&self) {
        let current = self.0.borrow_mut().pop();

        if let Some(mut current) = current {
            current.points.clear();
            self.0.replace(vec![current]);
        }
    }
}

struct Line {
    points: Vec<Point>,
    last: Point,
}

impl Line {
    fn new(start: Point) -> Self {
        Self {
            points: Vec::new(),
            last: start,
        }
    }

    fn push(&mut self, p: Point) {
        self.points.push(self.last);
        self.last = p;
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let mut points = self.points.iter();

        if let Some(point) = points.next() {
            ctx.move_to(point.x as f64, point.y as f64);

            for point in points {
                ctx.line_to(point.x as f64, point.y as f64);
            }

            ctx.line_to(self.last.x as f64, self.last.y as f64);
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
