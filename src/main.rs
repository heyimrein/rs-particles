use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    let system = ParticleSystem::new();

    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("particles"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}


#[derive(Default)]
struct ParticleSystem {
    emitting: bool,
    gravity: Vec2,
}

impl ParticleSystem {
    fn new() -> Self {
        ParticleSystem {
            ..Default::default()
        }
    }

    fn emitting(self: &mut Self, value: bool) {
        self.emitting = value;
    }

    fn gravity(self: &mut Self, value: Vec2) {
        self.gravity = value;
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem {
            emitting: true,
            gravity: vec2(0., 0.),
        }
    }
}


struct Particle {
    position: Vec2,
    velocity: Vec2
}
