mod chef;

use relm4::RelmApp;
use chef::app::{AppModel, AppState};

fn main () {
    let app = RelmApp::new("relm4.chef.main");
    app.run::<AppModel>(
        AppState::new("./chef.db".into())
    );
}
