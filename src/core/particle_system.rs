use glam::Vec2;

use super::particle_model::{
    calculate_bezier_point, ParticleCore, ParticleRenderData, ParticleSpawn, ParticleTypeTrait,
    SplineMotion, SplineParticle, HAS_ACCELERATION, HAS_ALPHA_ACCELERATION, HAS_ALPHA_VELOCITY,
    HAS_ROTATION_ACCELERATION, HAS_ROTATION_VELOCITY, HAS_SIZE_ACCELERATION, HAS_SIZE_VELOCITY,
    HAS_SPLINE_ACCELERATION, HAS_SPLINE_VELOCITY, HAS_VELOCITY,
};

pub struct ParticleSystem<T>
where
    T: ParticleTypeTrait,
{
    ballistic_particles: Vec<ParticleCore<T>>,
    spline_particles: Vec<SplineParticle<T>>,
}

impl<T> ParticleSystem<T>
where
    T: ParticleTypeTrait,
{
    pub fn new() -> Self {
        Self {
            ballistic_particles: Vec::new(),
            spline_particles: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.ballistic_particles.len() + self.spline_particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ballistic_particles.is_empty() && self.spline_particles.is_empty()
    }

    pub fn clear(&mut self) {
        self.ballistic_particles.clear();
        self.spline_particles.clear();
    }

    pub fn reserve_particles(&mut self, additional: u32) {
        self.ballistic_particles.reserve(additional as usize);
    }

    pub fn reserve_bundle<B>(&mut self, additional: u32) {
        let _ = std::marker::PhantomData::<B>;
        self.reserve_particles(additional);
    }

    pub fn new_particle(&mut self, particle_type: T, counter: u32, pos: Vec2, size: Vec2) -> usize {
        self.spawn(ParticleSpawn::new(particle_type, counter, pos, size))
    }

    pub fn spawn(&mut self, spawn: ParticleSpawn<T>) -> usize {
        let id = self.len();
        self.push_spawn(spawn);
        id
    }

    pub fn spawn_batch<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = ParticleSpawn<T>>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        if lower > 0 {
            self.ballistic_particles.reserve(lower);
        }

        for spawn in iter {
            self.push_spawn(spawn);
        }
    }

    pub fn spawn_ballistic_batch<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = ParticleSpawn<T>>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        if lower > 0 {
            self.ballistic_particles.reserve(lower);
        }

        for spawn in iter {
            debug_assert!(
                spawn.spline.is_none(),
                "spawn_ballistic_batch received spline spawn"
            );
            self.ballistic_particles
                .push(ParticleCore::from_spawn(&spawn));
        }
    }

    pub fn spawn_spline_batch<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = ParticleSpawn<T>>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        if lower > 0 {
            self.spline_particles.reserve(lower);
        }

        for spawn in iter {
            debug_assert!(
                spawn.spline.is_some(),
                "spawn_spline_batch received non-spline spawn"
            );
            self.spline_particles.push(SplineParticle {
                core: ParticleCore::from_spawn(&spawn),
                spline: SplineMotion::from_spawn(&spawn),
            });
        }
    }

    pub fn for_each_particle(&self, mut f: impl FnMut(ParticleRenderData<T>)) {
        for p in &self.ballistic_particles {
            f(p.into());
        }

        for p in &self.spline_particles {
            f((&p.core).into());
        }
    }

    pub fn step(&mut self) {
        let mut i = 0;
        while i < self.ballistic_particles.len() {
            let particle = &mut self.ballistic_particles[i];
            if particle.counter == 0 {
                self.ballistic_particles.swap_remove(i);
                continue;
            }

            particle.counter -= 1;
            step_core_particle(particle);
            i += 1;
        }

        let mut i = 0;
        while i < self.spline_particles.len() {
            let particle = &mut self.spline_particles[i];
            if particle.core.counter == 0 {
                self.spline_particles.swap_remove(i);
                continue;
            }

            particle.core.counter -= 1;
            step_core_particle(&mut particle.core);
            step_spline_motion(&mut particle.core, &mut particle.spline);
            i += 1;
        }
    }

    fn push_spawn(&mut self, spawn: ParticleSpawn<T>) {
        if spawn.spline.is_some() {
            self.spline_particles.push(SplineParticle {
                core: ParticleCore::from_spawn(&spawn),
                spline: SplineMotion::from_spawn(&spawn),
            });
        } else {
            self.ballistic_particles
                .push(ParticleCore::from_spawn(&spawn));
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

#[inline(always)]
fn step_core_particle<T>(particle: &mut ParticleCore<T>)
where
    T: ParticleTypeTrait,
{
    const FLAGS_LINEAR: u16 = HAS_VELOCITY | HAS_ACCELERATION;
    const FLAGS_ALPHA_ONLY: u16 = HAS_ALPHA_VELOCITY;
    const FLAGS_RICH_NO_ALPHA_ACC: u16 = HAS_VELOCITY
        | HAS_ACCELERATION
        | HAS_SIZE_VELOCITY
        | HAS_SIZE_ACCELERATION
        | HAS_ROTATION_VELOCITY
        | HAS_ROTATION_ACCELERATION
        | HAS_ALPHA_VELOCITY;
    const FLAGS_RICH_WITH_ALPHA_ACC: u16 = FLAGS_RICH_NO_ALPHA_ACC | HAS_ALPHA_ACCELERATION;

    match particle.flags {
        FLAGS_LINEAR => {
            particle.velocity += particle.acceleration;
            particle.pos += particle.velocity;
        }
        FLAGS_ALPHA_ONLY => {
            particle.alpha = (particle.alpha + particle.alpha_velocity).clamp(0.0, 1.0);
        }
        FLAGS_RICH_NO_ALPHA_ACC => {
            particle.velocity += particle.acceleration;
            particle.pos += particle.velocity;

            particle.size_velocity += particle.size_acceleration;
            particle.size += particle.size_velocity;
            particle.size = particle.size.max(Vec2::ZERO);

            particle.rotation_velocity += particle.rotation_acceleration;
            particle.rotation += particle.rotation_velocity;

            particle.alpha = (particle.alpha + particle.alpha_velocity).clamp(0.0, 1.0);
        }
        FLAGS_RICH_WITH_ALPHA_ACC => {
            particle.velocity += particle.acceleration;
            particle.pos += particle.velocity;

            particle.size_velocity += particle.size_acceleration;
            particle.size += particle.size_velocity;
            particle.size = particle.size.max(Vec2::ZERO);

            particle.rotation_velocity += particle.rotation_acceleration;
            particle.rotation += particle.rotation_velocity;

            particle.alpha_velocity += particle.alpha_acceleration;
            particle.alpha = (particle.alpha + particle.alpha_velocity).clamp(0.0, 1.0);
        }
        flags => step_core_particle_generic(particle, flags),
    }
}

#[inline(always)]
fn step_core_particle_generic<T>(particle: &mut ParticleCore<T>, flags: u16)
where
    T: ParticleTypeTrait,
{
    if (flags & HAS_VELOCITY) != 0 {
        if (flags & HAS_ACCELERATION) != 0 {
            particle.velocity += particle.acceleration;
        }
        particle.pos += particle.velocity;
    }

    if (flags & HAS_SIZE_VELOCITY) != 0 {
        if (flags & HAS_SIZE_ACCELERATION) != 0 {
            particle.size_velocity += particle.size_acceleration;
        }
        particle.size += particle.size_velocity;
        particle.size = particle.size.max(Vec2::ZERO);
    }

    if (flags & HAS_ROTATION_VELOCITY) != 0 {
        if (flags & HAS_ROTATION_ACCELERATION) != 0 {
            particle.rotation_velocity += particle.rotation_acceleration;
        }
        particle.rotation += particle.rotation_velocity;
    }

    if (flags & HAS_ALPHA_VELOCITY) != 0 {
        if (flags & HAS_ALPHA_ACCELERATION) != 0 {
            particle.alpha_velocity += particle.alpha_acceleration;
        }
        particle.alpha = (particle.alpha + particle.alpha_velocity).clamp(0.0, 1.0);
    }
}

#[inline(always)]
fn step_spline_motion<T>(particle: &mut ParticleCore<T>, spline: &mut SplineMotion)
where
    T: ParticleTypeTrait,
{
    if spline.has(HAS_SPLINE_VELOCITY) {
        if spline.has(HAS_SPLINE_ACCELERATION) {
            spline.velocity += spline.acceleration;
        }
        spline.t = (spline.t + spline.velocity).clamp(0.0, 1.0);
    }

    let new_pos = calculate_bezier_point(spline.t, spline.point_1, spline.point_2, spline.point_3);
    if spline.strength == 1.0 {
        particle.pos = new_pos;
    } else {
        particle.pos += (new_pos - particle.pos) * spline.strength;
    }
}
