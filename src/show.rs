use cursive::views::{Dialog, TextView, LinearLayout, ListView, PaddedView, SelectView};
use cursive::Cursive;

use crate::pokemon;
use crate::search;

pub fn pokemon(s: &mut Cursive, pokemon: &pokemon::Pokemon) {
    s.pop_layer();
    s.add_layer(
        Dialog::around(render_layout(pokemon))
            .title(pokemon.title())
            .button("Back to Search", search::show_input),
    );
}

fn render_layout(pokemon: &pokemon::Pokemon) -> LinearLayout {
    return LinearLayout::vertical()
        .child(description(pokemon))
        .child(stats(pokemon))
        .child(attacks(pokemon))
        .child(evolutions(pokemon))
        .child(previous_evolutions(pokemon))
}

fn description(pokemon: &pokemon::Pokemon) -> PaddedView<TextView> {
    let description = &pokemon.description();
    return PaddedView::lrtb(1, 1, 1, 0, TextView::new(description))
}

fn stats(pokemon: &pokemon::Pokemon) -> Dialog {
    let content = LinearLayout::horizontal()
                    .child(details(pokemon))
                    .child(base_stats(pokemon));
    return Dialog::around(content).title("Details")
}

fn details(pokemon: &pokemon::Pokemon) -> PaddedView<LinearLayout> {
    let layout = LinearLayout::vertical()
                    .child(
                        array_of_strings("Types", &pokemon.types)
                    )
                    .child(
                        array_of_strings("Resistant", &pokemon.resistant)
                    )
                    .child(
                        array_of_strings("Weaknesses", &pokemon.weaknesses)
                    );
    return PaddedView::lrtb(0, 0, 0, 0, layout)
}

fn base_stats(pokemon: &pokemon::Pokemon) -> PaddedView<LinearLayout> {
    let layout = LinearLayout::vertical()
                    .child(
                        range("Weight", &pokemon.weight)
                    )
                    .child(
                        range("Height", &pokemon.height)
                    )
                    .child(
                        float("Flee Rate", &pokemon.flee_rate)
                    )
                    .child(
                        integer("Max CP", &pokemon.max_cp)
                    )
                    .child(
                        integer("Max HP", &pokemon.max_hp)
                    );
    return PaddedView::lrtb(2, 0, 0, 0, layout)
}

fn attacks(pokemon: &pokemon::Pokemon) -> Dialog {
    let count = pokemon.attack_count();
    let title = format!("{count} Attacks");
    let content = LinearLayout::horizontal()
        .child(
            array_of_strings("Fast", &pokemon.fast_attacks())
        )
        .child(
            array_of_strings("Special", &pokemon.special_attacks())
        );
    return Dialog::around(content).title(title)
}

fn evolutions(pokemon: &pokemon::Pokemon) -> Dialog {
    let names = pokemon.evolution_names();
    return render_evolutions("Next Evolutions", names)
}

fn previous_evolutions(pokemon: &pokemon::Pokemon) -> Dialog {
    let names = pokemon.previous_evolution_names();
    return render_evolutions("Previous Evolutions", names)
}

fn render_evolutions(title: &str, names: Option<Vec<String>>) -> Dialog {
    let mut select = SelectView::new()
        .autojump();
    match names {
        Some(names) => {
            select.add_all_str(names);
            select.set_on_submit(search::submit);
            return Dialog::around(select).title(title)
        }
        None => return Dialog::around(TextView::new("None")).title(title)
    }
}

fn range(title: &str, range: &pokemon::Range) -> PaddedView<TextView> {
    let min_value = &range.minimum;
    let max_value = &range.maximum;
    let content = format!("{title}: {min_value} - {max_value}");
    return PaddedView::lrtb(0, 0, 1, 0, TextView::new(content))
}

fn array_of_strings(title: &str, array: &Vec<String>) -> PaddedView<LinearLayout> {
    let mut list = ListView::new();
    array.into_iter().for_each (|i| list.add_child("", TextView::new(i)));
    let layout = LinearLayout::horizontal()
        .child(TextView::new(format!("{title}:")))
        .child(list);
    return PaddedView::lrtb(1, 0, 1, 0, layout)
}

fn integer(title: &str, value: &i32) -> PaddedView<TextView> {
    let content = format!("{title}: {value}");
    return PaddedView::lrtb(0, 0, 1, 0, TextView::new(content))
}

fn float(title: &str, value: &f32) -> PaddedView<TextView> {
    let content = format!("{title}: {value}");
    return PaddedView::lrtb(0, 0, 1, 0, TextView::new(content))
}
