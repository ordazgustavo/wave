use console::style;

pub fn success(message: &str) -> String {
    format!("{} {}", style("success").green(), message)
}

pub fn warning(message: &str) -> String {
    format!("{} {}", style("warning").yellow(), message)
}
