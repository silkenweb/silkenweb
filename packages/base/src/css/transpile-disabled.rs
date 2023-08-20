use super::{NameMapping, Source, Transpile};

pub struct Version {}

impl Version {
    pub fn new(_major: u8, _minor: u8, _patch: u8) -> Self {
        Self {}
    }
}

pub fn transpile(
    _source: &mut Source,
    _validate: bool,
    _transpile: Option<Transpile>,
) -> Result<Option<Vec<NameMapping>>, String> {
    Err("To use CSS transpilation, you must enable the \"css-transpile\" feature".to_string())
}
