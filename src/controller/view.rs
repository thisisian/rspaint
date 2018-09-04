use controller::canvas::Canvas;
use cairo::Context;

pub trait CanvasView {
   fn update(&mut self, Canvas);
}

pub struct CairoView {
    canvas_ctx: Context,
}

impl CanvasView for CairoView {
    fn update(&mut self, canvas: Canvas) {
        unimplemented!()
    }
}

impl CairoView {
    pub fn new() -> CairoView {
        unimplemented!()
    }
}