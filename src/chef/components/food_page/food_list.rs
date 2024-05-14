mod food_row;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::{ComponentParts, SimpleComponent};

use food_row::{FoodRow, FoodRowMessage};
use crate::chef::models::Food;

#[derive(Default, Debug)]
pub struct FoodListState {}

#[derive(Debug)]
pub struct FoodListModel {
    #[allow(dead_code)]
    state: FoodListState,
    foodlist: FactoryVecDeque<FoodRow>,
}


#[derive(Default, Debug)]
pub enum FoodListCommand {
    #[default]
    NoCommand,
    AddEntry(Food),
    InsertEntry(usize, Food),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex)
}

#[derive(Default, Debug)]
pub enum FoodListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for FoodListModel {
    type Init = FoodListState;
    type Input = FoodListCommand;
    type Output = FoodListMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_min_content_height: 360,

                #[local_ref]
                food_listbox -> gtk::ListBox {
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
        let foodlist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |message| match message {
                FoodRowMessage::NoMessage =>
                    FoodListCommand::NoCommand,
                FoodRowMessage::DeleteMe(index) =>
                    FoodListCommand::DeleteEntry(index),
                FoodRowMessage::UpdateMe(index) =>
                    FoodListCommand::UpdateEntry(index),
            });
        let model = FoodListModel {
            state: init,
            foodlist
        };
        let food_listbox = model.foodlist.widget();
        
        // food_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FoodListCommand::NoCommand => {}
            FoodListCommand::AddEntry(food) => {
                self.foodlist.guard().push_back(food);
            }
            FoodListCommand::InsertEntry(index, food) => {
                self.foodlist.guard().remove(index);
                self.foodlist.guard().insert(index, food);
            }
            FoodListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.foodlist.guard().remove(i);
                sender.output(FoodListMessage::RequestRemoval(i))
                    .expect("failed to request food removal");
            }
            FoodListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                // self.foodlist.guard().remove(i);
                sender.output(FoodListMessage::RequestUpdate(i))
                    .expect("failed to update food removal");
            }
        }
    }
}
