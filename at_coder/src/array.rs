use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::iter::Iterator;
use std::fmt;

pub struct DimensionArray3<T> ((usize,usize,usize), Vec<T>);


impl<T> Index<(usize,usize,usize)> for DimensionArray3<T> {
    type Output = T;

    fn index(&self, (x,y,z): (usize,usize,usize)) -> &T {
        let (s_x,s_y,s_z) = self.0;
        let s_ind = s_x * s_y;
        let index = s_ind * x + s_y * y + z;

        &self.1[index]
    }
}

impl<T> IndexMut<(usize,usize,usize)> for DimensionArray3<T> {

    fn index_mut(&mut self, (x,y,z): (usize,usize,usize)) -> &mut T {
        let (s_x,s_y,s_z) = self.0;
        let s_ind = s_x * s_y;

        let index = s_ind * x + s_y * y + z;
        &mut self.1[index]
    }
}



impl<T:Clone> DimensionArray3<T> {

    pub fn dimensions(&self) -> (usize,usize,usize) {
        self.0
    }

    pub fn new((x,y,z): (usize,usize,usize), initial:T) -> Self {
        let size = x * y * z;
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(initial.clone());
        }

        DimensionArray3 ((x,y,z), data)
    }
}

impl<T: fmt::Display> fmt::Display for DimensionArray3<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let (s_x,s_y,s_z) = self.0;
        write!(formatter, "DimensionArray3[\n");
        for i in 0..s_x {
            write!(formatter, "[");

            for j in 0..(s_y - 1) {
                for k in 0..(s_z - 1) {
                    write!(formatter, "{},", self[(i,j,k)]);
                }
                write!(formatter, "{},\n", self[(i,j,s_z-1)]);
            }

            for k in 0..(s_z - 1) {
                write!(formatter, "{},", self[(i,s_y - 1,k)]);
            }
            write!(formatter, "{}", self[(i,s_y - 1,s_z-1)]);


            write!(formatter, "]\n");
        }
        write!(formatter, "]")
    }
}


impl<T: fmt::Debug> fmt::Debug for DimensionArray3<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let (s_x,s_y,s_z) = self.0;
        write!(formatter, "DimensionArray3[\n");
        for i in 0..s_x {
            write!(formatter, "[");

            for j in 0..(s_y - 1) {
                for k in 0..(s_z - 1) {
                    write!(formatter, "{:?},", self[(i,j,k)]);
                }
                write!(formatter, "{:?},\n", self[(i,j,s_z-1)]);
            }

            for k in 0..(s_z - 1) {
                write!(formatter, "{:?},", self[(i,s_y - 1,k)]);
            }
            write!(formatter, "{:?}", self[(i,s_y - 1,s_z-1)]);


            write!(formatter, "]\n");
        }
        write!(formatter, "]")
    }
}
