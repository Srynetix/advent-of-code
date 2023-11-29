//! Common

/// Boarding pass.
#[derive(Debug, PartialEq, Eq)]
pub struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    /// Convert entries to boarding passes.
    ///
    /// # Arguments
    ///
    /// * `entries` - Boarding entries
    pub fn from_entries(entries: &str) -> Vec<Self> {
        entries.lines().map(Self::from_entry).collect()
    }

    /// Convert entry to boarding pass.
    ///
    /// # Arguments
    ///
    /// * `entry` - Boarding entry
    pub fn from_entry(entry: &str) -> Self {
        // Convert row as binary to decimal (7 first letters)
        let row = usize::from_str_radix(
            &entry
                .chars()
                .take(7)
                .map(|x| match x {
                    'F' => '0',
                    'B' => '1',
                    e => panic!("Unknown letter {}", e),
                })
                .collect::<String>(),
            2,
        )
        .unwrap();

        // Convert column as binary to decimal (3 last letters)
        let column = usize::from_str_radix(
            &entry
                .chars()
                .skip(7)
                .take(3)
                .map(|x| match x {
                    'L' => '0',
                    'R' => '1',
                    e => panic!("Unknown letter {}", e),
                })
                .collect::<String>(),
            2,
        )
        .unwrap();

        Self { row, column }
    }

    /// Get seat ID from boarding pass.
    pub const fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_entry() {
        assert_eq!(
            BoardingPass::from_entry("FBFBBFFRLR"),
            BoardingPass { row: 44, column: 5 }
        );
        assert_eq!(
            BoardingPass::from_entry("BFFFBBFRRR"),
            BoardingPass { row: 70, column: 7 }
        );
        assert_eq!(
            BoardingPass::from_entry("FFFBBBFRRR"),
            BoardingPass { row: 14, column: 7 }
        );
        assert_eq!(
            BoardingPass::from_entry("BBFFBBFRLL"),
            BoardingPass {
                row: 102,
                column: 4
            }
        );
    }

    #[test]
    fn test_get_seat_id() {
        assert_eq!(BoardingPass { row: 44, column: 5 }.get_seat_id(), 357);
        assert_eq!(BoardingPass { row: 70, column: 7 }.get_seat_id(), 567);
        assert_eq!(BoardingPass { row: 14, column: 7 }.get_seat_id(), 119);
        assert_eq!(
            BoardingPass {
                row: 102,
                column: 4
            }
            .get_seat_id(),
            820
        );
    }
}
