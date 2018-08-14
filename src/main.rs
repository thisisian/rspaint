extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate cairo;

use gio::prelude::*;
use gio::MenuExt;

use gtk::prelude::*;
use gtk::ApplicationWindow;
use gtk::Orientation::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::env::args;
use std::option::Option::*;
use std::f64::consts::SQRT_2;
use std::f64::consts::PI;

pub mod enums;
use enums::*;

#[derive(Clone)]
struct Canvas {
    drawing_area: gtk::DrawingArea,
    surface: Rc<RefCell<Option<cairo::Surface>>>,
    context: Rc<RefCell<Option<cairo::Context>>>,
}

struct GlobalState {
    tool: Option<Tool>,
    fg_color: (f64, f64, f64),
    bg_color: (f64, f64, f64),
}

impl GlobalState {
    fn get_fg_cairo_pattern(&self) -> cairo::SolidPattern {
       cairo::SolidPattern::from_rgb(self.fg_color.0, self.fg_color.1, self.fg_color.2)
    }
    fn get_bg_cairo_pattern(&self) -> cairo::SolidPattern {
        cairo::SolidPattern::from_rgb(self.bg_color.0, self.bg_color.1, self.bg_color.2)
    }
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let global_state: Rc<RefCell<GlobalState>> = Rc::new(RefCell::new(GlobalState {
        tool: None,
        fg_color: (0., 0., 0.),
        bg_color: (1., 1., 1.),
    }));

    let canvas: Canvas = Canvas {
        drawing_area: gtk::DrawingArea::new(),
        surface: Rc::new(RefCell::new(None)),
        context: Rc::new(RefCell::new(None)),
    };

    window.set_title("RSPaint");
    window.set_default_size(500, 500); // TODO: Set to screen resolution?
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let h_box = gtk::Box::new(Horizontal, 0);

    let tool_box = gtk::Box::new(Vertical, 0);
    build_tool_box(&tool_box, global_state.clone());

    let canvas_box = gtk::Box::new(Vertical, 0);
    configure_canvas(canvas.clone(), global_state.clone());
    canvas_box.pack_start(&canvas.drawing_area, false, false, 10);

    h_box.pack_start(&tool_box, false, false, 0);
    h_box.pack_start(&canvas_box, false, false, 10);
    window.add(&h_box);

    build_menu(application);
    window.show_all();
}

fn configure_canvas(canvas: Canvas,
                    global_state: Rc<RefCell<GlobalState>>) {

    let surface = canvas.surface.clone();
    let context = canvas.context.clone();
    let da = canvas.drawing_area.clone();
    da.set_size_request(400, 400);

    let state_clone = global_state.clone();
    let clear_surface = move |surf: &cairo::Surface| {
        let cr = cairo::Context::new(surf);
        cr.set_antialias(cairo::Antialias::None);
        let ptn = &state_clone.borrow().get_bg_cairo_pattern();
        cr.set_source(ptn);
        cr.paint();
    };

    // Runs when drawing area is configured
    let context_clone = context.clone();
    let surface_clone = surface.clone();
    da.connect_configure_event(move |canv, _| {
        surface_clone.replace(Some(gdk::Window::create_similar_surface(&canv.get_window()
            .expect("Failed to get canvas window"),
                                                                       cairo::Content::Color,
                                                                       canv.get_allocated_width(),
                                                                       canv.get_allocated_height())
            .expect("Failed to create surface")));
        clear_surface(surface_clone.borrow().as_ref().unwrap());
        surface_clone.borrow().as_ref().unwrap();
        context_clone.replace(Some(cairo::Context::new(&surface_clone.borrow().as_ref().unwrap())));
        context_clone.borrow().as_ref().unwrap().set_antialias(cairo::Antialias::None);
        true
    });

    // When surface is drawn
    let surface_clone = surface.clone();
    let context_clone = context.clone();
    da.connect_draw(move |_, cr| {
        cr.set_source_surface(&surface_clone.borrow().as_ref().unwrap(), 0., 0.);
        cr.paint();
        Inhibit(false)
    });

    let last_position : Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));
    // When mouse is clicked on canvas
    let state_clone = global_state.clone();
    let context_clone = context.clone();
    let surface_clone = surface.clone();
    let last_position_clone = last_position.clone();
    da.connect_button_press_event(move |canv, event| {
        let (x, y) = event.get_position();
        let tool = state_clone.borrow().tool;
        match tool {
            Some(Tool::Pencil) => {
                let ptn = &state_clone.borrow().get_fg_cairo_pattern();
                draw_dot(canv, context_clone.borrow().as_ref().unwrap(), ptn, x, y, 10.0);
                last_position_clone.replace(Some((x, y)));
            },
            Some(Tool::Eraser) => {
                let ptn = &state_clone.borrow().get_bg_cairo_pattern();
                draw_dot(canv, context_clone.borrow().as_ref().unwrap(), ptn, x, y, 10.0);
                last_position_clone.replace(Some((x, y)));
            }
            _ => {},
        }
        context_clone.borrow().as_ref().unwrap().move_to(x, y); // TODO: Choose whether to use this or not
        Inhibit(false)
    });

    // TODO: Reset last position when button is released

    // When cursor moves across canvas
    let context_clone = context.clone();
    let state_clone = global_state.clone();
    let surface_clone = surface.clone();
    let last_position_clone = last_position.clone();
    da.connect_motion_notify_event(move |da, event| {
        let (x, y) = event.get_position();
        let button_state = event.get_state();
        let tool = state_clone.borrow().tool;
        let last_position_exists = last_position_clone.borrow().as_ref().is_some();
        let cr = context_clone.borrow();
        if button_state == gdk::ModifierType::BUTTON1_MASK {
            match tool {
                Some(Tool::Pencil) => {
                    let ptn = &state_clone.borrow().get_fg_cairo_pattern();
                    if last_position_exists == true {
                        let last_x = last_position_clone.borrow().as_ref().unwrap().0;
                        let last_y = last_position_clone.borrow().as_ref().unwrap().1;
                        draw_line(da, cr.as_ref().unwrap(), ptn, last_x, last_y, x, y, 10.0);
                    } else {
                        draw_dot(da, context.borrow().as_ref().unwrap(), ptn, x, y, 10.0);
                        last_position_clone.replace(Some((x, y)));
                    }
                }
                Some(Tool::Eraser) => {
                    let ptn = &state_clone.borrow().get_bg_cairo_pattern();
                    if last_position_exists == true {
                        let last_x = last_position_clone.borrow().as_ref().unwrap().0;
                        let last_y = last_position_clone.borrow().as_ref().unwrap().1;
                        draw_line(da, cr.as_ref().unwrap(), ptn, last_x, last_y, x, y, 10.0);
                    } else {
                        draw_dot(da, context.borrow().as_ref().unwrap(), ptn, x, y, 10.0);
                        last_position_clone.replace(Some((x, y)));
                    }
                },
                _ => {},
            }
        }
    last_position.replace(Some((x, y)));
    Inhibit(false)
    });

    // Register the events.
    da.add_events(gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32|
                      gdk::EventMask::BUTTON_MOTION_MASK.bits() as i32);
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

fn build_tool_box(tool_box: &gtk::Box, state: Rc<RefCell<GlobalState>>) {
    let pencil_button = gtk::ToggleButton::new();
    let eraser_button = gtk::ToggleButton::new();

    let pencil_icon = gtk::Image::new_from_icon_name("face-smile", gtk::IconSize::SmallToolbar.into());
    let eraser_icon = gtk::Image::new_from_icon_name("face-sad", gtk::IconSize::SmallToolbar.into());
    pencil_button.set_image(&pencil_icon);
    eraser_button.set_image(&eraser_icon);
    pencil_button.set_label("Pencil");
    eraser_button.set_label("Eraser");

    let pencil_button_clone = pencil_button.clone();
    let state_clone = state.clone();
    pencil_button.connect_toggled(move |this| {
        if this.get_active() == true {
            let mut new_state = state_clone.borrow_mut();
            new_state.tool = Some(Tool::Pencil);
            pencil_button_clone.set_active(false);
        }
    });
    let eraser_button_clone = eraser_button.clone();
    let state_clone = state.clone();
    eraser_button.connect_toggled(move |this| {
        if this.get_active() == true {
            let mut new_state = state_clone.borrow_mut();
            new_state.tool = Some(Tool::Eraser);
            eraser_button_clone.set_active(false);
        }
    });

    tool_box.pack_start(&pencil_button, false, false, 10);
    tool_box.pack_start(&eraser_button, false, false, 0);
}

fn build_menu(application: &gtk::Application) {
    let menu_bar = gio::Menu::new();

    let file_menu = gio::Menu::new();
    let file_sec1 = gio::Menu::new();
    let file_sec2 = gio::Menu::new();

    file_sec1.append("New", "app.new");
    file_sec1.append("Open", "app.open");
    file_sec1.append("Save", "app.save");
    file_sec1.append("Save As", "app.save_as");

    file_sec2.append("Exit", "app.exit");

    file_menu.append_section(None, &file_sec1);
    file_menu.append_section(None, &file_sec2);

    menu_bar.append_submenu("File", &file_menu);

    let edit_menu = gio::Menu::new();
    let edit_sec1 = gio::Menu::new();
    let edit_sec2 = gio::Menu::new();
    let edit_sec3 = gio::Menu::new();

    edit_sec1.append("Undo", "app.undo");
    edit_sec1.append("Repeat", "app.repeat");
    edit_sec2.append("Cut", "app.cut");
    edit_sec2.append("Copy", "app.copy");
    edit_sec2.append("Paste", "app.paste");
    edit_sec2.append("Clear Selection", "app.clear_selection");
    edit_sec2.append("Select All", "app.select_all");
    edit_sec3.append("Copy To", "app.copy_to");
    edit_sec3.append("Copy From", "app.copy_from");

    edit_menu.append_section(None, &edit_sec1);
    edit_menu.append_section(None, &edit_sec2);
    edit_menu.append_section(None, &edit_sec3);

    menu_bar.append_submenu("Edit", &edit_menu);

    application.set_menubar(&menu_bar);
}

fn main() {
    let application = gtk::Application::new("org.github.thisisian.rspaint",
                                            gio::ApplicationFlags::empty()).expect("Initialization Failed...");

    application.connect_startup(|app| {
        build_ui(app)
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());

    gtk::main();
}
