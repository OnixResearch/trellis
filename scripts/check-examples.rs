#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic example documentation checker.
//!
//! The pure core checks that documented example links exist and that each
//! checked-in example has a matching `cargo run --example <name>` command in
//! README/module-guide text. Cargo compilation is handled separately by
//! `cargo check --examples`.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const EXAMPLES_DIR: &str = "examples";
const EXAMPLE_PREFIX: &str = "examples/";
const DOC_EXAMPLE_PREFIX: &str = "../examples/";
const RUST_EXTENSION: &str = ".rs";
const README_PATH: &str = "README.md";
const MODULE_GUIDE_PATH: &str = "docs/module-guide.md";
const COMMAND_PREFIX: &str = "cargo run --example ";
const IDENT_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[derive(Debug, Eq, PartialEq)]
struct ExampleCheck {
    examples: BTreeSet<String>,
    links: BTreeSet<String>,
    errors: Vec<String>,
}

fn is_ident(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|ch| IDENT_CHARS.contains(ch))
}

fn example_name(path: &str) -> Option<String> {
    let relative = path.strip_prefix(EXAMPLE_PREFIX)?;
    let name = relative.strip_suffix(RUST_EXTENSION)?;
    if is_ident(name) { Some(name.to_string()) } else { None }
}

fn normalize_example_href(href: &str) -> Option<String> {
    let without_fragment = href.split_once('#').map_or(href, |(path, _fragment)| path);
    let source_path = without_fragment
        .strip_prefix(DOC_EXAMPLE_PREFIX)
        .or_else(|| without_fragment.strip_prefix(EXAMPLE_PREFIX))?;
    if !source_path.ends_with(RUST_EXTENSION) || source_path.contains('/') {
        return None;
    }
    Some(format!("{EXAMPLE_PREFIX}{source_path}"))
}

fn extract_example_links(markdown: &str) -> BTreeSet<String> {
    let mut links = BTreeSet::new();
    let mut rest = markdown;
    while let Some(open_index) = rest.find("](") {
        rest = &rest[open_index + "](".len()..];
        let Some(close_index) = rest.find(')') else {
            break;
        };
        let href = &rest[..close_index];
        if let Some(path) = normalize_example_href(href) {
            links.insert(path);
        }
        rest = &rest[close_index + ")".len()..];
    }
    links
}

fn command_name_from(rest: &str) -> String {
    rest.chars().take_while(|ch| IDENT_CHARS.contains(*ch)).collect()
}

fn documented_commands(markdown: &str) -> BTreeSet<String> {
    let mut commands = BTreeSet::new();
    for line in markdown.lines() {
        let mut rest = line;
        while let Some(index) = rest.find(COMMAND_PREFIX) {
            rest = &rest[index + COMMAND_PREFIX.len()..];
            let name = command_name_from(rest);
            if is_ident(&name) {
                commands.insert(name);
            }
        }
    }
    commands
}

fn check_examples(example_paths: &BTreeSet<String>, readme: &str, module_guide: &str) -> ExampleCheck {
    let mut links = extract_example_links(readme);
    links.extend(extract_example_links(module_guide));
    let mut commands = documented_commands(readme);
    commands.extend(documented_commands(module_guide));
    let mut errors = Vec::new();

    if example_paths.is_empty() {
        errors.push(format!("{EXAMPLES_DIR}/ must contain at least one runnable example"));
    }

    for link in &links {
        if !example_paths.contains(link) {
            errors.push(format!("documented example link `{link}` does not exist"));
        }
    }

    for path in example_paths {
        if !links.contains(path) {
            errors.push(format!("example `{path}` is not linked from README or module guide"));
        }
        let Some(name) = example_name(path) else {
            errors.push(format!("example path `{path}` is not a flat Rust example"));
            continue;
        };
        if !commands.contains(&name) {
            errors.push(format!("example `{path}` is missing `cargo run --example {name}` documentation"));
        }
    }

    ExampleCheck {
        examples: example_paths.clone(),
        links,
        errors,
    }
}

fn repo_root_from_args() -> Result<Option<PathBuf>, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(Some(PathBuf::from("."))),
        [flag] if flag == "--self-test" => Ok(None),
        [root] => Ok(Some(PathBuf::from(root))),
        _ => Err("usage: check-examples.rs [--self-test|ROOT]".to_string()),
    }
}

fn collect_example_paths(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut paths = BTreeSet::new();
    let dir = root.join(EXAMPLES_DIR);
    if !dir.exists() {
        return Ok(paths);
    }
    let entries = fs::read_dir(&dir).map_err(|err| format!("failed to read {EXAMPLES_DIR}/: {err}"))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read an examples/ entry: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            paths.insert(format!("{EXAMPLE_PREFIX}{file_name}"));
        }
    }
    Ok(paths)
}

fn run(root: &Path) -> Result<ExampleCheck, String> {
    let examples = collect_example_paths(root)?;
    let readme =
        fs::read_to_string(root.join(README_PATH)).map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    let guide = fs::read_to_string(root.join(MODULE_GUIDE_PATH))
        .map_err(|err| format!("failed to read {MODULE_GUIDE_PATH}: {err}"))?;
    Ok(check_examples(&examples, &readme, &guide))
}

fn docs_for(example_name: &str) -> String {
    format!("See [`{example_name}`](examples/{example_name}.rs).\n\n```bash\ncargo run --example {example_name}\n```\n")
}

fn self_test() -> Result<(), String> {
    let mut examples = BTreeSet::new();
    examples.insert("examples/core_basics.rs".to_string());

    let positive = check_examples(&examples, &docs_for("core_basics"), "");
    if !positive.errors.is_empty() {
        return Err(format!("positive fixture failed: {}", positive.errors.join("; ")));
    }

    let missing_link = check_examples(&examples, "cargo run --example core_basics", "");
    if !missing_link.errors.iter().any(|err| err.contains("not linked")) {
        return Err(format!("missing-link fixture missed expected error: {}", missing_link.errors.join("; ")));
    }

    let missing_command = check_examples(&examples, "[`core`](examples/core_basics.rs)", "");
    if !missing_command.errors.iter().any(|err| err.contains("missing `cargo run")) {
        return Err(format!("missing-command fixture missed expected error: {}", missing_command.errors.join("; ")));
    }

    Ok(())
}

fn main() -> ExitCode {
    match repo_root_from_args() {
        Ok(None) => match self_test() {
            Ok(()) => {
                println!("✓ examples checker self-test passed");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("✗ examples checker self-test failed: {err}");
                ExitCode::FAILURE
            }
        },
        Ok(Some(root)) => match run(&root) {
            Ok(check) if check.errors.is_empty() => {
                println!("✓ {} examples are linked with run commands", check.examples.len());
                ExitCode::SUCCESS
            }
            Ok(check) => {
                for error in check.errors {
                    eprintln!("✗ {error}");
                }
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("✗ {err}");
                ExitCode::FAILURE
            }
        },
        Err(usage) => {
            eprintln!("{usage}");
            ExitCode::FAILURE
        }
    }
}
