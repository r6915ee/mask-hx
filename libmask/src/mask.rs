//! # libmask
//!
//! `libmask` aims to simplify [Haxe](https://haxe.org)
//! development by providing version management capabilities.
//!
//!
//! [Haxe](https://haxe.org), unlike other toolchains and
//! programming languages, lacks unified compatibility between
//! versions; to work around this, complex systems need to
//! be set up to actually use good version management.
//!
//! `libmask` aims to provide an interface to allow programs
//! to do this easily. The de facto standard in programs that
//! use this library is `mask-hx`, which is its parent project,
//! but `libmask` is usable by anyone.

/// Handles configurations.
///
/// [config] is responsible for `.mask` configurations.
/// It acts as an interface to them.
pub mod config;
/// Handles executable launching.
///
/// [exec] handles running [Haxelib](https://lib.haxe.org) and the compiler.
/// [Haxelib](https://lib.haxe.org) is the official package manager for Haxe,
/// so most projects use it.
pub mod exec;
/// Handles Haxe versions.
///
/// [fetcher] is used for a lot of operations related to [Haxe](https://haxe.org)
/// versions. It is able to validate the existence of Haxe versions, pass their
/// data, and more.
pub mod fetcher;
