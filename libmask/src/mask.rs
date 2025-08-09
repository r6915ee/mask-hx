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

/// Handles Haxe versions.
///
/// [fetcher] is used for a lot of operations related to Haxe versions.
/// It is able to validate the existence of Haxe versions, pass their
/// data, and more.
pub mod fetcher;
