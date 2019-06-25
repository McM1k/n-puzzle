use crate::puzzle::Puzzle;
use std::cmp;
use std::fmt;
use std::cmp::Ordering;

pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub state: Puzzle,
    pub distance: usize,
    pub upper_state: Option<Box<Node>>,
    pub lower_state: Option<Box<Node>>,
    pub left_state: Option<Box<Node>>,
    pub right_state: Option<Box<Node>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.state)?;
        write!(f, "{} nodes away from the start position\n", self.distance)?;
        Ok(())
    }
}

impl cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Node {
    pub fn new_starting_node(state: Puzzle) -> Node {
        Node {
            state,
            distance: 0,
            upper_state: None,
            lower_state: None,
            left_state: None,
            right_state: None,
        }
    }

    //	pub fn add_node(curr_node: &Node, dir: Direction) -> Option<Node> {

    //	}

    pub fn swap_two_positions(
        puzzle: &Puzzle,
        x: usize,
        y: usize,
        x_next: usize,
        y_next: usize,
    ) -> Puzzle {
        let mut new_data = puzzle.data.clone();
        new_data[y][x] = puzzle.data[y_next][x_next];
        new_data[y_next][x_next] = puzzle.data[y][x];
        let size = new_data.len();

        Puzzle {
            data: new_data,
            size,
        }
    }

    pub fn get_void_position(puzzle: &Puzzle) -> (usize, usize) {
        let x = 0;
        let y = 0;

        for y in 0..puzzle.data.len() {
            for x in 0..puzzle.data[y].len() {
                if puzzle.data[y][x] == 0 {
                    return (x, y);
                }
            }
        }

        (x, y)
    }

    pub fn calculate_next_state(puzzle: &Puzzle, dir: Direction) -> Option<Puzzle> {
        let (x, y) = Node::get_void_position(puzzle);
        let mut new_puzzle;

        match dir {
            Direction::Up => {
                if y == 0 {
                    return None;
                } else {
                    new_puzzle = Node::swap_two_positions(puzzle, x, y, x, y - 1);
                }
            }
            Direction::Down => {
                if y == puzzle.size - 1 {
                    return None;
                } else {
                    new_puzzle = Node::swap_two_positions(puzzle, x, y, x, y + 1);
                }
            }
            Direction::Left => {
                if x == 0 {
                    return None;
                } else {
                    new_puzzle = Node::swap_two_positions(puzzle, x, y, x - 1, y);
                }
            }
            Direction::Right => {
                if x == puzzle.size - 1 {
                    return None;
                } else {
                    new_puzzle = Node::swap_two_positions(puzzle, x, y, x + 1, y);
                }
            }
        }

        Some(new_puzzle)
    }

    pub fn calculate_next_nodes(mut node: Node) -> Node {
        match Node::calculate_next_state(&node.state, Direction::Left) {
            Some(new_puzzle) => {
                node.left_state = Some(Box::new(Node {
                    state: new_puzzle,
                    distance: node.distance + 1,
                    left_state: None,
                    upper_state: None,
                    lower_state: None,
                    right_state: None,
                }))
            }
            None => node.left_state = None,
        }
        match Node::calculate_next_state(&node.state, Direction::Right) {
            Some(new_puzzle) => {
                node.right_state = Some(Box::new(Node {
                    state: new_puzzle,
                    distance: node.distance + 1,
                    left_state: None,
                    upper_state: None,
                    lower_state: None,
                    right_state: None,
                }))
            }
            None => node.right_state = None,
        }
        match Node::calculate_next_state(&node.state, Direction::Down) {
            Some(new_puzzle) => {
                node.lower_state = Some(Box::new(Node {
                    state: new_puzzle,
                    distance: node.distance + 1,
                    left_state: None,
                    upper_state: None,
                    lower_state: None,
                    right_state: None,
                }))
            }
            None => node.lower_state = None,
        }
        match Node::calculate_next_state(&node.state, Direction::Up) {
            Some(new_puzzle) => {
                node.upper_state = Some(Box::new(Node {
                    state: new_puzzle,
                    distance: node.distance + 1,
                    left_state: None,
                    upper_state: None,
                    lower_state: None,
                    right_state: None,
                }))
            }
            None => node.upper_state = None,
        }

        node
    }
}

#[cfg(test)]
mod node_tests {
    mod swap_two_positions {
        use crate::node::*;

        #[test]
        fn down_swap() {
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };
            let result_data = vec![vec![3, 1, 2], vec![0, 4, 5], vec![6, 8, 7]];
            let result_puzzle = Puzzle {
                data: result_data,
                size,
            };

            assert_eq!(Node::swap_two_positions(&puzzle, 0, 0, 0, 1), result_puzzle);
        }

        #[test]
        fn right_swap() {
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };
            let result_data = vec![vec![1, 0, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let result_puzzle = Puzzle {
                data: result_data,
                size,
            };

            assert_eq!(Node::swap_two_positions(&puzzle, 0, 0, 1, 0), result_puzzle);
        }
    }

    mod get_void_position {
        use crate::node::*;

        #[test]
        fn center_pos() {
            let size = 3;
            let data = vec![vec![4, 1, 2], vec![3, 0, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };

            let (x, y) = Node::get_void_position(&puzzle);
            assert!(x == 1 && y == 1);
        }

        #[test]
        fn lower_right_pos() {
            let size = 3;
            let data = vec![vec![4, 1, 2], vec![3, 7, 5], vec![6, 8, 0]];
            let puzzle = Puzzle { data, size };

            let (x, y) = Node::get_void_position(&puzzle);
            assert!(x == 2 && y == 2);
        }

        #[test]
        fn lower_left_pos() {
            let size = 3;
            let data = vec![vec![4, 1, 2], vec![3, 7, 5], vec![0, 8, 6]];
            let puzzle = Puzzle { data, size };

            let (x, y) = Node::get_void_position(&puzzle);
            assert!(x == 0 && y == 2);
        }
    }

    mod calculate_next_state {
        use crate::node::*;

        #[test]
        fn wrong_direction() {
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };

            assert!(Node::calculate_next_state(&puzzle, Direction::Up) == None);
        }

        #[test]
        fn good_direction() {
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };

            let result_data = vec![vec![3, 1, 2], vec![0, 4, 5], vec![6, 8, 7]];
            let result_puzzle = Puzzle {
                data: result_data,
                size,
            };

            assert!(Node::calculate_next_state(&puzzle, Direction::Down).unwrap() == result_puzzle);
        }
    }

    mod partial_eq {
        use crate::node::*;

        #[test]
        fn equals() {
            let len = 0;
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };
            let node = Node {
                state: puzzle,
                distance: len,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            let len2 = 1;
            let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle2 = Puzzle { data: data2, size };
            let next = Some(Box::new(node.clone()));
            let node2 = Node {
                state: puzzle2,
                distance: len2,
                upper_state: None,
                lower_state: next,
                left_state: None,
                right_state: None,
            };

            assert!(node == node2);
        }

        #[test]
        fn not_equals() {
            let len = 0;
            let size = 3;
            let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle = Puzzle { data, size };
            let node = Node {
                state: puzzle,
                distance: len,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let puzzle2 = Puzzle { data: data2, size };
            let node2 = Node {
                state: puzzle2,
                distance: len,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            assert!(node != node2);
        }
    }

    // mod add_node {
    // 	use crate::graph::*;

    // 	#[test]
    // 	fn node_is_correctly_added() {
    // 		let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
    // 		let puzzle = Puzzle{data, 3};
    // 		let node = Node::new_starting_node(puzzle);

    // 		let result_data = vec![vec![1, 0, 2], vec![3, 4, 5], vec![6, 8, 7]];
    // 		let result_puzzle = Puzzle{result_data, 3};

    // 		assert_eq!(Node::add_node(node, Direction::Left), Some(result_puzzle));
    // 	}

    // 	fn trying_an_unpossible_move() {
    // 		let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
    // 		let puzzle = Puzzle{data, 3};
    // 		let node = Node::new_starting_node(puzzle);

    // 		assert_eq!(Node::add_node(node, Direction::Left), None);
    // 	}
    // }
}
