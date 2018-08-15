extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use std::f64::consts::SQRT_2;
use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

use GlobalColors;
use canvas::Canvas;

pub struct Tools <'a, T>
    where T: 'a + Tool {
    current: &'a T,
    pencil: Pencil,
    eraser: Eraser,
}

pub trait Tool {
    fn new() -> Self;
    fn on_click(&self, x: f64, y: f64);
    fn on_movement(&self, x: f64, y: f64);
    fn on_release(&self, x:f64, y: f64);
}

pub struct Pencil {
    canvas: Rc<RefCell<Canvas>>,
    global_colors: Rc<RefCell<GlobalColors>>,
    width: f64,
}

impl Pencil {
    fn new(canvas: Rc<RefCell<Canvas>>,
           global_colors: Rc<RefCell<GlobalColors>>) -> Self {
        Pencil {
            canvas,
            global_colors,
            width: 1.,
        }
    }

    fn set_width(&mut self, width: f64) {
        self.width = width;
    }
}

impl Tool for Pencil {
    fn new() -> Self {
        panic!("Tool::new() should not be called to create a Pencil")
    }

    fn on_click(&self, x: f64, y: f64){
        draw_dot(&self.canvas.borrow().get_drawing_area(),
                 &self.canvas.borrow().get_context(),
                 &self.global_colors.borrow().get_fg_cairo_pattern(),
                 x, y, self.width);
    }

    fn on_movement(&self, x: f64, y: f64){
    }

    fn on_release(&self, x: f64, y: f64){
    }
}

pub struct Eraser {
    canvas: Rc<RefCell<Canvas>>,
    global_colors: Rc<RefCell<GlobalColors>>,
    width: f64,
}

impl Tool for Eraser {
    fn new() -> Self {
        panic!("Tool::new should not be called to create an Eraser")
    }
    fn on_click(&self, x: f64, y: f64){

    }
    fn on_movement(&self, x: f64, y: f64){

    }
    fn on_release(&self, x: f64, y: f64){

    }
}
//test test test

impl Eraser {

}

// Draw line on surface from x1, y1 to x2, y2.
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