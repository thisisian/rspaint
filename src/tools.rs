extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use std::f64::consts::SQRT_2;
use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;
use color::RGBColor;
use Controller;
use gdk::ModifierType;

// Default width for tools
const DEFAULT_WIDTH: usize = 5;
const DEFAULT_FG_COLOR: RGBColor = super::color::BLACK;
const DEFAULT_BG_COLOR: RGBColor = super::color::WHITE;

/// Toolset holds all tools and their states and settings
pub struct Toolset {
    width: usize,
    fg_color: RGBColor,
    bg_color: RGBColor,
    pencil: Pencil,
    eraser: Eraser,
}

impl Toolset {
    pub fn new() -> Self {
        // For default tool, declare pencil outside of struct declaration
        let pencil = Rc::new(RefCell::new(Pencil::new()));
        Toolset {
            fg_color: DEFAULT_FG_COLOR,
            bg_color: DEFAULT_BG_COLOR,
            width: DEFAULT_WIDTH,
            pencil: Pencil::new(),
            eraser: Eraser::new(),
        }
    }
}

// Tool is a tool which can be used on the canvas
pub trait Tool {
    fn on_click(&self, state: &gdk::ModifierType, settings: &Toolset, x: usize, y: usize);
    fn on_movement(&self, state: &gdk::ModifierType, settings: &Toolset, x1: usize, y1: usize, x2: usize, y2: usize);
    fn on_release(&self, state: &gdk::ModifierType, settings: &Toolset, x:usize, y: usize);
}

// Pencil 

struct Pencil {
}

impl Pencil {
    fn new() -> Self {
        Pencil {
        }
    }
}

impl Tool for Pencil {

    fn on_click(&self, button_state: &gdk::ModifierType, settings: &Toolset, x: usize, y: usize) {
    }

    fn on_movement(&self, button_state:&gdk::ModifierType, settings: &Toolset, x1: usize, y1: usize, x2:usize, y2: usize) {
    }

    fn on_release(&self, button_state:&gdk::ModifierType, settings: &Toolset, x: usize, y: usize) {
    }
}

// Eraser

pub struct Eraser {
}

impl Eraser {
    fn new() -> Self {
        Eraser {
        }
    }
}

impl Tool for Eraser {
    fn on_click(&self, state: &gdk::ModifierType, settings: &Toolset, x: usize, y: usize){
    }
    
    fn on_movement(&self, state: &gdk::ModifierType, settings: &Toolset, x1: usize, y1: usize, x2:usize, y2: usize){
    }

    fn on_release(&self, state: &gdk::ModifierType, settings: &Toolset, x: usize, y: usize){
    }
}

// Utility functions

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