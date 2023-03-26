# Chapter 01 : Getting Started

In this chapter, we discuss:

- Installing Rust on Linux, macOS, and Windows
- Writing a program that prints `Hello, world!`
- Using `cargo`, Rust’s package manager and build system

## Installation

### Installing rustup on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust.

You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```bash
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.

```bash
sudo apt-get install build-essential
```

## Installing rustup on Windows

On Windows, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for installing Rust.

When asked which workloads to install, include:

- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing

To check whether you have Rust installed correctly, open a shell and enter this line:

```bash
rustc --version
```
