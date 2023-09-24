use std::{
    cmp::min,
    collections::HashMap,
    ffi::OsStr,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use dialoguer::Select;
use grep::{
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder},
};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    test: String,
    term: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct CliData {
    prev_tests: HashMap<String, String>,
}

fn app_data_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "jmintb", "cargotst").map(|p| p.data_dir().to_path_buf())
}

fn app_data_file_path() -> Option<PathBuf> {
    app_data_path().map(|p| {
        let mut p = p;
        p.push("app_data.json");
        p
    })
}

impl CliData {
    fn init() -> Self {
        if let Ok(data) = serde_json::from_slice(
            std::fs::read(app_data_file_path().unwrap())
                .unwrap_or(vec![])
                .as_slice(),
        ) {
            data
        } else {
            Self::default()
        }
    }

    fn save_test(mut self, project_path: String, test_name: String) {
        self.prev_tests.insert(project_path, test_name);
        println!("dir: {:?}", app_data_file_path().unwrap());
        std::fs::create_dir_all(app_data_path().unwrap());

        std::fs::write(
            app_data_file_path().unwrap(),
            serde_json::to_string(&self).unwrap(),
        )
        .unwrap();
    }

    fn get_last_test(&self, project_path: String) -> Option<String> {
        println!("last path: {project_path}, {:?}", self.prev_tests);
        self.prev_tests.get(&project_path).cloned()
    }
}

fn exec_test(name: String) {
    let mut cmd = process::Command::new("cargo");
    cmd.arg("test").arg(name);

    cmd.exec();
}

fn main() {
    let cli_data = CliData::init();
    let wd = std::env::current_dir().unwrap();
    let wd = wd.file_name().unwrap().to_str().unwrap();
    let Cli {
        term: Some(term), ..
    } = Cli::parse()
    else {
        exec_test(cli_data.get_last_test(wd.to_string()).unwrap());
        return;
    };

    let dir = Path::new(".");
    let walker = walkdir::WalkDir::new(dir);
    let mut files: Vec<String> = vec![];
    for entry in walker {
        if let Ok(entry) = entry {
            let p = entry.path();
            if p.is_file() && p.extension() == Some(OsStr::new("rs")) {
                files.push(p.to_str().unwrap().to_owned());
            }
        }
    }

    let matcher = RegexMatcher::new(r"\#\[test\]\n\s*fn\s+.+\(").unwrap();
    let mut searcher = SearcherBuilder::new().multi_line(true).build();

    let mut scores: Vec<(u8, String, String)> = Vec::new();

    for f in files {
        searcher.search_path(
            matcher.clone(),
            Path::new(&f),
            UTF8(|_, line| {
                let mut split = line.split("\n");
                split.next();
                let name = split.next().unwrap().split("(").next().unwrap();
                let name = name.replace("fn", "").to_string();
                let name = name.trim();

                scores.push((
                    fuzzywuzzy::fuzz::partial_ratio(&term, name),
                    name.to_string(),
                    f.clone(),
                ));

                Ok(true)
            }),
        );
    }

    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let selection = Select::new()
        .with_prompt("Choose a test")
        .items(
            &scores
                .split_at(min(5, scores.len()))
                .0
                .iter()
                .map(|v| format!("{} in {}", v.1.clone(), v.2.split("/").last().unwrap()))
                .collect::<Vec<String>>(),
        )
        .interact()
        .unwrap();

    cli_data.save_test(wd.to_string(), scores[selection].1.clone());

    let mut cmd = process::Command::new("cargo");
    cmd.arg("test").arg(scores[selection].1.clone());

    cmd.exec();
}
