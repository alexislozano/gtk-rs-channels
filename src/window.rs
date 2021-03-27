use gtk::glib::{Continue, MainContext, Sender, PRIORITY_DEFAULT};
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindowBuilder, BoxBuilder, HeaderBarBuilder, Orientation,
};

use super::counter;

pub enum Message {
    Counter1(counter::Message),
    Counter2(counter::Message),
}

pub struct Model {
    counter1: counter::Model,
    counter2: counter::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            counter1: counter::Model::new(10),
            counter2: counter::Model::new(20),
        }
    }

    pub fn update(&mut self, msg: &Message) {
        match msg {
            Message::Counter1(msg) => self.counter1.update(msg),
            Message::Counter2(msg) => self.counter2.update(msg),
        }
    }
}

pub struct View {
    counter1: counter::View,
    counter2: counter::View,
}

impl View {
    pub fn new(model: &Model) -> Self {
        Self {
            counter1: counter::View::new(&model.counter1),
            counter2: counter::View::new(&model.counter2),
        }
    }

    pub fn present(&self, app: &Application) {
        let window = ApplicationWindowBuilder::new()
            .application(app)
            .title("Gtk-rs channels")
            .default_width(320)
            .default_height(240)
            .build();

        let header_bar = HeaderBarBuilder::new().build();

        let center_box = BoxBuilder::new()
            .halign(Align::Center)
            .valign(Align::Center)
            .orientation(Orientation::Vertical)
            .build();

        center_box.append(&self.counter1.present());
        center_box.append(&self.counter2.present());

        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&center_box));
        window.show();
    }

    pub fn transmit(&self, tx: Sender<Message>) {
        let _tx = tx.clone();
        let (counter1_tx, counter1_rx) = MainContext::channel(PRIORITY_DEFAULT);
        self.counter1.transmit(counter1_tx);
        counter1_rx.attach(None, move |msg| {
            _tx.send(Message::Counter1(msg)).unwrap();
            Continue(true)
        });

        let _tx = tx.clone();
        let (counter2_tx, counter2_rx) = MainContext::channel(PRIORITY_DEFAULT);
        self.counter2.transmit(counter2_tx);
        counter2_rx.attach(None, move |msg| {
            _tx.send(Message::Counter2(msg)).unwrap();
            Continue(true)
        });
    }

    pub fn refresh(&mut self, msg: &Message, model: &Model) {
        match msg {
            Message::Counter1(_) => self.counter1.refresh(&model.counter1),
            Message::Counter2(_) => self.counter2.refresh(&model.counter2),
        }
    }
}
