use std::{
    collections::{HashMap, HashSet},
    usize,
};

use itertools::Itertools;

const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
fn neighbors((i, j): (usize, usize), exclude: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    DELTAS
        .iter()
        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
        .filter(|(i, j)| i >= &0_i32 && j >= &0_i32)
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|t| !exclude.contains(t))
        .collect()
}

trait Mountain {
    /// Checks weather the provided index is a peak in the mountain (^)
    fn is_peak(self, idx: (usize, usize)) -> bool;
    /// Returns the positions of all spaces in the mountain
    fn find_all_spaces(self) -> HashSet<(usize, usize)>;
    // Returns the outer most peaks of the mountain
    fn outer_most_peaks(self) -> HashSet<(usize, usize)>;
}

impl Mountain for &[&str] {
    fn is_peak(self, (i, j): (usize, usize)) -> bool {
        self.get(i)
            .and_then(|s: &&str| s.chars().nth(j))
            .filter(|c: &char| *c == '^')
            .is_some()
    }

    fn find_all_spaces(self) -> HashSet<(usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(i, row): (usize, &&str)| {
                row.char_indices()
                    .filter(|(_, sign): &(usize, char)| *sign == ' ')
                    .map(move |(j, _): (usize, char)| (i, j))
            })
            .collect()
    }

    fn outer_most_peaks(self) -> HashSet<(usize, usize)> {
        (0..self.len())
            .flat_map(|i| (0..self[0].len()).map(move |j| (i, j)))
            .filter(|(i, j): &(usize, usize)| {
                *i == 0 || *j == 0 || *i == self.len() - 1 || *j == self[0].len() - 1
            })
            .filter(|idx| self.is_peak(*idx))
            .collect()
    }
}

/// https://www.codewars.com/kata/5c09ccc9b48e912946000157/train/rust
fn peak_height(mountain: &[&str]) -> u32 {
    // find all spaces and append to space list
    // around all spaces make everything a one and append to one list
    // around all ones make everything a two and append to two list
    // around all twos ...
    let mut spaces: HashSet<(usize, usize)> = mountain.find_all_spaces();

    // this map can be used to display the resulting mountain in the end
    let mut map: HashMap<(usize, usize), usize> = spaces.iter().map(|idx| (*idx, 0)).collect();

    let mut visited: HashSet<(usize, usize)> = spaces.clone();
    let mut current_iter: (u32, HashSet<(usize, usize)>) = (1_u32, mountain.outer_most_peaks());
    loop {
        for idx in spaces {
            current_iter.1.extend(
                neighbors(idx, &visited)
                    .iter()
                    .filter(|idx: &&(usize, usize)| mountain.is_peak(**idx)),
            );
        }
        if current_iter.1.len() == 0 {
            break;
        }
        map.extend(
            current_iter
                .1
                .clone()
                .iter()
                .map(move |idx| (*idx, current_iter.0 as usize)),
        );

        visited.extend(current_iter.1.clone());
        spaces = current_iter.1;
        current_iter = (current_iter.0 + 1, HashSet::new());
    }

    let grid = to_grid(map);
    println!("{grid}");

    current_iter.0 - 1_u32
}

#[cfg(test)]
mod sample_tests {
    use super::peak_height;

    #[test]
    fn example() {
        let mountain = [
            "^^^^^^        ",
            " ^^^^^^^^     ",
            "  ^^^^^^^     ",
            "  ^^^^^       ",
            "  ^^^^^^^^^^^ ",
            "  ^^^^^^      ",
            "  ^^^^        ",
        ];
        assert_eq!(
            peak_height(&mountain),
            3,
            "\nYour result (left) did not match expected result (right)"
        );
    }

    #[test]
    fn example2() {
        let mountain = [
            "     ^^^^^^",
            " ^^^^^^^^  ",
            "^^^^^^^^^  ",
            "  ^^^^^^^^ ",
            "  ^^^^^^^^^",
            "^^^^^^^^^^^",
            "^^^^^^^^^^^",
            "  ^^^^^^^^^",
            "  ^^^^^^^^ ",
            "  ^^^^^^^  ",
            "  ^^^^^^   ",
            "   ^^^^^^  ",
            "    ^^^^^  ",
            "      ^^   ",
        ];
        assert_eq!(
            peak_height(&mountain),
            5,
            "\nYour result (left) did not match expected result (right)"
        );
    }

    #[test]
    fn example3() {
        let mountain = [
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
            "^^^^^^^^^^^^^^^^^^^^^",
        ];
        assert_eq!(
            peak_height(&mountain),
            11,
            "\nYour result (left) did not match expected result (right)"
        );
    }

    #[test]
    fn example4() {
        let mountain = [
            "      ^^^^^^^^^      ",
            "    ^^^^^^^^^^^^^    ",
            "  ^^^^^^^^^^^^^^^^^  ",
            " ^^^^^^^     ^^^^^^^ ",
            "^^^^^^^       ^^^^^^^",
            "^^^^^^^       ^^^^^^^",
            "^^^^^^^       ^^^^^^^",
            " ^^^^^^^     ^^^^^^^ ",
            "  ^^^^^^^^^^^^^^^^^  ",
            "    ^^^^^^^^^^^^^    ",
            "      ^^^^^^^^^      ",
        ];
        assert_eq!(
            peak_height(&mountain),
            4,
            "\nYour result (left) did not match expected result (right)"
        );
    }
}

fn to_grid(spaces: HashMap<(usize, usize), usize>) -> String {
    // Determine grid size
    let max_row = spaces.keys().map(|(r, _)| *r).max().unwrap_or(0);
    let max_col = spaces.keys().map(|(_, c)| *c).max().unwrap_or(0);

    // Initialize empty grid with spaces
    let mut grid: Vec<Vec<char>> = vec![vec![' '; max_col + 1]; max_row + 1];

    // Fill in the values
    for ((r, c), ch) in &spaces {
        grid[*r][*c] = match *ch {
            0 => ' ',
            1..=9 => char::from_digit(*ch as u32, 10).unwrap(),
            _ => '^',
        };
    }

    grid.into_iter()
        .map(|row| row.into_iter().join(""))
        .join("\n")
}
