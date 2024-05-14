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
pub struct RecipeFormModel {
    state: models::Recipe,
}

#[derive(Default, Debug)]
pub enum RecipeFormMessage {
    #[default]
    NoMessage,
    Submit(models::Recipe),
}
#[derive(Default, Debug)]
pub enum RecipeFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::Recipe),
    ChangeName(String),
}
#[derive(Default, Debug)]
pub enum RecipeFormAction {
    #[default]
    NoAction,
    Fill,
}

#[relm4::component(pub)]
impl Component for RecipeFormModel {
    type Init = models::Recipe;
    type Input = RecipeFormCommand;
    type Output = RecipeFormMessage;
    type CommandOutput = RecipeFormAction;
    view! {
        #[root]
        gtk::Box {           
            set_orientation: gtk::Orientation::Horizontal,
            #[name(name_entry)]
            adw::EntryRow {
                // #[watch(skip_init)]
                // set_text: self.state.name.as_ref(),
                set_title: "Nome",
                set_hexpand: true,
                connect_changed[sender] => move |entry| {
                    let name = entry.text().to_string();
                    sender.input(
                        RecipeFormCommand::ChangeName(name)
                    )
                }
            },
            #[name(send_button)]
            gtk::Button {
                set_icon_name: "document-new",
                set_size_request: (50, 32),
                connect_clicked[sender] => move |_| {
                    sender.input(RecipeFormCommand::Send)
                }
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = RecipeFormModel { state: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            RecipeFormCommand::NoCommand => {}
            RecipeFormCommand::Send => {
                // todo!("validation");
                sender.output(RecipeFormMessage::Submit(
                    self.state.clone()
                )).expect("failed to submit form");
            }
            RecipeFormCommand::Receive(food) => {
                dbg!(food.clone());
                self.state = food;
                sender.spawn_command(|sender|
                    sender.emit(RecipeFormAction::Fill)
                );
            }
            RecipeFormCommand::ChangeName(text) => {
                self.state.name = text;
            }
        }
    }
}
