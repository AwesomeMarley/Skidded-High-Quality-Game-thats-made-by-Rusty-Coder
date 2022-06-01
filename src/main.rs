//Importing other rs files from src folder

// mod color;
// mod input_gravity;
// mod three_d;
// mod input;
// mod ui;
// mod first_person;
mod person_game;



#[macroquad::main("Rust Graphics Test")]
//Title of window it opens

//Async function for macroquad to work
async fn main() {
    // color::run().await;
    // input_gravity::run().await;
    // three_d::run().await;
    // input::run().await;
    // ui::run().await;
    // first_person::run().await;
    person_game::run().await;

}
