use canvas::Canvas;

extern crate cairo;
extern crate gdk;
extern crate gtk;

use gtk::prelude::*;

use std::f64::consts::SQRT_2;
use std::f64::consts::PI;

pub trait _Tool {
    fn on_click(&self, canvas: Canvas);
    fn on_movement(&self, canvas:Canvas);
    fn on_release(&self, canvas: Canvas);
}

pub struct Pencil {
    width: f64,
}

impl _Tool for Pencil {
    fn on_click(&self, canvas: Canvas){

    }
    fn on_movement(&self, canvas: Canvas){

    }
    fn on_release(&self, canvas: Canvas){

    }

}

pub struct Eraser {
    width: f64,
}

impl _Tool for Eraser {
    fn on_click(&self, canvas: Canvas){

    }
    fn on_movement(&self, canvas: Canvas){

    }
    fn on_release(&self, canvas: Canvas){

    }
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

fn draw_dot(da: &gtk::DrawingArea,
            cr: &cairo::Context,
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