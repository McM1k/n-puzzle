use crate::puzzle::Puzzle;

fn get_distance(x1: &usize, y1: &usize, x2: &usize, y2: &usize) -> usize {
    let x = (*x1 as i32 - *x2 as i32).abs();
    let y = (*y1 as i32 - *y2 as i32).abs();

    x as usize + y as usize
}

fn get_correct_point(
    x1: &usize,
    y2: &usize,
    data: &(Vec<Vec<usize>>),
    value: &usize,
) -> (usize, usize) {
    let x2 = 0;
    let y2 = 0;

    for y2 in 0..data.len() {
        for x2 in 0..data.len() {
            if data[y2][x2] == *value {
                return (x2, y2);
            }
        }
    }

    (x2, y2)
}

fn check_in_correct_column(final_data: &Vec<Vec<usize>>, value: &usize, x: &usize) -> bool {
    for y in 0..final_data.len() {
        if final_data[y][*x] == *value {
            return true;
        }
    }
    false
}

fn check_in_correct_row(final_data: &Vec<Vec<usize>>, value: &usize, y: &usize) -> bool {
    for x in 0..final_data.len() {
        if final_data[*y][x] == *value {
            return true;
        }
    }
    false
}

fn check_column_conflict(
    current_data: &Vec<Vec<usize>>,
    final_data: &Vec<Vec<usize>>,
    value: &usize,
    x: &usize,
    number_list: &Vec<usize>,
) -> usize {
    for y in 0..current_data.len() {
        let possible_conflict = current_data[y][*x];
        if final_data[y][*x] == *value
            && check_in_correct_column(&final_data, &possible_conflict, x)
            && number_list.contains(&possible_conflict)
        {
            return 2;
        }
    }
    0
}

fn check_row_conflict(
    current_data: &Vec<Vec<usize>>,
    final_data: &Vec<Vec<usize>>,
    value: &usize,
    y: &usize,
    number_list: &Vec<usize>,
) -> usize {
    for x in 0..current_data.len() {
        let possible_conflict = current_data[*y][x];
        if final_data[*y][x] == *value
            && check_in_correct_row(&final_data, &possible_conflict, y)
            && number_list.contains(&possible_conflict)
        {
            return 2;
        }
    }
    0
}

pub fn hamming_distance(puzzle: Puzzle) -> usize {
    // +1 per misplaced tiles (except empty one)
    let final_data = puzzle.get_final_state();
    let current_data = puzzle.data;
    let mut heuristic = 0;

    for y in 0..final_data.len() {
        for x in 0..final_data.len() {
            if final_data[y][x] != current_data[y][x] && current_data[y][x] != 0 {
                heuristic += 1;
            }
        }
    }

    heuristic
}

pub fn manhattan_distance(puzzle: Puzzle) -> usize {
    // +1 per move a misplaced tile as to do (except empty one)
    let final_data = puzzle.get_final_state();
    let current_data = puzzle.data;
    let mut heuristic = 0;

    for y in 0..current_data.len() {
        for x in 0..current_data.len() {
            if final_data[y][x] != current_data[y][x] && current_data[y][x] != 0 {
                let value = current_data[y][x];
                let (x2, y2) = get_correct_point(&x, &y, &final_data, &value);
                heuristic += get_distance(&x, &y, &x2, &y2);
            }
        }
    }

    heuristic
}

pub fn linear_conflict(puzzle: Puzzle) -> usize {
    // +2 when two tiles are in their goal row or column, but are reversed relative to their goal positions.  (except empty one)
    let final_data = puzzle.get_final_state();
    let current_data = puzzle.data;
    let mut heuristic = 0;
    let size = puzzle.size;
    let mut number_list: Vec<usize> = (0..(size * size)).collect();

    for y in 0..current_data.len() {
        for x in 0..current_data.len() {
            if final_data[y][x] != current_data[y][x] && current_data[y][x] != 0 {
                let value = current_data[y][x];
                let conflict_value = check_column_conflict(&current_data, &final_data, &value, &x, &number_list)
                    + check_row_conflict(&current_data, &final_data, &value, &y, &number_list);
                if conflict_value != 0usize {
                    heuristic += conflict_value;
                    number_list.remove(value);
                }
            }
        }
    }

    heuristic
}

//pub fn manhattan_linear_conflict_heuristic() {

//}

#[cfg(test)]
mod heuristic_tests {
    mod hamming_distance {
        use crate::heuristic::*;

        #[test]
        fn five_moved_tiles() {
            let size = 3;
            let data = vec![vec![1, 2, 8], vec![0, 3, 4], vec![7, 5, 6]];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle);

            assert_eq!(heuristic, 4);
        }

        #[test]
        fn no_moved_tile() {
            let size = 3;
            let data = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle);

            assert_eq!(heuristic, 0);
        }

        #[test]
        fn every_tiles_moved() {
            let size = 3;
            let data = vec![vec![7, 5, 6], vec![1, 2, 3], vec![8, 0, 4]];
            let puzzle = Puzzle { data, size };
            let heuristic = hamming_distance(puzzle);

            assert_eq!(heuristic, 8);
        }
    }

    mod manhattan_distance {
        use crate::heuristic::*;

        #[test]
        fn five_moved_tiles() {
            let size = 3;
            let data = vec![vec![1, 2, 8], vec![0, 3, 4], vec![7, 5, 6]];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle);

            assert_eq!(heuristic, 7);
        }

        #[test]
        fn no_moved_tile() {
            let size = 3;
            let data = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle);

            assert_eq!(heuristic, 0);
        }

        #[test]
        fn every_tiles_moved() {
            let size = 3;
            let data = vec![vec![7, 5, 6], vec![1, 2, 3], vec![8, 0, 4]];
            let puzzle = Puzzle { data, size };
            let heuristic = manhattan_distance(puzzle);

            assert_eq!(heuristic, 13);
        }
    }

    mod linear_conflict {
        use crate::heuristic::*;

        #[test]
        fn one_conflict_in_one_row() {
            let size = 3;
            let data = vec![vec![2, 1, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle);

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn two_conflicts_in_one_row() {
            let size = 3;
            let data = vec![vec![3, 2, 1], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle);

            assert_eq!(heuristic, 4);
        }

        #[test]
        fn one_conflict_in_one_column() {
            let size = 3;
            let data = vec![vec![8, 2, 3], vec![1, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle);

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn two_conflicts_in_one_column() {
            let size = 3;
            let data = vec![vec![7, 2, 3], vec![8, 0, 4], vec![1, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle);

            assert_eq!(heuristic, 4);
        }

        #[test]
        fn no_conflict() {
            let size = 3;
            let data = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let heuristic = linear_conflict(puzzle);

            assert_eq!(heuristic, 0);
        }
    }

    mod check_in_correct_column {
        use super::super::*;

        #[test]
        fn return_true() {
            let size = 3;
            let data = vec![vec![8, 2, 3], vec![1, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let x = 0;
            let value = 1;
            let in_correct_column = check_in_correct_column(&puzzle.get_final_state(), &value, &x);

            assert!(in_correct_column);
        }

        #[test]
        fn return_false() {
            let size = 3;
            let data = vec![vec![2, 1, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let x = 1;
            let value = 1;
            let in_correct_column = check_in_correct_column(&puzzle.get_final_state(), &value, &x);

            assert!(!in_correct_column);
        }
    }

    mod check_in_correct_row {
        use super::super::*;

        #[test]
        fn return_true() {
            let size = 3;
            let data = vec![vec![3, 2, 1], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let y = 0;
            let value = 1;
            let in_correct_column = check_in_correct_row(&puzzle.get_final_state(), &value, &y);

            assert!(in_correct_column);
        }

        #[test]
        fn return_false() {
            let size = 3;
            let data = vec![vec![8, 2, 3], vec![1, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let y = 1;
            let value = 1;
            let in_correct_column = check_in_correct_row(&puzzle.get_final_state(), &value, &y);

            assert!(!in_correct_column);
        }
    }

    mod check_column_conflict {
        use super::super::*;

        #[test]
        fn with_conflict() {
            let size = 3;
            let data = vec![vec![8, 2, 3], vec![1, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let mut number_list: Vec<usize> = (0..(puzzle.size * puzzle.size)).collect();
            let value = 1;
            let x = 0;
            let heuristic =
                check_column_conflict(&puzzle.data, &puzzle.get_final_state(), &value, &x, &number_list);

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn with_no_conflict() {
            let size = 3;
            let data = vec![vec![2, 8, 3], vec![1, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let mut number_list: Vec<usize> = (0..(puzzle.size * puzzle.size)).collect();
            let value = 1;
            let x = 0;
            let heuristic =
                check_column_conflict(&puzzle.data, &puzzle.get_final_state(), &value, &x, &number_list);

            assert_eq!(heuristic, 0);
        }
    }

    mod check_row_conflict {
        use super::super::*;

        #[test]
        fn with_conflict() {
            let size = 3;
            let data = vec![vec![3, 2, 1], vec![8, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let mut number_list: Vec<usize> = (0..(puzzle.size * puzzle.size)).collect();
            let value = 1;
            let y = 0;
            let heuristic = check_row_conflict(&puzzle.data, &puzzle.get_final_state(), &value, &y, &number_list);

            assert_eq!(heuristic, 2);
        }

        #[test]
        fn with_no_conflict() {
            let size = 3;
            let data = vec![vec![8, 1, 3], vec![2, 0, 4], vec![7, 6, 5]];
            let puzzle = Puzzle { data, size };
            let mut number_list: Vec<usize> = (0..(puzzle.size * puzzle.size)).collect();
            let value = 1;
            let y = 0;
            let heuristic = check_row_conflict(&puzzle.data, &puzzle.get_final_state(), &value, &y, &number_list);

            assert_eq!(heuristic, 0);
        }
    }
}
