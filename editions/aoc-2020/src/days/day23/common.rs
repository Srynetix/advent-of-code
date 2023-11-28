//! Common

/// Cup.
pub type Cup = usize;

/// Cups.
#[derive(Debug)]
pub struct Cups {
    head: Cup,
    data: Vec<Cup>,
}

impl Cups {
    /// Get next cup from `cup`.
    ///
    /// # Arguments
    ///
    /// * `cup` - Cup
    pub fn next(&self, cup: Cup) -> Cup {
        self.data[cup]
    }

    /// Set next cup link.
    ///
    /// # Arguments
    ///
    /// * `cup` - Source cup
    /// * `next` - Target cup
    pub fn set_next(&mut self, cup: Cup, next: Cup) {
        self.data[cup] = next;
    }

    /// Get previous cup.
    ///
    /// # Arguments
    ///
    /// * `cup` - Target cup
    pub fn prev_cup(&self, cup: Cup) -> Cup {
        let dest = cup - 1;
        if dest == 0 {
            self.data.len() - 1
        } else {
            dest
        }
    }

    /// Join cups to string, starting from cup one (and ignoring it).
    pub fn to_string_from_one(&self) -> String {
        cups_to_string(&self, 1)[1..].to_string()
    }
}

impl ToString for Cups {
    fn to_string(&self) -> String {
        cups_to_string(&self, self.head)
    }
}

/// Join cups to string, starting with `head`.
///
/// # Arguments
///
/// * `cups` - Cups
/// * `head` - Starting head
pub fn cups_to_string(cups: &Cups, head: Cup) -> String {
    let mut output = String::new();
    let mut cursor = head;
    output.push_str(&cursor.to_string());

    for _ in 1..cups.data.len() - 1 {
        cursor = cups.next(cursor);
        if cursor == head {
            break;
        }

        output.push_str(&cursor.to_string());
    }

    output
}

/// Parse cups.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_cups(input: &str) -> Cups {
    let mut output = Vec::new();
    let chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    output.resize(chars.len() + 1, 0);

    // Link each other
    for w in chars.windows(2) {
        output[w[0]] = w[1];
    }

    // Link last with first
    output[chars[chars.len() - 1]] = chars[0];

    Cups {
        head: chars[0],
        data: output,
    }
}

/// Prepare million cups.
///
/// # Arguments
///
/// * `input` - Input string
#[allow(clippy::needless_range_loop)]
pub fn prepare_million_cups(input: &str) -> Cups {
    let mut output = Vec::new();
    let chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    output.resize(1_000_000 + 1, 0);

    for w in chars.windows(2) {
        output[w[0]] = w[1];
    }

    // Now add millions
    for i in 10..=1_000_000 {
        output[i] = i + 1
    }

    // Link last char with 10
    output[chars[chars.len() - 1]] = 10;

    // Link last with first
    let output_len = output.len();
    output[output_len - 1] = chars[0];

    Cups {
        head: chars[0],
        data: output,
    }
}

/// Execute a game step.
///
/// # Arguments
///
/// * `cups` - Cups
pub fn game_step(cups: &mut Cups) {
    // Get current cursor
    let head = cups.head;

    // Get next three
    let t0 = cups.next(head);
    let t1 = cups.next(t0);
    let t2 = cups.next(t1);
    let next = cups.next(t2);

    // Get destination cup
    let mut dest = cups.prev_cup(head);
    while dest == t0 || dest == t1 || dest == t2 {
        dest = cups.prev_cup(dest);
    }

    // Update links
    cups.set_next(head, next);
    cups.set_next(t2, cups.next(dest));
    cups.set_next(dest, t0);
    cups.head = next;
}

/// Run `n` steps of simulation.
///
/// # Arguments
///
/// * `cups` - Cups
/// * `n` - Steps
pub fn run_steps(cups: &mut Cups, n: usize) {
    for _ in 0..n {
        game_step(cups);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "389125467";

    #[test]
    fn test_parse_cups() {
        let cups = parse_cups(SAMPLE);
        assert_eq!(cups.to_string(), SAMPLE);
    }

    #[test]
    fn test_10_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 10);
        assert_eq!(cups.to_string_from_one(), "92658374");
    }

    #[test]
    fn test_100_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 100);
        assert_eq!(cups.to_string_from_one(), "67384529");
    }

    #[test]
    fn test_million() {
        let mut cups = prepare_million_cups(SAMPLE);
        run_steps(&mut cups, 10_000_000);

        assert_eq!(cups.next(1), 934_001);
        assert_eq!(cups.next(cups.next(1)), 159_792);
    }
}
