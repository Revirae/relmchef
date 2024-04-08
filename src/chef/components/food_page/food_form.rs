use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{ComponentParts, SimpleComponent};

#[derive(Debug)]
pub struct FormModel {
    state: FormState,
}

#[derive(Default, Debug)]
pub struct FormState {
    name: String,
    brand: String,
    cost: f64,
    weight: f64,
    volume: f64
}

#[derive(Default, Debug)]
pub enum FormCommand {
    #[default]
    NoCommand,
    ChangeBrand(String),
}

#[derive(Default, Debug)]
pub enum FormMessage {
    #[default]
    NoMessage,
}

#[relm4::component(pub)]
impl SimpleComponent for FormModel {
    type Init = FormState;
    type Input = FormCommand;
    type Output = FormMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            adw::EntryRow {
                #[watch]
                set_text: model.state.name.as_ref(),
                set_title: "Nome",
            },
            adw::EntryRow {
                #[watch]
                set_text: model.state.brand.as_ref(),
                set_title: "Marca",
                connect_changed => move |entry| {
                    sender.input(
                        FormCommand::ChangeBrand(entry.text().to_string())
                    );
                }
                
            },
            adw::SpinRow {
                set_title: "Custo",
                #[watch]
                set_value: model.state.cost,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            adw::SpinRow {
                set_title: "Peso",
                #[watch]
                set_value: model.state.weight,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            adw::SpinRow {
                set_title: "Volume",
                #[watch]
                set_value: model.state.volume,
                set_digits: 2,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let state = FormState::default();
        let model = FormModel { state };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FormCommand::NoCommand => {}
            FormCommand::ChangeBrand(text) => {
                self.state.brand = text;
            }
        }
    }
}
