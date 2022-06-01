use macroquad::prelude::*;




pub async fn run() {

    let mut move_speed: f32 = 0.15;
    let look_speed: f32 = 0.12;

    let ground_y: f32 = 3.0;

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let mut y = ground_y;
    let mut vel_y = 0.0;
    let jump_y = 0.25;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;


    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    ).normalize();
    let mut front_move = vec3(
        yaw.cos() * pitch.cos(),
        ground_y,
        yaw.sin() * pitch.cos(),
    ).normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, ground_y, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    


    set_cursor_grab(true);
    show_mouse(false);








    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            // break;
            set_cursor_grab(false);
            show_mouse(true);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            set_cursor_grab(true);
            show_mouse(false);
        }


        if is_key_down(KeyCode::W) {
            if is_key_down(KeyCode::LeftControl) {
                move_speed = 0.3;
            }else{
                move_speed = 0.15;
            }
            position += front_move * move_speed;
        }
        if is_key_down(KeyCode::S) {
            position -= front_move * move_speed;
        }
        if is_key_down(KeyCode::A) {
            position -= right * move_speed;
        }
        if is_key_down(KeyCode::D) {
            position += right * move_speed;
        }

        // if is_key_down(KeyCode::LeftShift) {
        //     ground_y = 1.0;
        // }else{
        //     ground_y = 2.0;
        // }

        if is_key_down(KeyCode::Space) && y <= ground_y {
            vel_y += jump_y;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * look_speed;
        pitch += mouse_delta.y * delta * -look_speed;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        
        y += vel_y;


        if y > ground_y {
            vel_y -= 0.01;
        }else{
            vel_y = 0.0;
            y = ground_y;
            position.y = ground_y;
        }

        position.y = y;
        

        // println!("vel: {}, y: {}",vel_y, y);
        

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        front_move = vec3(
            yaw.cos() * pitch.cos(),
            0.0,
            yaw.sin() * pitch.cos(),
        ).normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 2.0, BLACK, GRAY);


        draw_cube_wires(vec3(0., 0., 0.), vec3(2., 2., 2.), GREEN);

        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        // Back to screen space, render some text

        set_default_camera();

        // draw_text(
        //     format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
        //     10.0,
        //     48.0 + 18.0,
        //     30.0,
        //     BLACK,
        // );

        next_frame().await
    }


}