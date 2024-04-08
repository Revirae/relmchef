use relm4::{adw, gtk};
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt, OrientableExt,
    EditableExt,
};
use adw::prelude::PreferencesRowExt;

use relm4::{ComponentParts, SimpleComponent};

#[derive(Debug)]
pub struct FormModel {
    name: String
    brand: String,
    cost: f64,
    weight: f64,
    volume: f64
}

#[derive(Default, Debug)]
pub struct FormState;

#[derive(Default, Debug)]
pub struct FormCommand;

#[derive(Default, Debug)]
pub struct FormMessage;

#[relm4::component(pub)]
impl SimpleComponent for FormModel {
    type Init = FormState;
    type Input = FormCommand;
    type Output = FormMessage;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            adw::EntryRow {
                #[watch]
                set_text: model.name.as_ref(),
                set_title: "Nome",
            },
            adw::EntryRow {
                #[watch]
                set_text: model.brand.as_ref(),
                set_title: "Marca",
            },
            adw::SpinRow {
                set_title: "Custo",
                set_value: model.cost,
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            adw::SpinRow {
                set_title: "Peso",
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
            adw::SpinRow {
                set_title: "Volume",
                set_adjustment: Some(&gtk::Adjustment::new(
                    0., 0., 9999., 0.05, 0.5, 10.
                )),
            },
        }
    }
    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let model = FormModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
