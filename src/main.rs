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
    println!("Found {name} - id: {}, weight: {}", pokemon.id, pokemon.weight.minimum)
}
