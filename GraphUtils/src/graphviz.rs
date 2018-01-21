use std::collections::HashMap;



pub struct GraphVizDiGraph {
   edges: Vec<GraphVizEdge>,              // reference edges
   nodes: HashMap<String, Option<String>> // reference to any labels given to nodes
}

pub struct GraphVizEdge {
    label : Option<String>,
    fromNode : String, 
    toNode : String 
}
