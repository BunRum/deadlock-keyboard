use std::mem::offset_of;

use super::{
    entities::{CCitadelHeroComponent, CitadelHeroSpawnData_t, CitadelPlayerPawnLayout, HeroID_t},
    property::EntityProperty,
};

pub struct Health;
impl EntityProperty for Health {
    type Value = i32;

    const OFFSET: usize = offset_of!(CitadelPlayerPawnLayout, health);
}

pub struct MaxHealth;
impl EntityProperty for MaxHealth {
    type Value = i32;

    const OFFSET: usize = offset_of!(CitadelPlayerPawnLayout, max_health);
}

pub struct Level;
impl EntityProperty for Level {
    type Value = i32;

    const OFFSET: usize = offset_of!(CitadelPlayerPawnLayout, level);
}

pub struct HeroId;
impl EntityProperty for HeroId {
    type Value = u32;

    const OFFSET: usize = offset_of!(CitadelPlayerPawnLayout, hero_component)
        + offset_of!(CCitadelHeroComponent, spawned_hero)
        + offset_of!(CitadelHeroSpawnData_t, hero_id)
        + offset_of!(HeroID_t, value);
}
