use relm4::gtk;
use relm4::{Component, ComponentParts, ComponentSender};

use crate::chef::models;


#[derive(Debug)]
pub struct PortionFormModel {
    state: models::Portion,
}

#[derive(Default, Debug)]
pub enum PortionFormMessage {
    #[default]
    NoMessage,
}

#[derive(Default, Debug)]
pub enum PortionFormCommand {
    #[default]
    NoCommand,
    Send,
    Receive(models::Portion),
}

#[relm4::component(pub)]
impl Component for PortionFormModel {
    type Init = models::Portion;
    type Input = PortionFormCommand;
    type Output = PortionFormMessage;
    type CommandOutput = ();
    view! {
        #[root]
        gtk::Box {}
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PortionFormModel { state: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
