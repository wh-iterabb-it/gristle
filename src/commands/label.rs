use std::error::Error;
use std::fs::File;
use std::io::Read;

use clap::ArgMatches;
use reqwest;

use crate::cli::Opts;

/// struct for the label command
pub struct GithubLabelMaker<'a> {
    client: &'a reqwest::Client,
    opts: &'a Opts,
}

impl<'a> GithubLabelMaker<'a> {
    /// create a new label command
    pub fn new(client: &'a reqwest::Client, opts: &'a Opts) -> Self {
        Self { client, opts }
    }

    /// run the label command
    pub fn run(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        let repo = matches.value_of("repo").unwrap();
        let labels = read_labels(matches.value_of("labels").unwrap())?;
        if matches.is_present("delete") {
            delete_labels(self.client, repo, &labels)?;
        } else {
            make_labels(self.client, repo, &labels)?;
        }
        Ok(())
    }
}

/// struct for a label
#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    name: String,
    color: String,
    description: Option<String>,
}

/// read labels from a json file
fn read_labels(path: &str) -> Result<Vec<Label>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let labels = serde_json::from_str(&contents)?;
    Ok(labels)
}

/// Makes an http request to github to make labels
fn make_labels(
    client: &reqwest::Client,
    repo: &str,
    labels: &[Label],
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/labels", repo);
    let mut response = client.post(&url).json(labels).send()?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("{}: {}", response.status(), body).into())
    }
}

/// Makes an http request to github to delete labels
fn delete_labels(
    client: &reqwest::Client,
    repo: &str,
    labels: &[Label],
) -> Result<(), Box<dyn Error>> {
    for label in labels {
        let url = format!("https://api.github.com/repos/{}/labels/{}", repo, label.name);
        let mut response = client.delete(&url).send()?;
        let mut body = String::new();
        response.read_to_string(&mut body)?;
        if !response.status().is_success() {
            return Err(format!("{}: {}", response.status(), body).into());
        }
    }
    Ok(())
}

