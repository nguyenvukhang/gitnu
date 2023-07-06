import os
import subprocess
import re
import sys
from typing import Callable

# Obtained from semver.org
# https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
#
# With an added v? in front to support an optional 'v' prefix.
SEMVER_REGEX_SRC = (
    "(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)"
    + "(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))"
    + "?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"
)
SEMVER_REGEX = re.compile("^v?" + SEMVER_REGEX_SRC + "$")
CARGO_TOML_VERSION_REGEX = re.compile('^version = "(.*)"')

DIR_PATH = os.path.dirname(os.path.realpath(__file__))
PROJECT_ROOT = os.path.join(DIR_PATH, "..")


# fmt: off
# Cargo stuff
def cargo_toml_path(): return os.path.join(PROJECT_ROOT, "Cargo.toml")
def get_cargo_toml():
    with open(cargo_toml_path()) as f: lines = f.readlines()
    return lines or []
def set_cargo_toml(lines):
    with open(cargo_toml_path(), "w") as f: f.writelines(lines)
# fmt: on


def run_sh(*cmd):
    return subprocess.check_output(cmd).decode("utf-8").split("\n")


def get_semver_tags():
    all_tags = run_sh("git", "tag")
    semver_tags = filter(SEMVER_REGEX.match, all_tags)

    def version_or_none(text):
        try:
            return Version.from_str(text)
        except Exception:
            return None

    versions = list(map(version_or_none, semver_tags))
    Version.sort(versions)
    return versions


def split_trailing_number(text):
    ptr = len(text)
    for i in text[::-1]:
        try:
            int(i)
            ptr -= 1
        except Exception:
            break
    return text[:ptr], int(text[ptr:])


def assert_version_match(v_cargo, v_git_tag):
    if not v_cargo == v_git_tag:
        msg = "Cargo.toml version does not match latest git tag."
        msg += "\n\tCargo.toml: %s" % v_cargo
        msg += "\n\tGit tags  : %s" % v_git_tag
        print(msg)
        sys.exit(1)


class Version:
    def __init__(
        s, major, minor, patch, pre
    ):  # type: (int, int, int, str|None) -> None
        s.major, s.minor, s.patch, s.pre = major, minor, patch, pre

    # sort by prerelease with a rank list
    @staticmethod
    def sort(arr):  # type: (list[Version]) -> None
        PRERELEASE_RANKS = {"alpha": 1, "beta": 2, None: 3}
        arr.sort(key=lambda v: v.pre_number())
        arr.sort(key=lambda v: PRERELEASE_RANKS.get(v.pre_name(), 0))

        arr.sort(key=lambda v: v.patch)
        arr.sort(key=lambda v: v.minor)
        arr.sort(key=lambda v: v.major)

    @staticmethod
    def from_str(text):
        if text == None:
            raise ValueError("Tried to init Version with None")

        match = SEMVER_REGEX.search(text)
        if match == None:
            raise ValueError('Invalid SemVer: "%s"' % (text))

        # after passing the regex, the rest is safe:
        major, minor, patch, pre = match.groups()[:4]
        major, minor, patch = map(int, [major, minor, patch])
        return Version(major, minor, patch, pre)

    def __str__(self):
        if self.pre is None:
            return "%s.%s.%s" % (self.major, self.minor, self.patch)
        return "%s.%s.%s-%s" % (self.major, self.minor, self.patch, self.pre)

    def __eq__(a, b):
        return (
            a.major == b.major
            and a.minor == b.minor
            and a.patch == b.patch
            and a.pre == b.pre
        )

    @staticmethod
    def split_pre(text):
        ptr = len(text)
        for i in text[::-1]:
            try:
                int(i)
                ptr -= 1
            except Exception:
                break
        return text[:ptr], int(text[ptr:])

    def pre_name(self):
        if self.pre == None:
            return None
        return Version.split_pre(self.pre)[0]

    def pre_number(self):
        if self.pre == None:
            return 0
        return Version.split_pre(self.pre)[1]

    # 1.2.3-alpha1 -> INVALID
    # 1.2.3 -> 2.0.0-alpha1
    def next_major_pre(self):
        if self.pre != None:
            msg = "Unexpected existing pre. Use next_patch_pre() instead."
            raise ValueError(msg)
        self.next_minor_pre()
        self.minor = 0
        self.major += 1
        return self

    # 1.2.3-alpha1 -> INVALID
    # 1.2.3 -> 1.3.0-alpha1
    def next_minor_pre(self):
        if self.pre != None:
            msg = "Unexpected existing pre. Use next_patch_pre() instead."
            raise ValueError(msg)
        self.next_patch_pre()
        self.patch = 0
        self.minor += 1
        return self

    # 1.2.3-alpha1 -> 1.2.3-alpha2
    # 1.2.3 -> 1.2.4-alpha1
    def next_patch_pre(self):
        if self.pre == None:
            self.patch += 1
            self.pre = "alpha1"
        else:
            text, num = Version.split_pre(self.pre)
            self.pre = "%s%d" % (text, num + 1)
        return self

    def release(self):
        self.pre = None
        return self

    @staticmethod
    def from_toml(toml_lines):
        regexed = list(map(lambda x: CARGO_TOML_VERSION_REGEX.match(x), toml_lines))

        try:  # throws an error if no regex matches are found
            index = next(i for i, v in enumerate(regexed) if v is not None)
        except StopIteration:
            raise ValueError("Version attribute not found in Cargo.toml")

        match = list(filter(bool, regexed))[0]
        version_text = match.groups()[0]
        return Version.from_str(version_text), index

    @staticmethod
    def from_latest_tag():
        return get_semver_tags()[-1]

    def tag(self):
        return "v" + str(self)

    def is_release(self):
        return self.pre is None


def current_version():
    cargo_toml = get_cargo_toml()

    # get latest version
    v_cargo, _ = Version.from_toml(cargo_toml)
    # v_git_tag = Version.from_latest_tag()

    # assert match
    # assert_version_match(v_cargo, v_git_tag)

    return v_cargo


def increment(inc_fn):  # type: (Callable[[Version], Version]) -> None
    cargo_toml = get_cargo_toml()

    # get latest version
    v_cargo, version_line_index = Version.from_toml(cargo_toml)
    v_git_tag = Version.from_latest_tag()

    # assert match
    # assert_version_match(v_cargo, v_git_tag)
    print(v_cargo)
    print(v_git_tag)

    print("Current version: %s" % (v_cargo))

    # increment
    v_next = inc_fn(v_cargo)

    print("Next pre: %s" % (v_next))

    Git.update_and_commit(cargo_toml, version_line_index, v_next)
    if v_next.is_release():
        Git.tag(v_next)
    Git.push()


class Git:
    def update_and_commit(
        cargo_toml, line_num, ver
    ):  # type: (list[str], int, Version) -> None
        # change the line in Cargo.toml
        cargo_toml[line_num] = 'version = "%s"\n' % (ver)

        set_cargo_toml(cargo_toml)

        # build to update_and_commit Cargo.lock
        subprocess.run(["cargo", "build"])

        subprocess.run(["git", "add", "Cargo.toml", "Cargo.lock"])
        subprocess.run(["git", "commit", "-m", "ver: bump to %s" % ver.tag()])

    def tag(ver):  # type: (Version) -> None
        subprocess.run(["git", "tag", ver.tag()])
        subprocess.run(["git", "push", "--tags"])

    def push():
        subprocess.run(["git", "push"])


class App:
    # fmt: off
    def increment_major_pre(): increment(lambda x: x.next_major_pre())
    def increment_minor_pre(): increment(lambda x: x.next_minor_pre())
    def increment_patch_pre(): increment(lambda x: x.next_patch_pre())
    def increment_release():   increment(lambda x: x.release())

    def next_patch_pre():  print(current_version().next_patch_pre())
    def next_minor_pre():  print(current_version().next_minor_pre())
    def next_major_pre():  print(current_version().next_minor_pre())
    def next_release():  print(current_version().release())

    def current_version(): print(current_version())
    def latest_tag():      print(Version.from_latest_tag())
    # fmt: on


def main():
    if len(sys.argv) == 0:
        return

    app = {}

    app["increment-major-pre"] = App.increment_major_pre
    app["increment-minor-pre"] = App.increment_minor_pre
    app["increment-patch-pre"] = App.increment_patch_pre
    app["increment-release"] = App.increment_release

    app["next-major-pre"] = App.next_major_pre
    app["next-minor-pre"] = App.next_minor_pre
    app["next-patch-pre"] = App.next_patch_pre
    app["next-release"] = App.next_release

    app["current-version"] = App.current_version
    app["latest-tag"] = App.latest_tag

    run = lambda: ()
    try:
        run = app[sys.argv[1]]
    except KeyError:
        print("Invalid key. Please refer to %s" % (__file__))

    run()


if __name__ == "__main__":
    main()
