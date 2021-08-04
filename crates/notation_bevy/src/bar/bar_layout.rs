#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BarLayout {
    pub row: usize,
    pub col: usize,
}
impl BarLayout {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col, }
    }
}