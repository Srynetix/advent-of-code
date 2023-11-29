//! Part 2

use aoc_sx::{algo::fs::get_debug_path, indoc::indoc};

use super::{
    common::{save_paper_to_disk, TransparentPaper},
    INPUT,
};

pub fn run() -> String {
    let mut paper = TransparentPaper::from(INPUT);
    let path = get_debug_path().join("aoc2021-day13.png");

    while let Some(p) = paper.fold_next_rule() {
        paper = p;
    }

    save_paper_to_disk(&paper, &path);

    // Yes, answer is hardcoded, because I don't have time to apply OCR on the output. 😅
    assert_eq!(
        paper.to_string(),
        indoc! {"
        ..##.###..####..##..#..#..##..#..#.###..
        ...#.#..#....#.#..#.#..#.#..#.#..#.#..#.
        ...#.#..#...#..#....#..#.#..#.#..#.#..#.
        ...#.###...#...#....#..#.####.#..#.###..
        #..#.#....#....#..#.#..#.#..#.#..#.#.#..
        .##..#....####..##...##..#..#..##..#..#."
        }
    );

    "JPZCUAUR".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), "JPZCUAUR")
    }
}
