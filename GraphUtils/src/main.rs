extern crate regex;
mod matrix;
mod graph;
mod graphviz;
use matrix::Matrix;
use graph::Graph;
use graphviz::GraphVizDiGraph;


fn main() {
    let mut vec = vec![0,1];
    let mut mat : Matrix<i32> = Matrix::new(4, 4);
    println!("{:?}", vec);
    {
        let reference : &mut i32 = &mut vec[0];
        *reference = 1;
    }
    println!("{:?}", vec);

    println!("Matrix: {}", mat);
        if let Some(reference) = mat.get_mut(0,3) {
            *reference = 10;
        }
    println!("Matrix: {}", mat);

    println!("Hello world");

    let graph = Graph::<i32>::from_file("./graph.grp").unwrap();
    println!("{}", graph);
    println!("{:?}", graph.dfs(0));
}