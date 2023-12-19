//! [![github]](https://github.com/LittleBoxOfSunshine/Omnicopy-To-Output)&ensp;[![crates-io]](https://crates.io/crates/omnicopy_to_output)&ensp;[![docs-rs]](https://docs.rs/omnicopy_to_output)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Provides a generalized implementation for a "post build copy" operation, which isn't
//! well-supported in rust at time of writing. This crate is inspired by
//! <https://github.com/prenwyn/copy_to_output>, but implements more managed helpers + addresses some
//! of the missing scenarios (again, at time of writing).
//!
//! As the name implies, the goal here is to provide coverage for all possible build scenarios as
//! a stand-in until (if) there is a native solution in the rust tooling. If anything is missing,
//! please contribute!
//!
//! # Examples
//! - Use in `build.rs`
//!
//!
//! # Scenario Coverage
//!
//! Key motivations for the original fork were supporting workspaces + managed experience for cargo
//! cache instructions.
//!
//! We support accommodating:
//!     - Build types (e.g retail vs test; integration tests see files)
//!     - Target
//!     - Cross compilation (special case target)
//!     - Workspace or single crate build
//!
//! # Considerations
//!
//! This is in lieu of a better solution from cargo directly. In particular, it's worth noting that
//! [build scripts should not modify any files outside of the OUT_DIR directory](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
//! We're not modifying, but it's still not necessarily in the "spirit" of the instructions.
//!
//! # How it Works
//!
//! To locate the target directory, we must know the project root and the target.
//!
//! 1. Project root (to support workspaces) is determined using [project_root](https://docs.rs/project-root/latest/project_root/)
//! 2. From the root, the next path element is always `/target`
//! 3. Next, is either `/{profile}` if no specific target selector was provided or `/{target}/{profile}` if one is provided
//!     a. Get `{profile}` from `env:PROFILE`
//!     b. Get `{target}` from [build_target::target_triple](https://docs.rs/build-target/0.4.0/build_target/fn.target_triple.html)
//!     c. Determine which scheme is in use by testing if `env:OUT_DIR` contains `target/{target}`
//!

extern crate core;

use anyhow::{anyhow, Result};
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use project_root::get_project_root;
use std::env;
use std::path::Path;

pub fn copy_to_output(path: &str) -> Result<()> {
    copy_to_output_for_build_type(path, &env::var("PROFILE")?)
}

pub fn copy_to_output_for_build_type(path: &str, build_type: &str) -> Result<()> {
    let mut out_path = get_project_root()?;
    out_path.push("target");

    // This is a hack, ideally we would plug into https://docs.rs/cargo/latest/cargo/core/compiler/enum.CompileKind.html
    // However, since the path follows predictable rules https://doc.rust-lang.org/cargo/guide/build-cache.html
    // we can just check our parent path for the pattern target/{triple}/{profile}.
    // If it is present, we know CompileKind::Target was used, otherwise CompileKind::Host was used.
    let triple = build_target::target_triple()?;

    if env::var("OUT_DIR")?.contains(&triple) {
        out_path.push(triple);
    }

    out_path.push(build_type);

    // Overwrite existing files with same name
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_items(&[path], &out_path, &options)?;

    Ok(())
}

/// Copies files to output
pub fn copy_to_output_by_path(path: &Path) -> Result<()> {
    copy_to_output(path_to_str(path)?)
}

fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or(anyhow!("Could not convert file path to string"))
}

pub fn copy_to_output_by_path_for_build_type(path: &Path, build_type: &str) -> Result<()> {
    copy_to_output_for_build_type(path_to_str(path)?, build_type)
}
