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

/// Print to the standard output.
///
/// This macro functions identically to the [println] macro, except
/// it compares a required [OutputLevel] and the current [OutputLevel] to
/// see if the latter is greater or equal to the required output level,
/// and only printing if this comparison succeeds.
///
/// Additionally, `text` can be an expression. This is useful for concatenation
/// reasons, but more importantly, that means that the [format] macro can
/// be used as the value.
///
/// # Examples
///
/// ```
/// let current_level: OutputLevel = OutputLevel::Normal;
///
/// print_to_stdout!(OutputLevel::Normal, current_level, "The current output level is greater than the required output level");
/// ```
macro_rules! print_to_stdout {
    ($required_level: expr, $current_level: expr, $text: literal) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };

    ($required_level: expr, $current_level: expr, $text: expr) => {
        if $current_level as u8 >= $required_level as u8 {
            println!("{}", $text);
        }
    };
}

/// Defines the "output level" of various functions.
///
/// [OutputLevel] is useful to define how the program should
/// print to the standard output. It is ignored in some cases.
#[derive(Clone)]
pub enum OutputLevel {
    /// Only the bare minimum will be printed.
    Quiet,
    /// Some printing will be performed. However, it doesn't expose certain information.
    Normal,
    /// Print everything that is thrown.
    Verbose,
}

/// Handles Haxe versions.
///
/// [fetcher] is used for a lot of operations related to Haxe versions.
/// It is able to validate the existence of Haxe versions, pass their
/// data, and more.
pub mod fetcher;
