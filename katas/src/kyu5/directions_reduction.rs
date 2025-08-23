/// https://www.codewars.com/kata/550f22f4d758534c1100025a/train/rust
/// 
/// Once upon a time, on a way through the old wild mountainous west,…
/// … a man was given directions to go from one point to another. The directions were "NORTH", "SOUTH", "WEST", "EAST". Clearly "NORTH" and "SOUTH" are opposite, "WEST" and "EAST" too.
///
/// Going to one direction and coming back the opposite direction right away is a needless effort. Since this is the wild west, with dreadful weather and not much water, it's important to save yourself some energy, otherwise you might die of thirst!
///
/// How I crossed a mountainous desert the smart way.
/// The directions given to the man are, for example, the following (depending on the language):
///
/// ["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"].
/// or
/// { "NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST" };
/// or
/// [North, South, South, East, West, North, West]
/// You can immediately see that going "NORTH" and immediately "SOUTH" is not reasonable, better stay to the same place! So the task is to give to the man a simplified version of the plan. A better plan in this case is simply:
///
/// ["WEST"]
/// or
/// { "WEST" }
/// or
/// [West]
/// Other examples:
/// In ["NORTH", "SOUTH", "EAST", "WEST"], the direction "NORTH" + "SOUTH" is going north and coming back right away.
///
/// The path becomes ["EAST", "WEST"], now "EAST" and "WEST" annihilate each other, therefore, the final result is [] (nil in Clojure).
///
/// In ["NORTH", "EAST", "WEST", "SOUTH", "WEST", "WEST"], "NORTH" and "SOUTH" are not directly opposite but they become directly opposite after the reduction of "EAST" and "WEST" so the whole path is reducible to ["WEST", "WEST"].
///
/// Task
/// Write a function dirReduc which will take an array of strings and returns an array of strings with the needless directions removed (W<->E or S<->N side by side).
///
/// The Haskell version takes a list of directions with data Direction = North | East | West | South.
/// The Clojure version returns nil when the path is reduced to nothing.
/// The Rust version takes a slice of enum Direction {North, East, West, South}.
/// The OCaml version takes a list of type direction = | North | South | East | West.
/// See more examples in "Sample Tests:"
/// Notes
/// Not all paths can be made simpler. The path ["NORTH", "WEST", "SOUTH", "EAST"] is not reducible. "NORTH" and "WEST", "WEST" and "SOUTH", "SOUTH" and "EAST" are not directly opposite of each other and can't become such. Hence the result path is itself : ["NORTH", "WEST", "SOUTH", "EAST"].
/// if you want to translate, please ask before translating.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc_procedural(arr: &[Direction]) -> Vec<Direction> {
    let mut arr = arr.to_vec();
    let mut idx = 0;

    while arr.len() > 1 && idx < arr.len() - 1 {
        match (arr[idx], arr[idx + 1]) {
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East) => {
                arr.remove(idx);
                arr.remove(idx);
                idx = idx.saturating_sub(1);
            }
            (_, _) => idx += 1,
        }
    }

    arr
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    arr.iter().fold(Vec::new(), |mut acc, &next| {
        match (acc.last(), next) {
            (Some(Direction::North), Direction::South)
            | (Some(Direction::South), Direction::North)
            | (Some(Direction::East), Direction::West)
            | (Some(Direction::West), Direction::East) => {
                acc.pop();
            }
            (_, _) => {
                acc.push(next);
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::kyu5::directions_reduction::dir_reduc_procedural;

    use super::{
        Direction::{self, *},
        dir_reduc,
    };

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }

    #[test]
    fn basic_procedural() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc_procedural(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc_procedural(&a), [North, West, South, East]);
    }
}
