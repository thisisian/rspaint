pub struct Shape {
    data: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

// TODO: Better way of instatiating Shapes
impl Shape {
    pub fn small_circle() -> Self {
        Shape {
            data: vec![vec![false, true, true, false],
                       vec![true, true, true, true],
                       vec![false, true, true, false]],
            width: 4,
            height: 4,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_data(&self) -> &Vec<Vec<bool>> {
        &self.data
    }
}