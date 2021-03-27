use gtk::glib::Sender;
use gtk::prelude::*;
use gtk::{Align, Button, ButtonBuilder, CenterBox, CenterBoxBuilder};

pub enum Message {
    Decrement,
    Increment,
    Reset,
}

pub struct Model {
    value: i8,
}

impl Model {
    pub fn new(value: i8) -> Self {
        Self { value }
    }

    pub fn update(&mut self, msg: &Message) {
        match msg {
            Message::Decrement => {
                if self.value > i8::MIN {
                    self.value -= 1
                }
            }
            Message::Increment => {
                if self.value < i8::MAX {
                    self.value += 1
                }
            }
            Message::Reset => {
                if self.value != 0 {
                    self.value = 0
                }
            }
        };
    }
}

pub struct View {
    reset_button: Button,
    decrement_button: Button,
    increment_button: Button,
}

impl View {
    pub fn new(model: &Model) -> Self {
        Self {
            reset_button: ButtonBuilder::new().label(&model.value.to_string()).build(),
            decrement_button: ButtonBuilder::new().label("-").build(),
            increment_button: ButtonBuilder::new().label("+").build(),
        }
    }

    pub fn present(&self) -> CenterBox {
        let center_box = CenterBoxBuilder::new()
            .halign(Align::Center)
            .valign(Align::Center)
            .margin_top(8)
            .margin_bottom(8)
            .build();

        center_box.set_start_widget(Some(&self.decrement_button));
        center_box.set_center_widget(Some(&self.reset_button));
        center_box.set_end_widget(Some(&self.increment_button));

        center_box
    }

    pub fn transmit(&self, tx: Sender<Message>) {
        let _tx = tx.clone();
        self.decrement_button
            .connect_clicked(move |_| _tx.send(Message::Decrement).unwrap());

        let _tx = tx.clone();
        self.increment_button
            .connect_clicked(move |_| _tx.send(Message::Increment).unwrap());

        let _tx = tx.clone();
        self.reset_button
            .connect_clicked(move |_| _tx.send(Message::Reset).unwrap());
    }

    pub fn refresh(&mut self, model: &Model) {
        let value = model.value;

        if value == 0 {
            self.reset_button.set_sensitive(false);
        } else if !self.reset_button.get_sensitive() {
            self.reset_button.set_sensitive(true);
        }

        if value == i8::MAX {
            self.increment_button.set_sensitive(false);
        } else if !self.reset_button.get_sensitive() {
            self.increment_button.set_sensitive(true);
        }

        if value == i8::MIN {
            self.decrement_button.set_sensitive(false);
        } else if !self.reset_button.get_sensitive() {
            self.decrement_button.set_sensitive(true);
        }

        self.reset_button.set_label(&value.to_string());
    }
}
