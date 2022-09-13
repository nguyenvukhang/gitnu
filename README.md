# gitnu

gitnu is script that adds numbers to git status.

## Install

gitnu can be installed by running `cargo install gitnu`.

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
$ gitnu add 2 5-7 # same as `gitnu add 2 5 6 7`
```

You can even mix file names with numbers.

One thing unique to gitnu that git doesn't do is you can map files to
commands:

```bash
gitnu -c rm 1 2 # same as `rm <file 1> <file 2>`
```
