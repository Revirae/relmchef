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

#[derive(Debug)]
pub enum RecipeRowAction {
    Edit,
    EditName,
    Delete,
}

impl RecipeRowAction {
    pub fn to_string(&self) -> String {
        match self {
            Self::Edit => { "editar".into() }
            Self::EditName => { "mudar nome".into() }
            Self::Delete => { "remover".into() }
        }
    }
}

#[derive(Default, Debug)]
pub enum RecipeRowCommand {
    #[default]
    NoCommand,
    Action(RecipeRowAction, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum RecipeRowMessage {
    #[default]
    NoMessage,
    DeleteMe(DynamicIndex),
    UpdateMyName(DynamicIndex),
    BuildMode(DynamicIndex),
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
                &RecipeRowAction::Edit.to_string(),
                &RecipeRowAction::EditName.to_string(),
                &RecipeRowAction::Delete.to_string(),
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
            root.connect_selected_item_notify(move |cr| {
                // dbg!("dafuk");
                let maybe_action = match cr.selected() {
                    1 => Some(RecipeRowAction::Edit),
                    2 => Some(RecipeRowAction::EditName),
                    3 => Some(RecipeRowAction::Delete),
                    _ => None
                };
                if let Some(action) = maybe_action {
                    sender.input(RecipeRowCommand::Action(action, index.clone()))
                }
            });
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
                let message = match action {
                    RecipeRowAction::Delete => RecipeRowMessage::DeleteMe(index),
                    RecipeRowAction::EditName => RecipeRowMessage::UpdateMyName(index),
                    RecipeRowAction::Edit => RecipeRowMessage::BuildMode(index),
                };
                sender.output(message)
                    .expect("failed to output recipe row message while processing selected Action");
            }
            RecipeRowCommand::NoCommand => {}
        }
    }
}
