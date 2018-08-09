extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate cairo;

use gtk::prelude::*;
use gio::prelude::*;
use cairo::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use gio::MenuExt;

use gtk::ApplicationWindow;
use gtk::Orientation::*;

use std::env::args;
use std::option::Option::*;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.set_title("RSPaint");
    window.set_default_size(500, 500);

    let h_box = gtk::Box::new(Horizontal, 0);
    let v_box = gtk::Box::new(Vertical, 0);

    let toolbar = gtk::Toolbar::new();
    build_toolbar(&toolbar);
    h_box.pack_start(&toolbar, false, false, 0);

    let canvas = gtk::DrawingArea::new();
    configure_canvas(&canvas);



    h_box.pack_start(&canvas, false, false, 10);
    window.add(&h_box);

    build_menu(application);
    window.show_all();
}


fn configure_canvas(canvas: &gtk::DrawingArea) {
    canvas.set_size_request(400, 100);
    let surface: Rc<RefCell<Option<cairo::Surface>>> = Rc::new(RefCell::new(None));
    let clear_surface = |surf: &cairo::Surface| {
        let cr = cairo::Context::new(surf);
        cr.set_source_rgb(1., 0.5, 0.5);
        cr.paint();
    };

    // When surface is configured
    let surface_clone = surface.clone();
    canvas.connect_configure_event(move |canv, _| {
        surface_clone.replace(Some(gdk::Window::create_similar_surface(&canv.get_window()
            .expect("Failed to get canvas window"),
                                                                       cairo::Content::Color,
                                                                       canv.get_allocated_width(),
                                                                       canv.get_allocated_height())
            .expect("Failed to create surface")));
        clear_surface(surface_clone.borrow().as_ref().unwrap());
        true
    });

    // When surface is drawn
    let surface_clone = surface.clone();
    canvas.connect_draw(move |_, cr| {
        cr.set_source_surface(&surface_clone.borrow().as_ref().unwrap(), 0., 0.);
        cr.paint();
        Inhibit(false)
    });

    // When mouse is clicked on canvas
    let surface_clone = surface.clone();
    canvas.connect_button_press_event(move |canv, event| {
        let (x, y) = event.get_position();
        draw_square(canv, surface_clone.borrow().as_ref().unwrap(), x, y);
        Inhibit(false)
    });

    // When cursor moves across canvas
    let surface_clone = surface.clone();
    canvas.connect_motion_notify_event(move |canv, event| {
        let (x, y) = event.get_position();
        let state = event.get_state();
        if state == gdk::ModifierType::BUTTON1_MASK {
            draw_square(canv, surface_clone.borrow().as_ref().unwrap(), x, y);
        }
        Inhibit(false)
    });

    // Register the events so that they will work.
    canvas.add_events(gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32|
        gdk::EventMask::BUTTON_MOTION_MASK.bits() as i32);
}

// Draw square on surface and invalidate area on widget
//
fn draw_square(drawing_area: &gtk::DrawingArea, surface: &cairo::Surface, x: f64, y: f64) {
    let cr = cairo::Context::new(surface);
    cr.rectangle(x-2., y-2., 4_f64, 4_f64);
    cr.fill();
    drawing_area.queue_draw_area((x as i32)-2, (y as i32)-2, 4, 4);
}

fn build_toolbar(toolbar: &gtk::Toolbar) {
    toolbar.set_orientation(Vertical);
    let pencil_icon = gtk::Image::new_from_icon_name("face-smile", gtk::IconSize::SmallToolbar.into());
    let eraser_icon = gtk::Image::new_from_icon_name("face-sad", gtk::IconSize::SmallToolbar.into());
    let pencil_button = gtk::ToolButton::new(&pencil_icon, "Pencil");
    let eraser_button = gtk::ToolButton::new(&eraser_icon, "Eraser");
    toolbar.insert(&pencil_button, 0);
    toolbar.insert(&eraser_button, -1);
}

fn build_menu(application: &gtk::Application) {
    let menu_bar = gio::Menu::new();

    let file_menu = gio::Menu::new();
    let file_sec1 = gio::Menu::new();
    let file_sec2 = gio::Menu::new();

    let image_menu = gio::Menu::new();
    let colors_menu = gio::Menu::new();
    let help_menu = gio::Menu::new();

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
