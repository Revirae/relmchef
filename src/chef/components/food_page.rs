// #![allow(unused)]
mod food_form;
mod food_list;

use relm4::gtk;
use gtk::prelude::OrientableExt;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
use uuid::Uuid;

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

use self::{
    food_form::FoodFormCommand,
    food_list::FoodListCommand
};


#[derive(Default, Debug)]
pub enum FoodPageMode {
    #[default]
    Inserting,
    Editing(usize),
    // Filtering,
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
    LoadFoodList(Vec<Food>),
    PutFood(Food),
    Remove(usize),
    Update(usize),
}

#[derive(Default, Debug)]
pub enum FoodPageMessage {
    #[default]
    NoMessage,
    CommitFood(Food),
    CommitFoodRemoval(Uuid),
    CommitFoodUpdate(Uuid, Food),
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
                FoodFormMessage::Submit(food) => {
                    FoodPageCommand::PutFood(food)    
                }
            });
        let food_list = FoodListModel::builder()
            .launch(FoodListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodListMessage::NoMessage => {
                    FoodPageCommand::NoCommand
                }
                FoodListMessage::RequestRemoval(index) => {
                    FoodPageCommand::Remove(index)
                }
                FoodListMessage::RequestUpdate(index) => {
                    FoodPageCommand::Update(index)
                }
            });

        let model = FoodPageModel  {
            state: init,
            food_form,
            food_list
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: relm4::prelude::ComponentSender<Self>) {
        match message {
            FoodPageCommand::NoCommand => {}
            FoodPageCommand::LoadFoodList(foodlist) => {
                self.state.foodlist = foodlist.clone();
                for food in foodlist {
                    self.food_list.emit(
                        FoodListCommand::AddEntry(food)
                    );
                }
            }    
            FoodPageCommand::PutFood(food) => {
                match self.state.mode {
                    FoodPageMode::Editing(index) => {
                        self.food_list.emit(
                            FoodListCommand::InsertEntry(index, food.clone())
                        );
                        sender.output(
                            FoodPageMessage::CommitFoodUpdate(food.id, food)
                        ).expect("failed to commit food update");
                        self.state.mode = FoodPageMode::Inserting;

                        self.food_form.emit(
                            FoodFormCommand::ChangeIcon("document-new".into())
                        );
                    }
                    FoodPageMode::Inserting => {
                        self.state.foodlist.push(food.clone());
                        self.food_list.emit(
                            FoodListCommand::AddEntry(food.clone())
                        );
                        sender.output(
                            FoodPageMessage::CommitFood(food)
                        ).expect("failed to commit food insertion");
                    }
                    // _ => {}
                }
            }
            FoodPageCommand::Remove(index) => {
                let id = self.state.foodlist.get(index).unwrap().id;
                self.state.foodlist.remove(index);
                sender.output(
                    FoodPageMessage::CommitFoodRemoval(id)
                ).expect("failed to commit food removal");
            }
            FoodPageCommand::Update(index) => {
                let food = self.state.foodlist.get(index).unwrap();
                self.food_form.emit(
                    FoodFormCommand::Receive(food.clone())
                );
                self.state.mode = FoodPageMode::Editing(index);
                self.food_form.emit(
                    FoodFormCommand::ChangeIcon("document-save".into())
                );
            }
        }
    }
}
