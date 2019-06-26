use crate::graph::Graph;
use crate::node::Node;

fn print_result(graph: Graph, final_node: Node) {
    println!("Total number of states ever selected in the opened set : {}\n", graph.closed_list.len() + graph.open_list.len());
    println!("Maximum number of states ever represented in  memory at the same time : {}\n", graph.max_states);
    println!("Number of moves : {}\n", final_node.distance);
    println!("solution sequence : \n");
    /* TODO : print sequence */
}

#[cfg(test)]
mod print_result{

}