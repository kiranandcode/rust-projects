use std::fmt::{Display, Formatter, Result, Write, Debug};

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    buffer: Vec<T>
}

fn initialize_matrix<T : Default>(rows: usize, cols: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(rows * cols);
    for i in 0..rows {
        for j in 0..cols {
            vec.push(Default::default());
        }
    }
    vec
}

impl<T : Display + Debug> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut buffer =  String::new();
        for i in 0..self.rows {
            write!(buffer, "{:?}", &self.buffer[i * self.cols .. (i+1) * self.cols])?;
        }

        write!(f, "{}", buffer)
    }
}

impl<T> Matrix<T>
    where T : Default {
    pub fn new(rows: usize, cols: usize) -> Self {
       Matrix {
           rows: rows,
           cols: cols,
           buffer: initialize_matrix(rows, cols)
       } 
    }

    pub fn get(&self, row : usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
           Some(& self.buffer[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row : usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
           Some(&mut self.buffer[row * self.cols + col])
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, row : usize, col: usize) -> &T {
           & self.buffer[row * self.cols + col]
    }

    pub fn get_mut_unchecked(&mut self, row : usize, col: usize) -> &mut T {
           &mut self.buffer[row * self.cols + col]
    }



}