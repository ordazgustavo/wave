use console::style;

pub fn success(message: &str) -> String {
    format!("{} {}", style("success").green(), message)
}
