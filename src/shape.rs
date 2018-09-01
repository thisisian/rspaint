struct Shape {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

// TODO: Better way of instatiating Shapes
impl Shape {
    pub fn small_circle() -> Self {
        Shape {
            data: vec![vec![0, 1, 1, 0],
                       vec![1, 1, 1, 1],
                       vec![0, 1, 1, 0]],
            width: 4,
            height: 4,
        }
    }
}
