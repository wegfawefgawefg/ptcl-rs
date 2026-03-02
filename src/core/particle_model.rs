use glam::Vec2;

pub trait ParticleTypeTrait: Copy + Send + Sync + 'static {}

pub(crate) const HAS_VELOCITY: u16 = 1 << 0;
pub(crate) const HAS_ACCELERATION: u16 = 1 << 1;
pub(crate) const HAS_SIZE_VELOCITY: u16 = 1 << 2;
pub(crate) const HAS_SIZE_ACCELERATION: u16 = 1 << 3;
pub(crate) const HAS_ROTATION_VELOCITY: u16 = 1 << 4;
pub(crate) const HAS_ROTATION_ACCELERATION: u16 = 1 << 5;
pub(crate) const HAS_ALPHA_VELOCITY: u16 = 1 << 6;
pub(crate) const HAS_ALPHA_ACCELERATION: u16 = 1 << 7;

pub(crate) const HAS_SPLINE_VELOCITY: u16 = 1 << 0;
pub(crate) const HAS_SPLINE_ACCELERATION: u16 = 1 << 1;

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
pub(crate) struct ParticleCore<T>
where
    T: ParticleTypeTrait,
{
    pub(crate) particle_type: T,
    pub(crate) counter: u32,
    pub(crate) pos: Vec2,
    pub(crate) size: Vec2,
    pub(crate) rotation: f32,
    pub(crate) draw_layer: u32,
    pub(crate) alpha: f32,
    pub(crate) velocity: Vec2,
    pub(crate) acceleration: Vec2,
    pub(crate) size_velocity: f32,
    pub(crate) size_acceleration: f32,
    pub(crate) rotation_velocity: f32,
    pub(crate) rotation_acceleration: f32,
    pub(crate) alpha_velocity: f32,
    pub(crate) alpha_acceleration: f32,
    pub(crate) flags: u16,
}

impl<T> ParticleCore<T>
where
    T: ParticleTypeTrait,
{
    pub(crate) fn from_spawn(spawn: &ParticleSpawn<T>) -> Self {
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
            flags,
        }
    }

    #[inline(always)]
    pub(crate) fn has(&self, mask: u16) -> bool {
        (self.flags & mask) != 0
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SplineMotion {
    pub(crate) t: f32,
    pub(crate) strength: f32,
    pub(crate) point_1: Vec2,
    pub(crate) point_2: Vec2,
    pub(crate) point_3: Vec2,
    pub(crate) velocity: f32,
    pub(crate) acceleration: f32,
    pub(crate) flags: u16,
}

impl SplineMotion {
    pub(crate) fn from_spawn<T>(spawn: &ParticleSpawn<T>) -> Self
    where
        T: ParticleTypeTrait,
    {
        let spline = spawn
            .spline
            .expect("internal error: spline lane requires spline state");

        let mut flags = 0u16;
        let velocity = if let Some(v) = spawn.spline_velocity {
            flags |= HAS_SPLINE_VELOCITY;
            v
        } else {
            0.0
        };

        let acceleration = if let Some(v) = spawn.spline_acceleration {
            flags |= HAS_SPLINE_ACCELERATION;
            v
        } else {
            0.0
        };

        Self {
            t: spline.t,
            strength: spline.strength,
            point_1: spline.point_1,
            point_2: spline.point_2,
            point_3: spline.point_3,
            velocity,
            acceleration,
            flags,
        }
    }

    #[inline(always)]
    pub(crate) fn has(&self, mask: u16) -> bool {
        (self.flags & mask) != 0
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SplineParticle<T>
where
    T: ParticleTypeTrait,
{
    pub(crate) core: ParticleCore<T>,
    pub(crate) spline: SplineMotion,
}

#[derive(Clone, Copy, Debug)]
pub struct ParticleRenderData<T>
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
}

impl<T> From<&ParticleCore<T>> for ParticleRenderData<T>
where
    T: ParticleTypeTrait,
{
    fn from(value: &ParticleCore<T>) -> Self {
        Self {
            particle_type: value.particle_type,
            counter: value.counter,
            pos: value.pos,
            size: value.size,
            rotation: value.rotation,
            draw_layer: value.draw_layer,
            alpha: value.alpha,
        }
    }
}

pub fn calculate_bezier_point(t: f32, point_1: Vec2, point_2: Vec2, point_3: Vec2) -> Vec2 {
    let one_minus_t = 1.0 - t;
    (point_1 * one_minus_t * one_minus_t) + (point_2 * 2.0 * one_minus_t * t) + (point_3 * t * t)
}
