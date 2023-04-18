use cursive::views::{Dialog};
use cursive;


mod pokemon;
mod data;
mod pokemon_finder;
mod search;
mod show;

use search::input;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(input())
            .title("Pokédex")
    );
    siv.run();
}
