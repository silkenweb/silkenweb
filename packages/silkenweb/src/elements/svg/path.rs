use std::fmt::Write;

use crate::attribute::{AsAttribute, Attribute};

#[derive(Copy, Clone)]
pub enum Offset {
    Abs,
    Rel,
}

#[derive(Default)]
pub struct Data(String);

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    pub fn move_to(self, offset: Offset, x: f64, y: f64) -> Self {
        self.cmd2(offset, 'm', [(x, y)])
    }

    pub fn lines_to(self, offset: Offset, ends: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd2(offset, 'l', ends)
    }

    pub fn horizontal_lines_to(self, offset: Offset, ends: impl IntoIterator<Item = f64>) -> Self {
        self.cmd1(offset, 'h', ends)
    }

    pub fn vertical_lines_to(self, offset: Offset, ends: impl IntoIterator<Item = f64>) -> Self {
        self.cmd1(offset, 'v', ends)
    }

    pub fn cubic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd6(offset, 'c', args)
    }

    pub fn smooth_cubic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd4(offset, 's', args)
    }

    pub fn quadradic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd4(offset, 'q', args)
    }

    pub fn smooth_quadradic_bezier_curves(
        self,
        offset: Offset,
        end_points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        self.cmd2(offset, 't', end_points)
    }

    pub fn elliptical_arc_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd7(offset, 'a', args)
    }

    fn cmd1(self, offset: Offset, cmd: char, cmds: impl IntoIterator<Item = f64>) -> Self {
        self.cmd(offset, cmd, cmds.into_iter().map(|x| [x]))
    }

    fn cmd2(self, offset: Offset, cmd: char, cmds: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd(offset, cmd, cmds.into_iter().map(|(x0, x1)| [x0, x1]))
    }

    fn cmd4(
        self,
        offset: Offset,
        cmd: char,
        cmds: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd(
            offset,
            cmd,
            cmds.into_iter().map(|(x0, x1, x2, x3)| [x0, x1, x2, x3]),
        )
    }

    fn cmd6(
        self,
        offset: Offset,
        cmd: char,
        cmds: impl IntoIterator<Item = (f64, f64, f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd(
            offset,
            cmd,
            cmds.into_iter()
                .map(|(x0, x1, x2, x3, x4, x5)| [x0, x1, x2, x3, x4, x5]),
        )
    }

    fn cmd7(
        self,
        offset: Offset,
        cmd: char,
        cmds: impl IntoIterator<Item = (f64, f64, f64, f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd(
            offset,
            cmd,
            cmds.into_iter()
                .map(|(x0, x1, x2, x3, x4, x5, x6)| [x0, x1, x2, x3, x4, x5, x6]),
        )
    }

    fn cmd<const COUNT: usize>(
        mut self,
        offset: Offset,
        cmd: char,
        cmds: impl IntoIterator<Item = [f64; COUNT]>,
    ) -> Self {
        let cmd = match offset {
            Offset::Abs => cmd.to_ascii_uppercase(),
            Offset::Rel => cmd.to_ascii_lowercase(),
        };
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
