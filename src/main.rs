use std::{ffi::OsStr, path::Path};

use grep::{
    regex::RegexMatcher,
    searcher::{sinks::UTF8, Searcher, SearcherBuilder},
};

fn main() {
    let dir = "../rust";
    let search = "sort_suggested";

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

    searcher.search_slice(
        matcher.clone(),
        "
          #[test]
                fn sort_suggested_structs_by_types() {
        check(
            r#
        "
        .as_bytes(),
        UTF8(|_, line| {
            println!("line: {}", line);
            Ok(true)
        }),
    );

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
                    fuzzywuzzy::fuzz::ratio(search, name),
                    name.to_string(),
                    f.clone(),
                ));

                Ok(true)
            }),
        );
    }
    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("results: {:#?}", scores.split_at(5).0);
}
