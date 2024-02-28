use macroquad::prelude::*;

fn configuration() -> Conf {
    Conf {
        window_title: String::from("Dead pixel check"),
        fullscreen: true,
        ..Default::default()
    }
}

fn any_input() -> bool {
    let any_key_pressed = get_keys_pressed().len() > 0;
    let any_mouse_button_pressed = is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Middle) || is_mouse_button_pressed(MouseButton::Right);
    let any_touches = touches().len() > 0;
    return any_key_pressed || any_mouse_button_pressed || any_touches;
}

fn cycle_color(color: &mut Color) {
    *color = if *color == WHITE {
        BLACK
    } else if *color == BLACK {
        RED
    } else if *color == RED {
        GREEN
    } else if *color == GREEN {
        BLUE
    } else {
        WHITE
    };
}

#[macroquad::main(configuration)]
async fn main() {
    let mut color = WHITE;

    loop {
        clear_background(color);

        if any_input() {
            cycle_color(&mut color);
        }

        next_frame().await;
    }
}
