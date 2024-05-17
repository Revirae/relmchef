mod recipe_row;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::RelmListBoxExt;
use relm4::{ComponentParts, SimpleComponent};

use crate::chef::components::recipe_page::recipe_list::recipe_row::RecipeRowMessage;
use crate::chef::models::Recipe;

use self::recipe_row::RecipeRow;


// #[derive(Default, Debug)]
// pub struct RecipeListState {}

#[derive(Debug)]
pub struct RecipeListModel {
    #[allow(dead_code)]
    // state: RecipeListState,
    recipelist: FactoryVecDeque<RecipeRow>,
}


#[derive(Default, Debug)]
pub enum RecipeListCommand {
    #[default]
    NoCommand,
    AddEntry(Recipe),
    InsertEntry(usize, Recipe),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex),
    BuildEntry(usize),
}

#[derive(Default, Debug)]
pub enum RecipeListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
    RequestBuilding(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for RecipeListModel {
    type Init = ();
    type Input = RecipeListCommand;
    type Output = RecipeListMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_min_content_height: 360,

                #[local_ref]
                recipe_listbox -> gtk::ListBox {}
            }
        }
    }
    fn init(
            _init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let recipelist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |message| match message {
                RecipeRowMessage::NoMessage =>
                    RecipeListCommand::NoCommand,
                RecipeRowMessage::DeleteMe(index) =>
                    RecipeListCommand::DeleteEntry(index),
                RecipeRowMessage::UpdateMyName(index) =>
                    RecipeListCommand::UpdateEntry(index),
            });
        let model = RecipeListModel {
            // state: init,
            recipelist
        };
        let recipe_listbox = model.recipelist.widget();
        recipe_listbox.connect_row_selected(move |list, maybe_row| {
            if let Some(row) = maybe_row {
                if let Some(index) = list.index_of_child(row) {
                    dbg!(index);
                    sender.input(RecipeListCommand::BuildEntry(index as usize))
                }
            }
        });
        // recipe_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            RecipeListCommand::NoCommand => {}
            RecipeListCommand::AddEntry(recipe) => {
                self.recipelist.guard().push_back(recipe);
            }
            RecipeListCommand::InsertEntry(index, recipe) => {
                self.recipelist.guard().remove(index);
                self.recipelist.guard().insert(index, recipe);
            }
            RecipeListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.recipelist.guard().remove(i);
                sender.output(RecipeListMessage::RequestRemoval(i))
                    .expect("failed to request recipe removal")
            }
            RecipeListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                sender.output(RecipeListMessage::RequestUpdate(i))
                    .expect("failed to request recipe update")
            }
            RecipeListCommand::BuildEntry(index) => {
                // let i = index.current_index();
                sender.output(RecipeListMessage::RequestBuilding(index))
                    .expect("failed to request recipe build mode")
            }
        }
    }
}
