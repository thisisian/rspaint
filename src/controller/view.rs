extern crate cairo;
extern crate gdk;

use controller::canvas::Canvas;
use gtk::DrawingArea;
use gtk::WidgetExt;
use cairo::Context;
use cairo::Surface;

pub trait CanvasView {
   fn update(&mut self, Canvas);
}

pub struct CairoView {
    ctx: Context,
    surface: Surface,
}

impl CanvasView for CairoView {
    fn update(&mut self, canvas: Canvas) {
        unimplemented!()
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
        CairoView {
            ctx,
            surface
        }
    }
}