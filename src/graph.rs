use crate::node::Node;
use crate::puzzle::Puzzle;

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
            .any(|n| n.state == node.state && n.f_score < node.f_score)
        {
            return false;
        }
        true
    }

    fn add_in_sorted_open_list(&mut self, node: Node) {
        if self.closed_list.contains(&node) && !self.is_lower_cost(&node) {
            println!("mdr");
            return;
        }

        self.open_list.insert(
            self.open_list
                .iter()
                .position(|n| {
                    node.f_score > n.f_score
                })
                .unwrap_or(self.open_list.len()),
            node,
        );


        //println!("{:?}", self.open_list);

        if self.max_states < self.open_list.len() {
            self.max_states += 1;
        }
    }

    fn add_child_nodes_to_open_list(&mut self, parent: Node) {
        let mut childs = Node::calculate_next_nodes(parent, self.heuristic);

        while !childs.is_empty() {
            let child = childs.pop();
            if child != None {
                self.add_in_sorted_open_list(child.unwrap());
            }
        }
    }

    pub fn a_star_greedy(state: Puzzle, heuristic: fn(&Puzzle) -> usize) {
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state, heuristic),
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
        let mut next_nodes = Node::next_nodes_to_vec(curr_node, self.heuristic);
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

    pub fn a_star(state: Puzzle, heuristic: fn(&Puzzle) -> usize) {
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state, heuristic),
            heuristic,
            max_states: 1,
        };
        graph.add_to_open_list(graph.start_node.clone());

        let mut curr_node;
        while !graph.open_list.is_empty() {
            curr_node = graph.open_list.pop().unwrap();

            if curr_node.state.data == Node::get_final_node(curr_node.state.size).state.data {
                crate::print_result::print_data(graph.clone(), curr_node.clone());
                crate::print_result::print_solution_with_retrieving(curr_node.clone(), graph);
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
                    data: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                    size: 3,
                }, heuristic::manhattan_distance),
                heuristic: heuristic::manhattan_linear_conflict_heuristic,
                max_states: 1,
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]],
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 8, 7]],
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
                    data: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                    size: 3,
                }, heuristic::manhattan_distance),
                heuristic: heuristic::manhattan_distance,
                max_states: 1,
            };

            let node1 = Node {
                state: Puzzle {
                    data: vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]],
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };

            let node2 = Node {
                state: Puzzle {
                    data: vec![vec![1, 2, 3], vec![8, 4, 5], vec![7, 6, 0]],
                    size: 3,
                },
                f_score: 5,
                distance: 3,
            };

            let node3 = Node {
                state: Puzzle {
                    data: vec![vec![1, 2, 3], vec![8, 4, 0], vec![7, 6, 5]],
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
