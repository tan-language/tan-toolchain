use tan::error::{Error, ErrorNote, ErrorVariant};

// #todo reuse the Position from tan?
// #todo split into `format_expr`, `format_error`.
// #todo add special support for formatting multiple errors?

pub fn format_error_note_pretty(note: &ErrorNote, input: &str, line_indent: &str) -> String {
    let Some(range) = &note.range else {
        return format!("{line_indent}= note: {}", note.text);
    };

    // #insight
    // Currently because of wrong input the line may be missing, so we cannot
    // unwrap.
    let line_str = input.lines().nth(range.start.line).unwrap_or("???");

    let len = range.end.index - range.start.index;

    // #todo use `^` or `-` depending on note importance, like Rust.

    let indicator = "^".repeat(len);

    // The indent of the annotation indicator (arrow).
    let indicator_indent = " ".repeat(range.start.col);

    // #todo trim the leading spaces from the line?

    format!(
        "{}|\n{}| {}\n{}|{} {} {}",
        line_indent,
        range.start.line + 1,
        line_str,
        line_indent,
        indicator_indent,
        indicator,
        note.text,
    )
}

#[allow(dead_code)]
pub fn format_error(error: &Error) -> String {
    format!("{} at {}", error.variant(), error.file_path)
}

// #todo don't print <input>
// #todo reuse this in format_error_pretty.
pub fn format_error_short(error: &Error) -> String {
    if let Some(note) = error.notes.first() {
        if let Some(range) = &note.range {
            let position = &range.start;
            return format!(
                "{}\n at {}:{}:{}",
                error.variant(),
                error.file_path,
                position.line + 1,
                position.col + 1,
            );
        }
    }
    format!("{}", error.variant())
}

// #todo don't print <input>:line:col, just line:col.
// #todo also format error without input.
// #todo implement this in ...Tan :)
// #todo format the error as symbolic expression.
// #todo format the error as JSON.
// #todo make more beautiful than Rust.
// #todo add as method to Ranged<E: Error>? e.g. `format_pretty`
pub fn format_error_pretty(error: &Error, input: &str) -> String {
    // if matches!(error.variant, ErrorVariant::Panic(..)) {
    //     return format_panic_pretty(error);
    // }

    // #todo find another name for main_note.
    // #todo potentially drinks errors.
    let Some(main_note) = error.notes.first() else {
        return format!("{}\n at {}", error.variant(), error.file_path);
    };

    let prologue = if let Some(range) = &main_note.range {
        let position = &range.start;
        format!(
            "{}\n at {}:{}:{}",
            error.variant(),
            error.file_path,
            position.line + 1,
            position.col + 1,
        )
    } else {
        format!("{}\n at {}", error.variant(), error.file_path)
    };

    // compute notes indent.
    let indent = error
        .notes
        .iter()
        .filter(|n| n.range.is_some())
        .fold(0usize, |acc, n| {
            n.range.as_ref().unwrap().start.line.max(acc)
        });
    let indent = " ".repeat(format!("{}", indent + 1).len());

    let notes: Vec<String> = error
        .notes
        .iter()
        .map(|note| format_error_note_pretty(note, input, &indent))
        .collect();

    // #todo special handling of errors!

    // match error.variant() {
    //     ErrorVariant::FailedUse(_, upstream_errors) => {
    //         println!(".....1");
    //         let mut infos = Vec::new();
    //         for ue in upstream_errors {
    //             // #todo ideally should pretty-print the source errors and provide input!
    //             println!(".....2");
    //             infos.push(format_error_short(ue))
    //         }
    //         format!("{prologue}\n{}\n{}", infos.join("\n"), notes.join("\n"))
    //     }
    //     _ => {
    //         format!("{prologue}\n{}", notes.join("\n"))
    //     }
    // }

    format!("{prologue}\n{}", notes.join("\n"))
}

pub fn format_panic_pretty(error: &Error) -> String {
    // #todo cleanup.

    let ErrorVariant::Panic(ref msg) = error.variant else {
        // should never happen.
        panic!("we need to go deeper");
    };

    let Some(note) = error.notes.first() else {
        // should never happen.
        panic!("we need to go deeper");
    };

    let range = note.range.as_ref().unwrap();

    format!(
        "{}\n at {}:{}:{}",
        msg,
        error.file_path,
        range.start.line + 1,
        range.start.col + 1
    )
}
