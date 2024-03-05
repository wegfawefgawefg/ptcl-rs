use glam::IVec2;

use crate::particle_system::ParticleTypeTrait;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParticleType {
    Smoke,
    Explosion,
    BloodBall,
}

impl ParticleTypeTrait for ParticleType {}

pub struct SampleRegion {
    pub pos: IVec2,
    pub size: IVec2,
}

// global public
const SMOKE: SampleRegion = SampleRegion {
    pos: IVec2 { x: 0, y: 269 },
    size: IVec2 { x: 65, y: 61 },
};

const EXPLOSION_FRAME_1: SampleRegion = SampleRegion {
    pos: IVec2 { x: 0, y: 164 },
    size: IVec2 { x: 45, y: 42 },
};

const EXPLOSION_FRAME_2: SampleRegion = SampleRegion {
    pos: IVec2 { x: 0, y: 40 },
    size: IVec2 { x: 62, y: 60 },
};

const EXPLOSION_FRAME_3: SampleRegion = SampleRegion {
    pos: IVec2 { x: 0, y: 101 },
    size: IVec2 { x: 61, y: 62 },
};

const EXPLOSION_FRAME_4: SampleRegion = SampleRegion {
    pos: IVec2 { x: 0, y: 206 },
    size: IVec2 { x: 65, y: 61 },
};

const BLOOD_BALL: SampleRegion = SampleRegion {
    pos: IVec2 { x: 63, y: 38 },
    size: IVec2 { x: 16, y: 17 },
};

pub fn get_sample_region(particle_type: ParticleType, counter: u32) -> &'static SampleRegion {
    match particle_type {
        ParticleType::Explosion => match counter {
            6..=7 => &EXPLOSION_FRAME_1,
            4..=5 => &EXPLOSION_FRAME_2,
            2..=3 => &EXPLOSION_FRAME_3,
            0..=1 => &EXPLOSION_FRAME_4,
            _ => &BLOOD_BALL,
        },
        ParticleType::Smoke => &SMOKE,
        ParticleType::BloodBall => &BLOOD_BALL,
    }
}
