use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub minimum: String,
    pub maximum: String,
}

#[derive(Serialize, Deserialize)]
pub struct References {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub r#type: String,
    pub damage: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Attacks {
    pub fast: Vec<Attack>,
    pub special: Vec<Attack>,
}

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub id: String,
    pub name: String,
    pub classification: String,
    pub types: Vec<String>,
    pub resistant: Vec<String>,
    pub weaknesses: Vec<String>,
    pub weight: Range,
    pub height: Range,
    pub fleeRate: f32,
    pub evolutions: Option<Vec<References>>,
    pub maxCP: i32,
    pub maxHP: i32,
    pub attacks: Attacks,
}
