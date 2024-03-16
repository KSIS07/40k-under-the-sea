use macroquad::prelude::*;

#[macroquad::main("40k-under-sea")]
async fn main() {
    const WATERBLUE: macroquad::color::Color = Color::new(0.0, 0.0, 0.2, 1.0);
    const ZOOM: f32 = 1.0;
    const SPEED: f32 = 4.0;

    const TOP_DECK: f32 = 516.0 * ZOOM;
    const BOTTOM_DECK: f32 = 756.0 * ZOOM;

    const PLAYER_WIDTH: f32 = 60. * ZOOM;
    const PLAYER_HEIGHT: f32 = 120.0 * ZOOM;

    let mut on_ladder: bool = false;

    let ladder_hitbox: Vec<f32> = vec![2942.0 * ZOOM, 2962.0 * ZOOM];

    let top_deck_hitbox: Vec<f32> = vec![1102.0 * ZOOM, 2994.0 * ZOOM];
    let bottom_deck_hitbox: Vec<f32> = vec![942.0 * ZOOM, 3154.0 * ZOOM];

    let background_image: Texture2D = load_texture("src/images/u-boat-detailed.png").await.unwrap();

    let mut player_x:f32 = 2086.0 * ZOOM;
    let mut player_y: f32 = TOP_DECK;

    let mut camera_x: f32 = 0.0;
    let mut camera_y: f32 = 0.0;
    let mut camera: Camera2D = Camera2D::from_display_rect(Rect::new(camera_x, camera_y, screen_width(), screen_height() * -1.0));

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
        }


        set_camera(&camera);
        camera_x = player_x - screen_width() / 2.0;
        camera_y = player_y - screen_height() / 2.0;
        camera = Camera2D::from_display_rect(Rect::new(camera_x, camera_y + screen_height(), screen_width(), screen_height() * -1.0));
        
        clear_background(WATERBLUE);

        draw_texture_ex(&background_image, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(background_image.width() * ZOOM, background_image.height() * ZOOM)),
            ..Default::default()
        },);
        
        draw_rectangle(player_x - PLAYER_WIDTH / 2.0, player_y - PLAYER_HEIGHT / 2.0, PLAYER_WIDTH, PLAYER_HEIGHT, BLACK);

        next_frame().await
    }
}