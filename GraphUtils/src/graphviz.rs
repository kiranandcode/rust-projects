use std::collections::HashMap;

use std::fmt::{Debug, Display, Formatter, Write, self};


pub struct GraphVizDiGraph {
    name: String,
   edges: Vec<GraphVizEdge>,              // reference edges
   nodes: HashMap<String, Option<String>> // reference to any labels given to nodes
}

pub struct GraphVizEdge {
    label : Option<String>,
    fromNode : String, 
    toNode : String 
}


impl GraphVizDiGraph {
    pub fn new(name : String) -> Self {
        GraphVizDiGraph {
            name,
            edges: Vec::new(),
            nodes: HashMap::new()
        }
    }

    pub fn with_node(&mut self, node_id : String) {
        self.nodes.entry(node_id).or_insert(None);
    }

    pub fn with_labeled_node(&mut self, node_id : String, label : String) {
        self.nodes.insert(node_id, Some(label));
    }

    pub fn add_edge(&mut self, node_id : String, to_id : String) {
            if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&to_id) {
                panic!("Attempted to add edge between unknown edges");
            }
            self.edges.push(GraphVizEdge {
                label: None,
                fromNode: node_id,
                toNode: to_id
            });
    }

    pub fn add_labelled_edge(&mut self, node_id : String, to_id : String, label : String) {
            if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&to_id) {
                panic!("Attempted to add edge between unknown edges");
            }
            self.edges.push(GraphVizEdge {
                label: Some(label),
                fromNode: node_id,
                toNode: to_id
            });
 
    }
}

impl Display for GraphVizEdge {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if(self.label.is_some()) {
            write!(f, "{} -> {} [label={}]", self.fromNode, self.toNode, self.label.as_ref().unwrap()) 
        } else {
            write!(f, "{} -> {}", self.fromNode, self.toNode) 
        }
    }
}

impl Display for GraphVizDiGraph {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer =  format!("digraph {} {{\n", self.name);
        for (node_id, node_label) in self.nodes.iter() {
            node_label.as_ref().map(|label| {
                write!(buffer, "{} [label={}];\n", node_id, label);
            });
        }
        for edge in self.edges.iter() {
            write!(buffer, "{};\n", edge);
        }

        write!(buffer, "}}");
        write!(f, "{}", buffer)
    }

}