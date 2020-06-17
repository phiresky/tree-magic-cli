# tree-magic-cli

Since [tree-magic-mini](tree-magic-cli) removes [the integrated cli](https://github.com/mbrubeck/tree_magic/commit/391a592d65100e2e473228c14d555c245451694c), this is that same CLI in a standalone crate.

## Installation

```
cargo install tree-magic-cli
```

## Usage

    tmagic 1.0.0
    Determines the MIME type of a file by traversing a filetype tree.

    USAGE:
        tmagic [FLAGS] [OPTIONS] <file>...

    FLAGS:
        -h, --help         Prints help information
        -r, --recursive    Search directories recursively
            --ugly         Print results as they come in, at expense of tab alignment
        -V, --version      Prints version information

    OPTIONS:
        -m, --match=<match>    Print only files that match given MIMEs, seperated by commas

    ARGS:
        <file>...    List of files or folders to check. Wildcards supported.
