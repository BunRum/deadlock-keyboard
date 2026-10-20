use serde::{Deserialize};

#[derive(Deserialize, Default)]
pub struct ColorsFile {
    pub colors: Vec<HeroColor>,
}

#[derive(Deserialize)]
pub struct HeroColor(pub String, pub [u8; 3]);