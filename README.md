# Rust CLI Tool

A small Rust CLI with an emphasis on Linux and creating automation tools that solve a problems. This was the basis to help develop my understanding of DevOps principles.

The original tool was adapted from a Python and Rust CLI tools course I took:
- [Rust CLI](https://github.com/alfredodeza/rust-cli-example)
- [included practice lab](./lab.md)

## Environment and Dev Workflow
Rust development requires certain tools to be installed on your system. See the links above if you need help setting up. Otherwise continue below.

As part of your development workflow, I use the following programs in the terminal regularly. These can be accessed via the Makefile using 'make help' or just 'make':

- `make fmt` `cargo fmt` - Formats your code to the Rust standard
- `make clippy` - Lints your code and helps you find errors and potential issues
- `make check` - Checks your code for errors and allows you to fix them before compiling (which means its faster!)
