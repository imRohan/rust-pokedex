use crate::pokemon;
use crate::data;

fn all() -> std::vec::IntoIter<pokemon::Pokemon> {
    return data::pokemons().into_iter();
}

fn filter_by_name(name: &String, search_terms: &str) -> bool {
    name.to_lowercase()
        .as_str()
        .contains(search_terms.to_lowercase().as_str())
}

pub fn find(search_terms: &str) -> Vec<pokemon::Pokemon> {
    all()
        .filter(|p| filter_by_name(&p.name, search_terms))
        .collect()
}
