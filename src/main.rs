use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "mozc-cli")]
#[command(author, version, about = "CLI tool to restart Mozc (Google Japanese Input)")]
struct Args {
    #[arg(value_enum)]
    command: CommandType,

    /// Force language (ja / en). Overrides system locale.
    #[arg(short, long)]
    lang: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CommandType {
    /// Restart Mozc server, renderer and client
    Restart,
    /// Show status of Mozc processes (not yet implemented)
    Status,
}

#[derive(Deserialize, Clone)]
struct Translations {
    title: String,
    restart_start: String,
    killed: String,
    stopped: String,
    started: String,
    win_restart_note: String,
    mac_restart_note: String,
    unsupported_os: String,
    restart_complete: String,
    #[allow(dead_code)]
    command_restart: String,
    #[allow(dead_code)]
    command_status: String,
    status_not_implemented: String,
    error: String,
    no_services_found: String,
}

static FALLBACK_EN: Lazy<Translations> = Lazy::new(|| Translations {
    title: "Mozc CLI".into(),
    restart_start: "[{os}] Restarting Mozc...".into(),
    killed: "  Killed: {proc}".into(),
    stopped: "  Stopped: {service}".into(),
    started: "  Started: {service}".into(),
    win_restart_note: "Windows: Please toggle the IME off/on from the taskbar, or re-login if necessary.".into(),
    mac_restart_note: "macOS: Restart completed. Switch input mode to verify.".into(),
    unsupported_os: "Unsupported OS.".into(),
    restart_complete: "✓ Mozc restart completed.".into(),
    command_restart: "restart".into(),
    command_status: "status".into(),
    status_not_implemented: "Status command is not implemented yet.".into(),
    error: "Error: {msg}".into(),
    no_services_found: "No Mozc services found.".into(),
});

fn find_locales_dir() -> PathBuf {
    // 1. Same directory as the executable
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("locales");
            if candidate.is_dir() {
                return candidate;
            }
        }
    }
    // 2. Relative to current working directory
    let cwd = Path::new("locales");
    if cwd.is_dir() {
        return cwd.to_path_buf();
    }
    // 3. Fallback: next to Cargo.toml during development
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("locales")
}

fn load_translations() -> HashMap<String, Translations> {
    let mut map = HashMap::new();
    let locales_dir = find_locales_dir();

    for lang in ["en", "ja"] {
        let path = locales_dir.join(format!("{}.json", lang));
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(trans) = serde_json::from_str::<Translations>(&content) {
                map.insert(lang.to_string(), trans);
            }
        }
    }
    map
}

static I18N: Lazy<HashMap<String, Translations>> = Lazy::new(load_translations);

fn t(key: &str, lang: &str, vars: &[(&str, &str)]) -> String {
    let trans = I18N
        .get(lang)
        .or_else(|| I18N.get("en"))
        .unwrap_or(&FALLBACK_EN);

    let mut text = match key {
        "restart_start" => trans.restart_start.clone(),
        "killed" => trans.killed.clone(),
        "stopped" => trans.stopped.clone(),
        "started" => trans.started.clone(),
        "win_restart_note" => trans.win_restart_note.clone(),
        "mac_restart_note" => trans.mac_restart_note.clone(),
        "unsupported_os" => trans.unsupported_os.clone(),
        "restart_complete" => trans.restart_complete.clone(),
        "status_not_implemented" => trans.status_not_implemented.clone(),
        "error" => trans.error.clone(),
        "no_services_found" => trans.no_services_found.clone(),
        _ => key.to_string(),
    };

    for (k, v) in vars {
        text = text.replace(&format!("{{{}}}", k), v);
    }
    text
}

fn detect_lang(forced: Option<&str>) -> String {
    if let Some(lang) = forced {
        let lang = lang.to_lowercase();
        if lang.starts_with("ja") {
            return "ja".to_string();
        }
        return "en".to_string();
    }

    // Check LANG / LC_ALL
    for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(val) = env::var(var) {
            if val.to_lowercase().starts_with("ja") {
                return "ja".to_string();
            }
        }
    }

    // Windows: try to detect via system
    #[cfg(windows)]
    {
        // Simple fallback; more accurate detection can be added later
        if let Ok(val) = env::var("LANG") {
            if val.to_lowercase().starts_with("ja") {
                return "ja".to_string();
            }
        }
    }

    "en".to_string()
}

fn kill_process(name: &str) -> bool {
    #[cfg(windows)]
    {
        Command::new("taskkill")
            .args(["/F", "/IM", name, "/T"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(windows))]
    {
        Command::new("pkill")
            .args(["-f", name])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

fn restart_windows(lang: &str) {
    let processes = [
        "GoogleIMEJaConverter.exe",
        "GoogleIMEJaRenderer.exe",
        "GoogleJapaneseInput.exe",
    ];

    for p in processes {
        let killed = kill_process(p);
        if killed {
            println!("{}", t("killed", lang, &[("proc", p)]));
        }
    }

    // Give the system a moment
    thread::sleep(Duration::from_millis(300));

    println!("{}", t("win_restart_note", lang, &[]));
}

fn restart_macos(lang: &str) {
    // Find running Mozc-related launchd services
    let output = Command::new("sh")
        .arg("-c")
        .arg("launchctl list 2>/dev/null | grep -E 'com\\.google\\.inputmethod\\.Japanese\\.(Converter|Renderer)' | awk '{print $3}'")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let services = String::from_utf8_lossy(&out.stdout);
            let mut found = false;

            for service in services.lines().map(str::trim).filter(|s| !s.is_empty()) {
                found = true;

                let _ = Command::new("launchctl")
                    .args(["stop", service])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
                println!("{}", t("stopped", lang, &[("service", service)]));

                thread::sleep(Duration::from_millis(200));

                let _ = Command::new("launchctl")
                    .args(["start", service])
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status();
                println!("{}", t("started", lang, &[("service", service)]));
            }

            if !found {
                println!("{}", t("no_services_found", lang, &[]));
            }
        }
        _ => {
            println!("{}", t("no_services_found", lang, &[]));
        }
    }

    println!("{}", t("mac_restart_note", lang, &[]));
}

fn restart_mozc(lang: &str) {
    let os = std::env::consts::OS;
    println!("{}", t("restart_start", lang, &[("os", os)]));

    match os {
        "windows" => restart_windows(lang),
        "macos" => restart_macos(lang),
        _ => {
            println!("{}", t("unsupported_os", lang, &[]));
            std::process::exit(1);
        }
    }

    println!("{}", t("restart_complete", lang, &[]));
}

fn main() {
    let args = Args::parse();
    let lang = detect_lang(args.lang.as_deref());

    match args.command {
        CommandType::Restart => restart_mozc(&lang),
        CommandType::Status => {
            println!("{}", t("status_not_implemented", &lang, &[]));
            std::process::exit(1);
        }
    }
}
