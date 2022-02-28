use std::{
    cell::RefCell,
    collections::HashSet,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use cssparser::{Parser, ParserInput, Token};
use grass::{Fs, OutputStyle, StdFs};

#[derive(Debug)]
struct DependencyTracker {
    dependencies: RefCell<HashSet<PathBuf>>,
    fs: StdFs,
}

impl DependencyTracker {
    fn new() -> Self {
        Self {
            dependencies: RefCell::new(HashSet::new()),
            fs: StdFs,
        }
    }
}

impl Fs for DependencyTracker {
    fn is_dir(&self, path: &Path) -> bool {
        self.fs.is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.fs.is_file(path)
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.dependencies
            .borrow_mut()
            .insert(fs::canonicalize(path)?);
        self.fs.read(path)
    }
}

pub fn class_names(
    filename: &str,
) -> Result<(impl Iterator<Item = PathBuf>, impl Iterator<Item = String>), Box<dyn Error>> {
    let dependency_tracker = DependencyTracker::new();
    let css = grass::from_path(
        filename,
        &grass::Options::default()
            .style(OutputStyle::Expanded)
            .quiet(true)
            .fs(&dependency_tracker),
    )?;

    let mut parser_input = ParserInput::new(&css);
    let mut input = Parser::new(&mut parser_input);
    let mut classes = HashSet::new();
    let mut prev_dot = false;

    while let Ok(token) = input.next() {
        if prev_dot {
            if let Token::Ident(class) = token {
                classes.insert(class.to_string());
            }
        }

        prev_dot = matches!(token, Token::Delim('.'));
    }

    Ok((
        dependency_tracker.dependencies.take().into_iter(),
        classes.into_iter(),
    ))
}
