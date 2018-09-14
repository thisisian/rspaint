extern crate cairo;
extern crate gtk;

use controller::view::CanvasView;
use controller::view::CairoView;
use controller::color;
use controller::color::RGBColor;

pub struct Canvas {
    view: Option<Box<CanvasView>>,
    pixels: Vec<Vec<RGBColor>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize, da: &gtk::DrawingArea) -> Self {
        let mut pixels = vec![vec![color::WHITE; width]; height];
        // TEST PIXELS
        pixels[0][1] = color::BLACK;
        pixels[1][0] = color::BLACK;
        pixels[3][0] = color::BLACK;
        pixels[0][3] = color::BLACK;
        // END TEST
        let mut ret = Canvas {
            view: Some(Box::new(CairoView::new(da))),
            pixels,
            width,
            height,
        };
        ret.update_view();
        ret
    }

    // Update `view` with pixel data from pixels
    pub fn update_view(&mut self) {
        let mut view = self.view.take().expect("Failed to get view when updating");
        view.update(self);
        self.view = Some(view);
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn borrow_pixels(&self) -> &Vec<Vec<RGBColor>> {
        &self.pixels
    }
}
