extern crate cairo;
extern crate gtk;

use shape;

#[derive(Clone)]
pub struct Canvas {
    canvas: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width, height) -> Self {
        Canvas {
            canvas: vec![vec![0; width], height],
            width,
            height,
        }
    }

    // TODO: When canvas is embiggened, it should fill with background color
    pub fn resize(&mut self, width, height) {
        canvas.resize(height, 0);
        canvas.to_iter().for_each(|x| x.resize(width, 0));
        self.width = width;
        self.height = height;
    }

    // Draw shape at (x, y)
    // TODO: Should draw with a color
    draw(&self, Shape: shape, usize: x, usize: y) {
        
    }
}