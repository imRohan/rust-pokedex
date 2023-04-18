use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, EditView, SelectView, TextView};
use cursive::Cursive;

use crate::pokemon_finder;
use crate::pokemon;
use crate::show;

pub fn show_input(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(input())
            .title("Pokédex")
    );
}

pub fn input() -> LinearLayout {
    let tutorial_text = "Search for a Pokemon by name";
    return LinearLayout::vertical()
        .child(
            TextView::new(tutorial_text)
        )
        .child(
            Dialog::around(
                EditView::new()
                    .on_submit(submit)
                    .with_name("name")
                )
                .padding_lrtb(1, 1, 0, 0)
        );
}

pub fn submit(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        show_error(s, "Please enter a name");
    } else {
        show_results(s, name);
    }
}

fn show_error(s: &mut Cursive, message: &str) {
    s.add_layer(Dialog::info(message));
}

fn show_results(s: &mut Cursive, name: &str) {
    let pokemon: Vec<pokemon::Pokemon> = pokemon_finder::find(&name);
    if pokemon.is_empty() {
        show_error(s, "No Pokemon Found");
    } else if pokemon.len() == 1 {
        show::pokemon(s, &pokemon[0]);
    } else {
        show_table(s, pokemon);
    }
}

fn show_table(s: &mut Cursive, items: Vec<pokemon::Pokemon>) {
    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();
    let names = items.iter().map(|i| &i.name );
    let count = items.len();
    let title = format!("Found {count} Pokemon");
    select.add_all_str(names);
    select.set_on_submit(show_results);
    s.pop_layer();
    s.add_layer(
        Dialog::around(select.scrollable())
            .title(title)
            .button("Back", show_input),
    );
}
