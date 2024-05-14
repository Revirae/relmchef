pub mod recipe_portion_row;
use self::recipe_portion_row::RecipePortionRow;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::{ComponentParts, SimpleComponent};

use crate::chef::{ components, models };
use components::product_page::recipe_portion_list::recipe_portion_row::RecipePortionRowMessage;
use models::RecipePortion;


#[derive(Default, Debug)]
pub struct RecipePortionListState {}

#[derive(Debug)]
pub struct RecipePortionListModel {
    #[allow(dead_code)]
    state: RecipePortionListState,
    portionlist: FactoryVecDeque<RecipePortionRow>,
}


#[derive(Default, Debug)]
pub enum RecipePortionListCommand {
    #[default]
    NoCommand,
    AddEntry(RecipePortion),
    InsertEntry(usize, RecipePortion),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex)
}

#[derive(Default, Debug)]
pub enum RecipePortionListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for RecipePortionListModel {
    type Init = RecipePortionListState;
    type Input = RecipePortionListCommand;
    type Output = RecipePortionListMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_min_content_height: 360,

                #[local_ref]
                portion_listbox -> gtk::ListBox {
                    connect_row_activated => |_, _| {}
                    // set_selection_mode: gtk::SelectionMode::None,
                    // set_activate_on_single_click: false,
                    // set_css_classes: &[&"boxed-list"],
                    // set_sensitive: false,
                    // set_receives_default: false,
                    // set_activatable: false,
                    // set_selectable: false,
                }
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let portionlist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |message| match message {
                RecipePortionRowMessage::NoMessage =>
                    RecipePortionListCommand::NoCommand,
                RecipePortionRowMessage::DeleteMe(index) =>
                    RecipePortionListCommand::DeleteEntry(index), //DeleteEntry
                RecipePortionRowMessage::UpdateMe(index) =>
                    RecipePortionListCommand::UpdateEntry(index),
            });
        let model = RecipePortionListModel {
            state: init,
            portionlist
        };
        let portion_listbox = model.portionlist.widget();
        
        // portion_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            RecipePortionListCommand::NoCommand => {}
            RecipePortionListCommand::AddEntry(portion) => {
                self.portionlist.guard().push_back(portion);
            }
            RecipePortionListCommand::InsertEntry(index, portion) => {
                self.portionlist.guard().remove(index);
                self.portionlist.guard().insert(index, portion);
            }
            RecipePortionListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.portionlist.guard().remove(i);
                sender.output(RecipePortionListMessage::RequestRemoval(i))
                    .expect("failed to request portion removal");
            }
            RecipePortionListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                // self.portionlist.guard().remove(i);
                sender.output(RecipePortionListMessage::RequestUpdate(i))
                    .expect("failed to request portion update");
            }
        }
    }
}
