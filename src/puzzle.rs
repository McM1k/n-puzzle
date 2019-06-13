extern crate rand;
use self::rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct Puzzle {
    data: Vec<Vec<usize>>,
    size: usize,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line_data in self.data.iter() {
            for value in line_data.iter() {
                write!(f, "{} ", value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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

    pub fn new_from_file(data: Vec<Vec<usize>>) -> Puzzle {
        let size = data.len();
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
}

#[cfg(test)]
mod puzzle_tests {
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
