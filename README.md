# gitnu

gitnu adds numbers to git status.

[![build status](https://github.com/nguyenvukhang/gitnu/workflows/ci/badge.svg?branch=main)](https://github.com/nguyenvukhang/gitnu/actions?query=branch%3Amain)
[![crates.io](https://img.shields.io/crates/d/gitnu?color=brightgreen)](https://crates.io/crates/gitnu)

## Install

gitnu can be installed by running `cargo install gitnu`.

## Usage

```bash
$ git nu status
# On branch master
# Untracked files:
# 1       .gitignore
# 2       README.md
# 3       doc/
# 4       src/
#
# nothing added to commit but untracked files present
```

Note the similarity of the output `git nu status` to that of `git status`.  
They are identical except for the numbers in front of filenames.

After `gitnu status`, you can now use numbers in place of filenames for git
commands:

```bash
$ git nu add 2
$ git nu status
# On branch master
# Changes to be committed:
# 1       new file:   README.md
#
# Untracked files:
# 2       .gitignore
# 3       doc/
# 4       src/
```

In general, whenever you used to do

```
git <command> [filenames and arguments]
```

You can now use

```
git nu <command> [file numbers and arguments]
```

gitnu will silently replace numbers with their filenames and pass everything
else intact into git.

gitnu accepts multiple arguments and even number ranges:

```bash
$ git nu add 2 5-7 # same as `git nu add 2 5 6 7`
```

You can even mix file names with numbers.
