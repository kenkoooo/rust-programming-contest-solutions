use serde::Deserialize;
use serde_json;
use std::collections::BTreeMap;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

fn main() {
    let mut writer = BufWriter::new(File::create("../README.md").unwrap());
    write_atcoder(&mut writer).unwrap();
}

fn write_atcoder<W>(writer: &mut W) -> Result<(), Box<std::error::Error>>
where
    W: std::io::Write,
{
    let mut file = File::open("../problems.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let problems: Vec<AtCoderProblem> = serde_json::from_str(&contents)?;
    let map = problems
        .iter()
        .map(|p| (p.id.as_str(), p))
        .collect::<BTreeMap<_, _>>();

    let entries = fs::read_dir("../atcoder/")?
        .flat_map(|entry| entry)
        .flat_map(|path| {
            path.path()
                .file_stem()
                .and_then(|file_stem| file_stem.to_str())
                .and_then(|file_stem| map.get(file_stem))
        })
        .map(|problem| {
            let mut filepath = PathBuf::from("./atcoder/");
            filepath.push(problem.id.as_str());
            filepath.set_extension("rs");
            let url = format!(
                "https://atcoder.jp/contests/{}/tasks/{}",
                problem.contest_id, problem.id
            );

            ProblemEntry {
                title: problem.title.clone(),
                url,
                filepath,
            }
        })
        .collect::<Vec<_>>();

    writeln!(writer, "# AtCoder");
    writeln!(writer, "| Problem | Solution |");
    writeln!(writer, "|:---|:---|");
    for entry in entries.into_iter() {
        let filepath = entry.filepath;
        writeln!(
            writer,
            "| [{title}]({url}) | [{filepath}]({filepath}) |",
            title = entry.title,
            url = entry.url,
            filepath = filepath.to_string_lossy().to_string(),
        );
    }
    Ok(())
}

struct ProblemEntry {
    title: String,
    url: String,
    filepath: PathBuf,
}

#[derive(Deserialize)]
struct AtCoderProblem {
    contest_id: String,
    id: String,
    title: String,
}
