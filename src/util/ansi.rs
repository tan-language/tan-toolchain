// #insight renamed to ansi from color as it contains general styling functions, not only color-related.

// #ref https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

#[allow(dead_code)]
pub fn bold(s: impl AsRef<str>) -> String {
    format!("\x1b[1m{}\x1b[0m", s.as_ref())
}

pub fn red(s: impl AsRef<str>) -> String {
    format!("\x1b[31m{}\x1b[0m", s.as_ref())
}

pub fn green(s: impl AsRef<str>) -> String {
    format!("\x1b[32m{}\x1b[0m", s.as_ref())
}
