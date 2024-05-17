pub mod food_portion_row;
use self::food_portion_row::FoodPortionRow;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::{ComponentParts, SimpleComponent};

use crate::chef::{ components, models };
use components::recipe_page::food_portion_list::food_portion_row::FoodPortionRowMessage;
use models::FoodPortion;


#[derive(Default, Debug)]
pub struct FoodPortionListState {}

#[derive(Debug)]
pub struct FoodPortionListModel {
    #[allow(dead_code)]
    state: FoodPortionListState,
    portionlist: FactoryVecDeque<FoodPortionRow>,
}


#[derive(Default, Debug)]
pub enum FoodPortionListCommand {
    #[default]
    NoCommand,
    Clear,
    AddEntry(FoodPortion),
    InsertEntry(usize, FoodPortion),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex)
}

#[derive(Default, Debug)]
pub enum FoodPortionListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for FoodPortionListModel {
    type Init = FoodPortionListState;
    type Input = FoodPortionListCommand;
    type Output = FoodPortionListMessage;
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
                FoodPortionRowMessage::NoMessage =>
                    FoodPortionListCommand::NoCommand,
                FoodPortionRowMessage::DeleteMe(index) =>
                    FoodPortionListCommand::DeleteEntry(index), //DeleteEntry
                FoodPortionRowMessage::EditMe(index) =>
                    FoodPortionListCommand::UpdateEntry(index),
            });
        let model = FoodPortionListModel {
            state: init,
            portionlist
        };
        let portion_listbox = model.portionlist.widget();
        // portion_listbox.connect_row_selected(|_,_|{});       
        // portion_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FoodPortionListCommand::NoCommand => {}
            FoodPortionListCommand::Clear => {
                self.portionlist.guard().clear();
            }
            FoodPortionListCommand::AddEntry(portion) => {
                self.portionlist.guard().push_back(portion);
            }
            FoodPortionListCommand::InsertEntry(index, portion) => {
                self.portionlist.guard().remove(index);
                self.portionlist.guard().insert(index, portion);
            }
            FoodPortionListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.portionlist.guard().remove(i);
                sender.output(FoodPortionListMessage::RequestRemoval(i))
                    .expect("failed to request food portion removal");
            }
            FoodPortionListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                sender.output(FoodPortionListMessage::RequestUpdate(i))
                    .expect("failed to request food portion update");
            }
        }
    }
}
