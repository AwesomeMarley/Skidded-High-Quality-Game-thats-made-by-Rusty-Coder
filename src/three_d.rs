use macroquad::prelude::*;

pub async fn run() {

    loop {
        clear_background(LIGHTGRAY);

        //Setting up 3d camera
        set_camera(&Camera3D {
            //Position of camera
            position: vec3(-20., 15., 0.),
            //Up which does something
            up: vec3(0., 1., 0.),
            //Direction camera is looking
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        //Drawing grid to help get a sense of 3d enviorment
        draw_grid(20, 1., BLACK, GRAY);

        //Drawing 3d cubes
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        //Drawing 3d sphere
        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        //Returning to original camera which is non 3d to place text on 2d screen
        set_default_camera();
        draw_text("3D is cool!", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}