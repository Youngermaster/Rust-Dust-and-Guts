use gtk::glib;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Clock");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(260, 40);

    let label = gtk::Label::new(None);
    label.set_text("Hello World");

    window.add(&label);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.clock"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
