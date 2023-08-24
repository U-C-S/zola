mod cli;
mod cmd;
mod messages;
mod prompt;

use std::path::Path;

use cmd::build;
use napi_derive::napi;

// #[derive(Debug, Clone, Serialize)]
// #[napi(object)]
// pub struct BuildParams<'a> {
//     pub root_dir: &'a Path,
//     pub config_file: &'a Path,
//     pub base_url: Option<&'a str>,
//     pub output_dir: Option<&'a Path>,
//     pub force: bool,
//     pub include_drafts: bool,
// }

#[napi]
pub fn zola_build(
    root_dir: String,
    config_file: String,
    base_url: Option<String>,
    output_dir: Option<String>,
    force: bool,
    include_drafts: bool,
) {
    let _ = build(
        Path::new(&root_dir),
        Path::new(&config_file),
        base_url.as_deref(),
        output_dir.as_ref().map(|t| Path::new(t.as_str())),
        force,
        include_drafts,
    );
}
