// #![allow(deprecated)]
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryComponent};
use libadwaita::ComboRow;
use libadwaita::prelude::{PreferencesRowExt, ComboRowExt, ActionRowExt};
use gtk::prelude::WidgetExt;

use crate::chef::models;

#[derive(Debug)]
pub struct ProductRow {
    title: String,
    subtitle: String,
    #[allow(dead_code)]
    index: DynamicIndex,
}

#[derive(Default, Debug)]
pub enum ProductRowCommand {
    #[default]
    NoCommand,
    Action(u32, DynamicIndex),
}

#[derive(Default, Debug)]
pub enum ProductRowMessage {
    #[default]
    NoMessage,
    DeleteMe(DynamicIndex),
    UpdateMe(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for ProductRow {
    type Init = models::Product;
    type Input = ProductRowCommand;
    type Output = ProductRowMessage;
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
                // sender.input(ProductRowCommand::Action(crow.index(), self.index));
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
                sender.input(ProductRowCommand::Action(
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
            ProductRowCommand::Action(action, index) => {
                dbg!(action);
                dbg!(index.clone());
                let message = match action {
                    2 => ProductRowMessage::DeleteMe(index),
                    1 => ProductRowMessage::UpdateMe(index),
                    _ => ProductRowMessage::NoMessage,
                };
                sender.output(message)
                    .expect("failed to output food row message while processing Action above");
            }
            ProductRowCommand::NoCommand => {}
        }
    }
}
