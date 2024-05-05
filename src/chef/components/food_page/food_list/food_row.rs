#![allow(deprecated)]
use libadwaita::{builders::ComboRowBuilder, ComboRow};
use relm4::{factory::DynamicIndex, Component};
use relm4::{gtk, ComponentController};
use relm4::prelude::FactoryComponent;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use libadwaita::prelude::ComboRowExt;
use relm4::adw::prelude::{
    PreferencesRowExt,
    ActionRowExt,
    ComboBoxExt,
};
use relm4_components::simple_adw_combo_row::SimpleComboRow;

use crate::chef::app::AppMessage;
use crate::chef::models;


#[derive(Debug)]
pub struct FoodRow {
    title: String,
    subtitle: String,
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
        ComboRow {
            set_hexpand: true,
            set_title: &self.title,
            set_subtitle: &self.subtitle,
            // set_title_selectable: false,
            
            #[wrap(Some)]
            set_model = &gtk::StringList::new(&[
                &"",
                &"----",
                &"excluir"
            ]),

            // set_selected: CONFIG.game.enhancements.gamescope.window_type.ordinal() as u32,

            connect_selected_notify => |row| unsafe {
                dbg!(row.selected());
            }
        }
    }

    fn init_widgets(
            &mut self,
            index: &Self::Index,
            root: Self::Root,
            returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: relm4::prelude::FactorySender<Self>,
        ) -> Self::Widgets {
        let menu_builder = SimpleComboRow::builder()
            .launch(SimpleComboRow{
                variants: vec!["a","b"],
                active_index: None
            })
            .forward(sender.input_sender(), |_| FoodRowCommand::NoCommand);
        let menu = menu_builder.widget();
            // .widget();
        // let menu = SimpleComc
        let widgets = view_output!();
        widgets
    }
    fn init_model(food: Self::Init, index: &Self::Index, sender: relm4::prelude::FactorySender<Self>) -> Self {
        // let menu = ComboRow::builder().build();
        let menu: &ComboRow = SimpleComboRow::<String>::builder().widget();
        Self {
            title: food.name,
            subtitle: food.brand,
        }
    }
}
