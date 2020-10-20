use semver::Version;
use std::{env, error::Error, fs, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var("OUT_DIR")?);
    let mut linker_script = fs::read_to_string("defmt.x.in")?;
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?;
    let version = if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        // no git info -> assume crates.io
        let semver = Version::parse(&std::env::var("CARGO_PKG_VERSION")?)?;
        if semver.major == 0 {
            // minor is breaking when major = 0
            format!("{}.{}", semver.major, semver.minor)
        } else {
            // ignore minor, patch, pre and build
            semver.major.to_string()
        }
    };
    linker_script = linker_script.replace("$DEFMT_VERSION", version.trim());
    fs::write(out.join("defmt.x"), linker_script)?;
    println!("cargo:rustc-link-search={}", out.display());
    Ok(())
}
