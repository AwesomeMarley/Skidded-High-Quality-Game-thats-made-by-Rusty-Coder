use macroquad::prelude::*;

//Importing macroquad ui library, all of it even though that is inefficient
use macroquad::ui::*;


pub async fn run(){
    //Making new string for the input to use
    let mut input_data = String::new();

    //Making number variable for the slider to use
    let mut slider_data = 0.0;

    //Tab labels in an array list
    let tabs = ["One", "Two", "Three"];

    //Variable of which tab is currently selected
    let mut current_tab: u32;

    loop {
        //Bad color so you can clearly see ui
        clear_background(YELLOW);
    
    //Makes a tabbar                            Size of tabbar              Array of tab labels
    current_tab = root_ui().tabbar(hash!(),vec2(150.0, 10.0),&tabs[0..3]);

    //Making input         id             label of the input    string it records it's data to
    root_ui().input_text(hash!(), "Input stuff here", &mut input_data);



    //Making slider     id               Label                    Range, f32 vars     Number it records data to
    root_ui().slider(hash!(), "A really cool slider", -10f32..10f32, &mut slider_data);

    

    //button in if statement for click response
    //                      Position of button, x,      y         Button label
    if root_ui().button(Vec2::new(50.0, 50.0), "Press me!") {
        //Prints the input and slider values
        println!("Input: {} and Slider: {}", input_data, slider_data);

        //Prints a msg depending on which current tab is selected
        match current_tab {
            0=>println!("One"),
            1=>println!("Two"),
            2=>println!("Three"),
            _=>println!("WHAT DID YOU DO, THIS SHOULD NEVER HAPPEN!!!"),
            
        }
    }

    next_frame().await;
}

}