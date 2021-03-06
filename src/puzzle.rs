extern crate rand;
use self::rand::Rng;
use std::cmp;
use std::fmt;

#[derive(Debug, Clone, Eq)]
pub struct Puzzle {
    pub data: Vec<usize>,
    pub size: usize,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = if self.size < 11 { 3 } else { 4 };
        for i in 0..self.data.len() {
            if i % self.size == 0 && i != 0 {
                writeln!(f)?;
            }
            write!(f, "{0:<1$}", self.data[i], padding)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl cmp::PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data /*&& self.size == other.size*/
    }
}

impl Puzzle {
    pub fn get_value(&self, x: usize, y: usize) -> usize {
        self.data[y * self.size + x]
    }

    pub fn get_position(&self, value: usize) -> (usize, usize) {
        let size = self.size;
        for pos in 0..(size * size) {
            if self.data[pos] == value {
                return (pos % size, pos / size);
            }
        }
        panic!("value not found inside the puzzle");
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: usize) {
        self.data[y * self.size + x] = value;
    }

    pub fn new(size: usize) -> Puzzle {
        if size < 2 {
            panic!("Size should be higher than 1")
        }

        let mut all_the_values: Vec<usize> = (0..(size * size)).collect();
        let mut data = vec![0usize; size * size];
        data.iter_mut().for_each(|value| {
            while {
                *value = all_the_values
                    [rand::thread_rng().gen_range(0, all_the_values.clone().iter().count())];
                !all_the_values.contains(value)
            } {}
            all_the_values.remove(
                all_the_values
                    .clone()
                    .iter()
                    .position(|&x| x == *value)
                    .unwrap(),
            );
        });
        let puzzle = Puzzle { data, size };
        if Puzzle::is_solvable(puzzle.clone()) {
            puzzle
        } else {
            Puzzle::new(size)
        }
    }

    pub fn new_from_file(data: Vec<usize>, size: usize) -> Puzzle {
        let puzzle = Puzzle { data, size };
        if !Puzzle::is_solvable(puzzle.clone()) {
            panic!("Unsolvable puzzle\n");
        } else {
            puzzle
        }
    }

    /*
     ** an inversion is a pair of tiles (a,b) such that a appears before b, but a>b.
     */
    pub fn inversion(puzzle: Puzzle) -> usize {
        let data = puzzle.data; //Puzzle::get_current_data_sequence(puzzle);
        let mut sort_count = 0;

        for i in 0..data.len() {
            let value = data[i];
            for other in data.iter().skip(i) {
                if value > *other && *other != 0 {
                    sort_count += 1;
                }
            }
        }

        sort_count
    }

    pub fn is_solvable(puzzle: Puzzle) -> bool {
        let size = puzzle.size;
        let goal_puzzle = Puzzle::get_final_state(size);
        let mut start_inversion = Puzzle::inversion(puzzle.clone());
        let mut goal_inversion = Puzzle::inversion(goal_puzzle.clone());
        if size % 2 == 0 {
            let (mut x1, mut y1) = (0, 0);
            let (mut x2, mut y2) = (0, 0);

            for i in 0..size {
                for j in 0..size {
                    if puzzle.get_value(j, i) == 0 {
                        x1 = j;
                        y1 = i;
                    }
                    if goal_puzzle.get_value(j, i) == 0 {
                        x2 = j;
                        y2 = i;
                    }
                }
            }
            start_inversion += (y1 * size + x1) / size;
            goal_inversion += (y2 * size + x2) / size;
            //return start_inversion % 2 != goal_inversion % 2;
        }
        start_inversion % 2 == goal_inversion % 2
    }

    pub fn get_final_state(size: usize) -> Puzzle {
        let data = vec![0usize; size * size];
        let mut puzzle = Puzzle { data, size };

        let mut current_number = 1;
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = size - 1;
        let mut max_y = size - 1;

        loop {
            for right in min_x..=max_x {
                puzzle.set_value(right, min_x, current_number);
                current_number += 1;
            }
            min_y += 1;
            for down in min_y..=max_y {
                puzzle.set_value(max_x, down, current_number);
                current_number += 1;
            }
            max_x -= 1;
            if current_number == size * size {
                puzzle.set_value(max_x, max_y, 0);
                break;
            }

            for left in (min_x..=max_x).rev() {
                puzzle.set_value(left, max_y, current_number);
                current_number += 1;
            }
            max_y -= 1;
            for up in (min_y..=max_y).rev() {
                puzzle.set_value(min_x, up, current_number);
                current_number += 1;
            }
            min_x += 1;
            if current_number == size * size {
                puzzle.set_value(max_x, max_y, 0);
                break;
            }
        }
        puzzle
    }
}

#[cfg(test)]
mod puzzle_tests {
    mod get_value {
        use crate::puzzle::Puzzle;

        #[test]
        fn get_0() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            assert_eq!(puzzle.get_value(1, 1), 0);
        }

        #[test]
        fn get_7() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            assert_eq!(puzzle.get_value(0, 2), 7);
        }

        #[test]
        fn get_15() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 9, 8, 0, 4, 10, 7, 6, 5, 11, 12, 13, 14, 15],
                size: 4,
            };

            assert_eq!(puzzle.get_value(3, 3), 15);
        }

        #[test]
        #[should_panic]
        fn get_out_of_bounds() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            puzzle.get_value(42, 42);
        }
    }

    mod get_position {
        use crate::puzzle::Puzzle;

        #[test]
        fn get_0() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            assert_eq!(puzzle.get_position(0), (1, 1));
        }

        #[test]
        fn get_7() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            assert_eq!(puzzle.get_position(7), (0, 2));
        }

        #[test]
        fn get_15() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 9, 8, 0, 4, 10, 7, 6, 5, 11, 12, 13, 14, 15],
                size: 4,
            };

            assert_eq!(puzzle.get_position(15), (3, 3));
        }

        #[test]
        #[should_panic]
        fn get_impossible() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            puzzle.get_position(9);
        }
    }

    mod set_value {
        use crate::puzzle::Puzzle;

        #[test]
        fn change_0_to_9() {
            let mut puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let changed = Puzzle {
                data: vec![1, 2, 3, 8, 9, 4, 7, 6, 5],
                size: 3,
            };

            puzzle.set_value(1, 1, 9);
            assert_eq!(puzzle, changed);
        }

        #[test]
        fn change_15_to_16() {
            let mut puzzle = Puzzle {
                data: vec![1, 2, 3, 9, 8, 0, 4, 10, 7, 6, 5, 11, 12, 13, 14, 15],
                size: 4,
            };
            let changed = Puzzle {
                data: vec![1, 2, 3, 9, 8, 0, 4, 10, 7, 6, 5, 11, 12, 13, 14, 16],
                size: 4,
            };

            puzzle.set_value(3, 3, 16);
            assert_eq!(puzzle, changed);
        }

        #[test]
        #[should_panic]
        fn change_out_of_bounds() {
            let mut puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            puzzle.set_value(42, 42, 9);
        }
    }

    mod get_final_position {
        use crate::puzzle::Puzzle;

        #[test]
        fn size_three() {
            let size = 3;
            let expected_final_state = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size,
            };
            let result_data = Puzzle::get_final_state(size);

            assert_eq!(result_data, expected_final_state);
        }
        /*
        1 2 3
        8 0 4
        7 6 5
        */

        #[test]
        fn size_five() {
            let size = 5;
            let expected_final_state = Puzzle {
                data: vec![
                    1, 2, 3, 4, 5, 16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12,
                    11, 10, 9,
                ],
                size,
            };
            let result_data = Puzzle::get_final_state(size);

            assert_eq!(result_data, expected_final_state);
        }
        /*
        1  2  3  4  5
        16 17 18 19 6
        15 24 0  20 7
        14 23 22 21 8
        13 12 11 10 9
        */

        #[test]
        fn size_ten() {
            let size = 10;
            let expected_final_state = Puzzle {
                data: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 36, 37, 38, 39, 40, 41, 42, 43, 44, 11, 35, 64,
                    65, 66, 67, 68, 69, 70, 45, 12, 34, 63, 84, 85, 86, 87, 88, 71, 46, 13, 33, 62,
                    83, 96, 97, 98, 89, 72, 47, 14, 32, 61, 82, 95, 0, 99, 90, 73, 48, 15, 31, 60,
                    81, 94, 93, 92, 91, 74, 49, 16, 30, 59, 80, 79, 78, 77, 76, 75, 50, 17, 29, 58,
                    57, 56, 55, 54, 53, 52, 51, 18, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19,
                ],
                size,
            };
            let result_data = Puzzle::get_final_state(size);

            assert_eq!(result_data, expected_final_state);
        }
        /*
        1  2  3  4  5  6  7  8  9  10
        36 37 38 39 40 41 42 43 44 11
        35 64 65 66 67 68 69 70 45 12
        34 63 84 85 86 87 88 71 46 13
        33 62 83 96 97 98 89 72 47 14
        32 61 82 95 0  99 90 73 48 15
        31 60 81 94 93 92 91 74 49 16
        30 59 80 79 78 77 76 75 50 17
        29 58 57 56 55 54 53 52 51 18
        28 27 26 25 24 23 22 21 20 19
        */

    }
    mod partial_eq {
        use crate::puzzle::*;

        #[test]
        fn equals() {
            let size = 3;

            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };

            let data2 = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle2 = Puzzle { data: data2, size };

            assert_eq!(puzzle, puzzle2);
        }

        #[test]
        fn not_equals() {
            let size = 3;

            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };

            let data2 = vec![0, 2, 1, 3, 4, 5, 6, 8, 7];
            let puzzle2 = Puzzle { data: data2, size };
            assert_ne!(puzzle, puzzle2);
        }
    }

    mod new {
        use crate::puzzle::*;

        #[test]
        #[should_panic]
        fn panic_because_size_less_than_two() {
            Puzzle::new(1);
        }

        #[test]
        fn correct_puzzle_of_size_three() {
            assert_eq!(Puzzle::new(3).size, 3);
        }

        #[test]
        fn correct_puzzle_of_size_ten() {
            assert_eq!(Puzzle::new(10).size, 10);
        }
    }

    mod is_solvable {
        use crate::puzzle::*;

        #[test]
        fn solvable_puzzle() {
            let puzzle = Puzzle {
                data: vec![0, 1, 2, 3, 4, 5, 6, 8, 7],
                size: 3,
            };
            assert!(Puzzle::is_solvable(puzzle));
        }

        #[test]
        fn solvable_puzzle_already_solved() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            assert!(Puzzle::is_solvable(puzzle));
        }

        #[test]
        fn solvable_puzzle_with_one_move() {
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 0, 8, 4, 7, 6, 5],
                size: 3,
            };
            assert!(Puzzle::is_solvable(puzzle));
        }

        #[test]
        fn solvable_puzzle_with_fifteen_moves() {
            let puzzle = Puzzle {
                data: vec![1, 8, 4, 0, 3, 5, 2, 7, 6],
                size: 3,
            };
            assert!(Puzzle::is_solvable(puzzle));
        }

        #[test]
        fn unsolvable_puzzle() {
            let puzzle = Puzzle {
                data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                size: 3,
            };
            assert_eq!(Puzzle::is_solvable(puzzle), false);
        }
    }
}

/*0 1 2
3 4 5
6 8 7

1 0 2
3 4 5
6 8 7

1 2 0
3 4 5
6 8 7*/
