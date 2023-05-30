use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);


        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("title"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}
