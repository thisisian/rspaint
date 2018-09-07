extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use std::f64::consts::SQRT_2;
use std::f64::consts::PI;
use std::collections::HashMap;
use controller::color::RGBColor;

// Default width for tools
const DEFAULT_WIDTH: usize = 5;
const DEFAULT_FG_COLOR: RGBColor = super::color::BLACK;
const DEFAULT_BG_COLOR: RGBColor = super::color::WHITE;

// Tool is a tool which can be used on the canvas
pub trait Tool {
    fn get_name(&self) -> &'static str;
    fn on_click(&self, x: usize, y: usize);
    fn on_movement(&self, x1: usize, y1: usize, x2: usize, y2: usize);
    fn on_release(&self, x:usize, y: usize);
}

// Pencil 

pub struct Brush {
    name: &'static str,
}

impl Brush {
    fn new() -> Self {
        Brush {
            name: "Brush",
        }
    }
}

impl Tool for Brush {

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn on_click(&self, x: usize, y: usize) {
        unimplemented!()
    }

    fn on_movement(&self,  x1: usize, y1: usize, x2:usize, y2: usize) {
        unimplemented!()
    }

    fn on_release(&self,  x: usize, y: usize) {
        unimplemented!()
    }
}

// Eraser

pub struct Eraser {
    name: &'static str,
}

impl Eraser {
    fn new() -> Self {
        Eraser {
            name: "Eraser",
        }
    }
}

impl Tool for Eraser {

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn on_click(&self, x: usize, y: usize){
        unimplemented!()
    }
    
    fn on_movement(&self, x1: usize, y1: usize, x2:usize, y2: usize){
        unimplemented!()
    }

    fn on_release(&self, x: usize, y: usize){
        unimplemented!()
    }
}

// Utility functions

// Creates a HashSet of all the tools  
pub fn create_toolset() -> HashMap<&'static str, Box<Tool>> {
    let mut ret: HashMap<&'static str, Box<Tool>> = HashMap::new();
    let eraser = Box::new(Eraser::new());
    ret.insert(eraser.get_name(), eraser);
    let brush = Box::new(Brush::new());
    ret.insert(brush.get_name(), brush);
    ret
}

fn draw_line(da: &gtk::DrawingArea,
             cr: &cairo::Context,
             ptn: &cairo::SolidPattern,
             x1: f64, y1: f64, x2: f64, y2: f64, width: f64) {
    cr.move_to(x1, y1);
    cr.line_to(x2, y2);
    cr.set_line_cap(cairo::LineCap::Round);
    cr.set_source(ptn);
    cr.set_line_width(width);
    cr.stroke();
    da.queue_draw();
}

fn draw_dot(da: &gtk::DrawingArea, cr: &cairo::Context,
            ptn: &cairo::SolidPattern, x: f64, y: f64, diameter: f64) {
    cr.arc(x, y, diameter/2., 0_f64, 2.*PI);
    cr.set_source(ptn);
    cr.fill();
    // Redraw area larger than rectangle due to floating point rounding
    let redraw_sz = (diameter * SQRT_2).ceil() as i32;
    let redraw_x = (x - redraw_sz as f64 / 2.).floor() as i32;
    let redraw_y = (y - redraw_sz as f64 / 2.).floor() as i32;
    da.queue_draw_area(redraw_x,
                       redraw_y,
                       redraw_sz,
                       redraw_sz);
}