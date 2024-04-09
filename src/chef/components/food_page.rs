#![allow(unused)]
mod food_form;
mod food_list;

use relm4::gtk;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt
};
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};

use crate::chef::models::Food;

use food_form::{
    FoodFormMessage,
    FoodFormModel,
};
use food_list::{
    FoodListMessage,
    FoodListModel,
    FoodListState
};

use self::food_list::FoodListCommand;


#[derive(Default, Debug)]
pub enum FoodPageMode {
    #[default]
    Inserting,
    Editing,
    Filtering,
}

#[derive(Default, Debug)]
pub struct FoodPageState {
    mode: FoodPageMode,
    foodlist: Vec<Food>,
}

#[derive(Debug)]
pub struct FoodPageModel {
    state: FoodPageState,
    food_form: Controller<FoodFormModel>,
    food_list: Controller<FoodListModel>,
}

#[derive(Default, Debug)]
pub enum FoodPageCommand {
    #[default]
    NoCommand,
    Load(Vec<Food>),
    Append(Food),
}

#[derive(Default, Debug)]
pub enum FoodPageMessage {
    #[default]
    NoMessage,
    Store(Vec<Food>)
}

#[relm4::component(pub)]
impl SimpleComponent for FoodPageModel  {
    type Init = FoodPageState;
    type Input = FoodPageCommand;
    type Output = FoodPageMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            model.food_form.widget(),           
            model.food_list.widget(),
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let food_form = FoodFormModel::builder()
            .launch(Food::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodFormMessage::NoMessage => {
                    FoodPageCommand::NoCommand
                }
                FoodFormMessage::Changed => {
                    FoodPageCommand::NoCommand
                }
                FoodFormMessage::Submit(food) => {
                    FoodPageCommand::Append(food)    
                }
            });
        let food_list = FoodListModel::builder()
            .launch(FoodListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodListMessage::NoMessage => {
                    FoodPageCommand::NoCommand
                }
            });
        let state = FoodPageState::default();
        let model = FoodPageModel  {
            state,
            food_form,
            food_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        match message {
            FoodPageCommand::NoCommand => {}
            FoodPageCommand::Load(foodlist) => {
                self.state.foodlist = foodlist.clone();
                for food in foodlist {
                    self.food_list.emit(
                        FoodListCommand::AddEntry(food)
                    );
                }
            }    
            FoodPageCommand::Append(food) => {
                match self.state.mode {
                    FoodPageMode::Inserting => {
                        self.food_list.emit(
                            FoodListCommand::AddEntry(food)
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
