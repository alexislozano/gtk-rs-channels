use gtk::glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::Application;

use std::env::args;

mod counter;
mod window;

fn main() {
    let app = Application::new(
        Some("com.github.alexislozano.gtk-rs-channels"),
        Default::default(),
    )
    .expect("Initialization failed...");

    app.connect_activate(|app| {
        let mut model = window::Model::new();
        let mut view = window::View::new(&model);

        let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);

        view.transmit(tx);
        view.present(app);

        rx.attach(None, move |msg| {
            model.update(&msg);
            view.refresh(&msg, &model);
            Continue(true)
        });
    });

    app.run(&args().collect::<Vec<_>>());
}
