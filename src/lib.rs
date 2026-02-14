use std::collections::HashMap;

use camino::Utf8PathBuf;

pub fn resolve(input: Input) -> Output {
    todo!()
}

pub struct Input {}

pub struct Output {
    /// Maps README file paths to new README contents
    readmes: HashMap<Utf8PathBuf, String>,
}
