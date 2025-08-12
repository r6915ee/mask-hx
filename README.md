# mask-hx

> *Generic [Haxe](https://haxe.org) version manager*

## About

`mask-hx` comes from the fact that the [Haxe](https://haxe.org) toolkit
has versions that differ very much, especially in syntax, even in minor
releases.  `mask-hx` aims to reduce the amount of struggle needed when
using projects that require a specific version by providing a handy, simple
interface to access your versions, all in one place.

## Installation

`mask-hx` can be installed from GitHub releases. You should rename the
binary to `mask-hx` and should make sure that it's in the `PATH`.

However, this is generally not preferred in some cases, as it takes some
additional time to set up. Because of this, you can use
[Cargo](https://crates.io) to install the binary, as you would with any
other crate:

```sh
# Install from Crates.io
cargo install mask-hx
# Install from Git
cargo install --git https://github.com/r6915ee/mask-hx.git
```

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
subfolder.

### Basic usage

Right when you view the help message, a particular flag stands out: the
`explicit` argument. This argument is one of three ways to specify a Haxe
version when using `mask-hx`, alongside the configuration mechanism and the
`MASK_VERSION` environment variable.

The argument takes the value of the version number of whatever Haxe
version you want to use, or in other terms, the filename of the subfolder
mentioned earlier. It overrides both other mechanisms. However, it's perhaps
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

The `MASK_VERSION` environment variable is a particularly special case.
Some Haxelibs register command-line aliases for themselves. However, the
`explicit` argument typically cannot be used in those aliases. To work around
this, the `MASK_VERSION` environment variable can be set to override
the configuration file during that run, or even work globally.

Do note, however, the order in which each system is used. The `explicit`
argument always comes first, and if that fails, then the `MASK_VERSION`
environment variable is checked; if that also fails, then the configuration
file will be used. Most subcommands will typically fail with a lack of a
configuration file afterwards.

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

## Tips and tricks

* If you want to make sure that your setup is ready for developing on a
  project containing a `.mask` configuration, then you can use the `check`
  subcommand. It can check all three version mechanisms.
* Including a `.mask` file in your source code repository is especially useful
  when you and your team are using a remote repository. They can make sure
  that everyone is using the right Haxe version.
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
