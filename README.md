# mask-hx

> *Generic [Haxe](https://haxe.org/) version manager*

## About

`mask-hx` comes from the fact that the [Haxe](https://haxe.org/) toolkit
has versions that differ substantially, especially in syntax, even in minor
releases. `mask-hx` aims to reduce the amount of struggle needed when
using projects that require a specific version by providing a handy, simple,
and transparent interface to access your versions individually, all in one
place.

## Installation

The primary way to install `mask-hx`, as of v0.2.0, is to use the
[Cargo](https://crates.io/) package manager to install the binary:

```sh
# Install from Crates.io
cargo install mask-hx
# Install from Git
cargo install --git https://codeberg.org/r6915ee/mask-hx.git
```

### NixOS Users

[NixOS](https://nixos.org/) users have their own way to install the program,
due to the lack of the usage of Cargo as a package manager, and the fact that
a specialized FHS environment needs to be built.

The repository contains a Nix package in the [`package.nix`](./package.nix)
file that can be loaded using the following example in the `configuration.nix`
file:

```nix
{ pkgs, ... }:
{
  # . . .
  environment.systemPackages = [
    # . . .
    (pkgs.callPackage "${pkgs.fetchgit {
      url = "https://codeberg.org/r6915ee/mask-hx.git";
      rev = "v0.2.0";
      # Ensure that you update the hash when performing an update and
      # installing for the first time!
      hash = "";
    }}/default.nix" {})
  ];
  # . . .
}
```

This calls the package provided by the repository, and then registers it as a
system package. The package is provided in an FHS environment that encompasses
both the output of `nixpkgs.rustPlatform.buildRustPackage` and Nixpkgs's Neko
package. This environment allows running FHS-only programs, such as the Haxe
binaries that `mask-hx` provides a layer for. This allows `mask-hx` to work
properly on NixOS.

## Usage

### Haxe versions

Haxe versions are located in the home directory, under a subfolder called
`.haxe` (in UNIX terms, the shortcut to this path is `~/.haxe`). Haxe versions
should preferrably be in a portable state.

Each Haxe version should be located in its own subfolder, and the filename
of the subfolder is the version number, as that's the standard (though
nothing's stopping you from naming it whatever you like). As an example, Haxe
[v4.2.5](https://haxe.org/download/version/4.2.5/) would be in the `4.2.5`
subfolder. The compiler and Haxelib need to be located in the root of this
subfolder, and so does the standard library. Installations should preferrably
be portable and not managed by an external program, examples of such kinds of
programs being Windows installers and package managers.

### Version Usage

Right when you view the help message, a particular flag stands out: the
`explicit` argument. This argument is one of three ways to specify a Haxe
version when using `mask-hx`, alongside the configuration system and the
`MASK_VERSION` environment variable.

```sh
mask-hx -e 4.2.5 exec
```

The argument takes the value of the version number of whatever Haxe
version you want to use, or in other terms, the filename of the subfolder
mentioned earlier. It overrides both other variants. However, it's perhaps
the rarest one.

The configuration mentioned earlier can be instantiated by using the
`switch` subcommand:

```sh
mask-hx switch 4.2.5
```

This does a couple of things:

1. The program checks if the specified version number is installed; if it is,
  then continue
2. A `.mask` file is created in the working directory if it doesn't exist
3. The `.mask` file is overwritten to the specified version number

`.mask` files tell `mask-hx` what Haxe version to use. At a maximum, they
can simply be files that contain the version number as a string. They are
useful for collaborative projects.

In addition, you can also specify an external configuration file using the
`config` flag. You can operate the program using the file provided instead of
the default `.mask` file.

```sh
mask-hx -c .lib.mask 4.2.5
```

The `MASK_VERSION` environment variable is a particularly special case.
Some Haxelibs register command-line aliases for themselves. However, the
`explicit` argument typically cannot be used in those aliases. To work around
this, the `MASK_VERSION` environment variable can be set to override
the configuration file during that run, or even work globally.

Do note, however, the order in which each system is used. The `explicit`
argument always comes first, and if that fails, then the `MASK_VERSION`
environment variable is checked; if that also fails, then the configuration
file will be used. `mask-hx` will automatically fail on most subcommands if a
Haxe version is not provided.

All three of these methods are valid **version reference** methods in the
terminology of `mask-hx`, which allow storing the version number to be used
for later.

### Invoking the compiler and Haxelib

Invoking the compiler can be done by using the `exec` subcommand:

```sh
mask-hx exec --help
```

Likewise, Haxelib can be invoked by using the `lib` subcommand:

```sh
mask-hx lib help
```

Both of these subcommands also override the `PATH` environment variable
for the programs they invoke to make sure that the Haxe and Haxelib
binaries that may be used during dependency installation are the right
ones, allowing certain frameworks like [Lime](https://lime.openfl.org/)
to function properly.

> [!NOTE]
> Every argument after `exec` and `lib` will typically be absorbed by the
> program being invoked!

## Development

The Nix shell will automatically drop in most tools necessary for development.

`mask-hx` is developed in [Rust](https://rust-lang.org/). Although it's
possible to develop using only standalone packages, most users may want to go
with Rustup to manage Rust, should they not be on a system that requires
packages to be from a single package manager.

### Building

`mask-hx` is simple to build. Although Cargo will automatically compile it when
installing, the actual operation is fast, and doesn't naturally depend on
most non-Rust libraries.

For recipe purposes, [Just](https://just.systems/) is employed. The most common
recipes are `build` and `doc`, each being an alias for certain operations in a
sequence:

```sh
just build # cargo build
just doc # cargo test --doc; cargo doc
```

### Contributing

First, ensure you have [`pre-commit`](https://pre-commit.com/) installed on
your system. You will then need to install all of the hooks:

```sh
pre-commit install
pre-commit install --hook-type commit-msg
```

This ensures that you perform tests, check your commit messages to follow
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), and
more before each commit.

Finally, you can fork and contribute as you normally would with any other Git
project.

## Tips and tricks

* If you want to make sure that your setup is ready for developing on a
  project containing a `.mask` configuration, then you can use the `check`
  subcommand. It can check all three version mechanisms.
* Including a `.mask` file in version control is especially useful when you
  and your team are using a remote repository. They can make sure that everyone
  is using the right Haxe version.
* You can set up command-line aliases to `mask-hx`'s respective subcommands.
  If you're using shell scripts, then a few examples for each command are
  available in the `examples/` subfolder of the repository, supporting typical
  Bash. Alongside this, you gain the benefit of certain programs like
  [Starship](https://starship.rs/) displaying the current Haxe version.
* The program contains `libmask`, the main functionality behind most of the
  work. `libmask` is a separate crate that can be used in other projects under
  the terms of the [MIT license](LICENSE), just like `mask-hx`.

## License

`mask-hx` and its library component, `libmask`, are both licensed under the
[MIT license](LICENSE).
