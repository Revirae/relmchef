use relm4::factory::DynamicIndex;
use relm4::{adw, gtk};
use relm4::prelude::FactoryComponent;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use crate::chef::models;


#[derive(Debug)]
pub struct FoodRow {
    title: String,
}

#[derive(Default, Debug)]
pub enum FoodRowCommand {
    #[default]
    NoCommand,
    DeleteMe(DynamicIndex),
}

#[derive(Default, Debug)]
pub enum FoodRowMessage {
    #[default]
    NoMessage,
}

#[relm4::factory(pub)]
impl FactoryComponent for FoodRow {
    type Init = models::Food;
    type Input = FoodRowCommand;
    type Output = FoodRowCommand;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            adw::ComboRow {
                set_title: &self.title,
            }
        }
    }
    fn init_model(init: Self::Init, index: &Self::Index, sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            title: init.name,
        }
    }
}
