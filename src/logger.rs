use colored::Colorize;

pub fn info(str: String, additional: String) {
    let output = format!(
        "{} {} {} {}",
        "[ Violin ]".bright_magenta(),
        "[ INFO ]".bright_blue(),
        additional.bright_blue(),
        str
    );
    println!("{}", output.as_str());
}

pub fn error(str: String, additional: String) {
    let output = format!(
        "{} {} {} {}",
        "[ Violin ]".bright_magenta(),
        "[ ERROR ]".bright_red(),
        additional.bright_red(),
        str
    );
    println!("{}", output.as_str());
}
