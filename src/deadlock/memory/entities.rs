use proc_mem::{ProcMemError, Process};

use super::property::EntityProperty;

#[repr(C)]
pub struct CitadelPlayerPawnLayout {
    pub(crate) _padding0: [u8; 0x350],
    pub(crate) max_health: i32,
    pub(crate) health: i32,
    pub(crate) _padding1: [u8; 0x1238 - 0x358],
    pub(crate) level: i32,
    pub(crate) _padding2: [u8; 0x1628 - 0x123C],
    pub(crate) hero_component: CCitadelHeroComponent,
}

#[repr(C)]
pub struct CCitadelHeroComponent {
    pub(crate) _padding0: [u8; 0x18],
    pub(crate) spawned_hero: CitadelHeroSpawnData_t,
}

#[repr(C)]
pub struct CitadelHeroSpawnData_t {
    pub(crate) _padding0: [u8; 0x08],
    pub(crate) hero_id: HeroID_t,
}

#[repr(C)]
pub struct HeroID_t {
    pub(crate) value: u32,
}

pub struct CitadelPlayerPawn {
    address: usize,
}

impl CitadelPlayerPawn {
    pub fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn read<P: EntityProperty>(&self, process: &Process) -> Result<P::Value, ProcMemError> {
        let address = self
            .address
            .checked_add(P::OFFSET)
            .ok_or(ProcMemError::ReadMemoryError)?;

        process.read_mem(address)
    }
}
