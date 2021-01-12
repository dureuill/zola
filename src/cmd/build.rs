use std::path::Path;

use errors::{Error, Result};
use site::Site;

use crate::console;

pub fn build(
    root_dir: &Path,
    config_file: &Path,
    base_url: Option<&str>,
    output_dir: Option<&Path>,
    include_drafts: bool,
    url_mode: Option<&str>,
) -> Result<()> {
    let mut site = Site::new(root_dir, config_file)?;
    if let Some(output_dir) = output_dir {
        site.set_output_path(output_dir);
    }
    if let Some(b) = base_url {
        site.set_base_url(b.to_string());
    }
    if let Some(url_mode) = url_mode {
        site.set_url_mode(url_mode)
            .map_err(|e| Error::chain("invalid value for argument '--url-mode'", e))?;
    }
    if include_drafts {
        site.include_drafts();
    }
    site.load()?;
    console::notify_site_size(&site);
    console::warn_about_ignored_pages(&site);
    site.build()
}
