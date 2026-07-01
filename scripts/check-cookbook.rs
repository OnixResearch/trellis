#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Deterministic cookbook documentation checker.
//!
//! The pure core validates recipe metadata and local Markdown links. The shell
//! owns filesystem reads and path-existence checks.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const COOKBOOK_PATH: &str = "docs/cookbook.md";
const DOCS_PREFIX: &str = "docs/";
const PARENT_PREFIX: &str = "../";
const RECIPE_PREFIX: &str = "## Recipe:";
const HTTP_PREFIX: &str = "http://";
const HTTPS_PREFIX: &str = "https://";
const ANCHOR_PREFIX: &str = "#";
const REQUIRED_FIELDS: &[&str] = &[
    "Goal:",
    "Files:",
    "Commands:",
    "Success:",
    "Failure mode:",
    "Snippet status:",
];

#[derive(Debug, Eq, PartialEq)]
struct CookbookCheck {
    recipe_count: usize,
    links: BTreeSet<String>,
    errors: Vec<String>,
}

fn recipe_sections(markdown: &str) -> Vec<(String, String)> {
    let mut recipes = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_body: Vec<String> = Vec::new();

    for line in markdown.lines() {
        if line.starts_with(RECIPE_PREFIX) {
            if let Some(heading) = current_heading.take() {
                recipes.push((heading, current_body.join("\n")));
                current_body.clear();
            }
            current_heading = Some(line.trim().to_string());
            continue;
        }
        if current_heading.is_some() && line.starts_with("## ") {
            let heading = current_heading.take().expect("heading exists when recipe is open");
            recipes.push((heading, current_body.join("\n")));
            current_body.clear();
            continue;
        }
        if current_heading.is_some() {
            current_body.push(line.to_string());
        }
    }

    if let Some(heading) = current_heading {
        recipes.push((heading, current_body.join("\n")));
    }

    recipes
}

fn normalize_local_href(href: &str) -> Option<String> {
    let without_fragment = href.split_once('#').map_or(href, |(path, _fragment)| path);
    if without_fragment.is_empty()
        || without_fragment.starts_with(HTTP_PREFIX)
        || without_fragment.starts_with(HTTPS_PREFIX)
        || without_fragment.starts_with(ANCHOR_PREFIX)
    {
        return None;
    }
    if let Some(path) = without_fragment.strip_prefix(PARENT_PREFIX) {
        return Some(path.to_string());
    }
    Some(format!("{DOCS_PREFIX}{without_fragment}"))
}

fn extract_local_links(markdown: &str) -> BTreeSet<String> {
    let mut links = BTreeSet::new();
    let mut rest = markdown;
    while let Some(open_index) = rest.find("](") {
        rest = &rest[open_index + "](".len()..];
        let Some(close_index) = rest.find(')') else {
            break;
        };
        let href = &rest[..close_index];
        if let Some(path) = normalize_local_href(href) {
            links.insert(path);
        }
        rest = &rest[close_index + ")".len()..];
    }
    links
}

fn check_cookbook(markdown: &str, existing_paths: &BTreeSet<String>) -> CookbookCheck {
    let recipes = recipe_sections(markdown);
    let links = extract_local_links(markdown);
    let mut errors = Vec::new();

    if recipes.is_empty() {
        errors.push(format!("{COOKBOOK_PATH} must contain at least one `{RECIPE_PREFIX}` section"));
    }

    for (heading, body) in &recipes {
        for field in REQUIRED_FIELDS {
            if !body.contains(field) {
                errors.push(format!("{heading} is missing required field `{field}`"));
            }
        }
        if body.contains("Snippet status:") && !body.contains("illustrative") && !body.contains("checked") {
            errors.push(format!("{heading} snippet status must say whether snippets are illustrative or checked"));
        }
    }

    for path in &links {
        if !existing_paths.contains(path) {
            errors.push(format!("cookbook link `{path}` does not exist"));
        }
    }

    CookbookCheck {
        recipe_count: recipes.len(),
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
        _ => Err("usage: check-cookbook.rs [--self-test|ROOT]".to_string()),
    }
}

fn collect_existing_paths(root: &Path, links: &BTreeSet<String>) -> BTreeSet<String> {
    links.iter().filter(|path| root.join(path).exists()).cloned().collect()
}

fn run(root: &Path) -> Result<CookbookCheck, String> {
    let cookbook =
        fs::read_to_string(root.join(COOKBOOK_PATH)).map_err(|err| format!("failed to read {COOKBOOK_PATH}: {err}"))?;
    let links = extract_local_links(&cookbook);
    let existing = collect_existing_paths(root, &links);
    Ok(check_cookbook(&cookbook, &existing))
}

fn complete_fixture(link: &str) -> String {
    format!(
        "# Cookbook\n\n## Recipe: Use a helper\n\nGoal: Use it.\n\nFiles: [{link}]({link})\n\nCommands:\n\n```bash\ncargo test\n```\n\nSuccess: The command passes.\n\nFailure mode: Missing link fails.\n\nSnippet status: illustrative.\n"
    )
}

fn self_test() -> Result<(), String> {
    let mut existing = BTreeSet::new();
    existing.insert("docs/module-guide.md".to_string());

    let positive = check_cookbook(&complete_fixture("module-guide.md"), &existing);
    if !positive.errors.is_empty() {
        return Err(format!("positive fixture failed: {}", positive.errors.join("; ")));
    }

    let missing_field = complete_fixture("module-guide.md").replace("Failure mode:", "Failure:");
    let missing_field_check = check_cookbook(&missing_field, &existing);
    if !missing_field_check.errors.iter().any(|err| err.contains("Failure mode:")) {
        return Err(format!("missing-field fixture missed expected error: {}", missing_field_check.errors.join("; ")));
    }

    let stale_link = check_cookbook(&complete_fixture("missing.md"), &existing);
    if !stale_link.errors.iter().any(|err| err.contains("does not exist")) {
        return Err(format!("stale-link fixture missed expected error: {}", stale_link.errors.join("; ")));
    }

    Ok(())
}

fn main() -> ExitCode {
    match repo_root_from_args() {
        Ok(None) => match self_test() {
            Ok(()) => {
                println!("✓ cookbook checker self-test passed");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("✗ cookbook checker self-test failed: {err}");
                ExitCode::FAILURE
            }
        },
        Ok(Some(root)) => match run(&root) {
            Ok(check) if check.errors.is_empty() => {
                println!(
                    "✓ cookbook contains {} recipes and {} checked local links",
                    check.recipe_count,
                    check.links.len()
                );
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
