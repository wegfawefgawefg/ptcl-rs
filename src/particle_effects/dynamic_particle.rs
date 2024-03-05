use glam::Vec2;

use crate::{graphics::DrawLayer, special_particle_getters};

use super::special_particle::{
    get_sample_region, ParticleEffect, ParticleEffectType, SampleRegion,
};

pub struct DynamicParticle {
    pub type_: ParticleEffectType,
    pub counter: u32,
    pub draw_layer: DrawLayer,

    pub pos: Vec2,
    pub size: Vec2,
    pub rot: f32,
    pub alpha: f32,

    pub vel: Vec2,
    pub svel: Vec2,
    pub rotvel: f32,
    pub alpha_vel: f32,
}

impl ParticleEffect for DynamicParticle {
    special_particle_getters!();

    fn step(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }

        self.pos += self.vel;
        self.size += self.svel;
        self.rot += self.rotvel;
        self.alpha += self.alpha_vel;

        self.size = self.size.max(Vec2::ZERO);
        self.alpha = self.alpha.min(1.0).max(0.0);
    }
}
