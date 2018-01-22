use std::collections::HashMap;

use std::fmt::{Debug, Display, Formatter, Write, self};


pub struct GraphVizDiGraph {
    name: String,
   edges: Vec<GraphVizEdge>,              // reference edges
   nodes: HashMap<String, GraphVizNode> // reference to any labels given to nodes
}

pub struct GraphVizEdge {
    label : Option<String>,
    fromNode : String, 
    toNode : String,
    attributes: Option<HashMap<String, String>>
}

pub struct GraphVizNode {
    label: Option<String>,
    attributes: Option<HashMap<String, String>>
}

impl GraphVizNode {
    fn new() -> Self {
        GraphVizNode {
            label: None,
            attributes: None
        }
    }

    pub fn with_label(&mut self, name: String) {
        self.label = Some(name);
    }

    pub fn with_attribute(&mut self, attribute : String, value : String) {
        match self.attributes.take() {
            Some(mut table) => {
                table.insert(attribute, value);
                self.attributes = Some(table);
            }
            None => {
                let mut table = HashMap::new();
                table.insert(attribute, value);
                self.attributes = Some(table);
            }
        }
    }

    pub fn append_to_label(&mut self, name : String) {
        match self.label.take() {
            Some(mut old_label) => {
                old_label.push_str(&name);
                self.label = Some(old_label);
            }
            None => {
                self.label = Some(name);
            }
        }
    }
}

impl GraphVizDiGraph {
    pub fn new(name : String) -> Self {
        GraphVizDiGraph {
            name,
            edges: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn with_node(&mut self, node_id : String) -> &mut GraphVizNode {
        self.nodes.entry(node_id).or_insert(GraphVizNode::new())
    }


    pub fn with_labeled_node(&mut self, node_id : String, label : String) -> &mut GraphVizNode {
        let mut node = self.nodes.entry(node_id).or_insert(GraphVizNode::new());
        node.with_label(label);
        node
    }

    pub fn add_edge(&mut self, node_id : String, to_id : String) -> &mut GraphVizEdge {
            if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&to_id) {
                panic!("Attempted to add edge between unknown edges");
            }
            self.edges.push(GraphVizEdge {
                label: None,
                fromNode: node_id,
                toNode: to_id,
                attributes: None
            });

            self.edges.last_mut().unwrap()
    }

    pub fn add_labelled_edge(&mut self, node_id : String, to_id : String, label : String) -> &mut GraphVizEdge {
            if !self.nodes.contains_key(&node_id) || !self.nodes.contains_key(&to_id) {
                panic!("Attempted to add edge between unknown edges");
            }
            self.edges.push(GraphVizEdge {
                label: Some(label),
                fromNode: node_id,
                toNode: to_id,
                attributes: None
            });
 
            self.edges.last_mut().unwrap()
    }
}

impl GraphVizEdge {

    pub fn append_to_label(&mut self, name : String) {
        match self.label.take() {
            Some(mut old_label) => {
                old_label.push_str(&name);
                self.label = Some(old_label);
            }
            None => {
                self.label = Some(name);
            }
        }
    }



    pub fn with_attribute(&mut self, attribute : String, value : String) {
        match self.attributes.take() {
            Some(mut table) => {
                table.insert(attribute, value);
                self.attributes = Some(table);
            }
            None => {
                let mut table = HashMap::new();
                table.insert(attribute, value);
                self.attributes = Some(table);
            }
        }

    }
}

impl Display for GraphVizEdge {


    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if(self.label.is_some() || self.attributes.is_some()) {
            let mut buffer = format!("[");
            let mut written = false;

            self.attributes.as_ref().map(|table| {
                let mut iter = table.iter();
                let mut item = iter.next();
                while let Some((attr, value)) = item {
                    written = true;
                    write!(buffer, "{}={}", attr, value);
                    item = iter.next();
                    if item.is_some() {
                        write!(buffer, ", ");
                    }
                }
            });

           self.label.as_ref().map(|label_str| {
                if written {
                        write!(buffer, ",");
                }
 
                write!(buffer, "label=\"{}\"", label_str);
            });
            write!(buffer, "]");


            write!(f, "{} -> {} {}", self.fromNode, self.toNode, buffer) 
        } else {
            write!(f, "{} -> {}", self.fromNode, self.toNode) 
        }
    }
}

impl Display for GraphVizNode {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        let mut buffer = format!("[");
        let mut written = false;
        self.attributes.as_ref().map(|table| {
            let mut iter = table.iter();
            let mut item = iter.next();
            while let Some((attr, value)) = item {
                written = true;
                write!(buffer, "{}={}", attr, value);
                item = iter.next();
                if item.is_some() {
                    write!(buffer, ",");
                }
            }
        });


        self.label.as_ref().map(|label_str| {
            if written {
                    write!(buffer, ",");
            }

            write!(buffer, "label=\"{}\"", label_str);
        });
        write!(buffer, "]");


        write!(f, "{}", buffer) 
    }
}

impl Display for GraphVizDiGraph {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buffer =  format!("digraph {} {{\n", self.name);
        for (node_id, node) in self.nodes.iter() {
            if node.label.is_some() || node.attributes.is_some() {
                write!(buffer, "{} {};\n", node_id, node);
            }
        }
        for edge in self.edges.iter() {
            write!(buffer, "{};\n", edge);
        }

        write!(buffer, "}}");
        write!(f, "{}", buffer)
    }

}