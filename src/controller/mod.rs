extern crate gtk;
extern crate gdk;
extern crate cairo;

use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub mod color;
mod canvas;
mod shape;
mod tools;

use self::tools::Toolset;
use self::canvas::Canvas;

const CANVAS_WIDTH: usize = 400;

pub struct Controller {
    drawing_area: gtk::DrawingArea,
    tools: Toolset,
    model: canvas::Canvas,
}

impl Controller {
    // Create the controller, creates a shared reference 
    pub fn new() -> (Rc<RefCell<Controller>>) {
        let ctrl = Rc::new(RefCell::new(Controller {
            drawing_area: gtk::DrawingArea::new(),
            tools: Toolset::new(),
            model: Canvas::new(CANVAS_WIDTH, CANVAS_WIDTH),
        }));

        Controller::init_drawing_area(ctrl.clone());
        ctrl
    }

    fn init_drawing_area(ctrl: Rc<RefCell<Controller>>) {
        ctrl.borrow().get_drawing_area().set_size_request(CANVAS_WIDTH as i32, CANVAS_WIDTH as i32);

        // Emits when drawing_area's window's position changes
        ctrl.borrow().drawing_area.connect_configure_event(|da, _| {
            true
        });

        // Emits when drawing area is redrawn
        ctrl.borrow().drawing_area.connect_draw(|da, ctx| {
            Inhibit(false)
        }); 

        ctrl.borrow().drawing_area.connect_button_press_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });

        ctrl.borrow().drawing_area.connect_motion_notify_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });

        ctrl.borrow().drawing_area.connect_button_release_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });
        
        //Register events
    }

    pub fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }
}