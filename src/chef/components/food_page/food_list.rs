mod food_row;

use relm4::ComponentSender;
use relm4::{adw, gtk};
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{ComponentParts, SimpleComponent};

use food_row::FoodRow;

use self::food_row::FoodRowCommand;
use crate::chef::models::Food;

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
    AddEntry(Food),
    DeleteEntry(DynamicIndex)
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
                food_listbox -> gtk::ListBox {
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
        let state = FoodListState::default();

        let foodlist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |cmd| match cmd {
                FoodRowCommand::NoCommand =>
                    FoodListCommand::NoCommand, 
                FoodRowCommand::DeleteMe(index) =>
                    FoodListCommand::DeleteEntry(index)   
            });
        let model = FoodListModel {
            state, foodlist
        };
        let food_listbox = model.foodlist.widget();
        // food_listbox.set_selection_mode(gtk::SelectionMode::None);
        // food_listbox.set_focusable(false);
        // food_listbox.set_can_focus(false);
        // food_listbox.bind_model(, )
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FoodListCommand::NoCommand => {}
            FoodListCommand::AddEntry(food) => {
                self.foodlist.guard().push_back(food);
            }
            FoodListCommand::DeleteEntry(index) => {
                self.foodlist.guard().remove(index.current_index());
            }
        }
    }
}
