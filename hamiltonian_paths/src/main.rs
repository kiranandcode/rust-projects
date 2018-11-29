use std::ops::{Index, IndexMut};

pub struct Matrix2D {
    rows: usize,
    cols: usize,
    data: Vec<u32>
}


impl Matrix2D {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..(rows * cols) {
            data.push(0);
        }

        Matrix2D {
            rows,
            cols,
            data
        }
    }
}

impl Index<(usize,usize)> for Matrix2D {
    type Output = u32;

    fn index(&self, (row, col):(usize,usize)) -> &u32 {
        assert!(row < self.rows);
        assert!(col < self.cols);
        assert!(row * self.cols + col < self.rows * self.cols);

        &self.data[row * self.cols + col]
    }
}


impl IndexMut<(usize,usize)> for Matrix2D {

    fn index_mut(&mut self, (row, col):(usize,usize)) -> &mut u32 {
        assert!(row < self.rows);
        assert!(col < self.cols);
        assert!(row * self.cols + col < self.rows * self.cols);

        &mut self.data[row * self.cols + col]
    }
}




pub struct Graph {
    nodes: u32,
    edges: Matrix2D
}

fn main() {
    println!("Hello, world!");
}
