#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#clang -c cargo -q -Zscript

//! Audit scheduler/fault-schedule exec helpers for module-local bare-call cycles.
//!
//! This intentionally keeps the old lightweight source scan semantics: function bodies are
//! discovered with a brace counter, per-file exec helpers are keyed by `(path, name)`, bare
//! intra-module calls create graph edges, and qualified intra-module calls are violations.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Function {
    name: String,
    kind: FunctionKind,
    path: PathBuf,
    line: usize,
    body: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum FunctionKind {
    Exec,
    Spec,
    Proof,
}

#[derive(Clone, Debug)]
struct QualifiedCallViolation {
    caller: Function,
    line: usize,
    snippet: String,
}

#[derive(Clone, Debug)]
struct DuplicateNameViolation {
    path: PathBuf,
    name: String,
    lines: Vec<usize>,
}

#[derive(Clone, Debug)]
struct ModuleAudit {
    path: PathBuf,
    exec_functions: Vec<Function>,
    edges: BTreeMap<Function, BTreeSet<Function>>,
    qualified_violations: Vec<QualifiedCallViolation>,
    duplicate_violations: Vec<DuplicateNameViolation>,
    cycles: Vec<Vec<Function>>,
}

fn usage() {
    eprintln!("usage: check-iterative-exec.rs <file.rs> [file.rs ...]");
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn parse_function_header(line: &str) -> Option<(FunctionKind, String)> {
    let trimmed = line.trim_start();
    let words: Vec<&str> =
        trimmed.split(|ch: char| ch.is_whitespace() || ch == '(').filter(|word| !word.is_empty()).collect();
    let fn_pos = words.iter().position(|word| *word == "fn")?;
    if fn_pos + 1 >= words.len() {
        return None;
    }
    let kind = if fn_pos > 0 && words[fn_pos - 1] == "spec" {
        FunctionKind::Spec
    } else if fn_pos > 0 && words[fn_pos - 1] == "proof" {
        FunctionKind::Proof
    } else {
        FunctionKind::Exec
    };
    let name = words[fn_pos + 1].trim_matches(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_')).to_string();
    if name.is_empty() { None } else { Some((kind, name)) }
}

fn extract_functions(path: &Path) -> Result<Vec<Function>, String> {
    let text = fs::read_to_string(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some((kind, name)) = parse_function_header(lines[i]) else {
            i += 1;
            continue;
        };
        let start = i;
        let mut brace_depth: isize = 0;
        let mut saw_open_brace = false;
        let mut j = i;
        while j < lines.len() {
            for ch in lines[j].chars() {
                match ch {
                    '{' => {
                        brace_depth += 1;
                        saw_open_brace = true;
                    }
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }
            j += 1;
            if saw_open_brace && brace_depth == 0 {
                break;
            }
        }
        out.push(Function {
            name,
            kind,
            path: path.to_path_buf(),
            line: start + 1,
            body: lines[start..j].join("\n"),
        });
        i = j;
    }
    Ok(out)
}

fn function_body_text(function: &Function) -> &str {
    function.body.find('{').map_or("", |idx| &function.body[idx + 1..])
}

fn line_for_offset(function: &Function, text: &str, offset: usize) -> usize {
    function.line + text[..offset].bytes().filter(|byte| *byte == b'\n').count()
}

fn find_duplicate_exec_names(exec_functions: &[Function]) -> Vec<DuplicateNameViolation> {
    let mut by_name: BTreeMap<String, Vec<&Function>> = BTreeMap::new();
    for function in exec_functions {
        by_name.entry(function.name.clone()).or_default().push(function);
    }
    by_name
        .into_iter()
        .filter_map(|(name, defs)| {
            (defs.len() > 1).then(|| DuplicateNameViolation {
                path: defs[0].path.clone(),
                name,
                lines: defs.iter().map(|function| function.line).collect(),
            })
        })
        .collect()
}

fn has_bare_call(text: &str, name: &str) -> bool {
    let bytes = text.as_bytes();
    let needle = name.as_bytes();
    if bytes.len() < needle.len() + 1 {
        return false;
    }
    let mut i = 0;
    while i + needle.len() < bytes.len() {
        if &bytes[i..i + needle.len()] == needle {
            let before_ok = i == 0 || !(is_ident(bytes[i - 1]) || bytes[i - 1] == b'.' || bytes[i - 1] == b':');
            let mut j = i + needle.len();
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if before_ok && j < bytes.len() && bytes[j] == b'(' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn qualified_call_snippets(text: &str, name: &str) -> Vec<(usize, String)> {
    let pattern = format!("::{name}");
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut search_from = 0;
    while let Some(rel) = text[search_from..].find(&pattern) {
        let pos = search_from + rel;
        let name_start = pos + 2;
        let mut j = name_start + name.len();
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        if j >= bytes.len() || bytes[j] != b'(' {
            search_from = pos + 2;
            continue;
        }
        let mut start = pos;
        while start > 0 {
            let byte = bytes[start - 1];
            if is_ident(byte) || byte == b':' {
                start -= 1;
            } else {
                break;
            }
        }
        let prefix = &text[start..pos];
        let first_segment = prefix.split("::").next().unwrap_or("");
        let first_ok = matches!(first_segment, "self" | "Self" | "super" | "crate")
            || first_segment.as_bytes().first().is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_');
        if first_ok && (start == 0 || !is_ident(bytes[start - 1])) {
            out.push((start, text[start..=j].to_string()));
        }
        search_from = pos + 2;
    }
    out
}

fn find_cycles(edges: &BTreeMap<Function, BTreeSet<Function>>) -> Vec<Vec<Function>> {
    let mut cycles = Vec::new();
    let mut temp = BTreeSet::new();
    let mut perm = BTreeSet::new();
    let mut stack: Vec<Function> = Vec::new();

    fn visit(
        node: &Function,
        edges: &BTreeMap<Function, BTreeSet<Function>>,
        temp: &mut BTreeSet<Function>,
        perm: &mut BTreeSet<Function>,
        stack: &mut Vec<Function>,
        cycles: &mut Vec<Vec<Function>>,
    ) {
        if perm.contains(node) {
            return;
        }
        if temp.contains(node) {
            if let Some(idx) = stack.iter().position(|function| function == node) {
                let mut cycle = stack[idx..].to_vec();
                cycle.push(node.clone());
                cycles.push(cycle);
            }
            return;
        }
        temp.insert(node.clone());
        stack.push(node.clone());
        if let Some(nexts) = edges.get(node) {
            for next in nexts {
                visit(next, edges, temp, perm, stack, cycles);
            }
        }
        stack.pop();
        temp.remove(node);
        perm.insert(node.clone());
    }

    for node in edges.keys() {
        visit(node, edges, &mut temp, &mut perm, &mut stack, &mut cycles);
    }
    cycles
}

fn build_module_audit(path: &Path) -> Result<ModuleAudit, String> {
    let functions = extract_functions(path)?;
    let exec_functions: Vec<Function> =
        functions.into_iter().filter(|function| function.kind == FunctionKind::Exec).collect();
    let duplicate_violations = find_duplicate_exec_names(&exec_functions);
    let mut edges: BTreeMap<Function, BTreeSet<Function>> =
        exec_functions.iter().cloned().map(|function| (function, BTreeSet::new())).collect();
    let mut qualified_violations = Vec::new();

    if duplicate_violations.is_empty() {
        let by_name: BTreeMap<String, Function> =
            exec_functions.iter().cloned().map(|function| (function.name.clone(), function)).collect();
        for function in &exec_functions {
            let body = function_body_text(function);
            for (name, callee) in &by_name {
                if has_bare_call(body, name) {
                    edges.entry(function.clone()).or_default().insert(callee.clone());
                }
                for (offset, snippet) in qualified_call_snippets(body, name) {
                    qualified_violations.push(QualifiedCallViolation {
                        caller: function.clone(),
                        line: line_for_offset(function, body, offset),
                        snippet,
                    });
                }
            }
        }
    }
    let cycles = find_cycles(&edges);
    Ok(ModuleAudit {
        path: path.to_path_buf(),
        exec_functions,
        edges,
        qualified_violations,
        duplicate_violations,
        cycles,
    })
}

fn format_function(function: &Function) -> String {
    format!("{} @ {}:{}", function.name, function.path.display(), function.line)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        usage();
        std::process::exit(if args.is_empty() { 2 } else { 0 });
    }
    let files: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
    let missing: Vec<String> =
        files.iter().filter(|path| !path.exists()).map(|path| path.display().to_string()).collect();
    if !missing.is_empty() {
        for path in missing {
            eprintln!("missing: {path}");
        }
        std::process::exit(2);
    }

    let audits: Vec<ModuleAudit> = files.iter().map(|path| build_module_audit(path)).collect::<Result<_, _>>()?;

    println!("iterative-exec audit");
    println!("files: {}", files.iter().map(|path| path.display().to_string()).collect::<Vec<_>>().join(", "));

    let mut failed = false;
    for audit in &audits {
        println!("module: {}", audit.path.display());
        println!("exec functions: {}", audit.exec_functions.len());
        let mut functions = audit.exec_functions.clone();
        functions.sort_by_key(|function| (function.line, function.name.clone()));
        for function in &functions {
            let callees = audit
                .edges
                .get(function)
                .map(|set| set.iter().map(|callee| callee.name.clone()).collect::<Vec<_>>().join(", "))
                .filter(|text| !text.is_empty())
                .unwrap_or_else(|| "(none)".to_string());
            println!("- {} -> {callees}", format_function(function));
        }
        if !audit.duplicate_violations.is_empty() {
            failed = true;
            println!("duplicate bare-name violations:");
            for violation in &audit.duplicate_violations {
                let lines = violation.lines.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                println!(
                    "- {}: exec helper name '{}' appears on lines {}",
                    violation.path.display(),
                    violation.name,
                    lines
                );
            }
        }
        if !audit.qualified_violations.is_empty() {
            failed = true;
            println!("qualified local-call violations:");
            for violation in &audit.qualified_violations {
                println!(
                    "- {} uses qualified local call {:?} at line {}",
                    format_function(&violation.caller),
                    violation.snippet,
                    violation.line
                );
            }
        }
        if !audit.cycles.is_empty() {
            failed = true;
            println!("cycles:");
            for cycle in &audit.cycles {
                println!("- {}", cycle.iter().map(format_function).collect::<Vec<_>>().join(" -> "));
            }
        }
    }
    println!("result: {}", if failed { "FAIL" } else { "PASS" });
    if failed {
        std::process::exit(1);
    }
    Ok(())
}
