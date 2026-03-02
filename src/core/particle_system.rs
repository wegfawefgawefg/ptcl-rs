use glam::Vec2;

pub trait ParticleTypeTrait: Copy + Send + Sync + 'static {}

const HAS_VELOCITY: u16 = 1 << 0;
const HAS_ACCELERATION: u16 = 1 << 1;
const HAS_SIZE_VELOCITY: u16 = 1 << 2;
const HAS_SIZE_ACCELERATION: u16 = 1 << 3;
const HAS_ROTATION_VELOCITY: u16 = 1 << 4;
const HAS_ROTATION_ACCELERATION: u16 = 1 << 5;
const HAS_ALPHA_VELOCITY: u16 = 1 << 6;
const HAS_ALPHA_ACCELERATION: u16 = 1 << 7;
const HAS_SPLINE: u16 = 1 << 8;
const HAS_SPLINE_VELOCITY: u16 = 1 << 9;
const HAS_SPLINE_ACCELERATION: u16 = 1 << 10;

#[derive(Clone, Copy, Debug)]
pub struct SplineState {
    pub t: f32,
    pub strength: f32,
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub point_3: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct ParticleSpawn<T>
where
    T: ParticleTypeTrait,
{
    pub particle_type: T,
    pub counter: u32,
    pub pos: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub draw_layer: u32,
    pub alpha: f32,
    pub velocity: Option<Vec2>,
    pub acceleration: Option<Vec2>,
    pub size_velocity: Option<f32>,
    pub size_acceleration: Option<f32>,
    pub rotation_velocity: Option<f32>,
    pub rotation_acceleration: Option<f32>,
    pub alpha_velocity: Option<f32>,
    pub alpha_acceleration: Option<f32>,
    pub spline: Option<SplineState>,
    pub spline_velocity: Option<f32>,
    pub spline_acceleration: Option<f32>,
}

impl<T> ParticleSpawn<T>
where
    T: ParticleTypeTrait,
{
    pub fn new(particle_type: T, counter: u32, pos: Vec2, size: Vec2) -> Self {
        Self {
            particle_type,
            counter,
            pos,
            size,
            rotation: 0.0,
            draw_layer: 0,
            alpha: 1.0,
            velocity: None,
            acceleration: None,
            size_velocity: None,
            size_acceleration: None,
            rotation_velocity: None,
            rotation_acceleration: None,
            alpha_velocity: None,
            alpha_acceleration: None,
            spline: None,
            spline_velocity: None,
            spline_acceleration: None,
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_draw_layer(mut self, draw_layer: u32) -> Self {
        self.draw_layer = draw_layer;
        self
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn with_acceleration(mut self, acceleration: Vec2) -> Self {
        self.acceleration = Some(acceleration);
        self
    }

    pub fn with_size_velocity(mut self, size_velocity: f32) -> Self {
        self.size_velocity = Some(size_velocity);
        self
    }

    pub fn with_size_acceleration(mut self, size_acceleration: f32) -> Self {
        self.size_acceleration = Some(size_acceleration);
        self
    }

    pub fn with_rotation_velocity(mut self, rotation_velocity: f32) -> Self {
        self.rotation_velocity = Some(rotation_velocity);
        self
    }

    pub fn with_rotation_acceleration(mut self, rotation_acceleration: f32) -> Self {
        self.rotation_acceleration = Some(rotation_acceleration);
        self
    }

    pub fn with_alpha_velocity(mut self, alpha_velocity: f32) -> Self {
        self.alpha_velocity = Some(alpha_velocity);
        self
    }

    pub fn with_alpha_acceleration(mut self, alpha_acceleration: f32) -> Self {
        self.alpha_acceleration = Some(alpha_acceleration);
        self
    }

    pub fn with_spline(mut self, spline: SplineState) -> Self {
        self.spline = Some(spline);
        self
    }

    pub fn with_spline_velocity(mut self, spline_velocity: f32) -> Self {
        self.spline_velocity = Some(spline_velocity);
        self
    }

    pub fn with_spline_acceleration(mut self, spline_acceleration: f32) -> Self {
        self.spline_acceleration = Some(spline_acceleration);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Particle<T>
where
    T: ParticleTypeTrait,
{
    pub particle_type: T,
    pub counter: u32,
    pub pos: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub draw_layer: u32,
    pub alpha: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub size_velocity: f32,
    pub size_acceleration: f32,
    pub rotation_velocity: f32,
    pub rotation_acceleration: f32,
    pub alpha_velocity: f32,
    pub alpha_acceleration: f32,
    pub spline_t: f32,
    pub spline_strength: f32,
    pub spline_point_1: Vec2,
    pub spline_point_2: Vec2,
    pub spline_point_3: Vec2,
    pub spline_velocity: f32,
    pub spline_acceleration: f32,
    flags: u16,
}

impl<T> Particle<T>
where
    T: ParticleTypeTrait,
{
    fn from_spawn(spawn: ParticleSpawn<T>) -> Self {
        let mut flags = 0u16;

        let velocity = if let Some(v) = spawn.velocity {
            flags |= HAS_VELOCITY;
            v
        } else {
            Vec2::ZERO
        };

        let acceleration = if let Some(v) = spawn.acceleration {
            flags |= HAS_ACCELERATION;
            v
        } else {
            Vec2::ZERO
        };

        let size_velocity = if let Some(v) = spawn.size_velocity {
            flags |= HAS_SIZE_VELOCITY;
            v
        } else {
            0.0
        };

        let size_acceleration = if let Some(v) = spawn.size_acceleration {
            flags |= HAS_SIZE_ACCELERATION;
            v
        } else {
            0.0
        };

        let rotation_velocity = if let Some(v) = spawn.rotation_velocity {
            flags |= HAS_ROTATION_VELOCITY;
            v
        } else {
            0.0
        };

        let rotation_acceleration = if let Some(v) = spawn.rotation_acceleration {
            flags |= HAS_ROTATION_ACCELERATION;
            v
        } else {
            0.0
        };

        let alpha_velocity = if let Some(v) = spawn.alpha_velocity {
            flags |= HAS_ALPHA_VELOCITY;
            v
        } else {
            0.0
        };

        let alpha_acceleration = if let Some(v) = spawn.alpha_acceleration {
            flags |= HAS_ALPHA_ACCELERATION;
            v
        } else {
            0.0
        };

        let (spline_t, spline_strength, spline_point_1, spline_point_2, spline_point_3) =
            if let Some(spline) = spawn.spline {
                flags |= HAS_SPLINE;
                (
                    spline.t,
                    spline.strength,
                    spline.point_1,
                    spline.point_2,
                    spline.point_3,
                )
            } else {
                (0.0, 0.0, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO)
            };

        let spline_velocity = if let Some(v) = spawn.spline_velocity {
            flags |= HAS_SPLINE_VELOCITY;
            v
        } else {
            0.0
        };

        let spline_acceleration = if let Some(v) = spawn.spline_acceleration {
            flags |= HAS_SPLINE_ACCELERATION;
            v
        } else {
            0.0
        };

        Self {
            particle_type: spawn.particle_type,
            counter: spawn.counter,
            pos: spawn.pos,
            size: spawn.size,
            rotation: spawn.rotation,
            draw_layer: spawn.draw_layer,
            alpha: spawn.alpha,
            velocity,
            acceleration,
            size_velocity,
            size_acceleration,
            rotation_velocity,
            rotation_acceleration,
            alpha_velocity,
            alpha_acceleration,
            spline_t,
            spline_strength,
            spline_point_1,
            spline_point_2,
            spline_point_3,
            spline_velocity,
            spline_acceleration,
            flags,
        }
    }

    #[inline(always)]
    fn has(&self, mask: u16) -> bool {
        (self.flags & mask) != 0
    }
}

pub struct ParticleSystem<T>
where
    T: ParticleTypeTrait,
{
    pub particles: Vec<Particle<T>>,
}

impl<T> ParticleSystem<T>
where
    T: ParticleTypeTrait,
{
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    pub fn reserve_particles(&mut self, additional: u32) {
        self.particles.reserve(additional as usize);
    }

    pub fn reserve_bundle<B>(&mut self, additional: u32) {
        let _ = std::marker::PhantomData::<B>;
        self.reserve_particles(additional);
    }

    pub fn new_particle(&mut self, particle_type: T, counter: u32, pos: Vec2, size: Vec2) -> usize {
        self.spawn(ParticleSpawn::new(particle_type, counter, pos, size))
    }

    pub fn spawn(&mut self, spawn: ParticleSpawn<T>) -> usize {
        let id = self.particles.len();
        self.particles.push(Particle::from_spawn(spawn));
        id
    }

    pub fn spawn_batch<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = ParticleSpawn<T>>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        if lower > 0 {
            self.particles.reserve(lower);
        }

        for spawn in iter {
            self.particles.push(Particle::from_spawn(spawn));
        }
    }

    pub fn step(&mut self) {
        let mut i = 0;
        while i < self.particles.len() {
            let particle = &mut self.particles[i];
            if particle.counter == 0 {
                self.particles.swap_remove(i);
                continue;
            }
            particle.counter -= 1;

            if particle.has(HAS_VELOCITY) {
                if particle.has(HAS_ACCELERATION) {
                    particle.velocity += particle.acceleration;
                }
                particle.pos += particle.velocity;
            }

            if particle.has(HAS_SIZE_VELOCITY) {
                if particle.has(HAS_SIZE_ACCELERATION) {
                    particle.size_velocity += particle.size_acceleration;
                }
                particle.size += particle.size_velocity;
                particle.size = particle.size.max(Vec2::ZERO);
            }

            if particle.has(HAS_ROTATION_VELOCITY) {
                if particle.has(HAS_ROTATION_ACCELERATION) {
                    particle.rotation_velocity += particle.rotation_acceleration;
                }
                particle.rotation += particle.rotation_velocity;
            }

            if particle.has(HAS_ALPHA_VELOCITY) {
                if particle.has(HAS_ALPHA_ACCELERATION) {
                    particle.alpha_velocity += particle.alpha_acceleration;
                }
                particle.alpha = (particle.alpha + particle.alpha_velocity).clamp(0.0, 1.0);
            }

            if particle.has(HAS_SPLINE) {
                if particle.has(HAS_SPLINE_VELOCITY) {
                    if particle.has(HAS_SPLINE_ACCELERATION) {
                        particle.spline_velocity += particle.spline_acceleration;
                    }
                    particle.spline_t =
                        (particle.spline_t + particle.spline_velocity).clamp(0.0, 1.0);
                }

                let new_pos = calculate_bezier_point(
                    particle.spline_t,
                    particle.spline_point_1,
                    particle.spline_point_2,
                    particle.spline_point_3,
                );
                if particle.spline_strength == 1.0 {
                    particle.pos = new_pos;
                } else {
                    particle.pos += (new_pos - particle.pos) * particle.spline_strength;
                }
            }

            i += 1;
        }
    }
}

impl<T> Default for ParticleSystem<T>
where
    T: ParticleTypeTrait,
{
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_bezier_point(t: f32, point_1: Vec2, point_2: Vec2, point_3: Vec2) -> Vec2 {
    let one_minus_t = 1.0 - t;
    (point_1 * one_minus_t * one_minus_t) + (point_2 * 2.0 * one_minus_t * t) + (point_3 * t * t)
}
