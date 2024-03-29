use crate::{prelude::Aliases, Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum GitStatus {
    Short,
    Normal,
}

impl GitStatus {
    pub fn short(&mut self) {
        *self = GitStatus::Short;
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub (crate)enum GitCommand {
    // full list found from running `git help --all`
    Status(GitStatus),
    Add, Am, Annotate, Apply, Archimport, Archive, Attributes, Bisect, Blame,
    Branch, Bugreport, Bundle, CatFile, CheckAttr, CheckIgnore, CheckMailmap,
    CheckRefFormat, Checkout, CheckoutIndex, Cherry, CherryPick, Citool, Clean,
    Cli, Clone, Column, Commit, CommitGraph, CommitTree, Config, CountObjects,
    Credential, CredentialCache, CredentialStore, Cvsexportcommit, Cvsimport,
    Cvsserver, Daemon, Describe, Diagnose, Diff, DiffFiles, DiffIndex, DiffTree,
    Difftool, FastExport, FastImport, Fetch, FetchPack, FilterBranch,
    FmtMergeMsg, ForEachRef, ForEachRepo, FormatBundle, FormatChunk,
    FormatCommitGraph, FormatIndex, FormatPack, FormatPatch, FormatSignature,
    Fsck, Gc, GetTarCommitId, Gitk, Gitweb, Grep, Gui, HashObject, Help, Hook,
    Hooks, HttpBackend, Ignore, ImapSend, IndexPack, Init, Instaweb,
    InterpretTrailers, Log, LsFiles, LsRemote, LsTree, Mailinfo, Mailmap,
    Mailsplit, Maintenance, Merge, MergeBase, MergeFile, MergeIndex,
    MergeOneFile, MergeTree, Mergetool, Mktag, Mktree, Modules, MultiPackIndex,
    Mv, NameRev, Notes, P4, PackObjects, PackRedundant, PackRefs, PatchId,
    ProtocolCapabilities, ProtocolCommon, ProtocolHttp, ProtocolPack,
    ProtocolV2, Prune, PrunePacked, Pull, Push, Quiltimport, RangeDiff,
    ReadTree, Rebase, Reflog, Remote, Repack, Replace, RepositoryLayout,
    RequestPull, Rerere, Reset, Restore, RevList, RevParse, Revert, Revisions,
    Rm, Scalar, SendEmail, SendPack, ShI18n, ShSetup, Shortlog, Show,
    ShowBranch, ShowIndex, ShowRef, SparseCheckout, Stash, Stripspace,
    Submodule, Svn, Switch, SymbolicRef, Tag, UnpackFile, UnpackObjects,
    UpdateIndex, UpdateRef, UpdateServerInfo, Var, VerifyCommit, VerifyPack,
    VerifyTag, Version, WhatChanged, Worktree, WriteTree
}

impl GitCommand {
    pub fn skip_next_arg(&self, arg: &str) -> bool {
        use GitCommand as G;
        match (self, arg) {
            (G::Log, "-n") => true,
            _ => false,
        }
    }

    pub fn from_arg(aliases: &Aliases, arg: &str) -> Option<Self> {
        match Self::try_from(arg) {
            Ok(v) => Some(v),
            _ => match aliases.get(arg).map(Self::try_from) {
                Some(Ok(v)) => Some(v),
                _ => None,
            },
        }
    }
}

impl TryFrom<&String> for GitCommand {
    type Error = Error;
    fn try_from(arg: &String) -> Result<Self> {
        GitCommand::try_from(arg.as_str())
    }
}

impl TryFrom<&str> for GitCommand {
    type Error = Error;
    fn try_from(arg: &str) -> Result<Self> {
        use GitCommand::*;
        let command = match arg {
            "status" => Status(GitStatus::Normal),
            "add" => Add,
            "am" => Am,
            "annotate" => Annotate,
            "apply" => Apply,
            "archimport" => Archimport,
            "archive" => Archive,
            "attributes" => Attributes,
            "bisect" => Bisect,
            "blame" => Blame,
            "branch" => Branch,
            "bugreport" => Bugreport,
            "bundle" => Bundle,
            "cat-file" => CatFile,
            "check-attr" => CheckAttr,
            "check-ignore" => CheckIgnore,
            "check-mailmap" => CheckMailmap,
            "check-ref-format" => CheckRefFormat,
            "checkout" => Checkout,
            "checkout-index" => CheckoutIndex,
            "cherry" => Cherry,
            "cherry-pick" => CherryPick,
            "citool" => Citool,
            "clean" => Clean,
            "cli" => Cli,
            "clone" => Clone,
            "column" => Column,
            "commit" => Commit,
            "commit-graph" => CommitGraph,
            "commit-tree" => CommitTree,
            "config" => Config,
            "count-objects" => CountObjects,
            "credential" => Credential,
            "credential-cache" => CredentialCache,
            "credential-store" => CredentialStore,
            "cvsexportcommit" => Cvsexportcommit,
            "cvsimport" => Cvsimport,
            "cvsserver" => Cvsserver,
            "daemon" => Daemon,
            "describe" => Describe,
            "diagnose" => Diagnose,
            "diff" => Diff,
            "diff-files" => DiffFiles,
            "diff-index" => DiffIndex,
            "diff-tree" => DiffTree,
            "difftool" => Difftool,
            "fast-export" => FastExport,
            "fast-import" => FastImport,
            "fetch" => Fetch,
            "fetch-pack" => FetchPack,
            "filter-branch" => FilterBranch,
            "fmt-merge-msg" => FmtMergeMsg,
            "for-each-ref" => ForEachRef,
            "for-each-repo" => ForEachRepo,
            "format-bundle" => FormatBundle,
            "format-chunk" => FormatChunk,
            "format-commit-graph" => FormatCommitGraph,
            "format-index" => FormatIndex,
            "format-pack" => FormatPack,
            "format-patch" => FormatPatch,
            "format-signature" => FormatSignature,
            "fsck" => Fsck,
            "gc" => Gc,
            "get-tar-commit-id" => GetTarCommitId,
            "gitk" => Gitk,
            "gitweb" => Gitweb,
            "grep" => Grep,
            "gui" => Gui,
            "hash-object" => HashObject,
            "help" => Help,
            "hook" => Hook,
            "hooks" => Hooks,
            "http-backend" => HttpBackend,
            "ignore" => Ignore,
            "imap-send" => ImapSend,
            "index-pack" => IndexPack,
            "init" => Init,
            "instaweb" => Instaweb,
            "interpret-trailers" => InterpretTrailers,
            "log" => Log,
            "ls-files" => LsFiles,
            "ls-remote" => LsRemote,
            "ls-tree" => LsTree,
            "mailinfo" => Mailinfo,
            "mailmap" => Mailmap,
            "mailsplit" => Mailsplit,
            "maintenance" => Maintenance,
            "merge" => Merge,
            "merge-base" => MergeBase,
            "merge-file" => MergeFile,
            "merge-index" => MergeIndex,
            "merge-one-file" => MergeOneFile,
            "merge-tree" => MergeTree,
            "mergetool" => Mergetool,
            "mktag" => Mktag,
            "mktree" => Mktree,
            "modules" => Modules,
            "multi-pack-index" => MultiPackIndex,
            "mv" => Mv,
            "name-rev" => NameRev,
            "notes" => Notes,
            "p4" => P4,
            "pack-objects" => PackObjects,
            "pack-redundant" => PackRedundant,
            "pack-refs" => PackRefs,
            "patch-id" => PatchId,
            "protocol-capabilities" => ProtocolCapabilities,
            "protocol-common" => ProtocolCommon,
            "protocol-http" => ProtocolHttp,
            "protocol-pack" => ProtocolPack,
            "protocol-v2" => ProtocolV2,
            "prune" => Prune,
            "prune-packed" => PrunePacked,
            "pull" => Pull,
            "push" => Push,
            "quiltimport" => Quiltimport,
            "range-diff" => RangeDiff,
            "read-tree" => ReadTree,
            "rebase" => Rebase,
            "reflog" => Reflog,
            "remote" => Remote,
            "repack" => Repack,
            "replace" => Replace,
            "repository-layout" => RepositoryLayout,
            "request-pull" => RequestPull,
            "rerere" => Rerere,
            "reset" => Reset,
            "restore" => Restore,
            "rev-list" => RevList,
            "rev-parse" => RevParse,
            "revert" => Revert,
            "revisions" => Revisions,
            "rm" => Rm,
            "scalar" => Scalar,
            "send-email" => SendEmail,
            "send-pack" => SendPack,
            "sh-i18n" => ShI18n,
            "sh-setup" => ShSetup,
            "shortlog" => Shortlog,
            "show" => Show,
            "show-branch" => ShowBranch,
            "show-index" => ShowIndex,
            "show-ref" => ShowRef,
            "sparse-checkout" => SparseCheckout,
            "stash" => Stash,
            "stripspace" => Stripspace,
            "submodule" => Submodule,
            "svn" => Svn,
            "switch" => Switch,
            "symbolic-ref" => SymbolicRef,
            "tag" => Tag,
            "unpack-file" => UnpackFile,
            "unpack-objects" => UnpackObjects,
            "update-index" => UpdateIndex,
            "update-ref" => UpdateRef,
            "update-server-info" => UpdateServerInfo,
            "var" => Var,
            "verify-commit" => VerifyCommit,
            "verify-pack" => VerifyPack,
            "verify-tag" => VerifyTag,
            "version" | "--version" => Version,
            "whatchanged" => WhatChanged,
            "worktree" => Worktree,
            "write-tree" => WriteTree,
            _ => return Err(Error::NotGitCommand),
        };
        Ok(command)
    }
}
