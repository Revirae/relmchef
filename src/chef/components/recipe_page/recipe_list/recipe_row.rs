// #![allow(deprecated)]
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryComponent};
use libadwaita::ComboRow;
use libadwaita::prelude::{
    PreferencesRowExt,
    ComboRowExt,
    ActionRowExt,
    ListBoxRowExt,
};
use gtk::prelude::WidgetExt;

use crate::chef::models;


#[derive(Debug)]
pub struct RecipeRow {
    title: String,
    subtitle: String,
    #[allow(dead_code)]
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
            set_activatable: false,
            // set_selectable: false,
            set_hexpand: true,
            set_title: &self.title,
            set_subtitle: &self.subtitle,
        
            #[wrap(Some)]
            set_model = &gtk::StringList::new(&[
                &"",
                &"editar",
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
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
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
    fn init_model(recipe: Self::Init, index: &Self::Index, _sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            index: index.clone().into(),
            title: recipe.name,
            subtitle: String::new(),
        }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::FactorySender<Self>) {
        match message {
            RecipeRowCommand::Action(action, index) => {
                // dbg!(action);
                // dbg!(index.clone());
                let message = match action {
                    2 => RecipeRowMessage::DeleteMe(index),
                    1 => RecipeRowMessage::UpdateMe(index),
                    _ => RecipeRowMessage::NoMessage,
                };
                sender.output(message)
                    .expect("failed to output recipe row message while processing Action above");
            }
            RecipeRowCommand::NoCommand => {}
        }
    }
}
