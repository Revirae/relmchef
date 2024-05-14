// #![allow(deprecated)]
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryComponent};
use libadwaita::ComboRow;
use libadwaita::prelude::{PreferencesRowExt, ComboRowExt};
use relm4::adw::prelude::ActionRowExt;
use gtk::prelude::WidgetExt;

use crate::chef::models;

#[derive(Debug)]
pub struct PortionRow {
    title: String,
    subtitle: String,
    #[allow(dead_code)]
    index: DynamicIndex,
}

#[derive(Default, Debug)]
pub enum PortionRowCommand {
    #[default]
    NoCommand,
    #[allow(dead_code)]
    Action(u32, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum PortionRowMessage {
    #[default]
    NoMessage,
    #[allow(dead_code)]
    DeleteMe(DynamicIndex),
    #[allow(dead_code)]
    UpdateMe(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for PortionRow {
    type Init = models::FoodPortion;
    type Input = PortionRowCommand;
    type Output = PortionRowMessage;
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
                // sender.input(PortionRowCommand::Action(crow.index(), self.index));
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
                sender.input(PortionRowCommand::Action(
                    cr.selected(), 
                    index.clone()
                ))
            );
            let widgets = view_output!();     
            widgets
    }
    fn init_model(portion: Self::Init, index: &Self::Index, _sender: relm4::prelude::FactorySender<Self>) -> Self {
        Self {
            index: index.clone().into(),
            title: portion.ingredient.name,
            subtitle: portion.recipe.name,
        }
    }
}
