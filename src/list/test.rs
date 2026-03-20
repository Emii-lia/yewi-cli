use crate::list::parse_sitemap::parse_sitemap;

#[test]
fn parse_sitemap_returns_correct_components() {
  let sitemap = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
      <url>
        <loc>https://yewi.fiaro.app/docs/button</loc>
      </url>
      <url>
        <loc>https://yewi.fiaro.app/docs/input</loc>
      </url>
      <url>
        <loc>https://yewi.fiaro.app/docs/installation</loc>
      </url>
      <url>
        <loc>https://yewi.fiaro.app/docs/quick-start</loc>
      </url>
    </urlset>
  "#;

  let components = parse_sitemap(sitemap);
  assert_eq!(components, vec!["button", "input"]);
}

#[test]
fn parse_sitemap_returns_empty_vec_if_sitemap_is_empty() {
  let sitemap = "";
  assert_eq!(parse_sitemap(sitemap), Vec::<String>::new());
}

#[test]
fn parse_sitemap_returns_empty_vec_if_sitemap_is_invalid() {
  let sitemap = "invalid sitemap";
  assert_eq!(parse_sitemap(sitemap), Vec::<String>::new());
}