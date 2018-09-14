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
    canvas: Option<canvas::Canvas>,
}


impl Controller {
    // Create the controller, creates a shared reference 
    pub fn new(drawing_area: gtk::DrawingArea) -> (Rc<RefCell<Controller>>) {
        let tools: HashMap<&'static str, Box<Tool>> = tools::create_toolset();
        let fg_color = color::BLACK;
        let bg_color = color::WHITE;

        let ctrl = Rc::new(RefCell::new(Controller {
            tools,
            current_tool : None,
            fg_color,
            bg_color,
            drawing_area,
            canvas : None,
        }));
        
        ctrl.borrow_mut().set_tool("Brush");
        Controller::init_drawing_area(ctrl.clone());

        ctrl
    }

    fn init_drawing_area(ctrl: Rc<RefCell<Controller>>) {
        let da = ctrl.borrow().drawing_area.clone();
        da.set_size_request(CANVAS_WIDTH as i32, CANVAS_WIDTH as i32);

        // Emits when drawing_area's window's position changes
        let ctrl_clone = ctrl.clone();
        da.connect_configure_event(move |da, _| {
            ctrl_clone.borrow_mut().canvas = Some(Canvas::new(CANVAS_WIDTH, CANVAS_WIDTH, da));
            true
        });

        // Emits when drawing area is redrawn
        let ctrl_clone = ctrl.clone();
        da.connect_draw(move |da, ctx| {
            ctrl_clone.borrow_mut().canvas.as_mut().unwrap().update_view();
            let surface = ctrl_clone.borrow_surface();
            ctx.set_source_surface(surface);
            ctx.fill();
            da.queue_draw();
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

// Operations on view 

impl Controller {
    fn borrow_surface(&self) -> &cairo::Surface {
        self.canvas.as_ref().unwrap().borrow_view().borrow_surface()
    }
}

// Operations on Tools 

impl Controller {
    // Set `current_tool` to `tool` 
    pub fn set_tool(&mut self, tool: &'static str) {
        if self.current_tool.as_ref().map_or(false, |x| x.get_name() == tool) {
            // Short circuit if trying to set tool to current tool
            return
        }
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
