//! Version manager binary
//! Manages project versioning

use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Version manager command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Get current version
    Get {
        /// Path to Cargo.toml
        #[arg(short, long, default_value = "Cargo.toml")]
        cargo_toml: String,
    },

    /// Set new version
    Set {
        /// New version
        version: String,

        /// Path to Cargo.toml
        #[arg(short, long, default_value = "Cargo.toml")]
        cargo_toml: String,
    },

    /// Bump major version
    BumpMajor {
        /// Path to Cargo.toml
        #[arg(short, long, default_value = "Cargo.toml")]
        cargo_toml: String,
    },

    /// Bump minor version
    BumpMinor {
        /// Path to Cargo.toml
        #[arg(short, long, default_value = "Cargo.toml")]
        cargo_toml: String,
    },

    /// Bump patch version
    BumpPatch {
        /// Path to Cargo.toml
        #[arg(short, long, default_value = "Cargo.toml")]
        cargo_toml: String,
    },

    /// Check version format
    Check {
        /// Version to check
        version: String,
    },
}

/// Semantic version
#[derive(Debug, PartialEq, Eq)]
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SemVer {
    /// Parse semantic version from string
    fn parse(version: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err("Version must be in format major.minor.patch".to_string());
        }

        let major = parts[0]
            .parse()
            .map_err(|_| "Invalid major version".to_string())?;
        let minor = parts[1]
            .parse()
            .map_err(|_| "Invalid minor version".to_string())?;
        let patch = parts[2]
            .parse()
            .map_err(|_| "Invalid patch version".to_string())?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    /// Bump major version
    fn bump_major(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            patch: 0,
        }
    }

    /// Bump minor version
    fn bump_minor(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
        }
    }

    /// Bump patch version
    fn bump_patch(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
        }
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn main() {
    env_logger::init();

    let command = Command::parse();

    match command {
        Command::Get { cargo_toml } => {
            if let Err(e) = get_version(&cargo_toml) {
                eprintln!("Error getting version: {}", e);
            }
        }
        Command::Set {
            version,
            cargo_toml,
        } => {
            if let Err(e) = set_version(&version, &cargo_toml) {
                eprintln!("Error setting version: {}", e);
            }
        }
        Command::BumpMajor { cargo_toml } => {
            if let Err(e) = bump_major(&cargo_toml) {
                eprintln!("Error bumping major version: {}", e);
            }
        }
        Command::BumpMinor { cargo_toml } => {
            if let Err(e) = bump_minor(&cargo_toml) {
                eprintln!("Error bumping minor version: {}", e);
            }
        }
        Command::BumpPatch { cargo_toml } => {
            if let Err(e) = bump_patch(&cargo_toml) {
                eprintln!("Error bumping patch version: {}", e);
            }
        }
        Command::Check { version } => {
            if let Err(e) = check_version(&version) {
                eprintln!("Error checking version: {}", e);
            } else {
                println!("Version format is valid: {}", version);
            }
        }
    }
}

fn get_version(cargo_toml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(cargo_toml_path);
    if !path.exists() {
        return Err(format!("{} does not exist", cargo_toml_path).into());
    }

    // Read Cargo.toml
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Extract version
    if let Some(version_line) = content.lines().find(|line| line.starts_with("version = ")) {
        let version = version_line
            .split('"')
            .nth(1)
            .ok_or("Invalid version format")?;

        println!("Current version: {}", version);

        // Check if version is semantic
        if let Ok(semver) = SemVer::parse(version) {
            println!("Semantic version: {}", semver);
            println!("Major: {}", semver.major);
            println!("Minor: {}", semver.minor);
            println!("Patch: {}", semver.patch);
        } else {
            println!("Version is not in semantic format");
        }
    } else {
        return Err("Version not found in Cargo.toml".into());
    }

    Ok(())
}

fn set_version(version: &str, cargo_toml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(cargo_toml_path);
    if !path.exists() {
        return Err(format!("{} does not exist", cargo_toml_path).into());
    }

    // Check version format
    if SemVer::parse(version).is_err() {
        return Err("Version must be in semantic format (major.minor.patch)".into());
    }

    // Read Cargo.toml
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Update version
    let updated_content = content
        .lines()
        .map(|line| {
            if line.starts_with("version = ") {
                format!("version = \"{}\"", version)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Write back to Cargo.toml
    let mut file = File::create(path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Version set to: {}", version);
    println!("Cargo.toml updated successfully");

    Ok(())
}

fn bump_major(cargo_toml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(cargo_toml_path);
    if !path.exists() {
        return Err(format!("{} does not exist", cargo_toml_path).into());
    }

    // Read Cargo.toml
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Extract current version
    let current_version = content
        .lines()
        .find(|line| line.starts_with("version = "))
        .ok_or("Version not found in Cargo.toml")?
        .split('"')
        .nth(1)
        .ok_or("Invalid version format")?;

    // Parse and bump version
    let semver = SemVer::parse(current_version)?;
    let new_semver = semver.bump_major();
    let new_version = new_semver.to_string();

    // Update version
    let updated_content = content
        .lines()
        .map(|line| {
            if line.starts_with("version = ") {
                format!("version = \"{}\"", new_version)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Write back to Cargo.toml
    let mut file = File::create(path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Version bumped from {} to {}", current_version, new_version);
    println!("Cargo.toml updated successfully");

    Ok(())
}

fn bump_minor(cargo_toml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(cargo_toml_path);
    if !path.exists() {
        return Err(format!("{} does not exist", cargo_toml_path).into());
    }

    // Read Cargo.toml
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Extract current version
    let current_version = content
        .lines()
        .find(|line| line.starts_with("version = "))
        .ok_or("Version not found in Cargo.toml")?
        .split('"')
        .nth(1)
        .ok_or("Invalid version format")?;

    // Parse and bump version
    let semver = SemVer::parse(current_version)?;
    let new_semver = semver.bump_minor();
    let new_version = new_semver.to_string();

    // Update version
    let updated_content = content
        .lines()
        .map(|line| {
            if line.starts_with("version = ") {
                format!("version = \"{}\"", new_version)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Write back to Cargo.toml
    let mut file = File::create(path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Version bumped from {} to {}", current_version, new_version);
    println!("Cargo.toml updated successfully");

    Ok(())
}

fn bump_patch(cargo_toml_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(cargo_toml_path);
    if !path.exists() {
        return Err(format!("{} does not exist", cargo_toml_path).into());
    }

    // Read Cargo.toml
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Extract current version
    let current_version = content
        .lines()
        .find(|line| line.starts_with("version = "))
        .ok_or("Version not found in Cargo.toml")?
        .split('"')
        .nth(1)
        .ok_or("Invalid version format")?;

    // Parse and bump version
    let semver = SemVer::parse(current_version)?;
    let new_semver = semver.bump_patch();
    let new_version = new_semver.to_string();

    // Update version
    let updated_content = content
        .lines()
        .map(|line| {
            if line.starts_with("version = ") {
                format!("version = \"{}\"", new_version)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Write back to Cargo.toml
    let mut file = File::create(path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Version bumped from {} to {}", current_version, new_version);
    println!("Cargo.toml updated successfully");

    Ok(())
}

fn check_version(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    SemVer::parse(version)?;
    Ok(())
}
