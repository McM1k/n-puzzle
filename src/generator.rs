extern crate rand;

use self::rand::Rng;

pub fn new_puzzle(size: usize) -> Vec<Vec<usize>> {
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

#[cfg(test)]
mod generator_tests {
    mod new_puzzle {
        use crate::generator::*;

        #[test]
        #[should_panic]
        fn panic_because_size_less_than_three() {
            new_puzzle(2);
        }

        #[test]
        fn correct_puzzle_of_size_three() {
            let size: usize = 3;
            let mut all_the_values: Vec<usize> = (0..(size * size)).collect();

            let puzzle = new_puzzle(size);

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

            let puzzle = new_puzzle(size);

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
}
