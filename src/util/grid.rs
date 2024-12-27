use crate::util::point::Point;
use std::ops::{Index, IndexMut};

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
