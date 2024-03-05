use glam::Vec2;

pub struct StaticParticle {
    pub type_: ParticleEffectType,
    pub counter: u32,
    pub draw_layer: DrawLayer,

    pub pos: Vec2,
    pub size: Vec2,
    pub rot: f32,
    pub alpha: f32,
}

impl ParticleEffect for StaticParticle {
    special_Particle_getters!();

    fn step(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }
}
