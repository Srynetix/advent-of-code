//! Common

/// Handle map state, with an empty cell (`.`, or a tree `#`)
pub enum MapCell {
    /// Empty cell: `.`
    Empty,
    /// Tree cell: `#`
    Tree,
}

impl MapCell {
    /// Try to convert char to `MapCell`.
    ///
    /// # Arguments
    ///
    /// * `c` - Character
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Tree,
            o => panic!("Bad character {}", o),
        }
    }
}

/// Handle toboggan map data
pub struct TobogganMap {
    data: Vec<MapCell>,
    width: usize,
}

impl TobogganMap {
    /// Get map cell at `x` and `y` position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn get_xy(&self, x: usize, y: usize) -> Option<&MapCell> {
        let x = x % self.width;
        let pos = x + y * self.width;

        if pos < self.data.len() {
            Some(&self.data[pos])
        } else {
            None
        }
    }

    /// Follow slope using `offset_x` and `offset_y` offsets.
    ///
    /// # Arguments
    ///
    /// * `offset_x` - X offset
    /// * `offset_y` - Y offset
    pub fn follow_slope(&self, offset_x: usize, offset_y: usize) -> usize {
        let (mut x, mut y, mut tree_counter) = (0, 0, 0);
        while let Some(cell) = self.get_xy(x, y) {
            if let MapCell::Tree = cell {
                tree_counter += 1;
            }

            x += offset_x;
            y += offset_y;
        }

        tree_counter
    }

    /// Parse toboggan map from input text.
    ///
    /// # Arguments
    ///
    /// * `input` - Input text
    pub fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();

        let data: Vec<MapCell> = input
            .lines()
            .flat_map(|l| l.chars().map(MapCell::from_char))
            .collect();

        Self { data, width }
    }
}
