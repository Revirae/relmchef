use relm4::gtk;
use gtk::prelude::{
    ButtonExt, ToggleButtonExt,
    WidgetExt
};
use relm4::{ComponentParts, SimpleComponent};

#[derive(Debug)]
pub struct HeaderModel;

#[derive(Debug)]
pub enum Tab {
    Food,
    Recipe,
}

#[relm4::component(pub)]
impl SimpleComponent for HeaderModel {
    type Init = ();
    type Input = ();
    type Output = Tab;
    view! {
        #[root]
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "Alimentos",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(Tab::Food).unwrap()
                        }
                    },
                },
                gtk::ToggleButton {
                    set_label: "Receitas",
                    set_group: Some(&group),
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(Tab::Recipe).unwrap()
                        }
                    },
                },
            }
        }
    }

    fn init(
            _init: Self::Init,
            root: Self::Root,
            sender: relm4::prelude::ComponentSender<Self>,
) -> relm4::prelude::ComponentParts<Self> {
        let model = HeaderModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
