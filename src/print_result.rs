use crate::graph::Graph;
use crate::node::Node;

pub fn print_data(graph: Graph, final_node: Node) {
    println!(
        "Total number of states ever selected in the opened set : {}\n",
        graph.closed_list.len() + graph.open_list.len()
    );
    println!(
        "Maximum number of states ever represented in  memory at the same time : {}\n",
        graph.max_states
    );
    println!("Number of moves : {}\n", final_node.distance);
    println!("solution sequence : \n");
}

pub fn print_solution_with_retrieving(final_node: Node, graph: Graph) {
    println!("{}", recursive_path(&final_node, graph));
}

fn recursive_path(curr_node: &Node, graph: Graph) -> String {
    let str = if curr_node.distance > 0 {
        let prev_node = match select_previous_node(curr_node.clone(), graph) {
            Ok(prev_node) => recursive_path(&prev_node, graph),
            Err(str) => {str}
        };
        prev_node
    } else {
        String::from("")
    };
    str + &curr_node.state.to_string() + "\n"
}

fn select_previous_node(curr_node: Node, graph: Graph) -> Result<Node, String> {
    let childs = Node::calculate_next_nodes(curr_node, |x| 0);
    while !childs.is_empty() {
        for open in graph.open_list {
            if open.distance == curr_node.distance - 1 && open.state == curr_node.state {
                return Ok(open);
            }
        }

        for closed in graph.closed_list {
            if closed.distance == curr_node.distance - 1 && closed.state == curr_node.state {
                return Ok(closed);
            }
        }
    }

    Err("".to_string())
}

#[cfg(test)]
mod print_result {
    mod select_previous_node {
        use crate::node::*;
        use crate::print_result::*;
        use crate::puzzle::*;

        #[test]
        fn normal_use_case() {
            let data1 = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };
            let data3 = vec![vec![1, 2, 3], vec![8, 4, 5], vec![7, 6, 0]];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                f_score: 1,
                distance: 3,
            };
            let data2 = vec![vec![1, 2, 3], vec![8, 4, 0], vec![7, 6, 5]];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            assert_eq!(select_previous_node(node2), Ok(node1));
        }

        #[test]
        #[should_panic]
        fn dead_end() {
            let data1 = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };
            select_previous_node(node1);
        }
    }

    mod recursive_path {
        use crate::node::*;
        use crate::print_result::*;
        use crate::puzzle::*;

        #[test]
        fn print_small_solution() {
            let data1 = vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 0,
            };
            let data2 = vec![vec![1, 2, 3], vec![8, 4, 0], vec![7, 6, 5]];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };
            let data3 = vec![vec![1, 2, 3], vec![8, 4, 5], vec![7, 6, 0]];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            assert_eq!(
                recursive_path(&node3),
                "1   2   3   \n\
                 8   0   4   \n\
                 7   6   5   \n\
                 \n\
                 1   2   3   \n\
                 8   4   0   \n\
                 7   6   5   \n\
                 \n\
                 1   2   3   \n\
                 8   4   5   \n\
                 7   6   0   \n\
                 \n"
            );
        }
    }
}
