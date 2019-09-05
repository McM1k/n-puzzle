use crate::graph::Graph;
use crate::node::Node;
use std::time::SystemTime;

pub fn print_data(graph: Graph, final_node: Node, start_time: SystemTime) {
    println!("Number of moves : {}", final_node.distance);
    println!("Time elapsed : {:?}", start_time.elapsed().unwrap());
    println!("Open list size : {}", graph.open_list.len());
    println!("Closed list size : {}", graph.closed_list.len());
    println!(
        "Total number of states ever represented (closed + open) : {}",
        graph.closed_list.len() + graph.open_list.len()
    );
    println!(
        "Maximum number of states represented in open list : {}",
        graph.max_states
    );
}

pub fn print_solution_with_retrieving(final_node: Node, graph: Graph) {
    println!("Solution sequence :");
    print!("{}", recursive_path(&final_node, graph));
}

fn recursive_path(curr_node: &Node, graph: Graph) -> String {
    let str = if curr_node.distance > 0 {
        match select_previous_node(curr_node.clone(), graph.clone()) {
            Ok(prev_node) => recursive_path(&prev_node, graph),
            Err(str) => str,
        }
    } else {
        String::from("")
    };
    str + &curr_node.state.to_string() + "\n"
}

fn select_previous_node(curr_node: Node, graph: Graph) -> Result<Node, String> {
    //println!("{}", curr_node.clone().distance);
    let mut childs = Node::calculate_next_nodes(curr_node.clone(), graph.final_node, |_x, _y| 0, graph.g_mul, graph.h_mul);
    while !childs.is_empty() {
        let child = childs.pop().unwrap();
        //println!("{}", child.clone());
        for open in graph.open_list.clone() {
            if open.distance == curr_node.distance - 1 && open.state == child.clone().state {
                return Ok(open.clone());
            }
        }

        for closed in graph.closed_list.clone() {
            if closed.distance == curr_node.distance - 1 && closed.state == child.state {
                return Ok(closed.clone());
            }
        }
    }

    Err("".to_string())
}

#[cfg(test)]
mod print_result {
    mod select_previous_node {
        use crate::graph::Graph;
        use crate::node::*;
        use crate::print_result::*;
        use crate::puzzle::*;

        #[test]
        fn normal_use_case() {
            let data1 = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };
            let data3 = vec![1, 2, 3, 8, 4, 5, 7, 6, 0];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                f_score: 1,
                distance: 3,
            };
            let data2 = vec![1, 2, 3, 8, 4, 0, 7, 6, 5];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            let graph = Graph {
                open_list: vec![node1.clone(), node2.clone(), node3.clone()],
                closed_list: vec![],
                start_node: node3.clone(),
                final_node: node1.clone(),
                heuristic: |_a, _b| 0,
                max_states: 0,
            };

            assert_eq!(select_previous_node(node2, graph), Ok(node1));
        }

        #[test]
        fn dead_end() {
            let data1 = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };
            let start_node = Node {
                state: Puzzle {
                    data: vec![1, 2, 3, 4, 5, 6, 7, 8, 0],
                    size: 0,
                },
                f_score: 0,
                distance: 0,
            };

            let graph = Graph {
                open_list: vec![node1.clone()],
                closed_list: vec![],
                start_node,
                final_node: node1.clone(),
                heuristic: |_a, _b| 0,
                max_states: 0,
            };

            assert_eq!(select_previous_node(node1, graph), Err("".to_string()));
        }
    }

    mod recursive_path {
        use crate::graph::Graph;
        use crate::node::*;
        use crate::print_result::*;
        use crate::puzzle::*;

        #[test]
        fn print_small_solution() {
            let data1 = vec![1, 2, 3, 8, 0, 4, 7, 6, 5];
            let node1 = Node {
                state: Puzzle {
                    data: data1,
                    size: 3,
                },
                f_score: 1,
                distance: 0,
            };
            let data2 = vec![1, 2, 3, 8, 4, 0, 7, 6, 5];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                f_score: 1,
                distance: 1,
            };
            let data3 = vec![1, 2, 3, 8, 4, 5, 7, 6, 0];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                f_score: 1,
                distance: 2,
            };

            let graph = Graph {
                open_list: vec![node1.clone(), node2.clone(), node3.clone()],
                closed_list: vec![],
                start_node: node3.clone(),
                final_node: node1,
                heuristic: |_a, _b| 0,
                max_states: 0,
            };

            assert_eq!(
                recursive_path(&node3, graph),
                "1  2  3  \n\
                 8  0  4  \n\
                 7  6  5  \n\
                 \n\
                 1  2  3  \n\
                 8  4  0  \n\
                 7  6  5  \n\
                 \n\
                 1  2  3  \n\
                 8  4  5  \n\
                 7  6  0  \n\
                 \n"
            );
        }
    }
}
