use glam::{IVec2, Vec2};

pub trait ParticleType {
    fn get_sample_region(&self) -> &'static SampleRegion;
}

pub trait Particle<T: ParticleType> {
    fn step(&mut self);
    fn is_finished(&self) -> bool;

    fn get_pos(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_rot(&self) -> f32;
    fn get_counter(&self) -> u32;
    fn get_alpha(&self) -> f32;

    fn get_type(&self) -> T;
    fn get_sample_region(&self) -> &'static SampleRegion;
}

// /** put this at the top of your special effect definitions */
// #[macro_export]
// macro_rules! particle_effect_getters {
//     () => {
//         fn get_pos(&self) -> Vec2 {
//             self.pos
//         }
//         fn get_size(&self) -> Vec2 {
//             self.size
//         }
//         fn get_rot(&self) -> f32 {
//             self.rot
//         }
//         fn get_counter(&self) -> u32 {
//             self.counter
//         }

//         fn get_alpha(&self) -> f32 {
//             self.alpha
//         }
//         fn is_finished(&self) -> bool {
//             self.counter <= 0
//         }

//         fn get_type(&self) -> ParticleEffectType {
//             self.type_
//         }

//         fn get_sample_region(&self) -> &'static SampleRegion {
//             self.particle_type.get_sample_region()
//         }
//     };
// }

// #[derive(Clone, Copy)]
// pub enum ParticleEffectType {
//     GrenadeBoom,
//     BasicSmoke,
//     SimpleSpark,
//     Pow,
//     BloodBall,
//     LittleBrownShard,
// }

// pub enum ParticleEffectSampleRegionName {
//     BigExplosion,
//     LittleExplosion,
//     Spark,
//     Pow,
//     LittleSmoke,
//     BigSmoke,
//     ExplosionFrame1,
//     ExplosionFrame2,
//     ExplosionFrame3,
//     ExplosionFrame4,
//     BloodBall,
//     LittleBrownShard,
// }

// pub struct SampleRegion {
//     pub pos: IVec2,
//     pub size: IVec2,
// }

// pub fn get_sample_region(
//     special_effect_type: ParticleEffectType,
//     counter: u32,
// ) -> &'static SampleRegion {
//     let region_name = get_special_effect_sample_region_name(special_effect_type, counter);
//     get_special_effect_sample_region_from_name(region_name)
// }

// pub fn get_special_effect_sample_region_name(
//     special_effect_type: ParticleEffectType,
//     counter: u32,
// ) -> ParticleEffectSampleRegionName {
//     match special_effect_type {
//         ParticleEffectType::GrenadeBoom => match counter {
//             6..=7 => ParticleEffectSampleRegionName::ExplosionFrame1,
//             4..=5 => ParticleEffectSampleRegionName::ExplosionFrame2,
//             2..=3 => ParticleEffectSampleRegionName::ExplosionFrame3,
//             _ => ParticleEffectSampleRegionName::ExplosionFrame4,
//         },
//         ParticleEffectType::BasicSmoke => ParticleEffectSampleRegionName::BigSmoke,
//         ParticleEffectType::SimpleSpark => ParticleEffectSampleRegionName::Spark,
//         ParticleEffectType::Pow => ParticleEffectSampleRegionName::Pow,
//         ParticleEffectType::BloodBall => ParticleEffectSampleRegionName::BloodBall,
//         ParticleEffectType::LittleBrownShard => ParticleEffectSampleRegionName::LittleBrownShard,
//     }
// }

// pub fn get_special_effect_sample_region_from_name(
//     name: ParticleEffectSampleRegionName,
// ) -> &'static SampleRegion {
//     match name {
//         ParticleEffectSampleRegionName::BigExplosion => &SampleRegion {
//             pos: IVec2 { x: 0, y: 0 },
//             size: IVec2 { x: 45, y: 41 },
//         },
//         ParticleEffectSampleRegionName::LittleExplosion => &SampleRegion {
//             pos: IVec2 { x: 60, y: 0 },
//             size: IVec2 { x: 10, y: 8 },
//         },
//         ParticleEffectSampleRegionName::Spark => &SampleRegion {
//             pos: IVec2 { x: 63, y: 22 },
//             size: IVec2 { x: 7, y: 12 },
//         },
//         ParticleEffectSampleRegionName::Pow => &SampleRegion {
//             pos: IVec2 { x: 61, y: 12 },
//             size: IVec2 { x: 9, y: 7 },
//         },
//         ParticleEffectSampleRegionName::LittleSmoke => &SampleRegion {
//             pos: IVec2 { x: 71, y: 0 },
//             size: IVec2 { x: 10, y: 10 },
//         },
//         ParticleEffectSampleRegionName::BigSmoke => &SampleRegion {
//             pos: IVec2 { x: 0, y: 269 },
//             size: IVec2 { x: 65, y: 61 },
//         },
//         ParticleEffectSampleRegionName::ExplosionFrame1 => &SampleRegion {
//             pos: IVec2 { x: 0, y: 164 },
//             size: IVec2 { x: 45, y: 42 },
//         },
//         ParticleEffectSampleRegionName::ExplosionFrame2 => &SampleRegion {
//             pos: IVec2 { x: 0, y: 40 },
//             size: IVec2 { x: 62, y: 60 },
//         },
//         ParticleEffectSampleRegionName::ExplosionFrame3 => &SampleRegion {
//             pos: IVec2 { x: 0, y: 101 },
//             size: IVec2 { x: 61, y: 62 },
//         },
//         ParticleEffectSampleRegionName::ExplosionFrame4 => &SampleRegion {
//             pos: IVec2 { x: 0, y: 206 },
//             size: IVec2 { x: 65, y: 61 },
//         },
//         ParticleEffectSampleRegionName::BloodBall => &SampleRegion {
//             pos: IVec2 { x: 63, y: 38 },
//             size: IVec2 { x: 16, y: 17 },
//         },
//         ParticleEffectSampleRegionName::LittleBrownShard => &SampleRegion {
//             pos: IVec2 { x: 89, y: 5 },
//             size: IVec2 { x: 2, y: 1 },
//         },
//     }
// }
