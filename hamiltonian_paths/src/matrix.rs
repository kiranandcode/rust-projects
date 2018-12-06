use std::ops::{Index, IndexMut};
use std::iter::Iterator;
use std::fmt;



#[derive(Debug,Clone)]
pub struct Matrix2D {
    rows: usize,
    cols: usize,
    data: Vec<u32>
}


pub struct Matrix2DColumnIterator<'a> {
    base: &'a Matrix2D,
    pos: usize,
}

impl<'a> Iterator for Matrix2DColumnIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(1)
    }
}

impl fmt::Display for Matrix2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x{} [\n", "Matrix", self.rows, self.cols);

        for i in 0..self.rows-1 {
            write!(f, "\t{}, ", self[(i,0)]);

            for j in 1..self.cols-1 {
                write!(f, "{}, ", self[(i,j)]);
            }

            write!(f, "{},\n", self[(i,self.cols-1)]);
        }


        write!(f, "\t{}, ", self[(self.rows-1,0)]);
        for j in 1..self.cols-1 {
            write!(f, "{}, ", self[(self.rows-1,j)]);
        }
        write!(f, "{}\n]\n", self[(self.rows-1,self.cols-1)])
    }
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


