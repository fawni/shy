use owo_colors::OwoColorize;

pub fn info(text: impl ToString) {
    println!(
        "{}{}{} {}",
        "(".bright_black(),
        "+".green(),
        ")".bright_black(),
        text.to_string(),
    );
}
