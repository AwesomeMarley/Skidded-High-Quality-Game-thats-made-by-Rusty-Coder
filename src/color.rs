use macroquad::prelude::*;

//Declaring a new color called violet
pub const VIOLET: Color = Color::new(0.53, 0.24, 0.75, 1.00);


pub async fn run() {
    loop {
        //Clearing background to refresh graphics
        clear_background(BLACK);

        //Drawing text with the custom color, violet
        draw_text("HELLO", 100.0, 100.0, 100.0, VIOLET);

        //Awaiting next frame to loop
        next_frame().await
    }
}
