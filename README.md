# LSxD

A simple and naïve implementation of two unix commands ([`LS`](https://www.linux.org/docs/man1/ls.html) and [`DU`](https://www.linux.org/docs/man1/du.html)) written in [Rust](https://www.rust-lang.org/) ❤️

## Table of Contents
- [Features](#features)
- [Screenshots](#screenshots)
- [Install](#install)
- [Arguments](#arguments)
- [Limitation](#limitation)

## Features
- Displays the contents of a folder (including files and directories)
- Displays the human readable size of files
- Displays the human readable size of subdirectories
- Can be recursive (using `-d` argument)
- Sort by size (using `-s` argument)

## Screenshots

![lsxd without options](https://github.com/user-attachments/assets/2a919e89-0482-49ce-bf6d-e6f6231b7727)

![lsxd without -d option](https://github.com/user-attachments/assets/c81c677c-8405-4df5-98d2-13eb27cbe1da)

## Install

### Using cargo:

    ```bash
        cargo install lsxd
    ```

### Using GitHub releases

1. Download `lsxd` from [releases page](https://github.com/vi17250/lsxd/releases)

2. Unzip the .tar.gz archive
    ```bash
   tar -zxvf {ARCHIVE.tar.gz} 
    ```

3. Install `lsxd` binary:

   ```bash
    install -m 111 lsxd /opt/lsxd
   ```

4. Run `lsxd` from everywhere on your computer 
    ```bash
     echo export PATH=$PATH:/opt/lsxd/ >> {PATH/TO/YOUR/.BASHRC_FILE}
    ```

## Arguments

|Arg|Description|Default|Example|
|-|-|-|-|
|`-d`|Display depth|0|-d 2|
|`-s`|Sort by size|false|-s|

## Limitation

Due to the way the Linux file system works, folder sizes are rounded to the block size (4.0kb)
