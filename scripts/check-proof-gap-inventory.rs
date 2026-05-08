#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
struct MarkerSpec {
    key: &'static str,
    text: &'static str,
    expected: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExpectedSite {
    path: &'static str,
    next_fn: &'static str,
    classification: &'static str,
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

const EXPECTED_EXTERNAL_BODY_SITES: &[ExpectedSite] = &[
    ExpectedSite {
        path: "src/chain.rs",
        next_fn: "pub fn compute_entry_hash(",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/chain.rs",
        next_fn: "pub proof fn hash_collision_free(",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/nix_version.rs",
        next_fn: "pub fn compare_versions(a: &Vec<u8>, b: &Vec<u8>) -> (result: i8)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/snapshot_integrity.rs",
        next_fn: "pub proof fn dual_hash_collision_resistant(a1: u64, b1: u64, a2: u64, b2: u64)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/snapshot_integrity.rs",
        next_fn: "pub fn dual_hash(a: u64, b: u64) -> (result: u64)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_did.rs",
        next_fn: "pub proof fn axiom_base58btc_round_trip(bytes: Seq<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_did.rs",
        next_fn: "pub proof fn axiom_base58btc_injective(a: Seq<u8>, b: Seq<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_did.rs",
        next_fn: "pub fn base58btc_encode(bytes: &Vec<u8>) -> (result: Vec<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_did.rs",
        next_fn: "pub fn base58btc_decode(encoded: &Vec<u8>) -> (result: Option<Vec<u8>>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub proof fn axiom_sign_verify_round_trip(sk: Seq<u8>, msg: Seq<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub proof fn axiom_wrong_key_rejects(sk: Seq<u8>, pk: Seq<u8>, msg: Seq<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub proof fn axiom_wrong_message_rejects(sk: Seq<u8>, msg1: Seq<u8>, msg2: Seq<u8>)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub fn sign(sk: &SecretKey, message: &Vec<u8>) -> (result: Ed25519Signature)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub fn verify(pk: &PublicKey, message: &Vec<u8>, sig: &Ed25519Signature) -> (result: bool)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/ucan_signature.rs",
        next_fn: "pub fn public_from_secret(sk: &SecretKey) -> (result: PublicKey)",
        classification: "permanent-trust-boundary",
    },
    ExpectedSite {
        path: "src/web_of_trust_principal.rs",
        next_fn: "pub fn blake3_digest_bridge(input: &[u8]) -> (result: Vec<u8>)",
        classification: "permanent-trust-boundary",
    },
];

#[derive(Debug)]
struct Args {
    root: PathBuf,
    write_report: Option<PathBuf>,
    fixture_extra_external_body: bool,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
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
    let mut write_report = None;
    let mut fixture_extra_external_body = false;
    let mut fixture_site_substitution = false;
    let mut fixture_classification_substitution = false;
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
            "--fixture-site-substitution" => fixture_site_substitution = true,
            "--fixture-classification-substitution" => fixture_classification_substitution = true,
            "--help" | "-h" => {
                println!(
                    "usage: check-proof-gap-inventory.rs [--root PATH] [--write-report PATH] [--fixture-extra-external-body] [--fixture-site-substitution] [--fixture-classification-substitution]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Args {
        root,
        write_report,
        fixture_extra_external_body,
        fixture_site_substitution,
        fixture_classification_substitution,
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

fn actual_count(result: &MarkerResult, fixture_extra_external_body: bool) -> usize {
    let fixture_extra = if fixture_extra_external_body && result.spec.key == "external_body" {
        1
    } else {
        0
    };
    result.hits.len() + fixture_extra
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
                path: Box::leak(hit.path.display().to_string().into_boxed_str()),
                next_fn: Box::leak(next_fn.clone().into_boxed_str()),
                classification: "permanent-trust-boundary",
            });
        }
    }
    if fixture_site_substitution && !actual.is_empty() {
        actual.remove(0);
        actual.push(ExpectedSite {
            path: "src/simulated_new_boundary.rs",
            next_fn: "pub fn simulated_new_boundary(",
            classification: "permanent-trust-boundary",
        });
    }
    if fixture_classification_substitution && !actual.is_empty() {
        let first = actual[0];
        actual[0] = ExpectedSite {
            path: first.path,
            next_fn: first.next_fn,
            classification: "closeable-structural-candidate",
        };
    }
    actual.sort();
    actual
}

fn site_check(
    result: &MarkerResult,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
) -> SiteCheck {
    let mut expected = EXPECTED_EXTERNAL_BODY_SITES.to_vec();
    expected.sort();
    let actual = actual_external_body_sites(result, fixture_site_substitution, fixture_classification_substitution);
    let missing = expected.iter().copied().filter(|site| !actual.contains(site)).collect();
    let unexpected = actual.iter().copied().filter(|site| !expected.contains(site)).collect();
    SiteCheck {
        expected,
        actual,
        missing,
        unexpected,
    }
}

fn render_site(site: ExpectedSite) -> String {
    format!("`{}` → `{}` classified `{}`", site.path, site.next_fn, site.classification)
}

fn render_report(
    results: &[MarkerResult],
    fixture_extra_external_body: bool,
    fixture_site_substitution: bool,
    fixture_classification_substitution: bool,
    site_check: &SiteCheck,
) -> String {
    let mut out = String::new();
    out.push_str("# Proof-Gap Inventory Report\n\n");
    out.push_str("Source root: `src/`\n\n");
    if fixture_extra_external_body {
        out.push_str("Fixture: simulated one extra `#[verifier::external_body]` marker.\n\n");
    }
    if fixture_site_substitution {
        out.push_str("Fixture: simulated same-count external-body site substitution.\n\n");
    }
    if fixture_classification_substitution {
        out.push_str("Fixture: simulated same-site external-body classification substitution.\n\n");
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
    out.push_str("\n## External Body Site List and Classifications\n\n");
    out.push_str(&format!("Status: `{site_status}`\n\n"));
    out.push_str(&format!("Expected sites: {}\n\n", site_check.expected.len()));
    out.push_str(&format!("Actual sites: {}\n\n", site_check.actual.len()));
    if site_check.missing.is_empty() {
        out.push_str("Missing expected sites: none.\n\n");
    } else {
        out.push_str("Missing expected sites:\n");
        for site in &site_check.missing {
            out.push_str(&format!("- {}\n", render_site(*site)));
        }
        out.push('\n');
    }
    if site_check.unexpected.is_empty() {
        out.push_str("Unexpected sites: none.\n\n");
    } else {
        out.push_str("Unexpected sites:\n");
        for site in &site_check.unexpected {
            out.push_str(&format!("- {}\n", render_site(*site)));
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
    let sites = site_check(external_body, args.fixture_site_substitution, args.fixture_classification_substitution);
    let report = render_report(
        &results,
        args.fixture_extra_external_body,
        args.fixture_site_substitution,
        args.fixture_classification_substitution,
        &sites,
    );
    if let Some(path) = args.write_report {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
        fs::write(&path, report.as_bytes()).map_err(|err| format!("failed to write {}: {err}", path.display()))?;
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
