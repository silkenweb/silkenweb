use super::{Css, NameMapping, Transpile, TranspileError};

pub struct Version {}

impl Version {
    pub fn new(_major: u8, _minor: u8, _patch: u8) -> Self {
        Self {}
    }
}

pub fn transpile(
    _source: &mut Css,
    _validate: bool,
    _transpile: Option<Transpile>,
) -> Result<Option<Vec<NameMapping>>, TranspileError> {
    Err(TranspileError::Disabled)
}
