use colored::Colorize;

pub fn colorize(label: &str, fg: &str, bg: &str) -> String {
  format!("{}", label.color(fg).on_color(bg))
}