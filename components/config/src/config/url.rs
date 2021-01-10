use serde_derive::{Deserialize, Serialize};

/// The path of the rendered pages
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UrlMode {
    /// The default, each page is rendered in its own directory: "page_name/index.html"
    Directory,
    /// Pages are rendered in "page_name.html"
    PageName,
}

impl Default for UrlMode {
    fn default() -> Self {
        UrlMode::Directory
    }
}
