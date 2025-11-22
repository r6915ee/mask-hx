# Contributing

If you're looking for some recommendations on contributing, then you've come to
the right place.

## Nix

Specialized Nix environments are available to work on
[NixOS](https://nixos.org/). A Nix shell and basic FHS environment are
available for use, and can be used as you typically would with most other
projects supporting the Nix workflow.

Do be aware, however, that the Nix environments provided do not support Flakes
for compatibility reasons.

## Development Tools

The Nix shell will automatically drop in most tools necessary for development.
For other systems, the following programs should be available for efficient
development:

* [The Rust toolchain](https://rust-lang.org/), including **rust-analyzer**
* [pre-commit](https://pre-commit.com/)
* [just](https://just.systems/)
* [just-formatter](https://github.com/eli-yip/just-formatter)
* [just-lsp](https://github.com/terror/just-lsp)

All dependencies of these programs should be available as well, including Haxe
itself. Both the Nix shell and installing the programs manually are valid
methods of using these development tools.

## Git Hooks

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
