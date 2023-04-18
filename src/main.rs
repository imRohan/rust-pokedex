use clap::Parser;

mod global;
mod data;
mod pokemon;

#[derive(Parser)]
struct Input {
    // The name of the Pokemon
    name: String,
}

fn main() {
    let args = Input::parse();
    let name = args.name;
    let pokemon: global::Pokemon = pokemon::find(&name);
    let id = &pokemon.id;
    let weight_min = &pokemon.weight.minimum;
    let fast_count = &pokemon.attacks.fast.len();
    let special_count = &pokemon.attacks.special.len();
    let attack_count = fast_count + special_count;
    println!("Found {name} - id: {}, weight: {}, attacks: {}, fast: {}, special: {}", id, weight_min, attack_count, fast_count, special_count);
}
