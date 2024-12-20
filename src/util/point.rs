pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
