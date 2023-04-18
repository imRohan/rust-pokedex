use crate::global;
use crate::data;

fn all() -> std::vec::IntoIter<global::Pokemon> {
    return data::pokemons().into_iter();
}

pub fn find(name: &str) -> global::Pokemon {
    let mut pokemons: Vec<global::Pokemon> = all()
        .filter(|p| p.name.to_lowercase() == name.to_lowercase())
        .collect();
    return pokemons.remove(0)
}
