#![allow(unused)]
mod food_form;

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

#[derive(Debug)]
pub struct PageModel{
    food_form: Controller<food_form::FormModel>,
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
        let food_form = food_form::FormModel::builder()
            .launch(food_form::FormState::default())
            .forward(sender.input_sender(), |msg| match msg {
                _ => { PageCommand::NoCommand }
            });
        let model = PageModel { food_form };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
