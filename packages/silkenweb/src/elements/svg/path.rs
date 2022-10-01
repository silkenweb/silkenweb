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

    pub fn move_to(self, x: f64, y: f64) -> Self {
        self.cmd2('M', [(x, y)])
    }

    pub fn move_by(self, x: f64, y: f64) -> Self {
        self.cmd2('m', [(x, y)])
    }

    pub fn lines_to(self, ends: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd2('L', ends)
    }

    pub fn lines_by(self, ends: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd2('l', ends)
    }

    pub fn quadradic_bezier_curves_to(
        self,
        args: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd4('Q', args)
    }

    pub fn smooth_quadradic_bezier_curves_by(
        self,
        end_points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        self.cmd2('t', end_points)
    }

    // TODO: Add methods for other path commands

    fn cmd2(self, cmd: char, cmds: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd(cmd, cmds.into_iter().map(|(x0, x1)| [x0, x1]))
    }

    fn cmd4(self, cmd: char, cmds: impl IntoIterator<Item = (f64, f64, f64, f64)>) -> Self {
        self.cmd(
            cmd,
            cmds.into_iter().map(|(x0, x1, x2, x3)| [x0, x1, x2, x3]),
        )
    }

    fn cmd<const COUNT: usize>(
        mut self,
        cmd: char,
        cmds: impl IntoIterator<Item = [f64; COUNT]>,
    ) -> Self {
        let mut cmds = cmds.into_iter().peekable();

        if cmds.peek().is_some() {
            if !self.0.is_empty() {
                self.0.push(' ');
            }

            self.0.push(cmd);

            for args in cmds {
                let mut args = args.into_iter();
                write!(&mut self.0, " {}", args.next().unwrap()).unwrap();

                for arg in args {
                    write!(&mut self.0, ",{arg}").unwrap();
                }
            }
        }

        self
    }
}

impl AsAttribute<Data> for Data {}

impl Attribute for Data {
    fn text(&self) -> Option<std::borrow::Cow<str>> {
        Some(self.0.as_str().into())
    }
}
