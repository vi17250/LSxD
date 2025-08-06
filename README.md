# LSxD
> It's like LS, but with a D 😛

A simple and naïve implementation of two unix commands ([`LS`](https://www.linux.org/docs/man1/ls.html) and [`DU`](https://www.linux.org/docs/man1/du.html)) written in [Rust](https://www.rust-lang.org/) ❤️

## Key features
- Displays the contents of a folder (including files and directories)
- Displays the human readable size of files
- Displays the human readable size of subdirectories
- Can be recursive (using `-d` argument)

## Screenshots

![lsxd without options](https://github.com/user-attachments/assets/2a919e89-0482-49ce-bf6d-e6f6231b7727)

![lsxd without -d option](https://github.com/user-attachments/assets/c81c677c-8405-4df5-98d2-13eb27cbe1da)

## Limitation

Due to the way the Linux file system works, folder sizes are rounded to the block size (4.0kb)
