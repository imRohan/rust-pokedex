use crate::global;
use crate::data;

fn all() -> std::vec::IntoIter<global::Pokemon> {
    return data::pokemons().into_iter();
}

pub fn find(name: &str) -> global::Pokemon {
    let pokemons: Vec<global::Pokemon> = all()
        .filter(|p| p.name.to_lowercase() == name.to_lowercase())
        .collect();
    let found = &pokemons[0];
    return global::Pokemon {
        id: found.id.clone(),
        name: found.name.clone(),
        classification: found.classification.clone(),
        types: found.types.clone(),
        resistant: found.resistant.clone(),
        weaknesses: found.weaknesses.clone(),
        weight: global::Range { 
            minimum: found.weight.minimum.clone(),
            maximum: found.weight.maximum.clone(),
        },
        height: global::Range { 
            minimum: found.height.minimum.clone(),
            maximum: found.height.maximum.clone(),
        },
        fleeRate: found.fleeRate.clone(),
        maxCP: found.maxCP.clone(),
        maxHP: found.maxHP.clone(),
    };
}
