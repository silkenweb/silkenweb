//! Tools to construct an SVG path.
use std::fmt::Write;

use silkenweb_signals_ext::value::Value;

use crate::attribute::{AsAttribute, Attribute};

/// Absolute or relative indicator for a path component.
#[derive(Copy, Clone)]
pub enum Offset {
    Abs,
    Rel,
}

/// Construct path data for the [`d`] attribute
///
/// # Example
///
/// ```
/// # use silkenweb::{dom::Dry, prelude::*};
/// # use svg::{
/// #     attributes::Presentation,
/// #     path,
/// #     path::{Data, Offset::Abs},
/// #     Path,
/// # };
/// let path: Path<Dry> = path().d(Data::new()
///     .move_to(Abs, 10.0, 10.0)
///     .lines_to(Abs, [(20.0, 20.0), (30.0, 30.0)]));
///
/// assert_eq!(
///     r#"<path d="M 10,10 L 20,20 30,30"></path>"#,
///     path.freeze().to_string()
/// );
/// ```
/// [`d`]: `super::attributes::Presentation::d`
#[derive(Default, Clone)]
pub struct Data(String);

impl Data {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    /// The `m/M` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#moveto_path_commands
    pub fn move_to(self, offset: Offset, x: f64, y: f64) -> Self {
        self.cmd2(offset, 'm', [(x, y)])
    }

    /// The `l/L` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#lineto_path_commands
    pub fn lines_to(self, offset: Offset, ends: impl IntoIterator<Item = (f64, f64)>) -> Self {
        self.cmd2(offset, 'l', ends)
    }

    /// The `h/H` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#lineto_path_commands
    pub fn horizontal_lines_to(self, offset: Offset, ends: impl IntoIterator<Item = f64>) -> Self {
        self.cmd1(offset, 'h', ends)
    }

    /// The `v/V` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#lineto_path_commands
    pub fn vertical_lines_to(self, offset: Offset, ends: impl IntoIterator<Item = f64>) -> Self {
        self.cmd1(offset, 'v', ends)
    }

    /// The `c/C` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#cubic_b%C3%A9zier_curve
    pub fn cubic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd6(offset, 'c', args)
    }

    /// The `s/S` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#cubic_b%C3%A9zier_curve
    pub fn smooth_cubic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd4(offset, 's', args)
    }

    /// The `q/Q` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#quadratic_b%C3%A9zier_curve
    pub fn quadradic_bezier_curves(
        self,
        offset: Offset,
        args: impl IntoIterator<Item = (f64, f64, f64, f64)>,
    ) -> Self {
        self.cmd4(offset, 'q', args)
    }

    /// The `t/T` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#quadratic_b%C3%A9zier_curve
    pub fn smooth_quadradic_bezier_curves(
        self,
        offset: Offset,
        end_points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        self.cmd2(offset, 't', end_points)
    }

    /// The `a/A` commands ([MDN Docs])
    ///
    /// [MDN Docs]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#elliptical_arc_curve
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
impl AsAttribute<Data> for String {}
impl<'a> AsAttribute<Data> for &'a str {}

impl Attribute for Data {
    type Text<'a> = &'a str;

    fn text(&self) -> Option<Self::Text<'_>> {
        Some(self.0.as_str())
    }
}

impl Value for Data {}
