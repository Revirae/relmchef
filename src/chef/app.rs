#![allow(unused)]
use crate::chef::components::{header, food_page};

use relm4::{ adw, gtk};
use gtk::prelude::{
    GtkWindowExt, OrientableExt,
    WidgetExt
};
use relm4::{
    SimpleComponent,
    Component,
    ComponentParts,
    ComponentSender,
    Controller,
    ComponentController
};
use relm4::RelmWidgetExt;

#[derive(Default, Debug)]
pub enum AppMode {
    #[default]
    FoodInventory,
    Recipes,
}

#[derive(Default)]
pub struct AppState {
    page: String,
}

#[derive(Default, Debug)]
pub enum AppCommand {
    #[default] 
    NoCommand,
    SetMode(AppMode)
}


#[derive(Debug)]
pub enum AppMessage {
}

pub struct AppModel {
    state: AppState,
    header: Controller<header::Model>,
    food_page: Controller<food_page::PageModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = AppState;
    type Input = AppCommand;
    type Output = AppMessage;

    view! {
        gtk::Window {
            set_title: Some("Chef"),
            set_titlebar: Some(model.header.widget()),
            #[name(main_stack)]
            gtk::Stack {
                #[watch]
                set_visible_child_name: model.state.page.as_ref(),
                add_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    model.food_page.widget(),
                } -> {
                    set_name: "food_page"
                },
                add_child = &gtk::Label {
                    set_label: "dafuq"
                } -> {
                    set_name: "recipe_page",
                },
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let state = AppState::default();
        let header: Controller<header::Model> = header::Model::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                header::Tab::Food => 
                    AppCommand::SetMode(AppMode::FoodInventory),
                header::Tab::Recipe =>
                    AppCommand::SetMode(AppMode::Recipes)
            });
        let food_page = food_page::PageModel::builder()
            .launch(food_page::PageState::default())
            .forward(sender.input_sender(), |msg| match msg {
                _ => { AppCommand::NoCommand }               
            });
        let model = AppModel { state, header, food_page };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppCommand::SetMode(mode) => {
                self.state.page = match mode {
                    AppMode::FoodInventory => 
                        "food_page".to_owned(),
                    AppMode::Recipes =>
                        "recipe_page".to_owned(),
                }
            }
            AppCommand::NoCommand => {}
        }
    }
}
