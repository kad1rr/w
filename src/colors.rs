pub struct Colors {}

impl Colors {
    pub fn yellow_bg(message: &str) -> String {
        format!("\x1b[43;30m{}\x1b[0m", message)
    }

    pub fn yellow(message: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", message)
    }

    pub fn blue_bg(message: &str) -> String {
        format!("\x1b[44m{}\x1b[0m", message)
    }

    pub fn blue(message: &str) -> String {
        format!("\x1b[34m{}\x1b[0m", message)
    }
}
