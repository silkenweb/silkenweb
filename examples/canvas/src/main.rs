use html::canvas;
use silkenweb::{clone, mount, prelude::*, window};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Touch, TouchList};

fn main() {
    let mouse_last_point = Mutable::new(None);
    let touch_last_point = Mutable::new(None);
    let cv = canvas()
        .width("800px")
        .height("600px")
        .on_mousedown({
            clone!(mouse_last_point);
            move |ev, _| mouse_last_point.set(Some(Point::from_mouse_event(ev)))
        })
        .on_mousemove({
            clone!(mouse_last_point);
            move |ev, elem| {
                if let Some(start_point) = mouse_last_point.get() {
                    let ctx = context(&elem);

                    ctx.begin_path();
                    ctx.move_to(start_point.x as f64, start_point.y as f64);
                    ctx.line_to(ev.offset_x() as f64, ev.offset_y() as f64);
                    ctx.stroke();

                    mouse_last_point.set(Some(Point::from_mouse_event(ev)));
                }
            }
        })
        .on_touchstart({
            clone!(touch_last_point);
            move |ev, elem| {
                let touches = ev.changed_touches();
                touch_last_point.set(touches.item(0).map(Point::from_touch));

                draw_touches(&elem, touches, &touch_last_point, 1);
            }
        })
        .on_touchmove({
            clone!(touch_last_point);
            move |ev, elem| draw_touches(&elem, ev.changed_touches(), &touch_last_point, 0)
        });

    window::on_mouseup(move |_| mouse_last_point.set(None)).perpetual();
    window::on_touchcancel(move |_| touch_last_point.set(None)).perpetual();

    mount("app", cv);
}

fn draw_touches(
    canvas: &HtmlCanvasElement,
    touches: TouchList,
    touch_last_point: &Mutable<Option<Point>>,
    start_index: u32,
) {
    if let Some(start_point) = touch_last_point.get() {
        let ctx = context(canvas);
        ctx.begin_path();
        ctx.move_to(start_point.x as f64, start_point.y as f64);

        let mut index = start_index;

        while let Some(touch) = touches.item(index) {
            let point = Point::from_touch(touch);
            touch_last_point.set(Some(point));
            ctx.line_to(point.x as f64, point.y as f64);
            index += 1;
        }

        ctx.stroke();
    }
}

fn context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_touch(t: Touch) -> Self {
        Self {
            x: t.client_x(),
            y: t.client_y(),
        }
    }

    fn from_mouse_event(ev: MouseEvent) -> Self {
        Self {
            x: ev.offset_x(),
            y: ev.offset_y(),
        }
    }
}
