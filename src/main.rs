use macroquad::prelude::*;

#[macroquad::main("40k-under-sea")]
async fn main() {
    const WATERBLUE: macroquad::color::Color = Color::new(0.0, 0.0, 0.2, 1.0);
    const ZOOM: f32 = 4.0;
    const SPEED: f32 = 1.0;

    const TOP_DECK: f32 = 112.0 * ZOOM;
    const BOTTOM_DECK: f32 = 172.0 * ZOOM;

    let mut on_ladder: bool = false;

    let ladder_hitbox: Vec<f32> = vec![710.0 * ZOOM, 731.0 * ZOOM];
    let sonar_hitbox: Vec<f32> = vec![565.0 * ZOOM, 587.0 * ZOOM];
    let steer_hitbox: Vec<f32> = vec![453.0 * ZOOM, 468.0 * ZOOM];

    let top_deck_hitbox: Vec<f32> = vec![261.0 * ZOOM, 731.0 * ZOOM];
    let bottom_deck_hitbox: Vec<f32> = vec![218.0 * ZOOM, 771.0 * ZOOM];

    let background_image: Texture2D = load_texture("images/u-boat/u-boat-detailed.png").await.unwrap();
    background_image.set_filter(FilterMode::Nearest);
    let player_image: Texture2D = load_texture("images/player/player.png").await.unwrap();
    player_image.set_filter(FilterMode::Nearest);

    let mut player_x: f32 = 506.0 * ZOOM;
    let mut player_y: f32 = TOP_DECK;

    let mut camera_x: f32;
    let mut camera_y: f32;
    let mut camera: Camera2D;

    loop {

        if is_key_down(KeyCode::D) {
            if on_ladder == false {
            player_x += 1.0 * SPEED * ZOOM;
            }
        }
        if is_key_down(KeyCode::A) {
            if on_ladder == false {
            player_x -= 1.0 * SPEED * ZOOM;
            }
        }
        if is_key_down(KeyCode::S) {
            if on_ladder {
            player_y += 1.0 * SPEED * ZOOM;
            }
        }
        if is_key_down(KeyCode::W) {
            if on_ladder {
            player_y -= 1.0 * SPEED * ZOOM;
            }
        }
        
        if player_y > BOTTOM_DECK {
            player_y = BOTTOM_DECK;
        }
        if player_y < TOP_DECK {
            player_y = TOP_DECK;
        }

        if player_y == TOP_DECK {
            if player_x <= top_deck_hitbox[0] {
                player_x = top_deck_hitbox[0]
            }
            if player_x >= top_deck_hitbox[1] {
                player_x = top_deck_hitbox[1]
            }
        }
        if player_y == BOTTOM_DECK {
            if player_x <= bottom_deck_hitbox[0] {
                player_x = bottom_deck_hitbox[0]
            }
            if player_x >= bottom_deck_hitbox[1] {
                player_x = bottom_deck_hitbox[1]
            }
        }

        if is_key_pressed(KeyCode::E) {
            if on_ladder {
                if player_y == TOP_DECK || player_y == BOTTOM_DECK {
                    on_ladder = false;
                }
            } else {
                if player_x >= ladder_hitbox[0] && player_x <= ladder_hitbox[1] {
                    on_ladder = true;
                }
            }
            if player_x >= sonar_hitbox[0] && player_x <= sonar_hitbox[1] && player_y == TOP_DECK {
                sonar();
            }
        }

        if on_ladder {
            player_x = 722.0 * ZOOM;
        }

        camera_x = player_x + player_image.width() / 2.0 * ZOOM - screen_width() / 2.0;
        camera_y = player_y + player_image.height() / 2.0 * ZOOM - screen_height() / 2.0;
        camera = Camera2D::from_display_rect(Rect::new(camera_x, camera_y + screen_height(), screen_width(), screen_height() * -1.0));
        set_camera(&camera);

        clear_background(WATERBLUE);

        draw_texture_ex(&background_image, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(background_image.width() * ZOOM, background_image.height() * ZOOM)),
            ..Default::default()
        },);
        
        draw_texture_ex(&player_image, player_x, player_y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(player_image.width() * ZOOM, player_image.height() * ZOOM)),
            ..Default::default()
        },);
        

        next_frame().await
    }
}

async fn sonar() {
    let mut zoom: f32 = 1.0;

    const BACKGROUND_GRAY: macroquad::color::Color = Color::new(122.0, 134.0, 153.0, 1.0);

    let sonar_image: Texture2D = load_texture("images/u-boat/sonar.png").await.unwrap();
    sonar_image.set_filter(FilterMode::Nearest);

    let mut camera_x: f32;
    let mut camera_y: f32;
    let mut camera: Camera2D;

    loop {

        camera_x = sonar_image.width() / 2.0 * zoom - screen_width() / 2.0;
        camera_y = sonar_image.height() / 2.0 * zoom - screen_height() / 2.0;
        camera = Camera2D::from_display_rect(Rect::new(camera_x, camera_y + screen_height(), screen_width(), screen_height() * -1.0));
        set_camera(&camera);

        clear_background(BACKGROUND_GRAY);

        draw_texture_ex(&sonar_image, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(sonar_image.width() * zoom, sonar_image.height() * zoom)),
            ..Default::default()
        },);

        
        next_frame().await
    }
}