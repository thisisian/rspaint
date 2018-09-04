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
mod view;

use self::tools::Toolset;
use self::canvas::Canvas;
use self::color::RGBColor;
use self::tools::Tool;
use enums::ToolNames;

const CANVAS_WIDTH: usize = 400;

pub struct Controller {
    model: canvas::Canvas,
    drawing_area: gtk::DrawingArea,
    tools: Toolset,
    current_tool: ToolNames,
    fg_color: RGBColor,
    bg_color: RGBColor,
}


impl Controller {
    // Create the controller, creates a shared reference 
    pub fn new() -> (Rc<RefCell<Controller>>) {
        let drawing_area = gtk::DrawingArea::new();
        let tools = Toolset::new();
        let model = Canvas::new(CANVAS_WIDTH, CANVAS_WIDTH);
        let fg_color = color::BLACK;
        let bg_color = color::WHITE;

        let ctrl = Rc::new(RefCell::new(Controller {
            drawing_area,
            tools,
            model,
            current_tool : ToolNames::Brush,
            fg_color,
            bg_color,
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
        
        //TODO: Register events
    }

    pub fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }

    pub fn set_tool(&mut self, tool: ToolNames) {
        self.current_tool = tool;
    }
}

// Operations on the model

impl Controller {

}

// Operations on view

impl Controller {
    fn update_view() {

    }
}