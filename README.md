# gitnu

gitnu is script that adds numbers to git status.

## Install

gitnu can be installed by running `pip install gitnu`.

<!-- TODO: add required Python version -->

If you can't wait for the absolute blazingly latest awesomeness and want to install directly from GitHub, use:

```
pip install git+https://github.com/nguyenvukhang/gitnu
```

## Usage

```bash
$ gitnu status
# On branch master
# Untracked files:
# 1       .gitignore
# 2       README.md
# 3       doc/
# 4       src/
#
# nothing added to commit but untracked files present
```

Note the similarity of the output `gitnu status` to that of `git status`.  
They are identical except for the numbers in front of filenames.

After `gitnu status`, you can now use numbers in place of filenames for git
commands:

```bash
$ gitnu add 2
$ gitnu
# On branch master
# Changes to be committed:
# 1       new file:   README.md
#
# Untracked files:
# 2       .gitignore
# 3       doc/
# 4       src/
```

(`gitnu` without arguments defaults to `gitnu status`)

In general, whenever you used to do

```
git <command> [filenames and arguments]
```

You can now use

```
gitnu <command> [file numbers and arguments]
```

gitnu will silently replace numbers with their filenames and pass everything
else intact into git.

gitnu accepts multiple arguments and even number ranges:

```bash
$ gitnu add 1 2 5-8
```

has the same result as typing

```bash
$ gitnu add 1 2 5 6 7 8
```

<!-- [gitnu-status]: https://github.com/nguyenvukhang/gitnu/blob/dev/doc/gitnu-status.jpg?raw=true -->
<!-- [gitnu-add]: https://github.com/nguyenvukhang/gitnu/blob/dev/doc/gitnu-add.jpg?raw=true -->
<!-- [gitnu-range]: https://github.com/nguyenvukhang/gitnu/blob/dev/doc/gitnu-range.jpg?raw=true -->
<!-- [gitnu-diff]: https://github.com/nguyenvukhang/gitnu/blob/dev/doc/gitnu-diff.jpg?raw=true -->
