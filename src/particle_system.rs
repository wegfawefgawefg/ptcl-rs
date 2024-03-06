use glam::Vec2;
use hecs::World;

//////////////// COMPONENTS ////////////////

pub struct Counter {
    pub counter: u32,
}

pub struct DrawLayer {
    pub draw_layer: u32,
}

//////////// position
pub struct Position {
    pub pos: Vec2,
}

pub struct Velocity {
    pub vel: Vec2,
}

pub struct Acceleration {
    pub acc: Vec2,
}

//////////// size
pub struct Size {
    pub size: Vec2,
}

pub struct SizeVelocity {
    pub size_vel: f32,
}

pub struct SizeAcceleration {
    pub size_acc: f32,
}

//////////// rotation
pub struct Rotation {
    pub rot: f32,
}

pub struct RotationVelocity {
    pub rot_vel: f32,
}

pub struct RotationAcceleration {
    pub rot_acc: f32,
}

//////////// alpha
pub struct Alpha {
    pub alpha: f32,
}

pub struct AlphaVelocity {
    pub alpha_vel: f32,
}

pub struct AlphaAcceleration {
    pub alpha_acc: f32,
}

//////////// spline
pub struct Spline {
    pub t: f32,
    pub strength: f32,
    pub point_1: Vec2,
    pub point_2: Vec2,
    pub point_3: Vec2,
}

pub struct SplineVelocity {
    pub tvel: f32,
}

pub struct SplineAcceleration {
    pub tacc: f32,
}

///////////////////    MAGIC       ////////////////////
pub trait ParticleTypeTrait: Sync {}

pub struct ParticleTypeComponent<T: ParticleTypeTrait> {
    pub particle_type: T,
}

//////////////////// PARTICLE SYSTEM ////////////////////

pub struct ParticleSystem<T>
where
    T: ParticleTypeTrait + Send + Sync + 'static,
{
    pub world: hecs::World,
    _marker: std::marker::PhantomData<T>,
}

impl<T> ParticleSystem<T>
where
    T: ParticleTypeTrait + 'static + std::marker::Send,
{
    pub fn new() -> Self {
        Self {
            world: World::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn new_particle(
        &mut self,
        particle_type: T,
        counter: u32,

        pos: Vec2,
        size: Vec2,
    ) -> hecs::Entity {
        self.world.spawn((
            particle_type, // Directly store the user-defined type enum
            Counter { counter },
            Position { pos },
            Size { size },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 1.0 },
        ))
    }

    pub fn add_draw_layer(&mut self, entity: hecs::Entity, draw_layer: u32) {
        let _ = self.world.insert_one(entity, DrawLayer { draw_layer });
    }

    pub fn add_velocity(&mut self, entity: hecs::Entity, vel: Vec2) {
        let _ = self.world.insert_one(entity, Velocity { vel });
    }

    pub fn add_acceleration(&mut self, entity: hecs::Entity, acc: Vec2) {
        let _ = self.world.insert_one(entity, Acceleration { acc });
    }

    pub fn add_size_velocity(&mut self, entity: hecs::Entity, size_vel: f32) {
        let _ = self.world.insert_one(entity, SizeVelocity { size_vel });
    }

    pub fn add_size_acceleration(&mut self, entity: hecs::Entity, size_acc: f32) {
        let _ = self.world.insert_one(entity, SizeAcceleration { size_acc });
    }

    pub fn add_rotation(&mut self, entity: hecs::Entity, rot: f32) {
        let _ = self.world.insert_one(entity, Rotation { rot });
    }

    pub fn add_rotation_velocity(&mut self, entity: hecs::Entity, rot_vel: f32) {
        let _ = self.world.insert_one(entity, RotationVelocity { rot_vel });
    }

    pub fn add_rotation_acceleration(&mut self, entity: hecs::Entity, rot_acc: f32) {
        let _ = self
            .world
            .insert_one(entity, RotationAcceleration { rot_acc });
    }

    pub fn add_alpha(&mut self, entity: hecs::Entity, alpha: f32) {
        let _ = self.world.insert_one(entity, Alpha { alpha });
    }

    pub fn add_alpha_velocity(&mut self, entity: hecs::Entity, alpha_vel: f32) {
        let _ = self.world.insert_one(entity, AlphaVelocity { alpha_vel });
    }

    pub fn add_alpha_acceleration(&mut self, entity: hecs::Entity, alpha_acc: f32) {
        let _ = self
            .world
            .insert_one(entity, AlphaAcceleration { alpha_acc });
    }

    pub fn add_spline(
        &mut self,
        entity: hecs::Entity,
        point_1: Vec2,
        point_2: Vec2,
        point_3: Vec2,
        strength: f32,
    ) {
        let _ = self.world.insert_one(
            entity,
            Spline {
                t: 0.0,
                strength,
                point_1,
                point_2,
                point_3,
            },
        );
    }

    pub fn add_spline_velocity(&mut self, entity: hecs::Entity, tvel: f32) {
        let _ = self.world.insert_one(entity, SplineVelocity { tvel });
    }

    pub fn add_spline_acceleration(&mut self, entity: hecs::Entity, tacc: f32) {
        let _ = self.world.insert_one(entity, SplineAcceleration { tacc });
    }

    pub fn step(&mut self) {
        let mut expired_particles = vec![];
        for (entity, counter) in self.world.query::<&Counter>().iter() {
            if counter.counter == 0 {
                expired_particles.push(entity);
            }
        }

        for entity in expired_particles {
            self.world.despawn(entity).unwrap();
        }

        // counter
        for (_, counter) in self.world.query::<&mut Counter>().iter() {
            if counter.counter > 0 {
                counter.counter -= 1;
            }
        }

        // pos
        for (_, (vel, acc)) in self.world.query::<(&mut Velocity, &Acceleration)>().iter() {
            vel.vel += acc.acc;
        }
        for (_, (pos, vel)) in self.world.query::<(&mut Position, &Velocity)>().iter() {
            pos.pos += vel.vel;
        }

        // size
        for (_, (size_vel, size_acc)) in self
            .world
            .query::<(&mut SizeVelocity, &SizeAcceleration)>()
            .iter()
        {
            size_vel.size_vel += size_acc.size_acc;
        }
        for (_, (size, size_vel)) in self.world.query::<(&mut Size, &SizeVelocity)>().iter() {
            size.size += size_vel.size_vel;
            size.size = size.size.max(Vec2::ZERO);
        }

        // rotation
        for (_, (rot_vel, rot_acc)) in self
            .world
            .query::<(&mut RotationVelocity, &RotationAcceleration)>()
            .iter()
        {
            rot_vel.rot_vel += rot_acc.rot_acc;
        }
        for (_, (rot, rot_vel)) in self
            .world
            .query::<(&mut Rotation, &RotationVelocity)>()
            .iter()
        {
            rot.rot += rot_vel.rot_vel;
        }

        // alpha
        for (_, (alpha_vel, alpha_acc)) in self
            .world
            .query::<(&mut AlphaVelocity, &AlphaAcceleration)>()
            .iter()
        {
            alpha_vel.alpha_vel += alpha_acc.alpha_acc;
        }
        for (_, (alpha, alpha_vel)) in self.world.query::<(&mut Alpha, &AlphaVelocity)>().iter() {
            alpha.alpha += alpha_vel.alpha_vel;
            alpha.alpha = alpha.alpha.min(1.0).max(0.0);
        }

        // spline
        for (_, (tvel, tacc)) in self
            .world
            .query::<(&mut SplineVelocity, &SplineAcceleration)>()
            .iter()
        {
            tvel.tvel += tacc.tacc;
        }

        for (_, (pos, spline, tvel)) in self
            .world
            .query::<(&mut Position, &mut Spline, &SplineVelocity)>()
            .iter()
        {
            spline.t += tvel.tvel;
            spline.t = spline.t.min(1.0).max(0.0);
            let new_pos =
                calculate_bezier_point(spline.t, spline.point_1, spline.point_2, spline.point_3);
            if spline.strength == 1.0 {
                pos.pos = new_pos;
            } else {
                let delta = new_pos - pos.pos;
                pos.pos += delta * spline.strength;
            }
        }
    }
}

pub fn calculate_bezier_point(t: f32, point_1: Vec2, point_2: Vec2, point_3: Vec2) -> Vec2 {
    let one_minus_t = 1.0 - t;
    (point_1 * one_minus_t * one_minus_t) + (point_2 * 2.0 * one_minus_t * t) + (point_3 * t * t)
}
