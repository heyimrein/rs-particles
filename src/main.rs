use std::time::SystemTime;
use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    let system = ParticleSystem::new()
        .gravity(vec2(0., 10.));

    loop {
        clear_background(BLACK);

        system.tick();

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
    /// Particle container
    particles: Vec<Particle>,

    /// Delta Time: time between last tick and current tick.
    dt: f32,

    /// Internal time cache.
    dt_prev_time: f32,
    /// Internal time cache.
    dt_time: SystemTime,

    /// Whether system is currently emitting particles.
    emitting: bool,

    /// Gravitational force applied to each particle over time.
    gravity: Vec2,
}

impl<'a> ParticleSystem {
    fn new() -> &'a mut Self {
        &mut ParticleSystem {
            ..Default::default()
        }
    }

    /// Tick every particle once.
    fn tick(self: &mut Self) {
        let cur = self.dt_time.elapsed().unwrap().as_secs_f32();
        self.dt = cur - self.dt_prev_time;
        self.dt_prev_time = cur;

        let mut particles = &mut self.particles;
        for i in 0..particles.len() {
            let particle = &mut particles[i];
            particle.position += particle.velocity * self.dt;
        }
    }

    /// Set `emitting` to `value`.
    fn emitting(self: &mut Self, value: bool) -> &mut Self {
        self.emitting = value;
        return self;
    }

    /// Set `gravity` to `value`.
    fn gravity(self: &mut Self, value: Vec2) -> &mut Self {
        self.gravity = value;
        return self;
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem {
            particles: vec![],
            emitting: true,
            gravity: vec2(0., 0.),
            dt: 0.,
            dt_prev_time: 0.,
            dt_time: SystemTime::now(),
        }
    }
}


struct Particle {
    position: Vec2,
    velocity: Vec2,
    size: f32,
}
