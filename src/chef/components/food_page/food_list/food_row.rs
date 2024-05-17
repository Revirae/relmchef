use gtk::prelude::WidgetExt;

use relm4::gtk;
use relm4::prelude::FactoryComponent;
use relm4::prelude::RelmWidgetExt;
use relm4::adw::prelude::ActionRowExt;
use relm4::factory::DynamicIndex;

use libadwaita::ComboRow;
use libadwaita::prelude::{
    PreferencesRowExt,
    ComboRowExt,
    ListBoxRowExt,
};

use crate::chef::models;


#[derive(Debug)]
pub struct FoodRow {
    title: String,
    subtitle: String,
    info_cost: String,
    info_qtd: String,
    // #[allow(dead_code)]
    // index: DynamicIndex,
}

#[derive(Default, Debug)]
pub enum FoodRowCommand {
    #[default]
    NoCommand,
    Action(u32, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum FoodRowMessage {
    #[default]
    NoMessage,
    DeleteMe(DynamicIndex),
    UpdateMe(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for FoodRow {
    type Init = models::Food;
    type Input = FoodRowCommand;
    type Output = FoodRowMessage;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        ComboRow {
            set_selectable: false,
            set_activatable: false,
            add_suffix = &gtk::Box {
                set_hexpand: true,
                gtk::Label {
                    set_margin_horizontal: 25,
                    set_text: &self.info_cost,
                },
                gtk::Label {
                    set_margin_horizontal: 25,
                    set_text: &self.info_qtd,
                }
            },
            set_activatable: false,
            // set_hexpand: true,
            set_title: &self.title,
            set_subtitle: &self.subtitle,
        
            #[wrap(Some)]
            set_model = &gtk::StringList::new(&[
                &"",
                &"editar",
                &"excluir"
            ]),
        }
    }

    fn init_widgets(
            &mut self,
            index: &Self::Index,
            root: Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: relm4::prelude::FactorySender<Self>,
        ) -> Self::Widgets {
            let index = index.clone();
            root.connect_selected_item_notify(move |cr|
                sender.input(FoodRowCommand::Action(
                    cr.selected(), 
                    index.clone()
                ))
            );
            let widgets = view_output!();     
            widgets
    }
    fn init_model(food: Self::Init, _index: &Self::Index, _sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            // index: index.clone().into(),
            title: food.name,
            subtitle: food.brand,
            info_cost: "R$ ".to_owned() + &food.cost.to_string(),
            info_qtd: food.weight.to_string() + " g",
        }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::FactorySender<Self>) {
        match message {
            FoodRowCommand::Action(action, index) => {
                let message = match action {
                    2 => FoodRowMessage::DeleteMe(index),
                    1 => FoodRowMessage::UpdateMe(index),
                    _ => FoodRowMessage::NoMessage,
                };
                sender.output(message)
                    .expect("failed to output food row message while processing Action above");
            }
            FoodRowCommand::NoCommand => {}
        }
    }
}
