# Contributing to Gitnu

## Main logic flow

1. **Preprocessing** (fallible)
   1. Get the path to the [git directory](#git-dir) relative to the
      current directory.
   2. Get [git aliases][git-aliases].
   3. Read the [cache file](#gitnu-cache-file) (regardless of git
      command).
2. **Parsing** (infallible)
   1. Pass the CLI arguments through a function to obtain a final list
      of arguments to actually run.
   2. This is where numbers and number-ranges are converted to
      pathspecs, depending on the [cache](#gitnu-cache-file).
3. **Running** (fallible)
   1. If git's sub-command is `status`, then print and parse the
      output to build the next cache.
   2. Otherwise, run the args as-is.

### Error handling

1. if **Preprocessing** fails:  
   Pass the raw CLI arguments into a `git` command and run that and
   use the same exit code.
2. if **Running** fails:  
   Non-issue. Failing here means there was probably something wrong
   with the user-supplied command.

### Non-trivial behavior

#### How does `git nu` manage to call `git-nu`

Based on git's [source code][git-source], in the function
`execv_dashed_external`, any executable that is in `$PATH` that is
named `git-foo-bar` can be ran with `git foo-bar`.

Additionally, flags placed after `git` and before `foo-bar` will be
seen by `git` but not by `git-foo-bar`.

#### Parsing arguments after `git` and before `nu`

Amazingly, this is not necessary as `git` will handle all flags in
this range.

For example, the `-C` flag will make git run from a different
directory, and that will be accounted for by `git`, and not `git-nu`.

Same with configurations set with the `-c` flag.

So running this command from `/foo`

```
> git -C /bar -c alias.zoom=ls-files nu zoom
```

Will actually run `git ls-files` on the repo located at `/bar`.

The caveat is that in code, `git-nu` sees `zoom` as the only argument
(apart from the binary path), and will effectively return `git zoom`.

Setting the directory (from `-C`) and aliasing `zoom` to `ls-files`
(from `-c`) are managed by `git`.

## Glossary

#### `git-dir`

This is the directory where the `HEAD` file is stored. On a normal
clone, it is at `.git/HEAD`, and in bare repositories it's in
`.git/worktrees/<worktree name>/HEAD`. So depending on the state of
the repo, the value of `git-dir` may be one of the following:

- `.git`
- `.git/worktrees/<worktree name>`

`git-dir` can be found by running the command `git rev-parse
--git-dir` from anywhere in the git workspace.

#### Gitnu cache file

Stored in the [git directory](#git-dir). It stores the output of the
last run of `git status` in that workspace.

The first line is the working directory from which `git status` was
ran.

The remaining lines are the ordered pathspecs of that run of `git
status`.

[git-aliases]: https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases
[git-source]: https://github.com/git/git/blob/master/git.c
