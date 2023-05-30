use std::time::SystemTime;
use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let win = window_conf();
    let mut system = ParticleSystem::new()
        .position(vec2(win.window_width as f32 / 2., win.window_height as f32 / 2.));

    let time = SystemTime::now();
    let mut prev_time = time.elapsed().unwrap().as_secs_f32();

    loop {
        clear_background(BLACK);
        let delta = time.elapsed().unwrap().as_secs_f32() - prev_time;

        system.tick(delta);
        system.draw();

        prev_time = time.elapsed().unwrap().as_secs_f32();
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


struct ParticleSystem {
    position: Vec2,
    emit_interval: f32,
    particles: Vec<Particle>,
    _interval_timer: f32,
}

impl ParticleSystem {
    fn new() -> Self {
        ParticleSystem {
            position: vec2(0., 0.),
            emit_interval: 0.5,
            particles: vec![],
            _interval_timer: 0.5,
        }
    }

    fn tick(self: &mut Self, delta: f32) {
        self._interval_timer -= delta;
        if self._interval_timer < 0. {
            self._interval_timer = self.emit_interval;
            self.particles.push(Particle::new());
        }

        let mut rem_buffer: Vec<usize> = vec![];
        for i in 0..self.particles.len() {
            let particle = &mut self.particles[i];
            particle.radius -= particle.decay_rate * delta;
            if particle.radius <= 0. {
                rem_buffer.push(i);
            }
        }
        for idx in rem_buffer {
            self.particles.remove(idx);
        }
    }

    fn draw(self: &mut Self) {
        for particle in &self.particles {
            draw_circle(
                self.position.x + particle.position.x,
                self.position.y + particle.position.y,
                particle.radius,
                WHITE
            );
        }
    }

    /// Set `position` to `value`.
    fn position(mut self: Self, value: Vec2) -> Self {
        self.position = value;
        return self;
    }
    
    /// Set `emit_interval` to `value`.
    fn emit_interval(mut self: Self, value: f32) -> Self {
        self.emit_interval = value;
        return self;
    }

    /// Set `particles` to `value`.
    fn particles(mut self: Self, value: Vec<Particle>) -> Self {
        self.particles = value;
        return self;
    }
}


struct Particle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    decay_rate: f32,
}

impl Particle {
    fn new() -> Self {
        Particle {
            position: vec2(0., 0.),
            velocity: vec2(0., 0.),
            radius: 8.,
            decay_rate: 8.,
        }
    }

    /// Set `position` to `value`.
    fn position(mut self: Self, value: Vec2) -> Self {
        self.position = value;
        return self;
    }
    
    /// Set `radius` to `value`.
    fn radius(mut self: Self, value: f32) -> Self {
        self.radius = value;
        return self;
    }

    /// Set `decay_rate` to `value`.
    fn decay_rate(mut self: Self, value: f32) -> Self {
        self.decay_rate = value;
        return self;
    }
}
