use crate::list::get_sitemap::get_sitemap;
use crate::list::parse_sitemap::parse_sitemap;

mod get_sitemap;
mod parse_sitemap;

pub fn list() -> Vec<String> {
  let sitemap = match get_sitemap() {
    Ok(s) => s,
    Err(_) => return vec![],
  };

  parse_sitemap(&sitemap)
}