extern crate cairo;
extern crate gdk;

use controller::canvas::Canvas;
use gtk::DrawingArea;
use gtk::WidgetExt;
use cairo::Context;
use cairo::Surface;
use controller::color::RGBColor;

pub trait CanvasView {
   fn update(&mut self, &Canvas);
}

pub struct CairoView {
    ctx: Context,
    surface: Surface,
}

impl CanvasView for CairoView {
    fn update(&mut self, canvas: &Canvas) {
        let width = canvas.get_width();
        let height = canvas.get_height();
        let pixel_data = canvas.borrow_pixels();
        for (i, row) in pixel_data.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                self.draw_pixel(j, i, pixel);
            }
        }
    }
}

impl CairoView {
    // Create new CairoView associated with drawing area `da`
    pub fn new(da: &DrawingArea) -> CairoView {
        let surface = gdk::Window::create_similar_surface(
                &da.get_window().expect("Failed to get drawing surface window"),
                cairo::Content::Color, 
                da.get_allocated_width(), 
                da.get_allocated_height())
            .expect("Failed to create surface");
        let ctx = Context::new(&surface);
        ctx.set_antialias(cairo::Antialias::None);
        CairoView {
            ctx,
            surface
        }
    }

    fn draw_pixel(&self, x: usize, y: usize, color: &RGBColor) {
        self.ctx.set_source(&color.as_cairo_pattern());
        self.ctx.rectangle(x as f64, y as f64, 1., 1.);
        self.ctx.fill();
    }
}