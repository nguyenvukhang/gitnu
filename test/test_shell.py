import unittest, shutil, tempfile, os, subprocess
from gitnu.shell import systemlist
from gitnu import log

STRING1 = "lox63af3ldbem2tlx7i8"
STRING2 = "ybk63ns3yqorz2gyk7v8"


def gitnu() -> list[str]:
    result = systemlist(["gitnu"])
    return result


def run(cmd: list[str], verbose: bool = False) -> None:
    if verbose:
        log.perma.gray(*cmd)
    p = subprocess.Popen(cmd, stdout=None if verbose else subprocess.PIPE)
    p.wait()
    if not verbose and p.stdout:
        p.stdout.close()


def run_list(cmd_list: list[list[str]], verbose: bool = False) -> None:
    for cmd in cmd_list:
        run(cmd, verbose)


def ls():
    run(["ls", "-la"], verbose=True)


def gs():
    run(["git", "status"], verbose=True)


def gn():
    run(["gitnu"], verbose=True)


def cat(filename: str):
    run(["cat", filename], verbose=True)


def initialize_git_repo():
    run(["git", "init"])


def make_new_file(filename: str, contents: str = STRING1):
    run(["touch", filename])
    with open(filename, "w") as f:
        f.write(contents)


def make_staged_new_file(filename: str):
    make_new_file(filename)
    run(["git", "add", filename])


def make_committed_new_file(filename: str):
    make_staged_new_file(filename)
    run(["git", "commit", "-m", "committed file %s" % filename])


def make_unstaged_modified(filename: str, contents: str = STRING2):
    make_committed_new_file(filename)
    with open(filename, "w") as f:
        f.write(contents)


def make_submodule(submodule_name: str):
    os.chdir("..")
    os.mkdir(submodule_name)
    os.chdir(submodule_name)
    initialize_git_repo()
    make_committed_new_file("submodule_first_file")
    os.chdir("../main_repo")
    run_list([["git", "submodule", "add", "../%s" % (submodule_name)]])


def make_committed_submodule(submodule_name: str):
    make_submodule(submodule_name)
    run_list([["git", "commit", "-m", "added submodule: %s" % submodule_name]])


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
        initialize_git_repo()
        make_committed_new_file("calista.txt")
        # log.perma.purple(" Setup tmp git repo.")
        # log.perma.purple("─────────────────────")

    # this runs after every function below
    def tearDown(self):
        # Remove the directory after the test
        shutil.rmtree(self.test_dir)
        # log.perma.purple("─────────────────────")
        # log.perma.purple(" Tore down temp dir.")

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
        make_committed_submodule('daniel_ricciardo')
        self.assertListEqual(
            gitnu(), ["On branch master", "nothing to commit, working tree clean"]
        )


if __name__ == "__main__":
    unittest.main()
