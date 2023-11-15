use html::canvas;
use silkenweb::{clone, mount, prelude::*, window};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Touch, TouchEvent};

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
            move |ev, _| {
                ev.prevent_default();
                let touches = ev.target_touches();

                if touches.length() == 1 {
                    touch_last_point.set(touches.item(0).map(Point::from_touch));
                }
            }
        })
        .on_touchmove({
            clone!(touch_last_point);
            move |ev, elem| draw_touch(ev, &elem, &touch_last_point)
        });

    window::on_mouseup(move |_| mouse_last_point.set(None)).perpetual();
    window::on_touchcancel(move |_| touch_last_point.set(None)).perpetual();

    mount("app", cv);
}

fn draw_touch(
    touch_event: TouchEvent,
    canvas: &HtmlCanvasElement,
    touch_last_point: &Mutable<Option<Point>>,
) {
    touch_event.prevent_default();
    let touches = touch_event.target_touches();

    if touches.length() != 1 {
        return;
    }

    if let Some(start_point) = touch_last_point.get() {
        let end_point = Point::from_touch(touches.item(0).unwrap());

        let ctx = context(canvas);
        ctx.begin_path();
        ctx.move_to(start_point.x as f64, start_point.y as f64);
        ctx.line_to(end_point.x as f64, end_point.y as f64);
        ctx.stroke();

        touch_last_point.set(Some(end_point));
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
