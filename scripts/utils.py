import os
import subprocess
import re
import sys

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


def cargo_toml_path():
    return os.path.join(PROJECT_ROOT, "Cargo.toml")


def run_sh(*cmd):
    env = os.environ.copy()
    return subprocess.check_output(cmd, env=env).decode("utf-8").split("\n")


def get_semver_tags():
    all_tags = run_sh("git", "tag")
    semver_tags = filter(lambda x: SEMVER_REGEX.match(x), all_tags)
    return list(semver_tags)


def get_cargo_toml():
    with open(cargo_toml_path()) as f:
        lines = f.readlines()
    return lines


def set_cargo_toml(lines):
    with open(cargo_toml_path(), "w") as f:
        f.writelines(lines)


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
        msg = "Cargo.toml version does not match latest git tag.\n"
        msg += "\tCargo.toml: %s\n" % v_cargo
        msg += "\tGit tags  : %s\n" % v_git_tag
        raise Exception(msg)


class Version:
    def __init__(self, major, minor, patch, prerelease):
        if major == None or minor == None or patch == None:
            raise ValueError("Detected None for Version.__init__")
        self.major = int(major)
        self.minor = int(minor)
        self.patch = int(patch)
        self.prerelease = prerelease

    def __str__(self):
        return self.fmt()

    def __eq__(a, b):
        return (
            a.major == b.major
            and a.minor == b.minor
            and a.patch == b.patch
            and a.prerelease == b.prerelease
        )

    def from_text(text):
        if text == None:
            raise ValueError("Version: tried to init with None")
        version = SEMVER_REGEX.search(text)
        if version == None:
            raise ValueError("Bad version format: %s" % (text))
        return Version(*version.groups()[:4])

    def from_toml(toml_lines):
        regexed = list(map(lambda x: CARGO_TOML_VERSION_REGEX.match(x), toml_lines))

        try:  # throws an error if no regex matches are found
            index = next(i for i, v in enumerate(regexed) if v is not None)
        except StopIteration:
            raise ValueError("Version attribute not found in Cargo.toml")

        match = list(filter(bool, regexed))[0]
        version_text = match.groups()[0]
        return Version.from_text(version_text), index

    def into_toml(self, toml_lines):
        def on_each(line):
            match = CARGO_TOML_VERSION_REGEX.match(line)
            if match:
                return
            pass

        return map(on_each, toml_lines)

    def from_latest_tag():
        return Version.from_text(get_semver_tags()[-1])

    def fmt(self):
        if self.prerelease == None:
            return "%s.%s.%s" % (self.major, self.minor, self.patch)
        return "%s.%s.%s-%s" % (self.major, self.minor, self.patch, self.prerelease)

    def clone(self):
        return Version(self.major, self.minor, self.patch, self.prerelease)

    def next(self):
        if self.prerelease == None:
            return self.next_patch()
        return self.next_prerelease()

    def next_major(self):
        clone = self.clone()
        clone.major += 1
        clone.minor, clone.patch, clone.prerelease = 0, 0, None
        return clone

    def next_minor(self):
        clone = self.clone()
        clone.minor += 1
        clone.patch, clone.prerelease = 0, None
        return clone

    def next_patch(self):
        clone = self.clone()
        clone.patch += 1
        clone.prerelease = None
        return clone

    def next_prerelease(self):
        clone = self.clone()
        if clone.prerelease == None:
            clone.patch += 1
            clone.prerelease = "alpha1"
        else:
            text, num = split_trailing_number(clone.prerelease)
            clone.prerelease = "%s%d" % (text, num + 1)
        return clone


def current_version():
    cargo_toml = get_cargo_toml()

    # get latest version
    v_cargo, _ = Version.from_toml(cargo_toml)
    v_git_tag = Version.from_latest_tag()

    # assert match
    assert_version_match(v_cargo, v_git_tag)

    return v_cargo


def increment_prerelease():
    cargo_toml = get_cargo_toml()

    # get latest version
    v_cargo, version_line_index = Version.from_toml(cargo_toml)
    v_git_tag = Version.from_latest_tag()

    # assert match
    assert_version_match(v_cargo, v_git_tag)

    # increment
    v_next = v_cargo.next_prerelease()

    print("Current version: %s" % (v_cargo))
    print("Next prerelease: %s" % (v_next))

    Git.update_and_tag(cargo_toml, version_line_index, v_next)
    Git.push()


class Git:
    def update_and_tag(cargo_toml, line_num, v_next):
        # change the line in Cargo.toml
        cargo_toml[line_num] = 'version = "%s"\n' % (v_next)

        with open(cargo_toml_path(), "w") as f:
            f.writelines(cargo_toml)

        # build to update Cargo.lock
        subprocess.run(["cargo", "build"])

        tag = "v%s" % v_next
        subprocess.run(["git", "add", "Cargo.toml", "Cargo.lock"])
        subprocess.run(["git", "commit", "-m", "ver: bump to %s" % tag])
        subprocess.run(["git", "tag", tag])

    def push():
        subprocess.run(["git", "push"])
        subprocess.run(["git", "push", "--tags"])


class App:
    def increment_prerelease():
        increment_prerelease()

    def current_version():
        print("%s" % (current_version()))

    def next_prerelease_version():
        print("%s" % current_version().next_prerelease())


def main():
    if len(sys.argv) == 0:
        return
    app = {}
    app["increment-prerelease"] = App.increment_prerelease
    app["current-version"] = App.current_version
    app["next-prerelease-version"] = App.next_prerelease_version

    run = lambda: ()
    try:
        run = app[sys.argv[1]]
    except KeyError:
        print("Invalid key. Please refer to %s" % (__file__))

    run()


if __name__ == "__main__":
    main()
