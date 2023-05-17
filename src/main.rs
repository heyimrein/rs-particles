use std::time::SystemTime;
use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    let system = ParticleSystem::new()
        .gravity(vec2(0., 10.));

    loop {
        clear_background(BLACK);

        system.tick();
        for i in 0..system.particles.len() {
            let particle = &mut system.particles[i];
            draw_poly(
                particle.position.x,
                particle.position.y,
                20,
                particle.size,
                0.,
                RED
            );
        }

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


struct ParticleSystem<'a> {
    /// Particle container
    particles: Vec<&'a mut Particle>,

    /// Position on-screen.
    position: Vec2,

    /// Whether system is currently emitting particles.
    emitting: bool,

    /// Gravitational force applied to each particle over time.
    gravity: Vec2,

    /// Delta Time: time between last tick and current tick.
    dt: f32,

    /// Internal time cache.
    dt_prev_time: f32,
    /// Internal time cache.
    dt_time: SystemTime,
}

impl<'a> ParticleSystem<'_> {
    fn new() -> &'a mut Self {
        &mut ParticleSystem {
            particles: vec![],
            position: Vec2::ZERO,
            emitting: true,
            gravity: vec2(0., 0.),
            dt: 0.,
            dt_prev_time: 0.,
            dt_time: SystemTime::now(),
        }
    }

    /// Tick every particle once.
    fn tick(self: &mut Self) {
        let cur = self.dt_time.elapsed().unwrap().as_secs_f32();
        self.dt = cur - self.dt_prev_time;
        self.dt_prev_time = cur;

        if self.dt_prev_time % 0.5 == 0. {
            self.particles.push(
                &mut Particle::new()
                    .position(self.position)
            )
        }

        let mut particles = &mut self.particles;
        for i in 0..particles.len() {
            let mut particle = &mut particles[i];

            particle.size -= self.dt * 50.;
            if particle.size <= 0. {
                particles.remove(i as usize);
            }

            particle.velocity -= self.gravity;

            particle.position += particle.velocity * self.dt;
        }
    }

    /// Set `position` to `value`.
    fn position(self: &mut Self, value: Vec2) -> &mut Self {
        self.position = value;
        return self;
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


#[derive(Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    size: f32,
}

impl Particle {
    fn new() -> Self {
        Particle {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            size: 50.,
        }
    }

    /// Set `position` to `value`.
    fn position(self: &mut Self, value: Vec2) -> &mut Self {
        self.position = value;
        return self;
    }

    /// Set `velocity` to `value`.
    fn velocity(self: &mut Self, value: Vec2) -> &mut Self {
        self.velocity = value;
        return self;
    }

    /// Set `size` to `value`.
    fn size(self: &mut Self, value: f32) -> &mut Self {
        self.size = value;
        return self;
    }
}
