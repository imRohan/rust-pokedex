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
    #[serde(rename = "fleeRate")]
    pub flee_rate: f32,
    pub evolutions: Option<Vec<References>>,
    #[serde(rename = "Previous evolution(s)")]
    pub previous_evolutions: Option<Vec<References>>,
    #[serde(rename = "maxCP")]
    pub max_cp: i32,
    #[serde(rename = "maxHP")]
    pub max_hp: i32,
    pub attacks: Attacks,
}

impl Pokemon {
    pub fn fast_attacks(&self) -> Vec<String> {
        return Pokemon::format_attacks(&self.attacks.fast)
    }

    pub fn special_attacks(&self) -> Vec<String> {
        return Pokemon::format_attacks(&self.attacks.special)
    }

    fn format_attacks(attacks: &Vec<Attack>) -> Vec<String> {
        return attacks.iter().map(|a| {
            let name = a.name.clone();
            let attack_type = a.r#type.clone();
            let damage = a.damage.clone();
            return format!("[{attack_type}] {name} - {damage}dmg")
        }).collect()
    }

    pub fn attack_count(&self) -> i32 {
        let total_fast = self.attacks.fast.len();
        let total_special = self.attacks.special.len();
        return (total_fast + total_special) as i32
    }

    pub fn evolution_names(&self) -> Option<Vec<String>> {
        match &self.evolutions {
            Some(references) => Some(references.iter().map(|r| r.name.clone()).collect()),
            None => None
        }
    }

    pub fn previous_evolution_names(&self) -> Option<Vec<String>> {
        match &self.previous_evolutions {
            Some(references) => Some(references.iter().map(|r| r.name.clone()).collect()),
            None => None
        }
    }

    pub fn title(&self) -> String {
        let name = &self.name;
        let id = &self.id;
        return format!("{name} - {id}")
    }

    pub fn description(&self) -> String {
        let name = &self.name;
        let id = &self.id;
        let pokemon_type= &self.types[0];
        let classification = &self.classification;
        return format!("{name} is a {pokemon_type} {classification} with pokedex number {id}")
    }
}
