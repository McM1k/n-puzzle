use crate::puzzle::Puzzle;
use std::cmp::Ordering;
use std::fmt;
extern crate strum;
extern crate strum_macros;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/*
#[derive(EnumIter)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}
*/

#[derive(Clone, Eq)]
pub struct Node {
    pub state: Puzzle,
    pub distance: usize,
    pub f_score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score && self.state == other.state
    }
}

/*
impl Clone for Node {
    fn clone(&self) -> Self {

        Node {
            state: self.state.clone(),
            distance: self.distance,
            upper_state: None,
            lower_state: None,
            left_state: None,
            right_state: None,
        }
    }
}*/

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{0:?}, {1}, {2}",
            self.state, self.distance, self.f_score
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} nodes away from the start position", self.distance)?;
        writeln!(f, "{}", self.state)?;
        Ok(())
    }
}

impl Node {
    pub fn partial_copy(&self) -> Node {
        Node {
            state: self.state.clone(),
            distance: self.distance,
            f_score: self.f_score,
        }
    }

    pub fn new_starting_node(state: Puzzle) -> Node {
        Node {
            state: state.clone(),
            distance: 0,
            f_score: 0,
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
        let mut new_puzzle = puzzle.clone();
        new_puzzle.set_value(x, y, puzzle.get_value(x_next, y_next));
        new_puzzle.set_value(x_next, y_next, puzzle.get_value(x, y));

        new_puzzle
    }

    /*
    pub fn get_void_position(puzzle: &Puzzle) -> (usize, usize) {
        puzzle.get_position(0)
    }
    */

    /*
    pub fn next_nodes_to_vec(
        node: &Node,
        final_node: &Node,
        heuristic: fn(&Puzzle, &Puzzle) -> usize,
    ) -> Vec<Node> {
        let mut next_nodes = vec![];
        let mut curr_puzzle;

        for dir in Direction::iter() {
            curr_puzzle = Node::calculate_next_state(&node.state, dir);
            if curr_puzzle != None {
                next_nodes.push(Node {
                    state: curr_puzzle.clone().unwrap(),
                    distance: node.distance + 1,
                    f_score: node.distance
                        + 1
                        + heuristic(&curr_puzzle.unwrap(), &final_node.state),
                });
            }
        }

        next_nodes
    }
    */

    pub fn calculate_next_states(puzzle: &Puzzle) -> Vec<Puzzle> {
        let (x, y) = puzzle.get_position(0);
        let mut childs= vec![];

        if y != 0{
            childs.push(Node::swap_two_positions(puzzle, x, y, x, y - 1));
        }
        if y != puzzle.size - 1 {
            childs.push(Node::swap_two_positions(puzzle, x, y, x, y + 1));
        }
        if x != 0 {
            childs.push(Node::swap_two_positions(puzzle, x, y, x - 1, y));
        }
        if x != puzzle.size - 1 {
            childs.push(Node::swap_two_positions(puzzle, x, y, x + 1, y));
        }

        childs
    }

    pub fn calculate_next_nodes(
        parent: Node,
        final_node: Node,
        heuristic: fn(&Puzzle, &Puzzle) -> usize,
    ) -> Vec<Node> {
        let mut childs = Vec::new();
        let next_states= Node::calculate_next_states(&parent.state);
        for state in next_states {
            childs.push(Node {
                state: state,
                distance: parent.distance + 1,
                f_score: parent.distance + 1 + heuristic(&state, &final_node.state),
                });
        }

        childs
    }

    pub fn get_final_node(size: usize) -> Node {
        Node {
            state: Puzzle::get_final_state(size),
            distance: 0,
            f_score: 0,
        }
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

            assert_eq!(Node::calculate_next_state(&puzzle, Direction::Up), None);
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
                f_score: len,
            };

            let len2 = 0;
            let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]];
            let puzzle2 = Puzzle { data: data2, size };
            let next = Some(Box::new(node.clone()));
            let node2 = Node {
                state: puzzle2,
                distance: len2,
                f_score: len2,
            };

            assert_eq!(node, node2);
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
                f_score: len,
            };

            let data2 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let puzzle2 = Puzzle { data: data2, size };
            let node2 = Node {
                state: puzzle2,
                distance: len,
                f_score: len + 1,
            };

            assert_ne!(node, node2);
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
