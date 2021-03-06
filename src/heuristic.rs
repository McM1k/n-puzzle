use crate::puzzle::Puzzle;

fn get_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x = (x1 as i32 - x2 as i32).abs();
    let y = (y1 as i32 - y2 as i32).abs();

    x as usize + y as usize
}

fn get_possible_values_in_row(
    puzzle: Puzzle,
    final_puzzle: Puzzle,
    value: usize,
    y: usize,
) -> Vec<usize> {
    let mut values = vec![];

    let (final_x, _) = final_puzzle.get_position(value);
    let (current_x, _) = puzzle.get_position(value);
    if final_x < current_x {
        for x in (final_x..current_x).rev() {
            values.push(puzzle.get_value(x, y));
        }
    } else {
        for x in (current_x + 1)..=final_x {
            values.push(puzzle.get_value(x, y));
        }
    }

    values
}

fn get_possible_values_in_column(
    puzzle: Puzzle,
    final_puzzle: Puzzle,
    value: usize,
    x: usize,
) -> Vec<usize> {
    let mut values = vec![];

    let (_, final_y) = final_puzzle.get_position(value);
    let (_, current_y) = puzzle.get_position(value);
    if final_y < current_y {
        for y in (final_y..current_y).rev() {
            values.push(puzzle.get_value(x, y));
        }
    } else {
        for y in (current_y + 1)..=final_y {
            values.push(puzzle.get_value(x, y));
        }
    }

    values
}

fn check_in_correct_column(puzzle: Puzzle, value: usize, x: usize) -> bool {
    for y in 0..puzzle.size {
        if puzzle.get_value(x, y) == value {
            return true;
        }
    }

    false
}

fn check_in_correct_row(puzzle: Puzzle, value: usize, y: usize) -> bool {
    for x in 0..puzzle.size {
        if puzzle.get_value(x, y) == value {
            return true;
        }
    }

    false
}

fn check_column_conflict(
    current_puzzle: Puzzle,
    final_puzzle: Puzzle,
    value: usize,
    x: usize,
    number_list: &[usize],
) -> usize {
    let possible_values =
        get_possible_values_in_column(current_puzzle, final_puzzle.clone(), value, x);
    for possible_value in possible_values {
        if check_in_correct_column(final_puzzle.clone(), possible_value, x)
            && number_list.contains(&possible_value)
            && possible_value != 0
        {
            return 1;
        }
    }
    0
}

fn check_row_conflict(
    current_puzzle: Puzzle,
    final_puzzle: Puzzle,
    value: usize,
    y: usize,
    number_list: &[usize],
) -> usize {
    let possible_values =
        get_possible_values_in_row(current_puzzle, final_puzzle.clone(), value, y);
    for possible_value in possible_values {
        if check_in_correct_row(final_puzzle.clone(), possible_value, y)
            && number_list.contains(&possible_value)
            && possible_value != 0
        {
            return 1;
        }
    }
    0
}

pub fn hamming_distance(puzzle: Puzzle, final_puzzle: Puzzle) -> usize {
    // +1 per misplaced tiles (except empty one)
    let mut heuristic = 0;

    for y in 0..puzzle.size {
        for x in 0..puzzle.size {
            if final_puzzle.get_value(x, y) != puzzle.get_value(x, y) && puzzle.get_value(x, y) != 0
            {
                heuristic += 1;
            }
        }
    }

    heuristic
}

pub fn manhattan_distance(puzzle: Puzzle, final_puzzle: Puzzle) -> usize {
    // +1 per move a misplaced tile as to do (except empty one)
    let mut heuristic = 0;

    for y in 0..puzzle.size {
        for x in 0..puzzle.size {
            if final_puzzle.get_value(x, y) != puzzle.get_value(x, y) && puzzle.get_value(x, y) != 0
            {
                let value = puzzle.get_value(x, y);
                let (x2, y2) = final_puzzle.get_position(value);
                heuristic += get_distance(x, y, x2, y2);
            }
        }
    }

    heuristic
}

pub fn linear_conflict(puzzle: Puzzle, final_puzzle: Puzzle) -> usize {
    // +2 when two tiles are in their goal row or column, but are reversed relative to their goal positions.  (except empty one)
    let mut heuristic = 0;
    let size = puzzle.size;
    let mut number_list: Vec<usize> = (0..(size * size)).collect();

    for y in 0..size {
        for x in 0..size {
            let final_value = &final_puzzle.get_value(x, y);
            let value = &puzzle.get_value(x, y);
            if final_value != value && *value != 0 {
                let conflict_value = check_column_conflict(
                    puzzle.clone(),
                    final_puzzle.clone(),
                    *value,
                    x,
                    &number_list,
                ) + check_row_conflict(
                    puzzle.clone(),
                    final_puzzle.clone(),
                    *value,
                    y,
                    &number_list,
                );
                if conflict_value != 0usize {
                    heuristic += conflict_value;
                    let index = number_list.iter().position(|x| *x == *value).unwrap();
                    number_list.remove(index);
                }
            }
        }
    }

    heuristic
}

pub fn manhattan_linear_conflict_heuristic(puzzle: Puzzle, final_puzzle: Puzzle) -> usize {
    manhattan_distance(puzzle.clone(), final_puzzle.clone())
        + 2 * linear_conflict(puzzle, final_puzzle)
}

#[cfg(test)]
mod heuristic_tests {
    mod hamming_distance {
        use super::super::Puzzle;
        use crate::heuristic::*;

        #[test]
        fn five_moved_tiles() {
            let size = 3;
            let data = vec![1, 2, 8, 0, 3, 4, 7, 5, 6];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 4);
        }

        #[test]
        fn no_moved_tile() {
            let size = 3;
            let data = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 0);
        }

        #[test]
        fn every_tiles_moved() {
            let size = 3;
            let data = vec![7, 5, 6, 1, 2, 3, 8, 0, 4];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 8);
        }
    }

    mod manhattan_distance {
        use super::super::Puzzle;
        use crate::heuristic::*;

        #[test]
        fn five_moved_tiles() {
            let size = 3;
            let data = vec![1, 2, 8, 0, 3, 4, 7, 5, 6];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 7);
        }

        #[test]
        fn no_moved_tile() {
            let size = 3;
            let data = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 0);
        }

        #[test]
        fn every_tiles_moved() {
            let size = 3;
            let data = vec![7, 5, 6, 1, 2, 3, 8, 0, 4];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 13);
        }
    }

    mod linear_conflict {
        use super::super::Puzzle;
        use crate::heuristic::*;

        #[test]
        fn one_conflict_in_one_row() {
            let size = 3;
            let data = vec![2, 1, 3, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 1);
        }

        #[test]
        fn two_conflicts_in_one_row() {
            let size = 3;
            let data = vec![3, 2, 1, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn one_conflict_in_one_column() {
            let size = 3;
            let data = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 1);
        }

        #[test]
        fn two_conflicts_in_one_column() {
            let size = 3;
            let data = vec![7, 2, 3, 8, 0, 4, 1, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn no_conflict() {
            let size = 3;
            let data = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 0);
        }
    }

    mod check_in_correct_column {
        use super::super::Puzzle;
        use crate::heuristic::*;

        #[test]
        fn return_true() {
            let size = 3;
            let data = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let x = 0;
            let value = 1;
            let in_correct_column =
                check_in_correct_column(Puzzle::get_final_state(puzzle.size), value, x);

            assert!(in_correct_column);
        }

        #[test]
        fn return_false() {
            let size = 3;
            let data = vec![2, 1, 3, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let x = 1;
            let value = 1;
            let in_correct_column =
                check_in_correct_column(Puzzle::get_final_state(puzzle.size), value, x);

            assert!(!in_correct_column);
        }
    }

    mod check_in_correct_row {
        use super::super::Puzzle;
        use crate::heuristic::*;

        #[test]
        fn return_true() {
            let size = 3;
            let data = vec![3, 2, 1, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let y = 0;
            let value = 1;
            let in_correct_column =
                check_in_correct_row(Puzzle::get_final_state(puzzle.size), value, y);

            assert!(in_correct_column);
        }

        #[test]
        fn return_false() {
            let size = 3;
            let data = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let y = 1;
            let value = 1;
            let in_correct_column =
                check_in_correct_row(Puzzle::get_final_state(puzzle.size), value, y);

            assert!(!in_correct_column);
        }
    }

    mod check_column_conflict {
        use super::super::*;

        #[test]
        fn with_conflict() {
            let size = 3;
            let data = vec![8, 2, 3, 1, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let number_list: Vec<usize> = (0..(size * size)).collect();
            let value = 1;
            let x = 0;
            let heuristic = check_column_conflict(
                puzzle,
                Puzzle::get_final_state(size),
                value,
                x,
                &number_list,
            );

            assert_eq!(heuristic, 1);
        }

        #[test]
        fn with_no_conflict() {
            let size = 3;
            let data = vec![2, 8, 3, 1, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let number_list: Vec<usize> = (0..(size * size)).collect();
            let value = 1;
            let x = 0;
            let heuristic = check_column_conflict(
                puzzle,
                Puzzle::get_final_state(size),
                value,
                x,
                &number_list,
            );

            assert_eq!(heuristic, 0);
        }
    }

    mod check_row_conflict {
        use super::super::*;

        #[test]
        fn with_conflict() {
            let size = 3;
            let data = vec![3, 2, 1, 8, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let number_list: Vec<usize> = (0..(size * size)).collect();
            let value = 1;
            let y = 0;
            let heuristic = check_row_conflict(
                puzzle,
                Puzzle::get_final_state(size),
                value,
                y,
                &number_list,
            );

            assert_eq!(heuristic, 1);
        }

        #[test]
        fn with_no_conflict() {
            let size = 3;
            let data = vec![8, 1, 3, 2, 0, 4, 7, 6, 5];
            let puzzle = Puzzle { data, size };
            let number_list: Vec<usize> = (0..(size * size)).collect();
            let value = 1;
            let y = 0;
            let heuristic = check_row_conflict(
                puzzle,
                Puzzle::get_final_state(size),
                value,
                y,
                &number_list,
            );

            assert_eq!(heuristic, 0);
        }
    }

    mod get_possible_values_in_row {
        use super::super::*;

        #[test]
        fn row_distance_of_one_with_incremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![2, 1, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            let value = 1;
            let possible_values = get_possible_values_in_row(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![2];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn row_distance_of_one_with_decremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![1, 3, 2, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let value = 3;
            let possible_values = get_possible_values_in_row(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![2];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn row_distance_of_two_with_incremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![2, 3, 1, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            let value = 1;
            let possible_values = get_possible_values_in_row(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![3, 2];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn row_distance_of_two_with_decremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![3, 1, 2, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let value = 3;
            let possible_values = get_possible_values_in_row(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![1, 2];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn row_distance_of_zero() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };

            let value = 1;
            let possible_values = get_possible_values_in_row(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![];

            assert_eq!(possible_values, wanted_values);
        }
    }

    mod get_possible_values_in_column {
        use super::super::*;

        #[test]
        fn column_distance_of_one_with_incremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![8, 2, 3, 1, 0, 4, 7, 6, 5],
                size: 3,
            };
            let value = 1;
            let possible_values = get_possible_values_in_column(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![8];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn column_distance_of_one_with_decremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 7, 0, 4, 8, 6, 5],
                size: 3,
            };
            let value = 7;
            let possible_values = get_possible_values_in_column(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![8];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn column_distance_of_two_with_incremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![8, 2, 3, 7, 0, 4, 1, 6, 5],
                size: 3,
            };
            let value = 1;
            let possible_values = get_possible_values_in_column(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![7, 8];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn column_distance_of_two_with_decremental_range() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![7, 2, 3, 1, 0, 4, 8, 6, 5],
                size: 3,
            };
            let value = 7;
            let possible_values = get_possible_values_in_column(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![1, 8];

            assert_eq!(possible_values, wanted_values);
        }

        #[test]
        fn column_distance_of_zero() {
            let final_puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let puzzle = Puzzle {
                data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                size: 3,
            };
            let value = 1;
            let possible_values = get_possible_values_in_column(puzzle, final_puzzle, value, 0);
            let wanted_values = vec![];

            assert_eq!(possible_values, wanted_values);
        }
    }

    mod manhattan_linear_conflict_heuristic {
        use super::super::*;

        #[test]
        fn example_case() {
            let size = 3;
            let data = vec![1, 2, 3, 7, 4, 0, 8, 5, 6];
            let puzzle = Puzzle { data, size };
            let heuristic =
                manhattan_linear_conflict_heuristic(puzzle, Puzzle::get_final_state(size));

            assert_eq!(heuristic, 9);
        }
    }
}
