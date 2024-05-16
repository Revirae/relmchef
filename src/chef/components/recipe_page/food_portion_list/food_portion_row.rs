// #![allow(deprecated)]
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryComponent};
use libadwaita::ComboRow;
use libadwaita::prelude::{PreferencesRowExt, ComboRowExt};
use relm4::adw::prelude::ActionRowExt;
use gtk::prelude::WidgetExt;

use crate::chef::models;

#[derive(Debug)]
pub struct FoodPortionRow {
    title: String,
    subtitle: String,
    // #[allow(dead_code)]
    // index: DynamicIndex,
}

#[derive(Default, Debug)]
pub enum FoodPortionRowCommand {
    #[default]
    NoCommand,
    #[allow(dead_code)]
    Action(FoodPortionRowAction, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum FoodPortionRowMessage {
    #[default]
    NoMessage,
    DeleteMe(DynamicIndex),
    EditMe(DynamicIndex),
}

#[derive(Debug)]
pub enum FoodPortionRowAction {
    Edit,
    // EditName,
    Delete,
}

impl FoodPortionRowAction {
    pub fn to_string(&self) -> String {
        match self {
            Self::Edit => { "editar".into() }
            // Self::EditName => { "mudar nome".into() }
            Self::Delete => { "remover".into() }
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for FoodPortionRow {
    type Init = models::FoodPortion;
    type Input = FoodPortionRowCommand;
    type Output = FoodPortionRowMessage;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        ComboRow {
            set_hexpand: true,
            set_title: &self.title,
            set_subtitle: &self.subtitle,
        
            #[wrap(Some)]
            set_model = &gtk::StringList::new(&[
                &"",
                &FoodPortionRowAction::Edit.to_string(),
                &FoodPortionRowAction::Delete.to_string(),
            ]),

            // set_selected: CONFIG.game.enhancements.gamescope.window_type.ordinal() as u32,
            // connect_selected_notify[sender] => move |crow| unsafe {
                // sender.input(FoodPortionRowCommand::Action(crow.index(), self.index));
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
                let maybe_action = match cr.selected() {
                    1 => Some(FoodPortionRowAction::Edit),
                    2 => Some(FoodPortionRowAction::Delete),
                    _ => None
                };
                if let Some(action) = maybe_action {
                    sender.input(FoodPortionRowCommand::Action(action, index.clone()))
                }
            });
            let widgets = view_output!();     
            widgets
    }
    fn init_model(portion: Self::Init, _index: &Self::Index, _sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            // index: index.clone().into(),
            title: portion.ingredient.name,
            subtitle: portion.recipe.name,
        }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::FactorySender<Self>) {
        match message {
            FoodPortionRowCommand::Action(action, index) => {
                let message = match action {
                    FoodPortionRowAction::Delete => FoodPortionRowMessage::DeleteMe(index),
                    FoodPortionRowAction::Edit => FoodPortionRowMessage::EditMe(index),
                };
                sender.output(message)
                    .expect("failed to output recipe row message while processing selected Action");
            }
            FoodPortionRowCommand::NoCommand => {}
        }
    }
}
