import unittest, shutil, tempfile, os, subprocess
from gitnu.shell import systemlist
from gitnu import log

STRING = "Still I Rise"


def gitnu() -> list[str]:
    result = systemlist(["gitnu"])
    return result


def run(cmd: list[str], verbose: bool = False) -> None:
    if verbose:
        log.gray(*cmd)
    p = subprocess.Popen(cmd, stdout=None if verbose else subprocess.PIPE)
    p.wait()
    if not verbose and p.stdout:
        p.stdout.close()


def run_list(cmd_list: list[list[str]], verbose: bool = False) -> None:
    for cmd in cmd_list:
        run(cmd, verbose)


class _git:
    def commit(self, message: str = "lalala"):
        run(["git", "commit", "-m", "%s" % message])

    def status(self):
        run(["git", "status"], verbose=True)

    def nu(self):
        run(["gitnu"], verbose=True)

    def log(self):
        run(["git", "log", "--all", "--oneline", "--graph"], verbose=True)

    def init(self):
        run(["git", "init"])

    def add(self, *filenames):
        run(["git", "add", *filenames])

    class _submodule:
        def add(self, submodule_path: str):
            run(["git", "submodule", "add", "--quiet", submodule_path])

    submodule = _submodule()


git = _git()


def ls():
    run(["ls", "-lA"], verbose=True)


def cat(filename: str):
    run(["cat", filename], verbose=True)


def make_new_file(filename: str):
    run(["touch", filename])
    change_file(filename)


def change_file(filename: str, contents: str = STRING):
    with open(filename, "a", encoding="UTF-8") as file:
        file.write(contents)


def make_staged_new_file(filename: str):
    make_new_file(filename)
    git.add(filename)


def make_committed_new_file(filename: str):
    make_staged_new_file(filename)
    git.commit("committed file %s" % filename)


def make_unstaged_modified(filename: str):
    make_committed_new_file(filename)
    change_file(filename)


def make_submodule(submodule_name: str):
    os.chdir("..")
    os.mkdir(submodule_name)
    os.chdir(submodule_name)
    git.init()
    make_committed_new_file("submodule_first_file")
    os.chdir("../main_repo")
    git.submodule.add("../%s" % (submodule_name))


def make_committed_submodule(submodule_name: str):
    make_submodule(submodule_name)
    git.commit("added submodule: %s" % submodule_name)


def make_modified_submodule(submodule_name: str):
    make_committed_submodule(submodule_name)
    os.chdir(submodule_name)
    change_file("smash")
    git.add("smash")
    git.commit("created smash")
    os.chdir("..")


class TestShellMethods(unittest.TestCase):
    # this runs before every function below
    def setUp(self):
        # Create a temporary directory
        self.test_dir = tempfile.mkdtemp()

        # cd into that temp dir
        # note that this script's cwd will not change unless
        # explicitly done so
        os.chdir(self.test_dir)

        # make a repo dir so can test submodules in root temp dir
        os.mkdir("main_repo")
        os.chdir("main_repo")
        git.init()
        make_committed_new_file("calista.txt")
        make_committed_new_file("MAIN_REPO.txt")
        # log.purple(" Setup tmp git repo.")
        # log.purple("─────────────────────")

    # this runs after every function below
    def tearDown(self):
        # Remove the directory after the test
        shutil.rmtree(self.test_dir)
        # log.purple("─────────────────────")
        # log.purple(" Tore down temp dir.")

    def test_unstaged_new_file(self):
        make_new_file("alpha.txt")
        self.assertListEqual(
            gitnu(),
            [
                "On branch master",
                "Untracked files:",
                '(use "git add <file>..." to include in what will be committed)',
                "1 \t\x1b[31malpha.txt\x1b[m",
                "",
                'nothing added to commit but untracked files present (use "git add" to track)',
            ],
        )

    def test_staged_new_file(self):
        make_staged_new_file("alpha.txt")
        self.assertListEqual(
            gitnu(),
            [
                "On branch master",
                "Changes to be committed:",
                '(use "git restore --staged <file>..." to unstage)',
                "1 \t\x1b[32mnew file:   alpha.txt\x1b[m",
                "",
            ],
        )

    def test_unstaged_modified(self):
        make_unstaged_modified("alpha.txt")
        self.assertListEqual(
            gitnu(),
            [
                "On branch master",
                "Changes not staged for commit:",
                '(use "git add <file>..." to update what will be committed)',
                '(use "git restore <file>..." to discard changes in working directory)',
                "1 \t\x1b[31mmodified:   alpha.txt\x1b[m",
                "",
                'no changes added to commit (use "git add" and/or "git commit -a")',
            ],
        )

    def test_new_submodule(self):
        make_submodule("daniel_ricciardo")
        self.assertListEqual(
            gitnu(),
            [
                "On branch master",
                "Changes to be committed:",
                '(use "git restore --staged <file>..." to unstage)',
                "1 \t\x1b[32mnew file:   .gitmodules\x1b[m",
                "2 \t\x1b[32mnew file:   daniel_ricciardo\x1b[m",
                "",
            ],
        )

    def test_committed_submodule(self):
        make_committed_submodule("daniel_ricciardo")
        self.assertListEqual(
            gitnu(), ["On branch master", "nothing to commit, working tree clean"]
        )

    def test_modified_submodule(self):
        make_modified_submodule("kimi_raikkonen")
        self.assertListEqual(
            gitnu(),
            [
                "On branch master",
                "Changes not staged for commit:",
                '(use "git add <file>..." to update what will be committed)',
                '(use "git restore <file>..." to discard changes in working directory)',
                "1 \t\x1b[31mmodified:   kimi_raikkonen\x1b[m (new commits)",
                "",
                'no changes added to commit (use "git add" and/or "git commit -a")',
            ],
        )


if __name__ == "__main__":
    unittest.main()
