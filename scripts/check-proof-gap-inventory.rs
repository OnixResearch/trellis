#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_MANIFEST: &str = "docs/proof-gap-sites.tsv";
const MANIFEST_HEADER: &str = "# verified-logic proof-gap-sites-v1";
const EXPECTED_EXTERNAL_BODY_COUNT: usize = 16;

#[derive(Clone, Copy, Debug)]
struct MarkerSpec {
    key: &'static str,
    text: &'static str,
    expected: usize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExpectedSite {
    path: String,
    next_fn: String,
    classification: String,
}

const MARKERS: &[MarkerSpec] = &[
    MarkerSpec {
        key: "external_body",
        text: "#[verifier::external_body]",
        expected: EXPECTED_EXTERNAL_BODY_COUNT,
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
    manifest: PathBuf,
    write_report: Option<PathBuf>,
    fixture_extra_external_body: bool,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
    fixture_stale_manifest: bool,
    fixture_missing_rationale: bool,
    fixture_blank_rationale: bool,
    fixture_malformed_rationale: bool,
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

#[derive(Debug)]
struct SiteCheck {
    expected: Vec<ExpectedSite>,
    actual: Vec<ExpectedSite>,
    missing: Vec<ExpectedSite>,
    unexpected: Vec<ExpectedSite>,
}

fn parse_args() -> Result<Args, String> {
    let mut root = PathBuf::from(".");
    let mut manifest = PathBuf::from(DEFAULT_MANIFEST);
    let mut write_report = None;
    let mut fixture_extra_external_body = false;
    let mut fixture_site_substitution = false;
    let mut fixture_classification_substitution = false;
    let mut fixture_stale_manifest = false;
    let mut fixture_missing_rationale = false;
    let mut fixture_blank_rationale = false;
    let mut fixture_malformed_rationale = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = PathBuf::from(args.next().ok_or("--root requires a path")?),
            "--manifest" => manifest = PathBuf::from(args.next().ok_or("--manifest requires a path")?),
            "--write-report" => {
                write_report = Some(PathBuf::from(args.next().ok_or("--write-report requires a path")?))
            }
            "--fixture-extra-external-body" => fixture_extra_external_body = true,
            "--fixture-site-substitution" => fixture_site_substitution = true,
            "--fixture-classification-substitution" => fixture_classification_substitution = true,
            "--fixture-stale-manifest" => fixture_stale_manifest = true,
            "--fixture-missing-rationale" => fixture_missing_rationale = true,
            "--fixture-blank-rationale" => fixture_blank_rationale = true,
            "--fixture-malformed-rationale" => fixture_malformed_rationale = true,
            "--help" | "-h" => {
                println!(
                    "usage: check-proof-gap-inventory.rs [--root PATH] [--manifest PATH] [--write-report PATH] [--fixture-extra-external-body] [--fixture-site-substitution] [--fixture-classification-substitution] [--fixture-stale-manifest] [--fixture-missing-rationale] [--fixture-blank-rationale] [--fixture-malformed-rationale]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Args {
        root,
        manifest,
        write_report,
        fixture_extra_external_body,
        fixture_site_substitution,
        fixture_classification_substitution,
        fixture_stale_manifest,
        fixture_missing_rationale,
        fixture_blank_rationale,
        fixture_malformed_rationale,
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

fn parse_manifest(
    root: &Path,
    manifest: &Path,
    fixture_stale_manifest: bool,
    fixture_missing_rationale: bool,
    fixture_blank_rationale: bool,
    fixture_malformed_rationale: bool,
) -> Result<Vec<ExpectedSite>, String> {
    let path = if manifest.is_absolute() {
        manifest.to_path_buf()
    } else {
        root.join(manifest)
    };
    let mut text = fs::read_to_string(&path).map_err(|err| format!("failed to read manifest {}: {err}", path.display()))?;
    if fixture_missing_rationale || fixture_blank_rationale || fixture_malformed_rationale {
        text = apply_rationale_fixture(&text, fixture_missing_rationale, fixture_blank_rationale, fixture_malformed_rationale)?;
    }
    let mut lines = text.lines();
    let header = lines.next().ok_or("manifest is empty")?;
    if header.trim_end() != MANIFEST_HEADER {
        return Err(format!("manifest malformed: first line must be `{MANIFEST_HEADER}`"));
    }
    let mut sites = Vec::new();
    for (idx, line) in text.lines().enumerate().skip(1) {
        let trimmed = line.trim_end();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = trimmed.split('\t').collect();
        if fields.len() != 4 {
            return Err(format!("manifest malformed at line {}: expected 4 tab-separated fields: path, function signature, classification, rationale", idx + 1));
        }
        if fields[..3].iter().any(|field| field.trim().is_empty()) {
            return Err(format!("manifest malformed at line {}: path, function signature, and classification must be non-empty", idx + 1));
        }
        if fields[3].trim().is_empty() {
            return Err(format!("manifest malformed at line {}: rationale must be non-empty", idx + 1));
        }
        sites.push(ExpectedSite {
            path: fields[0].to_string(),
            next_fn: fields[1].to_string(),
            classification: fields[2].to_string(),
        });
    }
    if fixture_stale_manifest && !sites.is_empty() {
        sites.remove(0);
        sites.push(ExpectedSite {
            path: "src/stale_manifest_boundary.rs".to_string(),
            next_fn: "pub fn stale_manifest_boundary(".to_string(),
            classification: "permanent-trust-boundary".to_string(),
        });
        sites.sort();
    }
    if sites.len() != EXPECTED_EXTERNAL_BODY_COUNT {
        return Err(format!(
            "manifest malformed: expected {EXPECTED_EXTERNAL_BODY_COUNT} sites, found {}",
            sites.len()
        ));
    }
    let mut sorted = sites.clone();
    sorted.sort();
    if sites != sorted {
        return Err("manifest malformed: site entries must be sorted by path, function, classification".to_string());
    }
    sorted.dedup();
    if sorted.len() != sites.len() {
        return Err("manifest malformed: duplicate site entries are not allowed".to_string());
    }
    for site in &sites {
        if site.classification != "permanent-trust-boundary" && site.classification != "closeable-structural-candidate"
        {
            return Err(format!(
                "manifest malformed: unsupported classification `{}` for {}",
                site.classification, site.path
            ));
        }
    }
    Ok(sites)
}

fn apply_rationale_fixture(
    text: &str,
    missing: bool,
    blank: bool,
    malformed: bool,
) -> Result<String, String> {
    let mut out = Vec::new();
    let mut changed = false;
    for (idx, line) in text.lines().enumerate() {
        if idx == 0 {
            out.push(line.to_string());
            continue;
        }
        if !changed && !line.trim_end().is_empty() && !line.starts_with('#') {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() != 4 {
                return Err("rationale fixture requires a valid four-field manifest row".to_string());
            }
            if missing {
                out.push(fields[..3].join("\t"));
            } else if blank {
                out.push(format!("{}\t", fields[..3].join("\t")));
            } else if malformed {
                out.push(format!("{}\t{}\textra-field", fields[..3].join("\t"), fields[3]));
            }
            changed = true;
        } else {
            out.push(line.to_string());
        }
    }
    if !changed {
        return Err("rationale fixture could not find a manifest row to mutate".to_string());
    }
    Ok(out.join("\n"))
}

fn actual_count(result: &MarkerResult, fixture_extra_external_body: bool) -> usize {
    result.hits.len() + usize::from(fixture_extra_external_body && result.spec.key == "external_body")
}

fn actual_external_body_sites(
    result: &MarkerResult,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
) -> Vec<ExpectedSite> {
    let mut actual = Vec::new();
    for hit in &result.hits {
        if let Some(next_fn) = &hit.next_fn {
            actual.push(ExpectedSite {
                path: hit.path.display().to_string(),
                next_fn: next_fn.clone(),
                classification: "permanent-trust-boundary".to_string(),
            });
        }
    }
    if fixture_site_substitution && !actual.is_empty() {
        actual.remove(0);
        actual.push(ExpectedSite {
            path: "src/simulated_new_boundary.rs".to_string(),
            next_fn: "pub fn simulated_new_boundary(".to_string(),
            classification: "permanent-trust-boundary".to_string(),
        });
    }
    if fixture_classification_substitution && !actual.is_empty() {
        actual[0].classification = "closeable-structural-candidate".to_string();
    }
    actual.sort();
    actual
}

fn site_check(
    expected: Vec<ExpectedSite>,
    result: &MarkerResult,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
) -> SiteCheck {
    let actual = actual_external_body_sites(result, fixture_site_substitution, fixture_classification_substitution);
    let missing = expected.iter().filter(|site| !actual.contains(site)).cloned().collect();
    let unexpected = actual.iter().filter(|site| !expected.contains(site)).cloned().collect();
    SiteCheck {
        expected,
        actual,
        missing,
        unexpected,
    }
}

fn render_site(site: &ExpectedSite) -> String {
    format!("`{}` → `{}` classified `{}`", site.path, site.next_fn, site.classification)
}

fn render_report(
    results: &[MarkerResult],
    manifest: &Path,
    fixture_extra_external_body: bool,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
    fixture_stale_manifest: bool,
    fixture_missing_rationale: bool,
    fixture_blank_rationale: bool,
    fixture_malformed_rationale: bool,
    site_check: &SiteCheck,
) -> String {
    let mut out = String::new();
    out.push_str("# Proof-Gap Inventory Report\n\n");
    out.push_str("Source root: `src/`\n\n");
    out.push_str(&format!("Manifest: `{}`\n\n", manifest.display()));
    if fixture_extra_external_body {
        out.push_str("Fixture: simulated one extra `#[verifier::external_body]` marker.\n\n");
    }
    if fixture_site_substitution {
        out.push_str("Fixture: simulated same-count external-body site substitution.\n\n");
    }
    if fixture_classification_substitution {
        out.push_str("Fixture: simulated same-site external-body classification substitution.\n\n");
    }
    if fixture_stale_manifest {
        out.push_str("Fixture: simulated stale manifest by removing one expected manifest site.\n\n");
    }
    if fixture_missing_rationale {
        out.push_str("Fixture: simulated manifest row missing rationale field.\n\n");
    }
    if fixture_blank_rationale {
        out.push_str("Fixture: simulated manifest row with blank rationale field.\n\n");
    }
    if fixture_malformed_rationale {
        out.push_str("Fixture: simulated manifest row with an extra rationale column.\n\n");
    }
    out.push_str("## Summary\n\n");
    out.push_str("| marker | expected | actual | status |\n");
    out.push_str("| --- | ---: | ---: | --- |\n");
    for result in results {
        let actual = actual_count(result, fixture_extra_external_body);
        let status = if actual == result.spec.expected { "ok" } else { "drift" };
        out.push_str(&format!("| `{}` | {} | {} | {} |\n", result.spec.key, result.spec.expected, actual, status));
    }
    let site_status = if site_check.missing.is_empty() && site_check.unexpected.is_empty() {
        "ok"
    } else {
        "drift"
    };
    out.push_str("\n## External Body Site List, Classifications, and Rationales\n\n");
    out.push_str(&format!("Status: `{site_status}`\n\n"));
    out.push_str(&format!("Expected sites from manifest: {}\n\n", site_check.expected.len()));
    out.push_str(&format!("Actual sites from source: {}\n\n", site_check.actual.len()));
    out.push_str(&format!("Manifest rationales: {} non-empty entries.\n\n", site_check.expected.len()));
    if site_check.missing.is_empty() {
        out.push_str("Missing expected sites: none.\n\n");
    } else {
        out.push_str("Missing expected sites:\n");
        for site in &site_check.missing {
            out.push_str(&format!("- {}\n", render_site(site)));
        }
        out.push('\n');
    }
    if site_check.unexpected.is_empty() {
        out.push_str("Unexpected sites: none.\n\n");
    } else {
        out.push_str("Unexpected sites:\n");
        for site in &site_check.unexpected {
            out.push_str(&format!("- {}\n", render_site(site)));
        }
        out.push('\n');
    }
    out.push_str("## Hits\n\n");
    for result in results {
        out.push_str(&format!("### `{}`\n\n", result.spec.key));
        if result.hits.is_empty() {
            out.push_str("No hits.\n\n");
        } else {
            for hit in &result.hits {
                out.push_str(&format!("- `{}:{}` `{}`", hit.path.display(), hit.line, hit.line_text));
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

fn run(args: Args) -> Result<bool, String> {
    let expected_sites = parse_manifest(
        &args.root,
        &args.manifest,
        args.fixture_stale_manifest,
        args.fixture_missing_rationale,
        args.fixture_blank_rationale,
        args.fixture_malformed_rationale,
    )?;
    let src = args.root.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src, &mut files).map_err(|err| format!("failed to scan {}: {err}", src.display()))?;
    files.sort();
    let mut results = Vec::new();
    for spec in MARKERS.iter().copied() {
        results.push(
            scan_marker(&args.root, &files, spec)
                .map_err(|err| format!("failed while scanning `{}`: {err}", spec.key))?,
        );
    }
    let external_body = results
        .iter()
        .find(|result| result.spec.key == "external_body")
        .ok_or("missing external_body marker spec")?;
    let sites = site_check(
        expected_sites,
        external_body,
        args.fixture_site_substitution,
        args.fixture_classification_substitution,
    );
    let report = render_report(
        &results,
        &args.manifest,
        args.fixture_extra_external_body,
        args.fixture_site_substitution,
        args.fixture_classification_substitution,
        args.fixture_stale_manifest,
        args.fixture_missing_rationale,
        args.fixture_blank_rationale,
        args.fixture_malformed_rationale,
        &sites,
    );
    if let Some(path) = &args.write_report {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(path, report.as_bytes()).map_err(|err| format!("failed to write {}: {err}", path.display()))?;
    }
    print!("{report}");
    let counts_ok = results
        .iter()
        .all(|result| actual_count(result, args.fixture_extra_external_body) == result.spec.expected);
    let sites_ok = sites.missing.is_empty() && sites.unexpected.is_empty();
    Ok(counts_ok && sites_ok)
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
