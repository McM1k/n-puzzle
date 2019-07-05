use crate::node::Direction;
use crate::node::Node;
use crate::puzzle::Puzzle;
use std::thread::sleep;

#[derive(Clone)]
pub struct Graph {
    pub open_list: Vec<Node>,
    pub closed_list: Vec<Node>,
    pub start_node: Node,
    pub heuristic: fn(&Puzzle) -> usize,
    pub max_states: usize,
}

impl Graph {
    fn add_to_open_list(&mut self, new_node: Node) {
        self.open_list.push(new_node);
    }

    fn add_to_closed_list(&mut self, new_node: Node) {
        //        let mut found;
        //        while found = self.open_list.iter().position(|n| n == node) != None {
        //            self.open_list.remove(found.unwrap());
        //        }
        self.closed_list.push(new_node);
    }

    fn is_lower_cost(&self, node: &Node) -> bool {
        if self
            .open_list
            .iter()
            .any(|n| n == node && n.distance < node.distance)
        {
            return false;
        }
        return true;
    }

    fn add_in_sorted_open_list(&mut self, opt: Option<Box<Node>>) {
        if opt == None {
            return;
        }
        let node = *opt.unwrap();

        if self.closed_list.contains(&node) {
            return;
        }

        if self.is_lower_cost(&node) {
            self.open_list.insert(
                self.open_list
                    .iter()
                    .position(|n| {
                        (self.heuristic)(&(node.state)) + node.distance
                            <= (self.heuristic)(&(n.state)) + n.distance
                    })
                    .unwrap_or(0),
                node,
            );
        }

        if self.max_states < self.open_list.len() {
            self.max_states += 1;
        }
    }

    pub fn a_star_greedy(state: Puzzle, heuristic: fn(&Puzzle) -> usize) {
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state),
            heuristic,
            max_states: 1,
        };
        graph.add_to_open_list(graph.start_node.partial_copy());

        if graph.recursive_search(&graph.clone().start_node) {
            return;
        } else {
            panic!("Solution not found");
        }
    }

    fn recursive_search(&mut self, curr_node: &Node) -> bool {
        if curr_node == &Node::get_final_node(curr_node.state.size) {
            crate::print_result::print_data(self.clone(), curr_node.partial_copy());
            println!("{}", curr_node);
            return true;
        }
        //println!("{}, score : {}",curr_node.clone(), curr_node.clone().distance + (self.clone().heuristic)(&(curr_node.clone().state)));
        let mut next_nodes = Node::next_nodes_to_vec(curr_node);
        next_nodes.sort_by(|a, b| {
            (b.distance + (self.heuristic)(&(b.state)))
                .partial_cmp(&(a.distance + (self.heuristic)(&(a.state))))
                .unwrap()
        });

        let mut child_node;
        while !next_nodes.is_empty() {
            child_node = next_nodes.pop().unwrap();

            if !self.closed_list.contains(&child_node) && self.is_lower_cost(&child_node) {
                self.add_to_open_list(child_node.partial_copy());
                if self.recursive_search(&child_node) {
                    println!("{}", curr_node);
                    return true;
                }
            }
        }

        self.add_to_closed_list(curr_node.partial_copy());
        println!("down");
        false
    }

    pub fn a_star_gluttony(state: Puzzle, heuristic: fn(&Puzzle) -> usize) {
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state),
            heuristic,
            max_states: 1,
        };
        graph.add_to_open_list(graph.start_node.clone());

        let mut curr_node;
        while !graph.clone().open_list.is_empty() {
            curr_node = graph.open_list.pop().unwrap();
            println!(
                "{}, score : {}",
                curr_node.clone(),
                curr_node.clone().distance + (graph.clone().heuristic)(&(curr_node.clone().state))
            );
            //println!("{:?}", &graph.open_list);
            //println!("{:?}", &graph.closed_list);
            if curr_node == Node::get_final_node(curr_node.state.size) {
                crate::print_result::print_data(graph.clone(), curr_node.clone());
                crate::print_result::print_solution_with_retrieving(curr_node.clone());
                return;
            }
            graph.add_to_closed_list(curr_node.clone());
            curr_node = Node::calculate_next_nodes(curr_node);
            graph.add_in_sorted_open_list(curr_node.left_state.clone());
            graph.add_in_sorted_open_list(curr_node.upper_state.clone());
            graph.add_in_sorted_open_list(curr_node.lower_state.clone());
            graph.add_in_sorted_open_list(curr_node.right_state.clone());
        }
        panic!("The graph has been completely explored, yet the goal state hasn't been reached");
    }
}

#[cfg(test)]
mod graph_tests {
    /*
    mod is_lower_cost {

        #[test]
        fn lower_cost() {

        }

        #[test]
        fn not_lower_cost() {

        }
    }

    mod add_in_sorted_open_list {
        use crate::graph::*;
        use crate::node::*;
        use crate::puzzle::*;
        use std::ptr::null;

        #[test]
        fn dont_add_if_unnecessary() {
            let mut graph = Graph {
                open_list: vec![],
                closed_list: vec![],
                start_node: Node {
                    state: Puzzle {
                        data: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                        size: 3,
                    },
                    distance: 1,
                    upper_state: None,
                    lower_state: None,
                    left_state: None,
                    right_state: None,
                },
                heuristic: null(),
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]],
                    size: 3,
                },
                distance: 2,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]],
                    size: 3,
                },
                distance: 3,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            graph.add_in_sorted_open_list(Some(Box::new(node1)));
            graph.add_in_sorted_open_list(Some(Box::new(node2)));

            assert!(graph.open_list.iter().len() == 1);
        }

        #[test]
        fn list_is_sorted() {
            let mut graph = Graph {
                open_list: vec![],
                closed_list: vec![],
                start_node: Node {
                    state: Puzzle {
                        data: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                        size: 3,
                    },
                    distance: 1,
                    upper_state: None,
                    lower_state: None,
                    left_state: None,
                    right_state: None,
                },
                heuristic: null(),
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]],
                    size: 3,
                },
                distance: 2,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![vec![0, 2, 1], vec![3, 4, 5], vec![6, 8, 7]],
                    size: 3,
                },
                distance: 3,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            let node3 = Node {
                state: Puzzle {
                    data: vec![vec![0, 2, 5], vec![3, 4, 1], vec![6, 8, 7]],
                    size: 3,
                },
                distance: 1,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };

            graph.add_in_sorted_open_list(Some(Box::new(node1)));
            graph.add_in_sorted_open_list(Some(Box::new(node2)));
            graph.add_in_sorted_open_list(Some(Box::new(node3)));

            assert!(graph.open_list.is_sorted());
        }
    }*/
}
