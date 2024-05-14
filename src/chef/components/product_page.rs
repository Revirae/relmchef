mod product_form;
mod recipe_portion_form;
mod product_list;
mod recipe_portion_list;

use relm4::gtk;
use relm4::prelude::ComponentSender;
use relm4::{
    ComponentParts,
    SimpleComponent,
    Controller,
    ComponentController,
    Component
};
// use gtk::prelude::{WidgetExt, OrientableExt};
use uuid::Uuid;

use crate::chef::components::product_page::product_form::ProductFormMessage;
use crate::chef::{ components, models };
use components::product_page::product_list::{ProductListMessage, ProductListState};
use components::product_page::recipe_portion_form::RecipePortionFormMessage;
use components::product_page::recipe_portion_list::{RecipePortionListMessage, RecipePortionListState};
use models::{Product, Recipe, RecipePortion};

use self::product_form::ProductFormModel;
use self::product_list::{ProductListCommand, ProductListModel};
use self::recipe_portion_form::RecipePortionFormModel;
use self::recipe_portion_list::{RecipePortionListCommand, RecipePortionListModel};


#[derive(Default, Debug)]
pub enum ProductPageMode {
    #[default]
    InsertingProduct,
    EditingProduct(usize),
    #[allow(dead_code)]
    InsertingRecipePortion,
    #[allow(dead_code)]
    EditingRecipePortion(usize),
}

#[derive(Default, Debug)]
pub struct ProductPageState {
    mode: ProductPageMode,
    productlist: Vec<Product>,
    recipeportionlist: Vec<RecipePortion>,
}

#[derive(Debug)]
pub struct ProductPageModel {
    state: ProductPageState,
    product_form: Controller<ProductFormModel>,
    product_list: Controller<ProductListModel>,
    recipeportion_form: Controller<RecipePortionFormModel>,
    recipeportion_list: Controller<RecipePortionListModel>,
}

#[derive(Default, Debug)]
pub enum ProductPageCommand {
    #[default]
    NoCommand,

    #[allow(dead_code)]
    LoadProductList(Vec<Product>),
    #[allow(dead_code)]
    PutProduct(Product),
    RemoveProduct(usize),
    UpdateProduct(usize),

    #[allow(dead_code)]
    LoadRecipePortionList(Vec<RecipePortion>),
    #[allow(dead_code)]
    PutRecipePortion(RecipePortion),
    RemoveRecipePortion(usize),
    UpdateRecipePortion(usize),

    #[allow(dead_code)]
    LoadRecipeIngredientList(Vec<Recipe>),
}

#[derive(Default, Debug)]
pub enum ProductPageMessage {
    #[default]
    NoMessage,

    CommitProduct(Product),
    CommitProductRemoval(Uuid),
    CommitProductUpdate(Uuid, Product),

    CommitRecipePortion(RecipePortion),
    CommitRecipePortionRemoval(Uuid),
    CommitRecipePortionUpdate(Uuid, RecipePortion),
}

#[relm4::component(pub)]
impl SimpleComponent for ProductPageModel  {
    type Init = ProductPageState;
    type Input = ProductPageCommand;
    type Output = ProductPageMessage;
    view! {
        #[root]
        gtk::Box {
            model.product_form.widget(),
            model.product_list.widget(),
            model.recipeportion_form.widget(),
            model.recipeportion_list.widget(),
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let product_form = ProductFormModel::builder()
            .launch(Product::default())
            .forward(sender.input_sender(), |msg| match msg {
                ProductFormMessage::NoMessage => {
                    ProductPageCommand::NoCommand
                }
                ProductFormMessage::Submit(product) => {
                    ProductPageCommand::PutProduct(product)    
                }
            });
        let recipeportion_form = RecipePortionFormModel::builder()
            .launch(RecipePortion::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipePortionFormMessage::NoMessage => {
                    ProductPageCommand::NoCommand
                }
                RecipePortionFormMessage::Submit(portion) => {
                    ProductPageCommand::PutRecipePortion(portion)
                }
            });
        
        let product_list = ProductListModel::builder()
            .launch(ProductListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                ProductListMessage::NoMessage => {
                    ProductPageCommand::NoCommand
                }
                ProductListMessage::RequestRemoval(index) => {
                    ProductPageCommand::RemoveProduct(index)
                }
                ProductListMessage::RequestUpdate(index) => {
                    ProductPageCommand::UpdateProduct(index)
                }
            });
        let recipeportion_list = RecipePortionListModel::builder()
            .launch(RecipePortionListState::default())
            .forward(sender.input_sender(), |msg| match msg {
                RecipePortionListMessage::NoMessage => {
                    ProductPageCommand::NoCommand
                }
                RecipePortionListMessage::RequestRemoval(index) => {
                    ProductPageCommand::RemoveRecipePortion(index)
                }
                RecipePortionListMessage::RequestUpdate(index) => {
                    ProductPageCommand::UpdateRecipePortion(index)
                }
            });
        
        let model = ProductPageModel  {
            state: init,
            product_form,
            recipeportion_form,
            product_list,
            recipeportion_list,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ProductPageCommand::NoCommand => {}
            ProductPageCommand::LoadProductList(productlist) => {
                self.state.productlist = productlist.clone();
                for product in productlist {
                    self.product_list.emit(
                        ProductListCommand::AddEntry(product)
                    );
                }
            }    
            ProductPageCommand::LoadRecipePortionList(portionlist) => {
                self.state.recipeportionlist = portionlist.clone();
                for portion in portionlist {
                    self.recipeportion_list.emit(
                        RecipePortionListCommand::AddEntry(portion)
                    );
                }
            }    
            ProductPageCommand::LoadRecipeIngredientList(_foodlist) => {
                // self.portion_form.emit(
                    // PortionFormCommand::ReceiveRecipeList(foodlist)
                // );
            }
            ProductPageCommand::PutProduct(product) => {
                match self.state.mode {
                    ProductPageMode::EditingProduct(index) => {
                        self.product_list.emit(
                            ProductListCommand::InsertEntry(index, product.clone())
                        );
                        sender.output(
                            ProductPageMessage::CommitProductUpdate(product.id, product)
                        ).expect("failed to commit product update");
                        self.state.mode = ProductPageMode::InsertingProduct;

                        // self.product_form.emit(
                            // ProductFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    ProductPageMode::InsertingProduct => {
                        self.state.productlist.push(product.clone());
                        self.product_list.emit(
                            ProductListCommand::AddEntry(product.clone())
                        );
                        sender.output(
                            ProductPageMessage::CommitProduct(product)
                        ).expect("failed to commit product insertion");
                    }
                    _ => {}
                }
            }
            ProductPageCommand::RemoveProduct(index) => {
                let id = self.state.productlist.get(index).unwrap().id;
                self.state.productlist.remove(index);
                sender.output(
                    ProductPageMessage::CommitProductRemoval(id)
                ).expect("failed to commit product removal");
            }
            ProductPageCommand::UpdateProduct(index) => {
                // let product = self.state.productlist.get(index).unwrap();
                // self.product_form.emit(
                    // ProductFormCommand::Receive(product.clone())
                // );
                self.state.mode = ProductPageMode::EditingProduct(index);
                // self.product_form.emit(
                    // ProductFormCommand::ChangeIcon("document-save".into())
                // );
            }
            ProductPageCommand::PutRecipePortion(portion) => {
                match self.state.mode {
                    ProductPageMode::EditingRecipePortion(index) => {
                        self.recipeportion_list.emit(
                            RecipePortionListCommand::InsertEntry(index, portion.clone())
                        );
                        sender.output(
                            ProductPageMessage::CommitRecipePortionUpdate(portion.inner.id, portion)
                        ).expect("failed to commit portion update");
                        self.state.mode = ProductPageMode::InsertingProduct;

                        // self.product_form.emit(
                            // RecipePortionFormCommand::ChangeIcon("document-new".into())
                        // );
                    }
                    ProductPageMode::InsertingRecipePortion => {
                        self.state.recipeportionlist.push(portion.clone());
                        self.recipeportion_list.emit(
                            RecipePortionListCommand::AddEntry(portion.clone())
                        );
                        sender.output(
                            ProductPageMessage::CommitRecipePortion(portion)
                        ).expect("failed to commit portion insertion");
                    }
                    _ => {}
                }
            }
            ProductPageCommand::RemoveRecipePortion(index) => {
                let id = self.state.productlist.get(index).unwrap().id;
                self.state.productlist.remove(index);
                sender.output(
                    ProductPageMessage::CommitRecipePortionRemoval(id)
                ).expect("failed to commit portion removal");
            }
            ProductPageCommand::UpdateRecipePortion(index) => {
                let _portion= self.state.recipeportionlist.get(index).unwrap();
                // self.portion_form.emit(
                //     RecipePortionFormCommand::Receive(portion.clone())
                // );
                self.state.mode = ProductPageMode::EditingProduct(index);
                // self.product_form.emit(
                    // RecipePortionFormCommand::ChangeIcon("document-save".into())
                // );
            }
        }
    }}
