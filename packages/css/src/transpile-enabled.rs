use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Context, Error};
use clonelet::clone;
use itertools::Itertools;
use lightningcss::{
    css_modules::{self, CssModuleExport},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Features, Targets},
};

use super::{Browsers, Css, NameMapping, Transpile};
use crate::TranspileError;

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    fn encode_for_lightning(self) -> u32 {
        u32::from_be_bytes([0, self.major, self.minor, self.patch])
    }
}

impl From<Browsers> for lightningcss::targets::Browsers {
    fn from(value: Browsers) -> Self {
        Self {
            android: value.android.map(Version::encode_for_lightning),
            chrome: value.chrome.map(Version::encode_for_lightning),
            edge: value.edge.map(Version::encode_for_lightning),
            firefox: value.firefox.map(Version::encode_for_lightning),
            ie: value.ie.map(Version::encode_for_lightning),
            ios_saf: value.ios_saf.map(Version::encode_for_lightning),
            opera: value.opera.map(Version::encode_for_lightning),
            safari: value.safari.map(Version::encode_for_lightning),
            samsung: value.samsung.map(Version::encode_for_lightning),
        }
    }
}

pub fn transpile(
    source: &mut Css,
    validate: bool,
    transpile: Option<Transpile>,
) -> Result<Option<Vec<NameMapping>>, TranspileError> {
    let modules = transpile.as_ref().is_some_and(|t| t.modules);
    let nesting = transpile.as_ref().is_some_and(|t| t.nesting);

    clone!(source.content);
    let warnings = validate.then(|| Arc::new(RwLock::new(Vec::new())));
    let filename = source
        .dependency
        .as_ref()
        .map_or_else(|| "<content>".to_string(), String::clone);
    let css_modules = modules.then(|| css_modules::Config {
        pattern: css_modules::Pattern::default(),
        dashed_idents: false,
    });
    let mut flags = ParserFlags::empty();
    flags.set(ParserFlags::NESTING, nesting);

    let mut stylesheet: StyleSheet = StyleSheet::parse(
        &content,
        ParserOptions {
            filename,
            css_modules,
            source_index: 0,
            error_recovery: !validate,
            warnings: warnings.clone(),
            flags,
        },
    )
    .map_err(|e| anyhow!("Parsing failed: {e}"))?;

    if let Some(warnings) = warnings {
        let warnings = warnings.read().unwrap();

        if !warnings.is_empty() {
            let warnings_text = warnings.iter().map(|w| w.to_string()).join("\n");

            return Err(Error::msg(warnings_text).into());
        }
    }

    if let Some(Transpile {
        minify,
        pretty,
        browsers,
        ..
    }) = transpile
    {
        let targets = Targets {
            browsers: browsers.map(Browsers::into),
            include: Features::empty(),
            exclude: Features::empty(),
        };

        if minify {
            // This does the structural minification and add/removes vendor prefixes.
            stylesheet
                .minify(MinifyOptions {
                    targets,
                    unused_symbols: HashSet::new(),
                })
                .context("Minify failed")?;
        }

        let css = stylesheet
            .to_css(PrinterOptions {
                // `minify` just controls the output format without doing more structural
                // minification.
                minify: !pretty && minify,
                source_map: None,
                project_root: None,
                targets,
                analyze_dependencies: None,
                pseudo_classes: None,
            })
            .context("CSS printing failed")?;
        source.content = css.code;

        return css.exports.map(name_mappings).transpose();
    }

    Ok(None)
}

fn name_mappings(
    exports: HashMap<String, CssModuleExport>,
) -> Result<Vec<NameMapping>, TranspileError> {
    exports
        .into_iter()
        .map(|(plain, mapping)| {
            if mapping.composes.is_empty() {
                Ok(NameMapping {
                    plain,
                    mangled: mapping.name,
                })
            } else {
                Err(Error::msg("`composes` is unsupported").into())
            }
        })
        .collect()
}
