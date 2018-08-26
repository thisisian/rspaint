extern crate gtk;
extern crate gdk;
extern crate cairo;

use gtk::prelude::*;
use tools::Toolset;
use std::cell::RefCell;
use std::rc::Rc;


const CANVAS_WIDTH: u32 = 400;

pub struct Controller {
    drawing_area: gtk::DrawingArea,
    tools: Toolset,
}

impl Controller {
    // Create the controller, creates a shared reference 
    pub fn new() -> (RefCell<Rc<Controller>>) {
        let ctrl = RefCell::new(Rc::new(Controller {
            drawing_area: gtk::DrawingArea::new(),
            tools: Toolset::new(),
        }));

        Controller::init_drawing_area(ctrl.clone());
        ctrl
    }

    fn init_drawing_area(ctrl: RefCell<Rc<Controller>>) {
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
    }

    fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }
}