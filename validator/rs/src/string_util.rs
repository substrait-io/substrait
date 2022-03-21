// SPDX-License-Identifier: Apache-2.0

//! Some misc. string utility functions.

/// Returns the given string as a quoted string.
pub fn as_quoted_string<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut result = String::with_capacity(s.len() + 2);
    result.push('"');
    for c in s.chars() {
        match c {
            '\\' => result += "\\\\",
            '"' => result += "\"",
            c => result.push(c),
        }
    }
    result.push('"');
    result
}

/// Returns the given string as-is if it's a valid identifier (i.e. if it
/// matches `[a-zA-Z_][a-zA-Z0-9_]*`), or returns it as an escaped string
/// otherwise, using (only) \" and \\ as escape sequences.
pub fn as_ident_or_string<S: AsRef<str>>(s: S) -> String {
    static IDENTIFIER_RE: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| regex::Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap());

    let s = s.as_ref();
    if IDENTIFIER_RE.is_match(s) {
        s.to_string()
    } else {
        as_quoted_string(s)
    }
}

/// Returns <n>th in English, using the correct suffix for the number.
pub fn describe_nth(index: u32) -> String {
    // Overkill? Yes. Couldn't help myself.
    match index {
        0 => String::from("zeroth"),
        1 => String::from("first"),
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("fourth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eighth"),
        9 => String::from("ninth"),
        10 => String::from("tenth"),
        11 => String::from("eleventh"),
        12 => String::from("twelfth"),
        13 => String::from("thirteenth"),
        14 => String::from("fourteenth"),
        15 => String::from("fifteenth"),
        16 => String::from("sixteenth"),
        17 => String::from("seventeenth"),
        18 => String::from("eighteenth"),
        19 => String::from("nineteenth"),
        20 => String::from("twentieth"),
        _ => match index % 10 {
            1 => format!("{index}st"),
            2 => format!("{index}nd"),
            3 => format!("{index}rd"),
            _ => format!("{index}th"),
        },
    }
}

/// Describes an index.
pub fn describe_index(index: i32) -> String {
    match index {
        i32::MIN..=-2 => format!("the {} to last", describe_nth(-index as u32)),
        -1 => String::from("the last"),
        0..=i32::MAX => format!("the {}", describe_nth((index + 1) as u32)),
    }
}
