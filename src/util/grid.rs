use crate::util::point::Point;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

pub struct Grid<T> {
    pub col_count: usize,
    pub row_count: usize,
    pub contents: Vec<T>,
}

impl<T> Grid<T> {
    pub fn checked_get(&self, row: &Option<usize>, col: &Option<usize>) -> Option<&T> {
        if row.is_none() || col.is_none() {
            return None;
        }

        let row = row.unwrap();
        let col = col.unwrap();

        if row > self.row_count - 1 || col > self.col_count - 1 {
            return None;
        }
        return self.contents.get(self.col_count * row + col);
    }

    pub fn find_index<P>(&self, mut predicate: P) -> Option<(usize, usize)>
    where
        P: FnMut(&T) -> bool,
    {
        let find_result = self
            .contents
            .iter()
            .enumerate()
            .find(move |(_index, value)| predicate(value));

        match find_result {
            None => None,
            Some((index, _)) => {
                let row = index / self.col_count;
                Some((row, index - row * self.col_count))
            }
        }
    }
}

impl<T: fmt::Display> Grid<T> {
    pub fn print(&self) {
        for row in 0..self.row_count {
            for col in 0..self.col_count {
                let point = Point::new(col, row);
                print!("{}", self[point]);
            }
            println!();
        }
        println!();
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        return &self.contents[self.col_count * point.y + point.x];
    }
}
impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        return &mut self.contents[self.col_count * point.y + point.x];
    }
}
