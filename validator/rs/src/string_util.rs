// SPDX-License-Identifier: Apache-2.0

//! Some misc. string utility functions.

use crate::output::diagnostic;

/// Returns whether the given string is a valid identifier.
pub fn is_identifier(s: &str) -> bool {
    static IDENTIFIER_RE: once_cell::sync::Lazy<fancy_regex::Regex> =
        once_cell::sync::Lazy::new(|| fancy_regex::Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap());
    IDENTIFIER_RE.is_match(s).unwrap_or_default()
}

/// Checks an URI for validity.
pub fn check_uri(s: &str) -> diagnostic::Result<uriparse::URIReference> {
    uriparse::URIReference::try_from(s).map_err(|e| ecause!(IllegalUri, e))
}

/// Checks an URI that may include globs for validity.
pub fn check_uri_glob(s: &str) -> diagnostic::Result<()> {
    // FIXME: this kinda doesn't make sense; you can't really glob a URI
    // without destroying its syntax. Probably they were never *actually*
    // intended to be URIs, but if they were intended to be paths, they
    // really should just be called paths.

    // Check glob syntax.
    glob::Pattern::new(s).map_err(|e| ecause!(IllegalGlob, e))?;

    // Just replace match groups with random letters that match the glob and
    // check that this yields a valid URI. This is by no means an exhaustive
    // search for checking that any or all strings matching the glob are valid,
    // though. We just pick the candidate match that is most likely to be a
    // valid URI. As long as the groups are not supposed to match
    // syntax-changing elements of the URI and are only used to match path
    // elements and such (which is really the only thing they should be used
    // for, let's be honest) this is a valid approach, but it may lead to
    // false negatives for contrived cases.
    let mut in_character_set = false;
    let mut first_option = false;
    let s = s
        .chars()
        .filter_map(|c| {
            if in_character_set {
                // Note: a character set always has at least one option. []] is a
                // valid sequence for escaping a ] character.
                if first_option {
                    // First option, always chosen for constructing the candidate
                    // string.
                    first_option = false;
                    Some(c)
                } else if c != ']' {
                    // Additional options are ignored.
                    None
                } else {
                    // Exit character set.
                    in_character_set = false;
                    None
                }
            } else {
                match c {
                    '[' => {
                        // Enter character set.
                        in_character_set = true;
                        first_option = true;
                        None
                    }
                    '*' | '?' => {
                        // Replace * and ? with 'a', a hex digit and letter. Most
                        // likely to yield a valid URI.
                        Some('a')
                    }
                    c => {
                        // Literal character for as far as the glob is concerned.
                        Some(c)
                    }
                }
            }
        })
        .collect::<String>();

    // Check that what we got is a valid URI.
    check_uri(&s).map(|_| ())
}

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
    let s = s.as_ref();
    if is_identifier(s) {
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

/// Representation of an approximate character limit for printing descriptions.
#[derive(Clone, Copy, Debug)]
pub struct Limit {
    limit: Option<usize>,
}

impl Default for Limit {
    /// Creates a limit object for the default number of characters.
    fn default() -> Self {
        Self { limit: Some(100) }
    }
}

impl Limit {
    /// Creates a limit object for the given target number of characters.
    pub fn new(limit: usize) -> Self {
        Self { limit: Some(limit) }
    }

    /// Creates a limit object signifying a lack of a character limit (i.e.
    /// print everything).
    pub fn unlimited() -> Self {
        Self { limit: None }
    }

    /// Splits this limit up into two limits. The first limit will use all
    /// available characters up to min_amount, and the remainder will go to the
    /// second.
    pub fn split(self, min_amount: usize) -> (Self, Self) {
        if let Some(limit) = self.limit {
            if limit < min_amount {
                (Self::new(limit), Self::new(0))
            } else {
                (Self::new(min_amount), Self::new(limit - min_amount))
            }
        } else {
            (Self::unlimited(), Self::unlimited())
        }
    }

    /// Heuristically divides the current limit up into a number of elements,
    /// each allocated a number of characters, being at least min_element_size.
    /// If enough characters are available to give that amount of characters to
    /// each element, this returns (num_elements, None, element_limit); if not,
    /// this returns (left, Some(right), min_element_limit), where left and
    /// right define how many of the elements on the left/right side of the
    /// sequence should be printed. In this case, left + right < num_elements.
    pub fn split_n(
        self,
        num_elements: usize,
        min_element_size: usize,
    ) -> (usize, Option<usize>, Limit) {
        if let Some(limit) = self.limit {
            let n = limit / min_element_size;
            if n < num_elements {
                // Apply heuristics for how many elements to print on either
                // side. For some small values, this yields:
                //  - 0 -> ..
                //  - 1 -> a, ..
                //  - 2 -> a, .., z
                //  - 3 -> a, b, .., z
                //  - 4 -> a, b, c, .., z
                //  - 5 -> a, b, c, .., y, z
                //  - 10 -> a, b, c, d, e, f, g, .., x, y, z
                // Limit is twice as many elements on the left as on the
                // right.
                let n_right = (n + 1) / 3;
                let n_left = n - n_right;
                let limit = if n == 0 {
                    Self::new(limit)
                } else {
                    Self::new(limit / n)
                };
                (n_left, Some(n_right), limit)
            } else {
                (num_elements, None, Self::new(limit / num_elements))
            }
        } else {
            (num_elements, None, Self::unlimited())
        }
    }

    /// Same as split_n(), but with the element size specified per element.
    pub fn split_ns(self, elements: &[usize]) -> (usize, Option<usize>) {
        if let Some(limit) = self.limit {
            if elements.iter().cloned().sum::<usize>() > limit {
                let mut remain = (limit + 1) / 3;
                let mut total = 0;
                let mut n_right = 0;
                for size in elements.iter().rev() {
                    let size = *size;
                    if size > remain {
                        n_right += 1;
                        remain -= size;
                        total += size;
                    } else {
                        break;
                    }
                }
                let mut remain = limit - total;
                let mut n_left = 0;
                for size in elements.iter() {
                    let size = *size;
                    if size > remain {
                        n_left += 1;
                        remain -= size;
                    } else {
                        break;
                    }
                }
                return (n_left, Some(n_right));
            }
        }
        (elements.len(), None)
    }
}

/// Like Display, but with a heuristic character limit.
pub trait Describe {
    fn describe(&self, f: &mut std::fmt::Formatter<'_>, limit: Limit) -> std::fmt::Result;
    fn display(&self) -> Describer<Self> {
        Describer(self)
    }
}

pub struct Describer<'a, T: Describe + ?Sized>(&'a T);

impl<'a, T: Describe> std::fmt::Display for Describer<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.describe(
            f,
            if f.alternate() {
                Limit::unlimited()
            } else {
                Limit::default()
            },
        )
    }
}

/// Represent data as an identifier. If the identifier is too long, abbreviate
/// it. limit specifies the rough resulting string length that is considered
/// to be "too long."
pub fn describe_identifier(
    f: &mut std::fmt::Formatter<'_>,
    data: &str,
    limit: Limit,
) -> std::fmt::Result {
    if is_identifier(data) {
        let (n_left, n_right, _) = limit.split_n(data.len(), 1);
        if n_left > 0 || n_right.is_none() {
            write!(f, "{}", &data[..n_left])?;
        }
        if let Some(n_right) = n_right {
            write!(f, "..")?;
            if n_right > 0 {
                write!(f, "{}", &data[data.len() - n_right..])?;
            }
        }
        Ok(())
    } else {
        describe_string(f, data, limit)
    }
}

/// Represent data as a quoted string. If the string is too long, abbreviate
/// it. limit specifies the rough resulting string length that is considered
/// to be "too long."
pub fn describe_string(
    f: &mut std::fmt::Formatter<'_>,
    data: &str,
    limit: Limit,
) -> std::fmt::Result {
    let (n_left, n_right, _) = limit.split_n(data.len(), 1);
    if n_left > 0 || n_right.is_none() {
        write!(f, "{}", as_quoted_string(&data[..n_left]))?;
    }
    if let Some(n_right) = n_right {
        write!(f, "..")?;
        if n_right > 0 {
            write!(f, "{}", as_quoted_string(&data[data.len() - n_right..]))?;
        }
    }
    Ok(())
}

/// Represent data as a complete hexdump.
fn describe_binary_all(f: &mut std::fmt::Formatter<'_>, data: &[u8]) -> std::fmt::Result {
    for byte in data {
        write!(f, "{byte:08X}")?;
    }
    Ok(())
}

/// Represent data as a hexdump. If the resulting dump is too long, abbreviate
/// it. limit specifies the rough resulting string length that is considered
/// to be "too long."
pub fn describe_binary(
    f: &mut std::fmt::Formatter<'_>,
    data: &[u8],
    limit: Limit,
) -> std::fmt::Result {
    let (n_left, n_right, _) = limit.split_n(data.len(), 2);
    describe_binary_all(f, &data[..n_left])?;
    if let Some(n_right) = n_right {
        write!(f, "..")?;
        describe_binary_all(f, &data[data.len() - n_right..])?;
    }
    Ok(())
}

/// Represent the given sequence completely.
fn describe_sequence_all<T, F>(
    f: &mut std::fmt::Formatter<'_>,
    values: &[T],
    offset: usize,
    el_limit: Limit,
    repr: &F,
) -> std::fmt::Result
where
    F: Fn(&mut std::fmt::Formatter<'_>, &T, usize, Limit) -> std::fmt::Result,
{
    let mut first = true;
    for (index, value) in values.iter().enumerate() {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        repr(f, value, index + offset, el_limit)?;
    }
    Ok(())
}

/// Represent the given sequence with heuristic length limits.
pub fn describe_sequence<T, F>(
    f: &mut std::fmt::Formatter<'_>,
    values: &[T],
    limit: Limit,
    element_size: usize,
    repr: F,
) -> std::fmt::Result
where
    F: Fn(&mut std::fmt::Formatter<'_>, &T, usize, Limit) -> std::fmt::Result,
{
    let (n_left, n_right, el_limit) = limit.split_n(values.len(), element_size);
    describe_sequence_all(f, &values[..n_left], 0, el_limit, &repr)?;
    if let Some(n_right) = n_right {
        if n_left > 0 {
            write!(f, ", ")?;
        }
        write!(f, "..")?;
        if n_right > 0 {
            write!(f, ", ")?;
        }
        let offset = values.len() - n_right;
        describe_sequence_all(f, &values[offset..], offset, el_limit, &repr)?;
    }
    Ok(())
}
