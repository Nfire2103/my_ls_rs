## Introduction

This project is my first projet in rust. It is a reimplementation of the `ls` command in rust. It is a simple project that I did to learn rust. I handle the following options `-alRtrd` and colors depending on file type or file extension.

## Installation

To install the project you can use the following command:

```bash
cargo install --path .
```

## Examples

Here are some examples of commands that you can do:

```bash
my_ls_rs README.md TODO src/
```

```bash
my_ls_rs -a ~
```

```bash
my_ls_rs -l /dev
```

```bash
my_ls_rs -R src/
```

```bash
my_ls_rs -alRtr
```
