extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gio::MenuExt;

use gtk::{Button, Window, WindowType, ApplicationWindow};
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
    window.set_default_size(500,500);

    let h_box = gtk::Box::new(Horizontal, 0);
    let v_box = gtk::Box::new(Vertical, 0);


    let toolbar = gtk::Toolbar::new();
    build_toolbar(&toolbar);
    v_box.pack_start(&toolbar, false, false, 0);
    window.add(&v_box);
    build_menu(application);
    window.show_all();
}

fn build_toolbar(toolbar: &gtk::Toolbar) {
    let pencil_icon = gtk::Image::new_from_icon_name("face-smile", gtk::IconSize::SmallToolbar.into());
    let eraser_icon = gtk::Image::new_from_icon_name("face-sad", gtk::IconSize::SmallToolbar.into());
    let pencil_button = gtk::ToolButton::new(&pencil_icon, "Pencil");
    let eraser_button = gtk::ToolButton::new(&eraser_icon, "Eraser");
    toolbar.insert(&pencil_button, 0);
    toolbar.insert(&eraser_button, 0);
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

fn build_status_bar() {
    unimplemented!()
}

fn build_pallette() {
    unimplemented!()
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
