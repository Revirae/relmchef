mod product_row;

use relm4::ComponentSender;
use relm4::gtk;
use relm4::factory::{DynamicIndex, FactoryVecDeque};
use gtk::prelude::{
    WidgetExt, OrientableExt,
};

use relm4::{ComponentParts, SimpleComponent};

use crate::chef::components::product_page::product_list::product_row::ProductRowMessage;
use crate::chef::models::Product;

use self::product_row::ProductRow;


#[derive(Default, Debug)]
pub struct ProductListState {}

#[derive(Debug)]
pub struct ProductListModel {
    #[allow(dead_code)]
    state: ProductListState,
    productlist: FactoryVecDeque<ProductRow>,
}


#[derive(Default, Debug)]
pub enum ProductListCommand {
    #[default]
    NoCommand,
    AddEntry(Product),
    InsertEntry(usize, Product),
    DeleteEntry(DynamicIndex),
    UpdateEntry(DynamicIndex),
}

#[derive(Default, Debug)]
pub enum ProductListMessage {
    #[default]
    NoMessage,
    RequestRemoval(usize),
    RequestUpdate(usize),
}


#[relm4::component(pub)]
impl SimpleComponent for ProductListModel {
    type Init = ProductListState;
    type Input = ProductListCommand;
    type Output = ProductListMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_min_content_height: 360,

                #[local_ref]
                product_listbox -> gtk::ListBox {
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
        let productlist = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |message| match message {
                ProductRowMessage::NoMessage =>
                    ProductListCommand::NoCommand,
                ProductRowMessage::DeleteMe(index) =>
                    ProductListCommand::DeleteEntry(index), //DeleteEntry
                ProductRowMessage::UpdateMe(index) =>
                    ProductListCommand::UpdateEntry(index),
            });
        let model = ProductListModel {
            state: init,
            productlist
        };
        let product_listbox = model.productlist.widget();
        
        // product_listbox.set_selection_mode(gtk::SelectionMode::None);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ProductListCommand::NoCommand => {}
            ProductListCommand::AddEntry(product) => {
                self.productlist.guard().push_back(product);
            }
            ProductListCommand::InsertEntry(index, product) => {
                self.productlist.guard().remove(index);
                self.productlist.guard().insert(index, product);
            }
            ProductListCommand::DeleteEntry(index) => {
                let i = index.current_index();
                self.productlist.guard().remove(i);
                sender.output(ProductListMessage::RequestRemoval(i))
                    .expect("failed to request product removal")
            }
            ProductListCommand::UpdateEntry(index) => {
                let i = index.current_index();
                // self.productlist.guard().remove(i);
                sender.output(ProductListMessage::RequestUpdate(i))
                    .expect("failed to request product update")
            }
        }
    }
}
