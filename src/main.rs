
use std::env;
use crate::cli::{ColorWhen, Opts};

#[cfg(not(windows))]

/// CLI command line options
/// This program will be used for Rust utilities. 
/// The first command will be to create labels on github.
/// CLI command line options
#[derive(Debug, StructOpt)]
#[structopt(name = "gristle")]

pub struct Opts {
    /// Github API token
    #[structopt(short = "t", long = "token", env = "GITHUB_TOKEN")]
    pub token: String,

    /// Github repository
    #[structopt(short = "r", long = "repo")]
    pub repo: String,

    /// JSON file containing labels
    #[structopt(short = "l", long = "labels")]
    pub labels: String,

    /// Delete existing labels
    #[structopt(short = "d", long = "delete")]
    pub delete: bool,

    /// Colorize output
    #[structopt(short = "c", long = "color", default_value = "auto")]
    pub color: ColorWhen,
}


let ansi_colors_support = true;

let interactive_terminal = std::io::stdout().is_terminal();

let colored_output = match opts.color {
    ColorWhen::Always => true,
    ColorWhen::Never => false,
    ColorWhen::Auto => {
        ansi_colors_support && env::var_os("NO_COLOR").is_none() && interactive_terminal
    }
};




