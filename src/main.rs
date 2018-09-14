
extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate cairo;

use gio::prelude::*;

use gtk::prelude::*;

use std::env::args;

pub mod enums;
<<<<<<< HEAD
mod controller;

use controller::{Controller};

// Builds UI of main window with gtk builder
fn build_ui(application: &gtk::Application) {
    let main_window_src = include_str!(r"./ui/main_window.ui");

    // TODO: Why is Builder::new_from_string() not available???
    let builder = gtk::Builder::new();
    builder.add_from_string(main_window_src).expect("Cannot add main_window_src");

    let main_window: gtk::Window = builder.get_object("main_window").expect("Could not get main_window");
    let brush_selector: gtk::ToggleButton = builder.get_object("brush_selector").expect("Could not get pencil_selector");
    let eraser_selector: gtk::ToggleButton = builder.get_object("eraser_selector").expect("Could not get eraser_selector");
    let drawing_area: gtk::DrawingArea = builder.get_object("drawing_area").expect("Could not get drawing_area");
    
    let controller = Controller::new(drawing_area);

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Tool selector buttons
    let controller_clone = controller.clone();
    let eraser_selector_clone = eraser_selector.clone();
    brush_selector.connect_clicked(move |_| {
        eraser_selector_clone.set_active(false);
        controller_clone.borrow_mut().set_tool("Brush");
    });

    let controller_clone = controller.clone();
    let brush_selector_clone = eraser_selector.clone();
    eraser_selector.connect_clicked(move |_| {
        brush_selector_clone.set_active(false);
        controller_clone.borrow_mut().set_tool("Eraser");
    });
    main_window.show_all();
}

fn main() {
    let application = gtk::Application::new("org.github.thisisian.rspaint",
                                            gio::ApplicationFlags::empty()).expect("Initialization Failed...");

    application.connect_startup(move |app| {
        build_ui(app)
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());

    gtk::main();
}
