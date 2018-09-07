extern crate gtk;
extern crate gdk;
extern crate cairo;

use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

pub mod color;
mod canvas;
mod shape;
mod tools;
mod view;

use self::canvas::Canvas;
use self::color::RGBColor;
use self::tools::Tool;

const CANVAS_WIDTH: usize = 400;

pub struct Controller {
    tools: HashMap<&'static str, Box<Tool>>,
    current_tool: Option<Box<Tool>>,
    fg_color: RGBColor,
    bg_color: RGBColor,
    drawing_area: gtk::DrawingArea,
    canvas: canvas::Canvas,
}


impl Controller {
    // Create the controller, creates a shared reference 
    pub fn new() -> (Rc<RefCell<Controller>>) {
        let tools: HashMap<&'static str, Box<Tool>> = tools::create_toolset();
        let fg_color = color::BLACK;
        let bg_color = color::WHITE;
        let drawing_area = gtk::DrawingArea::new();
        Controller::init_drawing_area(&drawing_area);
        let canvas = Canvas::new(CANVAS_WIDTH, CANVAS_WIDTH, &drawing_area);

        let ctrl = Rc::new(RefCell::new(Controller {
            tools,
            current_tool : None,
            fg_color,
            bg_color,
            drawing_area,
            canvas,
        }));
        
        ctrl.borrow_mut().swap_tool("Brush");

        ctrl
    }

    fn init_drawing_area(da: &gtk::DrawingArea) {
        da.set_size_request(CANVAS_WIDTH as i32, CANVAS_WIDTH as i32);

        // Emits when drawing_area's window's position changes
        da.connect_configure_event(|da, _| {
            true
        });

        // Emits when drawing area is redrawn
        da.connect_draw(|da, ctx| {
            Inhibit(false)
        }); 

        da.connect_button_press_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });

        da.connect_motion_notify_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });

        da.connect_button_release_event(|da, event| {
            let state = event.get_state();
            Inhibit(false)
        });
        
        //TODO: Register events
    }

    pub fn get_drawing_area(&self) -> gtk::DrawingArea {
        self.drawing_area.clone()
    }

}

// Operations on Tools 

impl Controller {
    // Swaps in `tool` into `current_tool`
    pub fn swap_tool(&mut self, tool: &'static str) {
        let new_tool = self.tools.remove(tool).expect("Failed to get tool");
        if self.current_tool.is_some() {
            let old_tool = self.current_tool.take().unwrap();
            self.tools.insert(old_tool.get_name(), old_tool);
        }
        self.current_tool = Some(new_tool);
    }
}

// Operations on the model

impl Controller {

}
