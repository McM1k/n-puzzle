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

pub fn print_solution_with_retrieving(final_node: Node) {
    println!("{}", recursive_path(&final_node));
}

fn recursive_path(curr_node: &Node) -> String {
    let str = if curr_node.distance > 0 {
        let prev_node = select_previous_node(curr_node.clone());
        recursive_path(&prev_node)
    } else {
        String::from("")
    };
    str + &curr_node.state.to_string() + "\n"
}

fn select_previous_node(node: Node) -> Node {
    if node.left_state != None && (*node.clone().left_state.unwrap()).distance == node.distance - 1
    {
        *node.left_state.unwrap()
    } else if node.right_state != None
        && (*node.clone().right_state.unwrap()).distance == node.distance - 1
    {
        *node.right_state.unwrap()
    } else if node.upper_state != None
        && (*node.clone().upper_state.unwrap()).distance == node.distance - 1
    {
        *node.upper_state.unwrap()
    } else if node.lower_state != None
        && (*node.clone().lower_state.unwrap()).distance == node.distance - 1
    {
        *node.lower_state.unwrap()
    } else {
        panic!("dead end while going through the solving path\n");
    }
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
                distance: 1,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };
            let data3 = vec![vec![1, 2, 3], vec![8, 4, 5], vec![7, 6, 0]];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                distance: 3,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };
            let data2 = vec![vec![1, 2, 3], vec![8, 4, 0], vec![7, 6, 5]];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                distance: 2,
                upper_state: None,
                lower_state: Some(Box::new(node3)),
                left_state: Some(Box::new(node1.clone())),
                right_state: None,
            };

            assert_eq!(select_previous_node(node2), node1);
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
                distance: 2,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
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
                distance: 0,
                upper_state: None,
                lower_state: None,
                left_state: None,
                right_state: None,
            };
            let data2 = vec![vec![1, 2, 3], vec![8, 4, 0], vec![7, 6, 5]];
            let node2 = Node {
                state: Puzzle {
                    data: data2,
                    size: 3,
                },
                distance: 1,
                upper_state: None,
                lower_state: None,
                left_state: Some(Box::new(node1)),
                right_state: None,
            };
            let data3 = vec![vec![1, 2, 3], vec![8, 4, 5], vec![7, 6, 0]];
            let node3 = Node {
                state: Puzzle {
                    data: data3,
                    size: 3,
                },
                distance: 2,
                upper_state: Some(Box::new(node2)),
                lower_state: None,
                left_state: None,
                right_state: None,
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
