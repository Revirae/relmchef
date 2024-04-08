use relm4::{adw, gtk, prelude::ComponentSender};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{ComponentParts, SimpleComponent};

#[derive(Default, Debug)]
pub struct FoodListState;

#[derive(Debug)]
pub struct FoodListModel {
    state: FoodListState,
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
            adw::ComboRow {
                set_title: "Nome",
            }
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let state = FoodListState::default();
        let model = FoodListModel { state };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
