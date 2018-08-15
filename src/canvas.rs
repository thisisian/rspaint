extern crate cairo;
extern crate gtk;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Canvas {
    drawing_area: gtk::DrawingArea,
    surface: Option<cairo::Surface>,
    context: Option<cairo::Context>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            drawing_area: gtk::DrawingArea::new(),
            surface: None,
            context: None,
        }
    }
    // Will panic if surface isn't initialized yet
    pub fn get_surface(&self) -> cairo::Surface {
        self.surface.as_ref().unwrap().clone()
    }
    // Will panic if context isn't initialized yet
    pub fn get_context(&self) -> cairo::Context {
        self.context.as_ref().unwrap().clone()
    }

    pub fn set_context(&mut self, cr: cairo::Context) {
        self.context = Some(cr);
    }

    pub fn set_surface(&mut self, surf: cairo::Surface) {
        self.surface = Some(surf);
    }

    pub fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }
}