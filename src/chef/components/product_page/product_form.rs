use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{Component, ComponentParts};

use crate::chef::models;

#[derive(Debug)]
pub struct ProductFormModel {
    state: models::Product,
}

#[derive(Default, Debug)]
pub enum ProductFormMessage {
    #[default]
    NoMessage,
    Submit(models::Product),
}
#[derive(Default, Debug)]
pub enum ProductFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::Product),
    ChangeName(String),
}
#[derive(Default, Debug)]
pub enum ProductFormAction {
    #[default]
    NoAction,
    Fill,
}

#[relm4::component(pub)]
impl Component for ProductFormModel {
    type Init = models::Product;
    type Input = ProductFormCommand;
    type Output = ProductFormMessage;
    type CommandOutput = ProductFormAction;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            #[name(name_entry)]
            adw::EntryRow {
                set_title: "Nome do produto",
                set_max_width_chars: 8,
                connect_changed[sender] => move |entry| {
                    let name = entry.text().to_string();
                    sender.input(
                        ProductFormCommand::ChangeName(name)
                    )
                }
            },
            #[name(send_button)]
            gtk::Button {
                set_icon_name: "document-new",
                set_size_request: (50, 32),
                connect_clicked[sender] => move |_| {
                    sender.input(ProductFormCommand::Send)
                }
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProductFormModel { state: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            ProductFormCommand::NoCommand => {}
            ProductFormCommand::Send => {
                // todo!("validation");
                sender.output(ProductFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit product form");
            }
            ProductFormCommand::Receive(product) => {
                dbg!(product.clone());
                self.state = product;
                sender.spawn_command(|sender|
                    sender.emit(ProductFormAction::Fill)
                );
            }
            ProductFormCommand::ChangeName(text) => {
                self.state.name = text;
            }
        }
    }
}
