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

use self::{food_form::{FormMessage, FormModel, FormState}, food_list::{FoodListMessage, FoodListModel, FoodListState}};

#[derive(Debug)]
pub struct PageModel{
    food_form: Controller<FormModel>,
    food_list: Controller<FoodListModel>,
}

#[derive(Default, Debug)]
pub struct PageState;

#[derive(Default, Debug)]
pub enum PageCommand {
    #[default]
    NoCommand,
}

#[derive(Default, Debug)]
pub enum PageMessage {
    #[default]
    NoMessage,
}

#[relm4::component(pub)]
impl SimpleComponent for PageModel {
    type Init = PageState;
    type Input = PageCommand;
    type Output = PageMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            model.food_form.widget(),           
            gtk::Label {
                set_label: "food page",
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let food_form = FormModel::builder()
            .launch(FormState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FormMessage::NoMessage => {
                    PageCommand::NoCommand
                }
                FormMessage::Changed => {
                    PageCommand::NoCommand
                }
            });
        let food_list = FoodListModel::builder()
            .launch(FoodListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                FoodListMessage::NoMessage => {
                    PageCommand::NoCommand
                }
            });
        let model = PageModel { food_form, food_list };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
