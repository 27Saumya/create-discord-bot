use crate::create::*;
use dialoguer::{Select, theme::ColorfulTheme};
use colored::{Colorize, ColoredString};

pub fn select_options() {
    let languages = vec![
        "Python - discord.py",
        "Python - disnake (fork of discord.py)",
        "JavaScript - discord.js",
        "Rust - serenity"
    ];
    let cogs = vec!["Yes", "No"];

    let language = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a programming language:")
        .items(&languages)
        .interact()
        .expect("Failed to get user input");

    let cog = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose whether you would like to use cogs/extensions (Recommended)")
        .items(&cogs)
        .default(0)
        .interact()
        .expect("Failed to get user input");

    match language {
        0 => generate_python_boilerplate(cog),
        1 => generate_python_disnake_boilerplate(cog),
        2 => generate_js_boilerplate(cog),
        3 => generate_rust_boilerplate(cog),
        _ => println!("Invalid selection"),
    }
}

pub fn error_message(message: &str) -> ColoredString {
    let printable_message = message.red();
    return printable_message;
}

pub fn success_message(message: &str) -> ColoredString {
    let printable_message = message.green();
    return printable_message;
}