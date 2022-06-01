use macroquad::prelude::*;


pub async fn run() {
    //Declaring velocity variables set at 0.0
    let mut vel_x = 100.0;
    let mut vel_y = 0.0;

    //Declaring inital position of ball at center of screen
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;


    loop {
        clear_background(LIGHTGRAY);

        //Drawing circle at last position to give blur effect
        draw_circle(x, y, 15.0, DARKGRAY);


        //Reducing x velocity frame
        if vel_x > 0.0 {
            vel_x -= 0.1;
        } else if vel_x < 0.0 {
            vel_x += 0.1;
        }

        //Reducing y velocity each frame
        if vel_y > 0.0 {
            vel_y -= 0.1;
        } else if vel_y < 0.0 {
            vel_y += 0.1;
        }


        //Moving ball according to key inputs

        //is_key_down is a function that returns if a certain keycode is pressed
        if is_key_down(KeyCode::Right) {
            vel_x += 0.2;
        }
        if is_key_down(KeyCode::Left) {
            vel_x -= 0.2;
        }
        if is_key_down(KeyCode::Down) {
            vel_y += 0.2;
        }
        if is_key_down(KeyCode::Up) {
            vel_y -= 0.2;
        }else {
            //If up key is not pressed, gives gravity effect to ball
            vel_y += 0.3;
        }

        //Ball collision with sides of screen
        if x >= screen_width() - 15.0 || x <= 15.0 {
            vel_x = -vel_x;
        }
        if y >= screen_height() - 15.0 || y <= 15.0 {
            vel_y = -vel_y;
        }

        //Addes velocity to position to simulate movement
        x += vel_x;
        y += vel_y;


        //Drawing ball at its new position
        draw_circle(x, y, 15.0, BLACK);

        //Waits for next frame to loop
        next_frame().await
    }
}
