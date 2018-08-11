extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate cairo;

use gtk::prelude::*;
use gio::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use gio::MenuExt;

use gtk::ApplicationWindow;
use gtk::Orientation::*;

use std::env::args;
use std::option::Option::*;

pub mod enums;
use enums::*;

// Make cloning simpler for closures (from http://gtk-rs.org/tuto/closures)
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    let tool_state: Rc<RefCell<Option<Tool>>> = Rc::new(RefCell::new(None));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.set_title("RSPaint");
    window.set_default_size(500, 500);

    let h_box = gtk::Box::new(Horizontal, 0);
    let v_box = gtk::Box::new(Vertical, 0);

    let tool_box = gtk::Box::new(Vertical, 0);
    build_tool_box(&tool_box, tool_state.clone());
    h_box.pack_start(&tool_box, false, false, 0);

    let canvas = gtk::DrawingArea::new();
    configure_canvas(&canvas, tool_state.clone());

    v_box.pack_start(&canvas, false, false, 10);
    h_box.pack_start(&v_box, false, false, 10);
    window.add(&h_box);

    build_menu(application);
    window.show_all();
}

fn configure_canvas(canvas: &gtk::DrawingArea, tool_state: Rc<RefCell<Option<Tool>>>) {
    canvas.set_size_request(400, 400);
    let surface: Rc<RefCell<Option<cairo::Surface>>> = Rc::new(RefCell::new(None));
    let context: Rc<RefCell<Option<cairo::Context>>> = Rc::new(RefCell::new(None));

    let clear_surface = |surf: &cairo::Surface| {
        let cr = cairo::Context::new(surf);
        cr.set_antialias(cairo::Antialias::None);
        cr.set_source_rgb(1., 0.5, 0.5);
        cr.paint();
    };

    // When surface is configured
    let context_clone = context.clone();
    let surface_clone = surface.clone();
    canvas.connect_configure_event(move |canv, _| {
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
    canvas.connect_draw(move |_, cr| {
        cr.set_source_surface(&surface_clone.borrow().as_ref().unwrap(), 0., 0.);
        cr.paint();
        Inhibit(false)
    });

    let last_position : Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));
    // When mouse is clicked on canvas
    let context_clone = context.clone();
    let surface_clone = surface.clone();
    let last_position_clone = last_position.clone();
    let tool_state_clone = tool_state.clone();
    canvas.connect_button_press_event(move |canv, event| {
        let (x, y) = event.get_position();
        match tool_state_clone.borrow().as_ref() {
            Some(Tool::Pencil) => {
                draw_square(canv, context_clone.borrow().as_ref().unwrap(), x, y);
                last_position_clone.replace(Some((x, y)));
            },
            _ => {},
        }
        Inhibit(false)
    });

    // TODO: Reset last position when button is released

    // When cursor moves across canvas
    let context_clone = context.clone();
    let surface_clone = surface.clone();
    let last_position_clone = last_position.clone();
    let tool_state_clone = tool_state.clone();
    canvas.connect_motion_notify_event(move |da, event| {
        let (x, y) = event.get_position();
        let state = event.get_state();
        let cur_tool = tool_state_clone.borrow().clone();
        let last_position_exists = last_position_clone.borrow().as_ref().is_some();
        if state == gdk::ModifierType::BUTTON1_MASK {
            match cur_tool {
                Some(Tool::Pencil) => {
                    if last_position_exists == true {
                        let last_x = last_position_clone.borrow().as_ref().unwrap().0;
                        let last_y = last_position_clone.borrow().as_ref().unwrap().1;
                        draw_line(da, context.borrow().as_ref().unwrap(), last_x, last_y, x, y);
                    } else {
                        draw_square(da, context.borrow().as_ref().unwrap(), x, y);
                        last_position_clone.replace(Some((x, y)));
                    }
                }
                Some(Tool::Eraser) => {},
                _ => {},
            }
        }
    last_position.replace(Some((x, y)));
    Inhibit(false)
    });

    // Register the events.
    canvas.add_events(gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32|
                      gdk::EventMask::BUTTON_MOTION_MASK.bits() as i32);
}

// Draw line on surface from x1, y1 to x2, y2.
fn draw_line(da: &gtk::DrawingArea, cr: &cairo::Context, x1: f64, y1: f64, x2: f64, y2: f64) {
    cr.move_to(x1, y1);
    cr.line_to(x2, y2);
    cr.set_line_width(1.0);
    cr.stroke();
    da.queue_draw();
}


fn draw_square(da: &gtk::DrawingArea, cr: &cairo::Context, x: f64, y: f64) {
    cr.rectangle(x, y, 1_f64, 1_f64);
    cr.fill();
    // Redraw area larger than rectangle due to floating point rounding
    da.queue_draw_area((x as i32) - 2, (y as i32) -2 , 4, 4);
}

fn build_tool_box(tool_box: &gtk::Box,
                  tool_state: Rc<RefCell<Option<Tool>>>) {
    let pencil_button = gtk::ToggleButton::new();
    let eraser_button = gtk::ToggleButton::new();

    let pencil_icon = gtk::Image::new_from_icon_name("face-smile", gtk::IconSize::SmallToolbar.into());
    let eraser_icon = gtk::Image::new_from_icon_name("face-sad", gtk::IconSize::SmallToolbar.into());
    pencil_button.set_image(&pencil_icon);
    eraser_button.set_image(&eraser_icon);
    pencil_button.set_label("Pencil");
    eraser_button.set_label("Eraser");

    let eraser_button_clone = eraser_button.clone();
    let pencil_button_clone = pencil_button.clone();
    let tool_state_clone = tool_state.clone();
    eraser_button.connect_toggled(move |this| {
        if this.get_active() == true {
            tool_state_clone.replace(Some(Tool::Eraser));
            pencil_button_clone.set_active(false);
        }
    });
    let tool_state_clone = tool_state.clone();
    pencil_button.connect_toggled(move |this| {
        if this.get_active() == true {
            tool_state_clone.replace(Some(Tool::Pencil));
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
