#![allow(dead_code)]

use crate::deadlock::colors::HeroColor;

use super::colors::ColorsFile;
use std::fs;
use std::path::Path;

pub fn get_current_hero(log_path: &Path) -> Option<String> {
    let bytes = fs::read(log_path).ok()?;
    let log = String::from_utf8_lossy(&bytes);
    let loaded_hero = regex::Regex::new(r"\[Server\] Loaded hero \d+/(hero_\w+)").ok()?;
    let client_hero =
        regex::Regex::new(r"VMDL Camera Pose Success!.*models/heroes(?:_wip|_staging)?/(\w+)/")
            .ok()?;

    log.lines().rev().find_map(|line| {
        loaded_hero
            .captures(line)
            .or_else(|| client_hero.captures(line))
            .map(|captures| {
                let raw = captures.get(1).unwrap().as_str().to_lowercase();
                let key = if raw.starts_with("hero_") {
                    raw
                } else {
                    format!("hero_{raw}")
                };
                key
            })
    })
}

pub fn get_hero_color(hero_name: &str) -> (u8, u8, u8) {
    let colors_json: ColorsFile = fs::read_to_string("src/deadlock/colors.json")
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();

    colors_json
        .colors
        .iter()
        .find(|HeroColor(name, _)| name == hero_name)
        .map(|&HeroColor(_, [r, g, b])| (r, g, b))
        .unwrap_or((255, 255, 255))
}

fn calculate_max_health(
    base_health: i32,
    level: u32,
    health_per_level: i32,
    vitality_percent: f32,
    base_health_percent: f32,
    flat_bonus_health: i32,
) -> i32 {
    let level_health = base_health + level.saturating_sub(1) as i32 * health_per_level;

    (level_health as f32 * (1.0 + (vitality_percent + base_health_percent) / 100.0)
        + flat_bonus_health as f32)
        .ceil() as i32
}
