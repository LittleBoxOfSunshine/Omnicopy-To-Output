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
//! - Use in `build.rs` with automatic discovery.
//!
//!   Path is relative to project root. If your resources are in your project, cargo will automatically detect
//!   changes and invalidate the cache as needed.
//!
//!   ```no_run
//!   use omnicopy_to_output::copy_to_output;
//!
//!   fn main() {
//!       // Copy everything recursively from the res folder and place into output.
//!       copy_to_output("res").expect("Could not copy");
//!   }
//!   ```
//!
//! - Use in `build.rs` with custom target (e.g. if your have different shared libraries for debug).
//!
//!   Note, if you used both your builds will fail. Each target directory only exists when that
//!   build is run. A full example would have conditional logic.
//!
//!   ```no_run
//!   use omnicopy_to_output::copy_to_output_for_profile;
//!
//!   fn main() {
//!       // Manually specify the profile (i.e. env:PROFILE)
//!       copy_to_output_for_profile("res/foo.dll", "release").expect("Could not copy");
//!       copy_to_output_for_profile("res/food.dll", "debug").expect("Could not copy");
//!       
//!   }
//!   ```
//!
//! - Invalidate Cache for external resources
//!
//!   Large resources may not exist in your project. We can still copy those to output, but cargo will
//!   not detect changes and invalidate the cache. Emitting [cargo:rerun-if-changed](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   instructions will inform cargo these files exist, but then will change cache invalidation to _only_
//!   what you specify. Note, as soon as you do this in one place the default "anything in package"
//!   rules no longer apply. This is something you ideally are configuring anyway though.
//!
//!   ```no_run
//!   use omnicopy_to_output::{copy_to_output, cargo_rerun_if_changed};
//!
//!   fn main() {
//!       let path_to_large_resources = "/path/to/large/resources";
//!       cargo_rerun_if_changed(path_to_large_resources);
//!       copy_to_output(path_to_large_resources).expect("Could not copy");
//!   }
//!
//! # Scenario Coverage
//!
//! Key motivations for the original fork were supporting workspaces + managed experience for cargo
//! cache instructions.
//!
//! We support accommodating:
//!     - Build types (e.g. retail vs test; integration tests see files)
//!     - [`cargo::CompileKind`] https://docs.rs/cargo/latest/cargo/core/compiler/enum.CompileKind.html
//!     - Target
//!     - Cross compilation (special case target)
//!     - Workspace or single crate build
//!
//! # Considerations
//!
//! This is in lieu of a better solution from cargo directly. In particular, it's worth noting that
//! [build scripts should not modify any files outside the OUT_DIR directory](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
//! We're not modifying, but it's still not necessarily in the "spirit" of the instructions.
//!
//! # How it Works
//!
//! To locate the target directory, we must know the project root and the target.
//!
//! 1. Determine if the output directory was overridden with `env:CARGO_TARGET_DIR`
//!     - If yes, use that path directly
//!     - If not, the path will default to `{workspace_root}/target` which we determine using [project_root](https://docs.rs/project-root/latest/project_root/).
//! 2. Determine which [`CompileKind`](https://docs.rs/cargo/latest/cargo/core/compiler/enum.CompileKind.html) is used. Cargo doesn't expose this directly.
//!     This crate is intended to be used in build scripts. Cargo provides `env:OUT_DIR` for build scripts. This isn't where we want to place these assets,
//!     but it does allow us to infer the `CompileKind`. If the triple + profile (`{target}/{profile}`) appears in the path, then `CompileKind::Target` was used.
//!     Otherwise, `CompileKind::Host` was used. Profile comes from `env:Profile` and target from [build_target::target_triple](https://docs.rs/build-target/latest/build_target/fn.target_triple.html).
//!     - For `CompileKind::Host` we concatenate `/{profile}`
//!     - For `CompileKind::Target` we concatenate `/{target}/{profile}`
//!

extern crate core;

use anyhow::{anyhow, Result};
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use project_root::get_project_root;
use std::env;
use std::path::{Path, PathBuf};

/// Copies files to output recursively
pub fn copy_to_output(path: &str) -> Result<()> {
    copy_to_output_for_profile(path, &env::var("PROFILE")?)
}

/// Copies files to output recursively
///
/// # Arguments
///
/// * `profile` - Manually specify the profile (i.e. env:PROFILE). Default is `debug` or `release`.
pub fn copy_to_output_for_profile(path: &str, profile: &str) -> Result<()> {
    // `CARGO_TARGET_DIR` is only set for CompileKind::Target. If set, respect it else use default.
    let mut out_path = if let Ok(custom_target_dir) = env::var("CARGO_TARGET_DIR") {
        let mut out_path = PathBuf::new();
        out_path.push(custom_target_dir);
        out_path
    } else {
        let mut out_path = get_project_root()?;
        out_path.push("target");
        out_path
    };

    // This is a hack, ideally we would plug into https://docs.rs/cargo/latest/cargo/core/compiler/enum.CompileKind.html
    // However, since the path follows predictable rules https://doc.rust-lang.org/cargo/guide/build-cache.html
    // we can just check our parent path for the pattern {target|env:CARGO_TARGET_DIR}/{triple}/{profile}.
    // If it is present, we know CompileKind::Target was used, otherwise CompileKind::Host was used.
    let triple = build_target::target_triple()?;
    let compile_kind_is_target = env::var("OUT_DIR")
        .expect(
            "env `OUT_DIR` not set by cargo. Are you running this function inside a build script?",
        )
        .contains(&format!(
            "{}{}{}",
            triple,
            std::path::MAIN_SEPARATOR,
            profile
        ));

    if compile_kind_is_target {
        out_path.push(triple);
    }

    out_path.push(profile);

    // Overwrite existing files with same name
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy_items(&[path], &out_path, &options)?;

    Ok(())
}

/// Copies files to output recursively
pub fn copy_to_output_by_path(path: &Path) -> Result<()> {
    copy_to_output(path_to_str(path)?)
}

fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or(anyhow!("Could not convert path to string"))
}

/// Copies files to output recursively
///
/// # Arguments
///
/// * `profile` - Manually specify the profile (i.e. env:PROFILE). Default is `debug` or `release`.
pub fn copy_to_output_by_path_for_profile(path: &Path, profile: &str) -> Result<()> {
    copy_to_output_for_profile(path_to_str(path)?, profile)
}

/// Emits [cargo:rerun-if-changed](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed).
/// NOTE: Once any `rerun-if-changed` is emitted, only the files specified are monitored. You can emit multiple times.
pub fn cargo_rerun_if_changed(path: &str) {
    println!("cargo:rerun-if-changed={}", path)
}

/// Emits [cargo:rerun-if-changed](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed).
/// NOTE: Once any `rerun-if-changed` is emitted, only the files specified are monitored. You can emit multiple times.
pub fn cargo_rerun_if_path_changed(path: &Path) -> Result<()> {
    cargo_rerun_if_changed(
        path.to_str()
            .ok_or(anyhow!("Could not convert project root path to string"))?,
    );
    Ok(())
}
