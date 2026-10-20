use proc_mem::Process;

use self::{
    entities::CitadelPlayerPawn,
    properties::{Health, HeroId, Level, MaxHealth},
};

pub mod entities;
pub mod properties;
pub mod property;

// client.dll global: C_CitadelPlayerPawn
const C_CITADEL_PLAYER_PAWN_GLOBAL_RVA: usize = 0x3400258;
// client.dll vtable for C_CitadelPlayerPawn
const C_CITADEL_PLAYER_PAWN_VTABLE_RVA: usize = 0x2748D20;

pub fn get_deadlock_process() -> Result<Process, proc_mem::ProcMemError> {
    Process::with_name("deadlock.exe")
}

// pretty much just returns the address of the C_CitadelPlayerPawn instance, which is a subclass of C_BaseEntity
pub fn get_pawn_address(process: &Process) -> Result<usize, proc_mem::ProcMemError> {
    let client = process.module("client.dll")?;
    let client_base = client.base_address();
    let pawn: usize = process.read_mem(client_base + C_CITADEL_PLAYER_PAWN_GLOBAL_RVA)?;

    if pawn == 0 {
        return Err(proc_mem::ProcMemError::ReadMemoryError);
    }

    let vtable: usize = process.read_mem(pawn)?;
    if vtable != client_base + C_CITADEL_PLAYER_PAWN_VTABLE_RVA {
        return Err(proc_mem::ProcMemError::ReadMemoryError);
    }

    Ok(pawn)
}

pub fn get_player_pawn(process: &Process) -> Result<CitadelPlayerPawn, proc_mem::ProcMemError> {
    Ok(CitadelPlayerPawn::new(get_pawn_address(process)?))
}

pub fn get_player_health(process: &Process) -> Result<i32, proc_mem::ProcMemError> {
    get_player_pawn(process)?.read::<Health>(process)
}

pub fn get_player_max_health(process: &Process) -> Result<i32, proc_mem::ProcMemError> {
    get_player_pawn(process)?.read::<MaxHealth>(process)
}

pub fn get_player_level(process: &Process) -> Result<i32, proc_mem::ProcMemError> {
    get_player_pawn(process)?.read::<Level>(process)
}

pub fn get_player_hero_id(process: &Process) -> Result<u32, proc_mem::ProcMemError> {
    get_player_pawn(process)?.read::<HeroId>(process)
}
