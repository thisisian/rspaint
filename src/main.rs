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

mod controller;

use controller::Controller;

fn initialize(application: &gtk::Application) {
    let controller = Controller::new();

    let window = ApplicationWindow::new(application);
    window.set_title("RSPaint");
    window.set_default_size(500, 500); // TODO: Set to screen resolution?
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let h_box = gtk::Box::new(Horizontal, 0);

    let tool_box = gtk::Box::new(Vertical, 0);
    build_tool_menu(&tool_box, controller.clone());

    let canvas_box = gtk::Box::new(Vertical, 0);
    canvas_box.pack_start(&controller.borrow().get_drawing_area(), false, false, 10);

    h_box.pack_start(&tool_box, false, false, 0);
    h_box.pack_start(&canvas_box, false, false, 10);
    window.add(&h_box);

    build_menu(application);
    window.show_all();
}

fn build_tool_menu(tool_box: &gtk::Box, controller: Rc<RefCell<Controller>>) {
    let pencil_button = gtk::ToggleButton::new();
    let eraser_button = gtk::ToggleButton::new();

    let pencil_icon = gtk::Image::new_from_icon_name("face-smile", gtk::IconSize::SmallToolbar.into());
    let eraser_icon = gtk::Image::new_from_icon_name("face-sad", gtk::IconSize::SmallToolbar.into());
    pencil_button.set_image(&pencil_icon);
    eraser_button.set_image(&eraser_icon);
    pencil_button.set_label("Pencil");
    eraser_button.set_label("Eraser");

    let pencil_button_clone = pencil_button.clone();
    pencil_button.connect_toggled(|this| {
    });
    let eraser_button_clone = eraser_button.clone();
    eraser_button.connect_toggled(|this| {
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

    application.connect_startup(move |app| {
        initialize(app)
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());

    gtk::main();
}