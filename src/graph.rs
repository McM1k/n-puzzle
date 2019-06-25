use crate::node::Node;
use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Graph {
    open_list: Vec<Node>,
    closed_list: Vec<Node>,
    start_node: Node,
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

    fn add_if_higher_cost(&mut self, opt: Option<Box<Node>>) {
        if opt == None {
            ()
        }
        let node = *opt.unwrap();

        if self
            .open_list
            .iter()
            .any(|n| n == &node && n.distance > node.distance)
        {
            self.open_list.insert(
                self.open_list
                    .iter()
                    .position(|n| self.heuristic(node) <= self.heuristic(n))
                    .unwrap_or(0),
                node,
            );
        }
    }

    pub fn a_star(state: Puzzle) {
        let mut graph = Graph {
            open_list: vec![],
            closed_list: vec![],
            start_node: Node::new_starting_node(state),
        };
        graph.add_to_open_list(graph.start_node);

        let mut curr_node;
        while !graph.open_list.is_empty() {
            curr_node = graph.open_list.pop().unwrap(); /* TODO : choose the node with lowest score(heuristic + distance) */
            //            if curr_node == /*TODO : final node*/ {
            //                /* TODO : afficher les trucs demandes dans le sujet */
            //                ()
            //            }
            curr_node = Node::calculate_next_nodes(curr_node);
            graph.add_if_higher_cost(curr_node.left_state);
            graph.add_if_higher_cost(curr_node.upper_state);
            graph.add_if_higher_cost(curr_node.lower_state);
            graph.add_if_higher_cost(curr_node.right_state);

            graph.add_to_closed_list(curr_node);
        }
        panic!("The graph has been completely explored, yet the goal state hasn't been reached");
    }
}

#[cfg(test)]
mod graph_tests {
    mod add_if_higher_cost {}
    //    mod add_to_closed_list {
    //
    //    }
}
