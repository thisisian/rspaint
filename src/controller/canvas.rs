extern crate cairo;
extern crate gtk;

use controller::shape::Shape;
use controller::color::RGBColor;
use controller::view::CanvasView;
use controller::view::CairoView;

pub struct Canvas {
    view: Box<CanvasView>,
    canvas: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            view: Box::new(CairoView::new()),
            canvas: vec![vec![0; width]; height],
            width,
            height,
        }
    }

    // TODO: When canvas is embiggened, it should fill with background color
    pub fn resize(&mut self, width: usize, height: usize) {
        self.canvas.resize(height, vec![0; width]);
        self.canvas.iter_mut().for_each(|x| x.resize(width, 0));
        self.width = width;
        self.height = height;
    }

    // Draw Shape at (x, y)
    pub fn draw(&mut self, shape: Shape, color: RGBColor, x: usize, y: usize) {
        shape.get_data().into_iter().flatten().enumerate().for_each(|(i, val)| {
            if val == &true {
                let rel_x = x - shape.get_width()/2 + i % shape.get_width();
                let rel_y = y - shape.get_height()/2 + i / shape.get_width();
                if rel_x <= self.width && rel_y <= self.height {
                    self.canvas[rel_x][rel_y] = color.as_usize();
                }
            }
        })
    }
}