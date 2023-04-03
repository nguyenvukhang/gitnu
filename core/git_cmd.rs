// found from running `git help --all`
#[rustfmt::skip]
pub fn is_default_git_cmd(token: &str) -> bool {
    match token {
        "add" | "am" | "annotate" | "apply" | "archimport" | "archive"
        | "attributes" | "bisect" | "blame" | "branch" | "bugreport" | "bundle"
        | "cat-file" | "check-attr" | "check-ignore" | "check-mailmap"
        | "check-ref-format" | "checkout" | "checkout-index" | "cherry"
        | "cherry-pick" | "citool" | "clean" | "cli" | "clone" | "column"
        | "commit" | "commit-graph" | "commit-tree" | "config" | "count-objects"
        | "credential" | "credential-cache" | "credential-store"
        | "cvsexportcommit" | "cvsimport" | "cvsserver" | "daemon" | "describe"
        | "diagnose" | "diff" | "diff-files" | "diff-index" | "diff-tree"
        | "difftool" | "fast-export" | "fast-import" | "fetch" | "fetch-pack"
        | "filter-branch" | "fmt-merge-msg" | "for-each-ref" | "for-each-repo"
        | "format-bundle" | "format-chunk" | "format-commit-graph"
        | "format-index" | "format-pack" | "format-patch" | "format-signature"
        | "fsck" | "gc" | "get-tar-commit-id" | "gitk" | "gitweb" | "grep"
        | "gui" | "hash-object" | "help" | "hook" | "hooks" | "http-backend"
        | "ignore" | "imap-send" | "index-pack" | "init" | "instaweb"
        | "interpret-trailers" | "log" | "ls-files" | "ls-remote" | "ls-tree"
        | "mailinfo" | "mailmap" | "mailsplit" | "maintenance" | "merge"
        | "merge-base" | "merge-file" | "merge-index" | "merge-one-file"
        | "merge-tree" | "mergetool" | "mktag" | "mktree" | "modules"
        | "multi-pack-index" | "mv" | "name-rev" | "notes" | "p4"
        | "pack-objects" | "pack-redundant" | "pack-refs" | "patch-id"
        | "protocol-capabilities" | "protocol-common" | "protocol-http"
        | "protocol-pack" | "protocol-v2" | "prune" | "prune-packed" | "pull"
        | "push" | "quiltimport" | "range-diff" | "read-tree" | "rebase"
        | "reflog" | "remote" | "repack" | "replace" | "repository-layout"
        | "request-pull" | "rerere" | "reset" | "restore" | "rev-list"
        | "rev-parse" | "revert" | "revisions" | "rm" | "scalar" | "send-email"
        | "send-pack" | "sh-i18n" | "sh-setup" | "shortlog" | "show"
        | "show-branch" | "show-index" | "show-ref" | "sparse-checkout"
        | "stash" | "status" | "stripspace" | "submodule" | "svn" | "switch"
        | "symbolic-ref" | "tag" | "unpack-file" | "unpack-objects"
        | "update-index" | "update-ref" | "update-server-info" | "var"
        | "verify-commit" | "verify-pack" | "verify-tag" | "version"
        | "whatchanged" | "worktree" | "write-tree" => true,
        _ => false,
    }
}
