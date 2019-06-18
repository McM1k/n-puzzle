use crate::puzzle::Puzzle;

pub fn hamming_distance(puzzle: Puzzle) -> usize {
    let final_data = puzzle.get_final_state();
    let mut heuristic = 0;

    for y in 0..final_data.len() {
        for x in 0..final_data.len() {
            if final_data[y][x] != puzzle.data[y][x] {
                heuristic += 1;
            }
        }
    }

    heuristic
}

fn get_distance(x1 : &usize, y1: &usize, x2 : &usize, y2: &usize) -> usize {
    println!("{} - {},  {} - {}", x1, x2, y1, y2);
    let x = (*x1 as i32 - *x2 as i32).abs();
    let y = (*y1 as i32 - *y2 as i32).abs();
    println!("{} {}", x, y);

    x as usize + y as usize
}

fn get_correct_point(x1: &usize, y2: &usize, data: &(Vec<Vec<usize>>), value : &usize) -> (usize, usize) {
    let x2= 0;
    let y2= 0;

    for y2 in 0..data.len() {
        for x2 in 0..data.len() {
            if data[y2][x2] == *value {
                println!("found real value at : {} {}", y2, x2);
                return (x2, y2);
            }
        }
    }

    (x2, y2)
}

pub fn manhattan_distance(puzzle: Puzzle) -> usize {
    let final_data = puzzle.get_final_state();
    let mut heuristic = 0;

    for y in 0..puzzle.data.len() {
        for x in 0..puzzle.data.len() {
            if final_data[y][x] != puzzle.data[y][x] {
                let value = puzzle.data[y][x];
                println!("found false value at : {} {}", y, x);
                println!("value : {}", value);
                let (x2, y2) = get_correct_point(&x, &y, &final_data, &value);
                heuristic += get_distance(&x, &y, &x2, &y2);
            }
        }
    }

    heuristic as usize
}

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

            assert_eq!(heuristic, 5);
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

            assert_eq!(heuristic, 9);
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

            assert_eq!(heuristic, 8);
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

            assert_eq!(heuristic, 14);
        }
    }
}