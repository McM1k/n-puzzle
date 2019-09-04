use crate::puzzle::Puzzle;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Eq)]
pub struct Node {
    pub state: Puzzle,
    pub g_score: usize,
    pub h_score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score().cmp(&other.f_score())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{0:?}, {1}, {2}",
            self.state, self.g_score, self.h_score
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} nodes away from the start position", self.g_score)?;
        writeln!(f, "{}", self.state)?;
        Ok(())
    }
}

impl Node {
    pub fn f_score(&self) -> usize {
        self.g_score + self.h_score
    }

    pub fn partial_copy(&self) -> Node {
        Node {
            state: self.state.clone(),
            g_score: self.g_score,
            h_score: self.h_score,
        }
    }

    pub fn new_starting_node(state: Puzzle) -> Node {
        Node {
            state: state.clone(),
            g_score: 0,
            h_score: 0,
        }
    }

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

    pub fn calculate_next_states(puzzle: &Puzzle) -> Vec<Puzzle> {
        let (x, y) = puzzle.get_position(0);
        let mut childs = vec![];

        if y != 0 {
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
        heuristic: fn(Puzzle, Puzzle) -> usize,
    ) -> Vec<Node> {
        let mut childs = Vec::new();
        let next_states = Node::calculate_next_states(&parent.state);
        for state in next_states {
            childs.push(Node {
                state: state.clone(),
                g_score: parent.g_score + 1,
                h_score: heuristic(state, final_node.clone().state),
            });
        }

        childs
    }

    pub fn get_final_node(size: usize) -> Node {
        Node {
            state: Puzzle::get_final_state(size),
            g_score: 0,
            h_score: 0,
        }
    }
}

#[cfg(test)]
mod node_tests {
    mod swap_two_positions {
        use super::super::Puzzle;
        use crate::node::Node;

        #[test]
        fn down_swap() {
            let size = 3;
            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };
            let result_data = vec![3, 1, 2, 0, 4, 5, 6, 8, 7];
            let result_puzzle = Puzzle {
                data: result_data,
                size,
            };

            assert_eq!(Node::swap_two_positions(&puzzle, 0, 0, 0, 1), result_puzzle);
        }

        #[test]
        fn right_swap() {
            let size = 3;
            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };
            let result_data = vec![1, 0, 2, 3, 4, 5, 6, 8, 7];
            let result_puzzle = Puzzle {
                data: result_data,
                size,
            };

            assert_eq!(Node::swap_two_positions(&puzzle, 0, 0, 1, 0), result_puzzle);
        }
    }

    mod calculate_next_state {
        use super::super::Puzzle;
        use crate::node::Node;

        #[test]
        fn two_dir() {
            let size = 3;

            let puzzle = Puzzle {
                data: vec![0, 1, 2, 3, 4, 5, 6, 8, 7],
                size,
            };

            let result1 = Puzzle {
                data: vec![3, 1, 2, 0, 4, 5, 6, 8, 7],
                size,
            };

            let result2 = Puzzle {
                data: vec![1, 0, 2, 3, 4, 5, 6, 8, 7],
                size,
            };

            assert_eq!(Node::calculate_next_states(&puzzle), vec![result1, result2]);
        }
    }

    mod partial_eq {
        use super::super::Puzzle;
        use crate::node::Node;

        #[test]
        fn equals() {
            let len = 0;
            let size = 3;
            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };
            let node = Node {
                state: puzzle,
                distance: len,
                f_score: len,
            };

            let len2 = 0;
            let data2 = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle2 = Puzzle { data: data2, size };
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
            let data = vec![0, 1, 2, 3, 4, 5, 6, 8, 7];
            let puzzle = Puzzle { data, size };
            let node = Node {
                state: puzzle,
                distance: len,
                f_score: len,
            };

            let data2 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let puzzle2 = Puzzle { data: data2, size };
            let node2 = Node {
                state: puzzle2,
                distance: len,
                f_score: len + 1,
            };

            assert_ne!(node, node2);
        }
    }
}
