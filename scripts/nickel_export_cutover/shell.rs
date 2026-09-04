use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use crate::model::EXPECTED_EXPORTER_REVISION;
use crate::model::ExportRequest;
use crate::model::Profile;
use crate::validation::safe_relative;

const EXPORTER_INSTALLABLE: &str =
    "github:OnixResearch/nickel-export/257fafc1c746f1faf156207043a4c826bfb16d49#nickel-export";
const NICKEL_INSTALLABLE: &str = "github:nickel-lang/nickel/1320a983e6c3d1e2fb53dd2464b084b4903b1426#default";
const PROFILE_SOURCE: &str = "nickel-export-cutover/profile.ncl";
const WRITE_MODE: &str = "--write";
const CHECK_MODE: &str = "--check";

#[derive(Clone, Debug)]
pub struct Tools {
    pub nickel: PathBuf,
    pub exporter: PathBuf,
}

pub fn resolve_tools(nickel: Option<PathBuf>, exporter: Option<PathBuf>) -> Result<Tools, String> {
    let nickel = match nickel {
        Some(path) => path,
        None => resolve_installable(NICKEL_INSTALLABLE, "bin/nickel")?,
    };
    let exporter = match exporter {
        Some(path) => path,
        None => resolve_installable(EXPORTER_INSTALLABLE, "bin/nickel-export")?,
    };
    require_file(&nickel, "Nickel evaluator")?;
    require_file(&exporter, "standalone Nickel exporter")?;
    Ok(Tools { nickel, exporter })
}

pub fn verify_tools(tools: &Tools, source_revision: &str, expected_version: &str) -> Result<(), String> {
    if source_revision != EXPECTED_EXPORTER_REVISION {
        return Err("standalone exporter revision drift".to_owned());
    }
    let output = Command::new(&tools.nickel)
        .arg("--version")
        .output()
        .map_err(|error| format!("run Nickel version: {error}"))?;
    let output = require_success(output, "Nickel version")?;
    let version = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    if version.trim() == expected_version {
        Ok(())
    } else {
        Err(format!("Nickel version drift: {}", version.trim()))
    }
}

pub fn generate_profile(root: &Path, tools: &Tools) -> Result<Vec<u8>, String> {
    run_nickel_export(root, tools, PROFILE_SOURCE, "json", None)
}

pub fn generate_request(root: &Path, tools: &Tools, field: &str) -> Result<Vec<u8>, String> {
    let selector = format!("requests.{field}");
    run_nickel_export(root, tools, PROFILE_SOURCE, "json", Some(&selector))
}

pub fn legacy_export(root: &Path, tools: &Tools, request: &ExportRequest) -> Result<Output, String> {
    let mut args = vec![
        OsString::from("export"),
        OsString::from("--format"),
        OsString::from(request.format.as_str()),
        OsString::from(request.source.as_str()),
    ];
    for import_path in &request.import_paths {
        args.push(OsString::from("--import-path"));
        args.push(root.join(import_path).into_os_string());
    }
    if !request.selector.is_empty() {
        args.push(OsString::from("--field"));
        args.push(OsString::from(request.selector.as_str()));
    }
    if !request.contract.is_empty() {
        args.push(OsString::from("--apply-contract"));
        args.push(root.join(&request.contract).into_os_string());
    }
    Command::new(&tools.nickel)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("run legacy Nickel export: {error}"))
}

pub fn standalone_export(
    root: &Path,
    tools: &Tools,
    profile: &Profile,
    request_path: &str,
    manifest_path: &str,
    write: bool,
) -> Result<Output, String> {
    let mode = if write { WRITE_MODE } else { CHECK_MODE };
    Command::new(&tools.exporter)
        .args([
            "export",
            "--spec",
            request_path,
            "--root",
            root.to_str().ok_or("repository root is not UTF-8")?,
            "--evaluator",
            tools.nickel.to_str().ok_or("Nickel path is not UTF-8")?,
            "--evaluator-identity",
            &profile.evaluator.identity,
            "--evaluator-version",
            &profile.evaluator.version,
            "--manifest",
            manifest_path,
            mode,
        ])
        .current_dir(root)
        .output()
        .map_err(|error| format!("run standalone Nickel export: {error}"))
}

pub fn require_success(output: Output, label: &str) -> Result<Output, String> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!(
            "{label} failed with status {:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub fn read_relative(root: &Path, relative: &str) -> Result<Vec<u8>, String> {
    if !safe_relative(relative) {
        return Err(format!("unsafe relative path `{relative}`"));
    }
    let path = root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| format!("inspect {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("expected direct file: {}", path.display()));
    }
    fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))
}

pub fn write_relative(root: &Path, relative: &str, bytes: &[u8]) -> Result<(), String> {
    if !safe_relative(relative) {
        return Err(format!("unsafe output path `{relative}`"));
    }
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    fs::write(&path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
}

pub fn copy_relative(source_root: &Path, destination_root: &Path, relative: &str) -> Result<(), String> {
    let bytes = read_relative(source_root, relative)?;
    write_relative(destination_root, relative, &bytes)
}

pub fn fresh_temp_root(label: &str) -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!("trellis-nickel-export-{label}-{}", std::process::id()));
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(format!("reset {}: {error}", root.display())),
    }
    fs::create_dir_all(&root).map_err(|error| format!("create {}: {error}", root.display()))?;
    Ok(root)
}

pub fn remove_temp_root(root: &Path) -> Result<(), String> {
    fs::remove_dir_all(root).map_err(|error| format!("remove {}: {error}", root.display()))
}

fn run_nickel_export(
    root: &Path,
    tools: &Tools,
    source: &str,
    format: &str,
    field: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut command = Command::new(&tools.nickel);
    command.args(["export", "--format", format]);
    if let Some(selector) = field {
        command.args(["--field", selector]);
    }
    let output = command
        .arg(source)
        .current_dir(root)
        .output()
        .map_err(|error| format!("run Nickel profile export: {error}"))?;
    Ok(require_success(output, "Nickel profile export")?.stdout)
}

fn resolve_installable(installable: &str, binary: &str) -> Result<PathBuf, String> {
    let output = Command::new("nix")
        .args(["build", "--no-link", "--print-out-paths", installable])
        .output()
        .map_err(|error| format!("resolve {installable}: {error}"))?;
    let output = require_success(output, "resolve pinned Nix tool")?;
    let stdout = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    let mut paths = stdout.lines().filter(|line| !line.trim().is_empty());
    let path = paths.next().ok_or("pinned Nix tool returned no output path")?;
    if paths.next().is_some() {
        return Err("pinned Nix tool returned multiple output paths".to_owned());
    }
    Ok(Path::new(path).join(binary))
}

fn require_file(path: &Path, label: &str) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("{label} is not a file: {}", path.display()))
    }
}
