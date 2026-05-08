#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
struct MarkerSpec {
    key: &'static str,
    text: &'static str,
    expected: usize,
}

const MARKERS: &[MarkerSpec] = &[
    MarkerSpec {
        key: "external_body",
        text: "#[verifier::external_body]",
        expected: 16,
    },
    MarkerSpec {
        key: "assume_specification",
        text: "assume_specification[",
        expected: 0,
    },
    MarkerSpec {
        key: "no_ensures",
        text: "@verified-logic:no-ensures",
        expected: 0,
    },
    MarkerSpec {
        key: "runtime_exempt",
        text: "@verified-logic:exempt",
        expected: 0,
    },
    MarkerSpec {
        key: "assume_call",
        text: "assume(",
        expected: 0,
    },
];

#[derive(Debug)]
struct Args {
    root: PathBuf,
    write_report: Option<PathBuf>,
    fixture_extra_external_body: bool,
}

#[derive(Debug)]
struct Hit {
    path: PathBuf,
    line: usize,
    line_text: String,
    next_fn: Option<String>,
}

#[derive(Debug)]
struct MarkerResult {
    spec: MarkerSpec,
    hits: Vec<Hit>,
}

fn parse_args() -> Result<Args, String> {
    let mut root = PathBuf::from(".");
    let mut write_report = None;
    let mut fixture_extra_external_body = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => {
                let value = args.next().ok_or("--root requires a path")?;
                root = PathBuf::from(value);
            }
            "--write-report" => {
                let value = args.next().ok_or("--write-report requires a path")?;
                write_report = Some(PathBuf::from(value));
            }
            "--fixture-extra-external-body" => fixture_extra_external_body = true,
            "--help" | "-h" => {
                println!("usage: check-proof-gap-inventory.rs [--root PATH] [--write-report PATH] [--fixture-extra-external-body]");
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Args {
        root,
        write_report,
        fixture_extra_external_body,
    })
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_rs_files(&path, out)?;
        } else if file_type.is_file() && path.extension() == Some(OsStr::new("rs")) {
            out.push(path);
        }
    }
    Ok(())
}

fn function_after(lines: &[&str], marker_index: usize) -> Option<String> {
    let end = usize::min(lines.len(), marker_index + 8);
    for line in lines.iter().take(end).skip(marker_index + 1) {
        let trimmed = line.trim();
        if trimmed.contains("fn ") {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn scan_marker(root: &Path, files: &[PathBuf], spec: MarkerSpec) -> io::Result<MarkerResult> {
    let mut hits = Vec::new();
    for path in files {
        let text = fs::read_to_string(path)?;
        let lines: Vec<&str> = text.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if line.contains(spec.text) {
                let relative = path.strip_prefix(root).unwrap_or(path).to_path_buf();
                hits.push(Hit {
                    path: relative,
                    line: idx + 1,
                    line_text: line.trim().to_string(),
                    next_fn: function_after(&lines, idx),
                });
            }
        }
    }
    hits.sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
    Ok(MarkerResult { spec, hits })
}

fn render_report(results: &[MarkerResult], fixture_extra_external_body: bool) -> String {
    let mut out = String::new();
    out.push_str("# Proof-Gap Inventory Report\n\n");
    out.push_str("Source root: `src/`\n\n");
    if fixture_extra_external_body {
        out.push_str("Fixture: simulated one extra `#[verifier::external_body]` marker.\n\n");
    }
    out.push_str("## Summary\n\n");
    out.push_str("| marker | expected | actual | status |\n");
    out.push_str("| --- | ---: | ---: | --- |\n");
    for result in results {
        let actual = actual_count(result, fixture_extra_external_body);
        let status = if actual == result.spec.expected { "ok" } else { "drift" };
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            result.spec.key, result.spec.expected, actual, status
        ));
    }
    out.push_str("\n## Hits\n\n");
    for result in results {
        out.push_str(&format!("### `{}`\n\n", result.spec.key));
        if result.hits.is_empty() {
            out.push_str("No hits.\n\n");
        } else {
            for hit in &result.hits {
                out.push_str(&format!(
                    "- `{}:{}` `{}`",
                    hit.path.display(), hit.line, hit.line_text
                ));
                if let Some(next_fn) = &hit.next_fn {
                    out.push_str(&format!(" → `{next_fn}`"));
                }
                out.push('\n');
            }
            out.push('\n');
        }
    }
    out
}

fn actual_count(result: &MarkerResult, fixture_extra_external_body: bool) -> usize {
    let fixture_extra = if fixture_extra_external_body && result.spec.key == "external_body" {
        1
    } else {
        0
    };
    result.hits.len() + fixture_extra
}

fn run(args: Args) -> Result<bool, String> {
    let src = args.root.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src, &mut files)
        .map_err(|err| format!("failed to scan {}: {err}", src.display()))?;
    files.sort();

    let mut results = Vec::new();
    for spec in MARKERS.iter().copied() {
        results.push(
            scan_marker(&args.root, &files, spec)
                .map_err(|err| format!("failed while scanning `{}`: {err}", spec.key))?,
        );
    }

    let report = render_report(&results, args.fixture_extra_external_body);
    if let Some(path) = args.write_report {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(&path, report.as_bytes())
            .map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    }

    print!("{report}");
    let ok = results
        .iter()
        .all(|result| actual_count(result, args.fixture_extra_external_body) == result.spec.expected);
    Ok(ok)
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(2);
        }
    };
    match run(args) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("proof-gap inventory drift detected");
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(2);
        }
    }
}
