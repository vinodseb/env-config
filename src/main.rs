use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Environment Config");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.vinodseb.env-config"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
