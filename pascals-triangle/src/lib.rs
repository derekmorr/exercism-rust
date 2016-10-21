pub struct PascalsTriangle { row_count: u32 }

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        fn row(i: u32) -> Vec<u32> {
            match i {
                0 => Vec::new(),
                1 => vec![1],
                2 => vec![1, 1]
            }
        }

        let z: Vec<u32> = row(4);
        (0..self.row_count+1).fold(Vec::new(), |acc, e| acc.push(row(e); ac))
    }
}
