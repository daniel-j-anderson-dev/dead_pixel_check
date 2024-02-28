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
    const PURE_RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    const PURE_GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    const PURE_BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);

    *color = if *color == WHITE {
        BLACK
    } else if *color == BLACK {
        PURE_RED
    } else if *color == PURE_RED {
        PURE_GREEN
    } else if *color == PURE_GREEN {
        PURE_BLUE
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
