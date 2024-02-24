<div align=center>

# TypeDown ✒️

 [![NixOS](https://img.shields.io/badge/Flakes-Nix-informational.svg?logo=nixos&style=for-the-badge)](https://nixos.org) ![License](https://img.shields.io/github/license/mordragt/type-down?style=for-the-badge) 

Simple Markup language, easily embeddable and extendable.

</div>

## About

TypeDown is a simple markup language wich takes inspiration from [Typst](https://typst.app/)
but focuses on being embeddable into your own projects.
Therefore it is better compared to Markdown.
If you would like to take a look on how TypeDown looks at the moment, see the `ok.typ` file under the `examples` folder.
While the syntax is still subject to change TypeDown will look similar on release.

## Installation

TypeDown is currently still in development and thus isn't packaged.
But in the future it will be packaged with Nix.
In the meantime you can use cargo to install TypeDown by cloning
this repository and running `cargo install --path .`.
Note that while there exists an experimental self-contained html backend,
by default pandoc is required.

## Usage

- `tyd check <path>`: checks the provided document and returns the generated ast.
- `tyd format <path>`: formats the provided document by printing to stdout.
- `tyd compile <html, docx, pdf, json> <source> [destination]`: Compiles the document to one of the provided formats and if a destination is provided saves the corresponding file there. Note that for pdf and docx you must provide a destination.

## Reference

1. [Typst](https://typst.app/)

