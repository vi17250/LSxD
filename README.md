# LSxD
> It's like LS, but with a D 😛

A simple and naïve implementation of two unix commands ([`LS`](https://www.linux.org/docs/man1/ls.html) and [`DU`](https://www.linux.org/docs/man1/du.html)) written in [Rust](https://www.rust-lang.org/) ❤️

## Table of Contents
- [Features](#features)
- [Screenshots](#screenshots)
- [Pre-requisites](#pre-requisites)
- [Install](#install)
- [Limitation](#limitation)

## Features
- Displays the contents of a folder (including files and directories)
- Displays the human readable size of files
- Displays the human readable size of subdirectories
- Can be recursive (using `-d` argument)

## Screenshots

![lsxd without options](https://github.com/user-attachments/assets/2a919e89-0482-49ce-bf6d-e6f6231b7727)

![lsxd without -d option](https://github.com/user-attachments/assets/c81c677c-8405-4df5-98d2-13eb27cbe1da)

## Pre-requisites

1. [Install `rustup`](https://www.rust-lang.org/tools/install) to compile the source code for our device/os

## Install

1. Clone this repository:

   ```bash
   git clone `https://github.com/vi17250/lsxd`
   ```
    ```bash
    cd lsxd
    ```

2. Build `lsxd` binary 
    ```bash
    cargo build
    ```

3. Install `lsxd` binary:

   ```bash
    cp /target/debug/lsxd /opt/lsxd
   ```

4. Run `lsxd` from everywhere on your computer 
    ```bash
     echo export PATH=$PATH:/opt/lsxd/ >> {PATH/TO/YOUR/.BASHRC_FILE}
    ```
## Limitation

Due to the way the Linux file system works, folder sizes are rounded to the block size (4.0kb)
