use deadlock_keyboard::deadlock::memory::{
    get_deadlock_process, get_player_pawn,
    properties::{Health, HeroId, Level, MaxHealth},
};

#[test]
pub fn test_read_player_properties() {
    let process = get_deadlock_process().expect("failed to get process");
    let pawn = get_player_pawn(&process).expect("failed to find player pawn");

    let health = pawn
        .read::<Health>(&process)
        .expect("failed to read player health");
    println!("Player health: {health}");

    let max_health = pawn
        .read::<MaxHealth>(&process)
        .expect("failed to read player max health");
    println!("Player max health: {max_health}");

    let level = pawn
        .read::<Level>(&process)
        .expect("failed to read player level");
    println!("Player level: {level}");

    let hero_id = pawn
        .read::<HeroId>(&process)
        .expect("failed to read player hero id");
    println!("Player hero id: {hero_id}");
}
