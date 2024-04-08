mod chef;

use relm4::RelmApp;
use chef::app;

fn main () {
    let app = RelmApp::new("relm4.chef.main");
    app.run::<app::AppModel>(
        app::AppState::default()
    )    
}
