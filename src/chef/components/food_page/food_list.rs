mod food_row;

use relm4::ComponentSender;
use relm4::{adw, gtk};
use relm4::factory::FactoryVecDeque;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{ComponentParts, SimpleComponent};

use food_row::FoodRow;

use self::food_row::FoodRowCommand;
// use crate::chef::models::Food;

#[derive(Default, Debug)]
pub struct FoodListState {
}

#[derive(Debug)]
pub struct FoodListModel {
    state: FoodListState,
    foodlist: FactoryVecDeque<FoodRow>,
}


#[derive(Default, Debug)]
pub enum FoodListCommand {
    #[default]
    NoCommand,
}

#[derive(Default, Debug)]
pub enum FoodListMessage {
    #[default]
    NoMessage,
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
                food_listbox -> gtk::ListBox {}
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let state = FoodListState::default();

        let foodlist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |cmd| match cmd {
               FoodRowCommand::NoCommand =>
                   FoodListCommand::NoCommand, 
            });
        let model = FoodListModel {
            state, foodlist
        };
        let food_listbox = model.foodlist.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
