use macroquad::prelude::*;




pub async fn run() {

    // let mut move_speed: f32 = 0.15;
    // let look_speed: f32 = 0.12;

    // const block_size: Vec3 = vec3(1., 1., 1.);

    let ground_y: f32 = 3.0;

    // let mut x = 0.0;
    // let mut switch = false;
    // let bounds = 8.0;

    // let mut y = ground_y;
    // let mut vel_y = 0.0;
    // let jump_y = 0.25;

    let world_up = vec3(0.0, 1.0, 0.0);
    let yaw: f32 = 1.18;
    let pitch: f32 = 0.0;


    let front_view = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    ).normalize();
    let front_move = vec3(
        yaw.cos() * pitch.cos(),
        ground_y,
        yaw.sin() * pitch.cos(),
    ).normalize();
    let side_move = front_view.cross(world_up).normalize();


    let grass_block_texture: Texture2D = load_texture("assets/minecraft_grass_block.png").await.unwrap();
    grass_block_texture.set_filter(FilterMode::Nearest);


    set_cursor_grab(true);
    show_mouse(false);


    let mut player = Player::new(
        vec3(0.0, ground_y, 0.0), 
        0.15, 
        0.25,
        0.05,
        2.0,
        0.12,
        front_view,
        front_move,
        side_move,
    );




    loop {
        let delta = get_frame_time();

        gen_blocks(grass_block_texture);

        if is_key_pressed(KeyCode::Escape) {
            // break;
            set_cursor_grab(false);
            show_mouse(true);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            set_cursor_grab(true);
            show_mouse(false);
        }


        player.move_player();



        player.player_look(delta);
        



        clear_background(SKYBLUE);

        // Going 3d!


        set_camera(&Camera3D {
            position: player.camera_position,
            up: player.up,
            target: player.camera_position + player.front_view,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);




        // for x in -19..21 {
        //     for z in -19..21 {
        //         place_block(x, 0, z, grass_block_texture);
        //     }
        // }

        place_block(5, 1, 8, grass_block_texture);

        place_block(-1, 1, 3, grass_block_texture);

        // draw_cube(return_coords_vec(5,1,8), block_size, grass_block_texture, BLACK);


        // draw_cube_wires(vec3(0., 0., 0.), vec3(2., 2., 2.), GREEN);

        // draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        // draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        // Back to screen space, render some text

        set_default_camera();

        draw_text("+", screen_width()/2.0, screen_height()/2.0, 60.0, LIGHTGRAY);

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


fn gen_blocks(ground_texture: Texture2D){
    for x in -5..5 {
        for z in -5..5 {
            place_block(x, 1, z, ground_texture);
        }
    }
}



fn place_block(x: i32, y: i32, z: i32, texture: Texture2D){
    draw_cube(return_coords_vec(x,y,z), vec3(1.0,1.0,1.0), texture, WHITE);
}


fn return_coords_vec(x: i32, y: i32, z: i32) -> Vec3 {
    let fx = return_coords_from_float(x as f32);
    let fy = y as f32-0.5;
    let fz = z as f32-0.5;
    return vec3(fx, fy, fz)
}

fn return_coords_from_float(coord: f32) -> f32 {
    if coord > 0. {
        let new_coord = coord - 0.5;
        return new_coord
    }else if coord < 0. {
        let new_coord = coord + 0.5;
        return new_coord
    }else{
        return 0.0
    }
}

fn return_player_coords(coord: f32) -> f32 {
    if coord < 0. {
        let new_coord = coord - 0.5;
        return new_coord
    }else if coord > 0. {
        let new_coord = coord + 0.5;
        return new_coord
    }else{
        return 0.0
    }
}



struct Player {
    position: Vec3,
    camera_position: Vec3,
    player_speed: f32,
    walk_speed: f32,
    sprint_speed: f32,
    crouch_speed: f32,
    height: f32,
    look_speed: f32,
    front_view: Vec3,
    front_move: Vec3,
    side_move: Vec3,
    up:  Vec3,
    yaw: f32,
    pitch: f32,
    x: f32,
    switch: bool,
    bounds: f32,
    last_mouse_position: Vec2,
    vel_y: f32,
    y: f32,
    ground_y: f32,
    jump_y: f32,
}

impl Player {
    fn new(player_position: Vec3, speed_walk: f32, speed_sprint: f32, speed_crouch: f32, player_height: f32, player_look: f32, view_front: Vec3, move_front: Vec3, move_side: Vec3) -> Player {
        Player {
            position: player_position,
            camera_position: player_position,
            player_speed: 1.,
            walk_speed: speed_walk,
            sprint_speed: speed_sprint,
            crouch_speed: speed_crouch,
            height: player_height,
            look_speed: player_look,
            front_view: view_front,
            front_move: move_front,
            side_move: move_side,
            up: vec3(0.,0.,0.),
            yaw: 1.18,
            pitch: 0.0,
            x: 0.0,
            switch: false,
            bounds: 8.0,
            last_mouse_position: mouse_position().into(),
            vel_y: 0.0,
            y: 0.0,
            ground_y: 0.0,
            jump_y: 0.25,
        }
    }
    

    fn move_player(&mut self){

        if is_key_down(KeyCode::LeftControl) {
            self.player_speed = self.sprint_speed;
            self.height = 2.0;
        }else if is_key_down(KeyCode::LeftShift){
            self.player_speed = self.crouch_speed;
            self.height = 1.5;
        }else{
            self.player_speed = self.walk_speed;
            self.height = 2.0;
        }

        if is_key_down(KeyCode::W) {
            self.position += self.front_move * self.player_speed;
        }
        if is_key_down(KeyCode::S) {
            self.position -= self.front_move * self.player_speed;
        }
        if is_key_down(KeyCode::A) {
            self.position -= self.side_move * self.player_speed;
        }
        if is_key_down(KeyCode::D) {
            self.position += self.side_move * self.player_speed;
        }


        if is_key_down(KeyCode::Space) && self.is_on_ground() {
            self.vel_y += self.jump_y;
        }


        if is_key_pressed(KeyCode::I){
            println!("X: {} Y: {} Z: {}", return_player_coords(self.position.x).round(), return_player_coords(self.position.y).round(), return_player_coords(self.position.z).round());
        }
        
        self.y += self.vel_y;

        if self.y > self.ground_y {
            self.vel_y -= 0.01;
        }else{
            self.vel_y = 0.0;
            self.y = self.ground_y;
            self.position.y = self.ground_y;
        }

        self.position.y = self.y;



        self.camera_position = vec3(self.position.x, self.position.y+self.height, self.position.z);
        

        // println!("vel: {}, y: {}",vel_y, y);

    }

    fn is_on_ground(&mut self) -> bool {
        if self.y <= self.ground_y {
            return true
        }else{
            return false
        }
    }


    fn player_look(&mut self, delta: f32){
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;
        self.last_mouse_position = mouse_position;

        self.yaw += mouse_delta.x * delta * self.look_speed;
        self.pitch += mouse_delta.y * delta * -self.look_speed;

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

        self.front_view = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        self.front_move = vec3(
            self.front_view.x,
            0.0,
            self.front_view.z,
        ).normalize();

        self.side_move = self.front_view.cross(vec3(0.0, 1.0, 0.0)).normalize();
        self.up = self.side_move.cross(self.front_view).normalize();

        self.x += if self.switch { 0.04 } else { -0.04 };
        if self.x >= self.bounds || self.x <= -self.bounds {
            self.switch = !self.switch;
        }
    }



    fn get_player_collisions(&self){
        
    }

    fn check_for_collisions(&self, start_vec: Vec3, end_vec: Vec3) -> bool {
        return true
    }

    fn apply_velocity(&self) {
        format!("{}", self.height);
    }

}