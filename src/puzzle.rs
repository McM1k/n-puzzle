extern crate rand;
use self::rand::Rng;
use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub data: Vec<Vec<usize>>,
    pub size: usize,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line_data in self.data.iter() {
            for value in line_data.iter() {
                print!("{:4}", value);
            }
            print!("\n");
        }
        Ok(())
    }
}

impl cmp::PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}

impl Puzzle {
    pub fn new(size: usize) -> Puzzle {
        if size < 3 {
            panic!("Size should be higher than 3")
        }

        let mut all_the_values: Vec<usize> = (0..(size * size)).collect();
        let mut data = vec![vec![0usize; size]; size];

        data.iter_mut().for_each(|one_line_data| {
            one_line_data.iter_mut().for_each(|value| {
                while {
                    *value = *(all_the_values
                        .get(rand::thread_rng().gen_range(0, all_the_values.clone().iter().count()))
                        .unwrap());
                    !all_the_values.contains(value)
                } {}
                all_the_values.remove(
                    all_the_values
                        .clone()
                        .iter()
                        .position(|&x| x == *value)
                        .unwrap(),
                );
            })
        });

        if Puzzle::is_solvable(&data) {
            let puzzle = Puzzle { data, size };
            puzzle
        } else {
            Puzzle::new(size)
        }
    }

    pub fn new_from_file(data: Vec<Vec<usize>>, size: usize) -> Puzzle {
        if !Puzzle::is_solvable(&data) {
            panic!("Unsolvable puzzle");
        } else {
            Puzzle { data, size }
        }
    }

    pub fn is_solvable(puzzle: &Vec<Vec<usize>>) -> bool {
        let mut data: Vec<usize> = Vec::new();
        puzzle
            .iter()
            .for_each(|line| line.iter().for_each(|value| data.push(*value)));
        let mut sort_count = 0;

        for _ in 0..data.len() {
            for i in 0..data.len() - 1 {
                if data[i] > data[i + 1] {
                    sort_count += 1;
                    data.swap(i, i + 1);
                }
            }
        }

        /*
         *	The solvable pattern is a snail one
         *	1  2  3
         *	8  0  4
         *	7  6  5
         */

        if sort_count % 2 == 0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn get_final_state(size: usize) -> Vec<Vec<usize>> {
        let mut data = vec![vec![0usize; size]; size];

        let mut current_number = 1;
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = size - 1;
        let mut max_y = size - 1;

        loop {
            for right in min_x..=max_x {
                data[min_x][right] = current_number;
                current_number += 1;
            }
            min_y += 1;
            for down in min_y..=max_y {
                data[down][max_x] = current_number;
                current_number += 1;
            }
            max_x -= 1;
            if current_number == size * size {
                data[max_y][max_x] = 0;
                break;
            }
            for left in (min_x..=max_x).rev() {
                data[max_y][left] = current_number;
                current_number += 1;
            }
            max_y -= 1;
            for up in (min_y..=max_y).rev() {
                data[up][min_x] = current_number;
                current_number += 1;
            }
            min_x += 1;
            if current_number == size * size {
                data[min_x][min_x] = 0;
                break;
            }
        }

        data
    }
}

#[cfg(test)]
mod puzzle_tests {
    mod get_final_position {
        use crate::puzzle::*;

        #[test]
        fn size_three() {
            let size = 3;
            let data = vec![vec![0usize; size]; size];
            let puzzle = Puzzle { data, size };
            let expected_final_state_data = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let result_data = Puzzle::get_final_state(3);

            assert_eq!(result_data, expected_final_state_data);
        }
        /*
        1 2 3
        8 0 4
        7 6 5
        */

        #[test]
        fn size_five() {
            let size = 5;
            let data = vec![vec![0usize; size]; size];
            let puzzle = Puzzle { data, size };
            let expected_final_state_data = vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 0, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9],
            ];
            let result_data = Puzzle::get_final_state(5);

            assert_eq!(result_data, expected_final_state_data);
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
            let data = vec![vec![0usize; size]; size];
            let puzzle = Puzzle { data, size };
            let expected_final_state_data = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![36, 37, 38, 39, 40, 41, 42, 43, 44, 11],
                vec![35, 64, 65, 66, 67, 68, 69, 70, 45, 12],
                vec![34, 63, 84, 85, 86, 87, 88, 71, 46, 13],
                vec![33, 62, 83, 96, 97, 98, 89, 72, 47, 14],
                vec![32, 61, 82, 95, 0, 99, 90, 73, 48, 15],
                vec![31, 60, 81, 94, 93, 92, 91, 74, 49, 16],
                vec![30, 59, 80, 79, 78, 77, 76, 75, 50, 17],
                vec![29, 58, 57, 56, 55, 54, 53, 52, 51, 18],
                vec![28, 27, 26, 25, 24, 23, 22, 21, 20, 19],
            ];
            let result_data = Puzzle::get_final_state(10);

            assert_eq!(result_data, expected_final_state_data);
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

            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };

            let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle2 = Puzzle { data: data2, size };

            assert!(puzzle == puzzle2);
        }

        #[test]
        fn not_equals() {
            let size = 3;

            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };

            let data2 = vec![vec![0, 2, 1], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle2 = Puzzle { data: data2, size };
            assert!(puzzle != puzzle2);
        }
    }

    mod new {
        use crate::puzzle::*;

        #[test]
        #[should_panic]
        fn panic_because_size_less_than_three() {
            Puzzle::new(2);
        }

        #[test]
        fn correct_puzzle_of_size_three() {
            let size: usize = 3;
            let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

            let puzzle = Puzzle::new(size);

            puzzle.data.iter().for_each(|one_line_data| {
                one_line_data.iter().for_each(|value| {
                    all_the_values.remove(
                        all_the_values
                            .clone()
                            .iter()
                            .position(|&x| x == *value)
                            .unwrap(),
                    );
                })
            });
            assert!(all_the_values.is_empty());
        }

        #[test]
        fn correct_puzzle_of_size_ten() {
            let size: usize = 10;
            let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

            let puzzle = Puzzle::new(size);

            puzzle.data.iter().for_each(|one_line_data| {
                one_line_data.iter().for_each(|value| {
                    all_the_values.remove(
                        all_the_values
                            .clone()
                            .iter()
                            .position(|&x| x == *value)
                            .unwrap(),
                    );
                })
            });
            assert!(all_the_values.is_empty());
        }
    }

    mod is_solvable {
        use crate::puzzle::*;

        #[test]
        fn solvable_puzzle() {
            let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            assert_eq!(Puzzle::is_solvable(&puzzle), true);
        }

        #[test]
        fn solvable_puzzle_already_solved() {
            let puzzle: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            assert_eq!(Puzzle::is_solvable(&puzzle), true);
        }

        #[test]
        fn unsolvable_puzzle() {
            let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            assert_eq!(Puzzle::is_solvable(&puzzle), false);
        }
    }
}
