//! Common

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

static PASSWORD_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)").unwrap());

/// Validate multiple passwords with count.
///
/// # Arguments
///
/// * `entries` - Input text
pub fn validate_multiple_passwords_with_count(entries: &str) -> usize {
    validate_multiple_passwords_with_fn(entries, validate_password_with_count)
}

/// Validate multiple passwords with character position.
///
/// # Arguments
///
/// * `entries` - Input text
pub fn validate_multiple_passwords_with_position(entries: &str) -> usize {
    validate_multiple_passwords_with_fn(entries, validate_password_with_position)
}

/// Validate multiple passwords using function.
///
/// # Arguments
///
/// * `entries` - Input text
/// * `func` - Function
pub fn validate_multiple_passwords_with_fn<F>(entries: &str, func: F) -> usize
where
    F: Fn(&str) -> bool,
{
    entries
        .lines()
        .filter_map(|s| if func(s) { Some(true as usize) } else { None })
        .sum::<usize>()
}

/// Validate password with count.
///
/// # Arguments
///
/// * `entry` - Password
pub fn validate_password_with_count(entry: &str) -> bool {
    let (min_v, max_v, char_v, password) = parse_password_entry(entry);
    let count = password.chars().filter(|c| *c == char_v).count();
    count <= max_v && count >= min_v
}

/// Validate password with character position.
///
/// # Arguments
///
/// * `entry` - Password
pub fn validate_password_with_position(entry: &str) -> bool {
    let (min_v, max_v, char_v, password) = parse_password_entry(entry);
    let bytes = password.as_bytes();
    let char_b = char_v as u8;
    let char_min = bytes[min_v - 1];
    let char_max = bytes[max_v - 1];

    if char_min == char_b && char_max == char_b {
        false
    } else if char_min == char_b {
        true
    } else {
        char_max == char_b
    }
}

/// Parse password entry.
///
/// # Arguments
///
/// * `entry` - Password
pub fn parse_password_entry(entry: &str) -> (usize, usize, char, &str) {
    let captures = PASSWORD_RGX.captures(entry).unwrap();

    (
        captures
            .name("min")
            .map(|x| x.as_str().parse().unwrap())
            .unwrap(),
        captures
            .name("max")
            .map(|x| x.as_str().parse().unwrap())
            .unwrap(),
        captures
            .name("char")
            .map(|x| x.as_str().parse().unwrap())
            .unwrap(),
        captures.name("password").map(|x| x.as_str()).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_password_entry() {
        assert_eq!(
            parse_password_entry("1-3 c: tototutu"),
            (1, 3, 'c', "tototutu")
        );
        assert_eq!(
            parse_password_entry("10-30 z: zzzzzz"),
            (10, 30, 'z', "zzzzzz")
        );
    }

    #[test]
    fn test_validate_password_with_count() {
        assert!(validate_password_with_count("1-3 c: ceci"));
        assert!(!validate_password_with_count("1-3 c: cccc"));
    }

    #[test]
    fn test_validate_password_with_position() {
        assert!(validate_password_with_position("1-3 c: cabc"));
        assert!(!validate_password_with_position("1-3 c: cacc"));
        assert!(validate_password_with_position("1-3 c: aacc"));
    }
}
