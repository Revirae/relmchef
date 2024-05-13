pub mod portion_row;
use self::portion_row::PortionRow;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::{ComponentParts, SimpleComponent};

use crate::chef::{ components, models };
use components::recipe_page::portion_list::portion_row::PortionRowMessage;
use models::FoodPortion;


#[derive(Default, Debug)]
pub struct PortionListState {}

#[derive(Debug)]
pub struct PortionListModel {
    #[allow(dead_code)]
    state: PortionListState,
    portionlist: FactoryVecDeque<PortionRow>,
}


#[derive(Default, Debug)]
pub enum PortionListCommand {
    #[default]
    NoCommand,
    AddEntry(FoodPortion),
    InsertEntry(usize, FoodPortion),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex)
}

#[derive(Default, Debug)]
pub enum PortionListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for PortionListModel {
    type Init = PortionListState;
    type Input = PortionListCommand;
    type Output = PortionListMessage;
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
                PortionRowMessage::NoMessage =>
                    PortionListCommand::NoCommand,
                PortionRowMessage::DeleteMe(index) =>
                    PortionListCommand::DeleteEntry(index), //DeleteEntry
                PortionRowMessage::UpdateMe(index) =>
                    PortionListCommand::UpdateEntry(index),
            });
        let model = PortionListModel {
            state: init,
            portionlist
        };
        let portion_listbox = model.portionlist.widget();
        
        // portion_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            PortionListCommand::NoCommand => {}
            PortionListCommand::AddEntry(portion) => {
                self.portionlist.guard().push_back(portion);
            }
            PortionListCommand::InsertEntry(index, portion) => {
                self.portionlist.guard().remove(index);
                self.portionlist.guard().insert(index, portion);
            }
            PortionListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.portionlist.guard().remove(i);
                sender.output(PortionListMessage::RequestRemoval(i))
                    .expect("failed to request portion removal");
            }
            PortionListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                // self.portionlist.guard().remove(i);
                sender.output(PortionListMessage::RequestUpdate(i))
                    .expect("failed to request portion update");
            }
        }
    }
}
