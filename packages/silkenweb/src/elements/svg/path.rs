use std::fmt::Write;

use crate::attribute::{AsAttribute, Attribute};

#[derive(Default)]
pub struct Data(String);

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        write!(&mut self.0, "M {x},{y}").unwrap();
        self
    }

    pub fn quadradic_bezier_curve_to(
        mut self,
        control_x: f64,
        control_y: f64,
        end_x: f64,
        end_y: f64,
    ) -> Self {
        write!(&mut self.0, "Q {control_x},{control_y} {end_x},{end_y}").unwrap();
        self
    }

    pub fn smooth_quadradic_bezier_curves_by(
        mut self,
        end_points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        write!(&mut self.0, "t").unwrap();

        for (end_x, end_y) in end_points.into_iter() {
            write!(&mut self.0, " {end_x},{end_y}").unwrap();
        }

        self
    }

    // TODO: Add methods for other path commands
}

impl AsAttribute<Data> for Data {}

impl Attribute for Data {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.0.as_str().into())
    }
}
