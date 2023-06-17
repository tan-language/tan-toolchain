use tan::error::Error;
use tan_fmt::format_error_pretty;

#[allow(dead_code)]
pub fn format_errors(errors: &[Error]) -> String {
    let mut error_strings = Vec::new();

    for error in errors {
        error_strings.push(format!("ERROR: {}", format_error_pretty(&error)));
    }

    format!("{}", error_strings.join("\n\n"))
}
