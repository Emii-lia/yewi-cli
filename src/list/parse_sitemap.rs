use regex::Regex;

pub(crate) fn parse_sitemap(sitemap: &str) -> Vec<String> {
  let re = Regex::new(r"<loc>https://yewi.fiaro.app/docs/([^<]+)</loc>").unwrap();
  re.captures_iter(sitemap)
    .map(|cap| cap[1].to_string().replace("-", "_"))
    .filter(|component| component != "installation" && component != "quick_start")
    .collect()
}