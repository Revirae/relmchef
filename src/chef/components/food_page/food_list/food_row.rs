#![allow(deprecated)]
use libadwaita::{builders::ComboRowBuilder, ComboRow, PreferencesRow};
// use relm4::gtk::traits::BoxExt;
use relm4::{factory::DynamicIndex, Component};
use relm4::{gtk, ComponentController};
use relm4::prelude::FactoryComponent;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use libadwaita::prelude::{PreferencesRowExt, ComboRowExt};
use relm4::{RelmWidgetExt,RelmContainerExt,RelmSetChildExt};
use relm4::adw::prelude::{
    // PreferencesRowExt,
    ActionRowExt,
    ComboBoxExt,
    ListBoxRowExt,
    BoxExt,
};
use relm4_components::simple_adw_combo_row::SimpleComboRow;

use crate::chef::app::AppMessage;
use crate::chef::models;


#[derive(Debug)]
pub struct FoodRow {
    title: String,
    subtitle: String,
    index: DynamicIndex,
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
        // gtk::Box {
            // append = 
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
            // connect_selected_notify[sender] => move |crow| unsafe {
                // sender.input(FoodRowCommand::Action(crow.index(), self.index));
            // }
        }
    }

    fn init_widgets(
            &mut self,
            index: &Self::Index,
            root: Self::Root,
            returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: relm4::prelude::FactorySender<Self>,
        ) -> Self::Widgets {
            // root.set_model()
            // sender.position()
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
    fn init_model(food: Self::Init, index: &Self::Index, sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            index: index.clone().into(),
            title: food.name,
            subtitle: food.brand,
        }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::FactorySender<Self>) {
        match message {
            FoodRowCommand::Action(action, index) => {
                dbg!(action);
                dbg!(index.clone());
                let message = match action {
                    2 => FoodRowMessage::DeleteMe(index),
                    1 => FoodRowMessage::UpdateMe(index),
                    _ => FoodRowMessage::NoMessage,
                };
                sender.output(message);
            }
            FoodRowCommand::NoCommand => {}
        }
    }
}
