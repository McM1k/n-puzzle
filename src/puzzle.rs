extern crate rand;
use self::rand::Rng;

struct Puzzle {
    data: Vec<Vec<usize>>,
}

impl Puzzle {
    pub fn new(size: usize) -> Vec<Vec<usize>> {
        if size < 3 {
            panic!("Size should be higher than 3")
        }

        let mut all_the_values: Vec<usize> = (0..(size * size)).collect();
        let mut puzzle = vec![vec![0usize; size]; size];

        puzzle.iter_mut().for_each(|one_line_data| {
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

        puzzle
    }

    pub fn is_solvable(puzzle: & Vec<Vec<usize>>) -> bool {
        let mut data : Vec<usize> = Vec::new();
        puzzle.iter().for_each(|line| line.iter().for_each(|value| data.push(*value)));
        let mut sort_count = 0;

        for _ in 0 ..= data.len() {
            for i in 0 .. data.len() {
                if data[i] > data[i + 1] {
                    sort_count += 1;
                    data.swap(i, i + 1);
                }
            }
        }

        if sort_count % 2 == 0 {
            return true;
        }
        else {
            return false;
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

            println!("{:?}", puzzle);

            puzzle.iter().for_each(|one_line_data| {
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

            println!("{:?}", puzzle);

            puzzle.iter().for_each(|one_line_data| {
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
        use crate::generator::is_solvable;

        #[test]
        fn solvable_puzzle_already_solved() {
            let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            assert_eq!(is_solvable(puzzle), true);
        }

        #[test]
        fn solvable_puzzle() {
            let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            assert_eq!(is_solvable(puzzle), true);
        }

        #[test]
        fn unsolvable_puzzle() {
            let puzzle: Vec<Vec<usize>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            assert_eq!(is_solvable(puzzle), false);
        }
    }
}
