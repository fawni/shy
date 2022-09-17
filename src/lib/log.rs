use owo_colors::OwoColorize;

pub(crate) fn info(text: impl ToString) {
    println!(
        "{}{}{} {}",
        "(".bright_black(),
        "+".green(),
        ")".bright_black(),
        text.to_string(),
    );
}
