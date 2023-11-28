//! Common

/// Seat state
#[derive(Debug, Copy, Clone)]
pub enum SeatState {
    /// No seat
    Floor,
    /// Free seat
    Free,
    /// Occupied seat
    Occupied,
}

impl SeatState {
    /// Creates seat state from character.
    ///
    /// # Arguments
    ///
    /// * `character` - Character
    pub fn from_char(character: char) -> Self {
        match character {
            '.' => Self::Floor,
            'L' => Self::Free,
            '#' => Self::Occupied,
            e => panic!("Bad seat state character: {}", e),
        }
    }

    /// Convert state to character.
    pub const fn to_char(self) -> char {
        match self {
            Self::Floor => '.',
            Self::Free => 'L',
            Self::Occupied => '#',
        }
    }
}

/// Seat layout stats
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SeatLayoutStats {
    /// Total seats
    pub total_seats: usize,
    /// Free seats
    pub free_seats: usize,
    /// Occupied seats
    pub occupied_seats: usize,
}

impl SeatLayoutStats {
    /// Creates empty stats.
    pub const fn new_empty() -> Self {
        Self {
            total_seats: 0,
            free_seats: 0,
            occupied_seats: 0,
        }
    }
}

/// Seat layout
#[derive(Debug)]
pub struct SeatLayout {
    frontbuffer: Vec<Vec<SeatState>>,
    backbuffer: Vec<Vec<SeatState>>,
}

impl SeatLayout {
    /// Creates seat layout from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        let data = input
            .lines()
            .map(|x| {
                x.trim()
                    .chars()
                    .map(SeatState::from_char)
                    .collect::<Vec<SeatState>>()
            })
            .collect::<Vec<Vec<SeatState>>>();

        Self {
            frontbuffer: data.clone(),
            backbuffer: data,
        }
    }

    /// Get layout size
    pub fn get_size(&self) -> (usize, usize) {
        (self.frontbuffer[0].len(), self.frontbuffer.len())
    }

    /// Update state using neighbors.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    /// * `old_state` - Old seat state
    fn update_state_from_neighbors(&self, x: usize, y: usize, old_state: SeatState) -> SeatState {
        match old_state {
            SeatState::Floor => old_state,
            SeatState::Free => {
                if self.count_neighbors_occupied_seats(x, y) == 0 {
                    SeatState::Occupied
                } else {
                    old_state
                }
            }
            SeatState::Occupied => {
                if self.count_neighbors_occupied_seats(x, y) >= 4 {
                    SeatState::Free
                } else {
                    old_state
                }
            }
        }
    }

    /// Update state using visibility.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    /// * `old_state` - Old seat state
    pub fn update_state_from_visibility(
        &self,
        x: usize,
        y: usize,
        old_state: SeatState,
    ) -> SeatState {
        match old_state {
            SeatState::Floor => old_state,
            SeatState::Free => {
                if self.count_visible_occupied_seats(x, y) == 0 {
                    SeatState::Occupied
                } else {
                    old_state
                }
            }
            SeatState::Occupied => {
                if self.count_visible_occupied_seats(x, y) >= 5 {
                    SeatState::Free
                } else {
                    old_state
                }
            }
        }
    }

    /// Swap front and back buffers.
    pub fn swap_buffers(&mut self) {
        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                self.frontbuffer[j][i] = self.backbuffer[j][i];
            }
        }
    }

    /// Step simulation.
    pub fn step(&mut self) -> SeatLayoutStats {
        let mut total_seats = 0;
        let mut free_seats = 0;
        let mut occupied_seats = 0;

        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                let old_state = self.frontbuffer[j][i];
                let new_state = self.update_state_from_neighbors(i, j, old_state);
                self.backbuffer[j][i] = new_state;

                match new_state {
                    SeatState::Floor => (),
                    SeatState::Free => {
                        total_seats += 1;
                        free_seats += 1;
                    }
                    SeatState::Occupied => {
                        total_seats += 1;
                        occupied_seats += 1;
                    }
                }
            }
        }

        self.swap_buffers();

        SeatLayoutStats {
            total_seats,
            free_seats,
            occupied_seats,
        }
    }

    /// Step simulation with visibility check.
    pub fn step_with_visibility(&mut self) -> SeatLayoutStats {
        let mut total_seats = 0;
        let mut free_seats = 0;
        let mut occupied_seats = 0;

        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                let old_state = self.frontbuffer[j][i];
                let new_state = self.update_state_from_visibility(i, j, old_state);
                self.backbuffer[j][i] = new_state;

                match new_state {
                    SeatState::Floor => (),
                    SeatState::Free => {
                        total_seats += 1;
                        free_seats += 1;
                    }
                    SeatState::Occupied => {
                        total_seats += 1;
                        occupied_seats += 1;
                    }
                }
            }
        }

        self.swap_buffers();

        SeatLayoutStats {
            total_seats,
            free_seats,
            occupied_seats,
        }
    }

    /// Run steps until the simulation is stable.
    pub fn run_until_stable(&mut self) -> SeatLayoutStats {
        let mut last_stats = SeatLayoutStats::new_empty();

        loop {
            let new_stats = self.step();
            if new_stats == last_stats {
                return new_stats;
            }
            last_stats = new_stats;
        }
    }

    /// Run steps with visibility until the simulation is stable.
    pub fn run_with_visibility_until_stable(&mut self) -> SeatLayoutStats {
        let mut last_stats = SeatLayoutStats::new_empty();

        loop {
            let new_stats = self.step_with_visibility();
            if new_stats == last_stats {
                return new_stats;
            }
            last_stats = new_stats;
        }
    }

    /// Count neighbors occupied seats for position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn count_neighbors_occupied_seats(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    let (nx, ny) = ((x as isize + i) as usize, (y as isize + j) as usize);
                    if let Some(SeatState::Occupied) = self.get_seat_state_at_position(nx, ny) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Count visible occupied seats for position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn count_visible_occupied_seats(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    let (mut nx, mut ny) = ((x as isize + i) as usize, (y as isize + j) as usize);
                    loop {
                        match self.get_seat_state_at_position(nx, ny) {
                            Some(SeatState::Occupied) => {
                                // Stop scan and count
                                count += 1;
                                break;
                            }
                            Some(SeatState::Floor) => {
                                // Continue to scan
                            }
                            None | Some(SeatState::Free) => {
                                // Stop scan
                                break;
                            }
                        }

                        nx = (nx as isize + i) as usize;
                        ny = (ny as isize + j) as usize;
                    }
                }
            }
        }

        count
    }

    /// Get seat state at position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn get_seat_state_at_position(&self, x: usize, y: usize) -> Option<SeatState> {
        let (w, h) = self.get_size();
        if x < w && y < h {
            Some(self.frontbuffer[y][x])
        } else {
            None
        }
    }

    /// Write layout to string
    pub fn write_to_string(&self) -> String {
        let (w, h) = self.get_size();

        let mut output = String::new();

        for j in 0..h {
            for i in 0..w {
                let c = self.frontbuffer[j][i];
                output.push(c.to_char());
            }
            output.push('\n');
        }

        output
    }

    /// Show layout
    pub fn show(&self) {
        println!("{}", self.write_to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_LAYOUT: &str = r###"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL"###;

    const SAMPLE_SCAN_1: &str = r###".......#.
    ...#.....
    .#.......
    .........
    ..#L....#
    ....#....
    .........
    #........
    ...#....."###;

    const SAMPLE_SCAN_2: &str = r###".............
    .L.L.#.#.#.#.
    ............."###;

    const SAMPLE_SCAN_3: &str = r###".##.##.
    #.#.#.#
    ##...##
    ...L...
    ##...##
    #.#.#.#
    .##.##."###;

    #[test]
    fn test_layout_parse() {
        let layout = SeatLayout::from_input(SAMPLE_LAYOUT);
        assert_eq!(layout.get_size(), (10, 10));
    }

    #[test]
    fn test_step() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT);

        let stats = layout.step();
        assert_eq!(stats.free_seats, 0);
        assert_eq!(stats.occupied_seats, stats.total_seats);
    }

    #[test]
    fn test_run_until_stable() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT);
        let stats = layout.run_until_stable();

        assert_eq!(stats.occupied_seats, 37);
    }

    #[test]
    fn test_run_with_visibility_until_stable() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT);
        let stats = layout.run_with_visibility_until_stable();

        assert_eq!(stats.occupied_seats, 26);
    }

    #[test]
    fn test_scan_1() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_1);
        assert_eq!(layout.count_visible_occupied_seats(3, 4), 8);
    }

    #[test]
    fn test_scan_2() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_2);
        assert_eq!(layout.count_visible_occupied_seats(1, 1), 0);
        assert_eq!(layout.count_visible_occupied_seats(3, 1), 1);
    }

    #[test]
    fn test_scan_3() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_3);
        assert_eq!(layout.count_visible_occupied_seats(3, 3), 0);
    }
}
