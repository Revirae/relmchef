#![allow(deprecated)]
use relm4::{factory::{DynamicIndex, FactoryComponent}, gtk};
use libadwaita::{builders::ComboRowBuilder, ComboRow, PreferencesRow};
use libadwaita::prelude::{PreferencesRowExt, ComboRowExt};
use relm4::{RelmWidgetExt,RelmContainerExt,RelmSetChildExt};
use relm4::adw::prelude::{
    // PreferencesRowExt,
    ActionRowExt,
    ComboBoxExt,
    ListBoxRowExt,
    BoxExt,
};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};

use crate::chef::models;

#[derive(Debug)]
pub struct RecipeRow {
    title: String,
    subtitle: String,
    index: DynamicIndex,
}

#[derive(Default, Debug)]
pub enum RecipeRowCommand {
    #[default]
    NoCommand,
    Action(u32, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum RecipeRowMessage {
    #[default]
    NoMessage,
    DeleteMe(DynamicIndex),
    UpdateMe(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for RecipeRow {
    type Init = models::Recipe;
    type Input = RecipeRowCommand;
    type Output = RecipeRowMessage;
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
            // connect_selected_notify[sender] => move |crow| unsafe {
                // sender.input(RecipeRowCommand::Action(crow.index(), self.index));
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
            let index = index.clone();
            root.connect_selected_item_notify(move |cr|
                sender.input(RecipeRowCommand::Action(
                    cr.selected(), 
                    index.clone()
                ))
            );
            let widgets = view_output!();     
            widgets
    }
    fn init_model(recipe: Self::Init, index: &Self::Index, sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            index: index.clone().into(),
            title: recipe.name,
            subtitle: String::new(),
        }
    }
}
