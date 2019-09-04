use crate::node::Node;
use crate::puzzle::Puzzle;
use std::time::SystemTime;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Graph {
    pub open_list: Vec<Node>,
    pub closed_list: Vec<Node>,
    pub start_node: Node,
    pub final_node: Node,
    pub heuristic: fn(Puzzle, Puzzle) -> usize,
    pub max_states: usize,
}

impl Graph {
    fn add_to_open_list(&mut self, new_node: Node) {
        self.open_list.push(new_node);
    }

    fn add_to_closed_list(&mut self, new_node: Node) {
        self.closed_list.push(new_node);
    }

    fn is_lower_cost(&self, node: &Node) -> bool {
        if self
            .open_list
            .iter()
            .any(|n| n.state == node.state && n.f_score() > node.f_score())
        {
            return false;
        }
        true
    }

    fn update_cost(&mut self, new_node: Node) {
        let pos = self
            .open_list
            .iter()
            .position(|n| n.state == new_node.state)
            .unwrap();
        let old_node = self.open_list[pos].clone();
        if old_node.f_score() > new_node.f_score() {
            self.open_list.remove(pos);
            self.open_list.insert(
                self.open_list
                    .iter()
                    .position(|n| new_node.f_score() > n.f_score())
                    .unwrap_or_else(|| self.open_list.len()),
                new_node,
            );
        }
    }

    fn add_in_sorted_open_list(&mut self, node: Node) {
        if self.closed_list.contains(&node) {
            return;
        }

        if self.open_list.contains(&node) {
            self.update_cost(node);
            return;
        }

        self.open_list.insert(
            self.open_list
                .iter()
                .position(|n| node.f_score() > n.f_score())
                .unwrap_or_else(|| self.open_list.len()),
            node,
        );

        //println!("{:?}", self.open_list);
        if self.max_states < self.open_list.len() {
            self.max_states += 1;
        }
    }

    fn add_child_nodes_to_open_list(&mut self, parent: Node) {
        let mut childs =
            Node::calculate_next_nodes(parent, self.clone().final_node, self.heuristic);

        while !childs.is_empty() {
            let child = childs.pop();
            if child != None {
                self.add_in_sorted_open_list(child.unwrap());
            }
        }
    }

    pub fn recursive(state: Puzzle, heuristic: fn(Puzzle, Puzzle) -> usize) {
        let start_time = SystemTime::now();
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state.clone()),
            final_node: Node::get_final_node(state.size),
            heuristic,
            max_states: 1,
        };
        graph.add_to_open_list(graph.start_node.partial_copy());

        if graph.recursive_search(graph.clone().start_node, start_time) {
            return;
        } else {
            panic!("Solution not found");
        }
    }

    fn recursive_search(&mut self, curr_node: Node, start_time: SystemTime) -> bool {
        if curr_node == self.final_node {
            crate::print_result::print_data(self.clone(), curr_node.partial_copy(), start_time);
            println!("{}", curr_node);
            return true;
        }
        //println!("{}, score : {}",curr_node.clone(), curr_node.clone().distance + (self.clone().heuristic)(&(curr_node.clone().state)));
        let mut next_nodes =
            Node::calculate_next_nodes(curr_node.clone(), self.clone().final_node, self.heuristic);
        next_nodes.sort_by(|a, b| {
            (b.clone().g_score + (self.heuristic)(b.clone().state, self.clone().final_node.state))
                .partial_cmp(
                    &(a.clone().g_score
                        + (self.heuristic)(a.clone().state, self.clone().final_node.state)),
                )
                .unwrap()
        });

        let mut child_node;
        while !next_nodes.is_empty() {
            child_node = next_nodes.pop().unwrap();

            if !self.closed_list.contains(&child_node) && self.is_lower_cost(&child_node) {
                self.add_to_open_list(child_node.partial_copy());
                if self.recursive_search(child_node, start_time) {
                    println!("{}", curr_node);
                    return true;
                }
            }
        }

        self.add_to_closed_list(curr_node.partial_copy());
        false
    }

    pub fn a_star(state: Puzzle, heuristic: fn(Puzzle, Puzzle) -> usize) {
        let start_time = SystemTime::now();

        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state.clone()),
            final_node: Node::get_final_node(state.size),
            heuristic,
            max_states: 1,
        };
        graph.add_to_open_list(graph.start_node.clone());

        let mut curr_node;
        while !graph.open_list.is_empty() {
            curr_node = graph.open_list.pop().unwrap();

            if curr_node.state.data == graph.final_node.state.data {
                crate::print_result::print_solution_with_retrieving(
                    curr_node.clone(),
                    graph.clone(),
                );
                crate::print_result::print_data(graph.clone(), curr_node.clone(), start_time);

                return;
            }

            graph.add_to_closed_list(curr_node.clone());
            graph.add_child_nodes_to_open_list(curr_node);
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
    */
    mod add_in_sorted_open_list {
        use crate::graph::*;
        use crate::heuristic;
        use crate::node::*;
        use crate::puzzle::*;

        #[test]
        fn dont_add_if_unnecessary() {
            let mut graph = Graph {
                open_list: vec![],
                closed_list: vec![],
                start_node: Node::new_starting_node(Puzzle {
                    data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    size: 3,
                }),
                final_node: Node::get_final_node(3),
                heuristic: heuristic::manhattan_linear_conflict_heuristic,
                max_states: 1,
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![0, 1, 2, 3, 4, 5, 6, 8, 7],
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![0, 1, 2, 3, 4, 5, 6, 8, 7],
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            graph.add_in_sorted_open_list(node1);
            graph.add_in_sorted_open_list(node2);

            assert_eq!(graph.open_list.iter().len(), 1);
        }

        #[test]
        fn list_is_sorted() {
            let mut graph = Graph {
                open_list: vec![],
                closed_list: vec![],
                start_node: Node::new_starting_node(Puzzle {
                    data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                    size: 3,
                }),
                final_node: Node::get_final_node(3),
                heuristic: heuristic::manhattan_distance,
                max_states: 1,
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![1, 2, 3, 8, 0, 4, 7, 6, 5],
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![1, 2, 3, 8, 4, 5, 7, 6, 0],
                    size: 3,
                },
                f_score: 5,
                distance: 3,
            };

            let node3 = Node {
                state: Puzzle {
                    data: vec![1, 2, 3, 8, 4, 0, 7, 6, 5],
                    size: 3,
                },
                f_score: 3,
                distance: 2,
            };

            graph.add_in_sorted_open_list(node1.clone());
            graph.add_in_sorted_open_list(node2.clone());
            graph.add_in_sorted_open_list(node3.clone());

            assert_eq!(graph.open_list.len(), 3);
            assert_eq!(graph.open_list[0], node2);
            assert_eq!(graph.open_list[1], node3);
            assert_eq!(graph.open_list[2], node1);
        }
    }
}
