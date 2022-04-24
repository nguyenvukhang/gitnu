import unittest, shutil, tempfile, os, subprocess

from gitnu.shell import system as i, system_std
from gitnu import log


def system(*args):
    a = subprocess.Popen(*args)
    a.wait()


class TestShellMethods(unittest.TestCase):
    def setUp(self):
        # Create a temporary directory
        self.test_dir = tempfile.mkdtemp()
        log.yellow("setUp done")

    def tearDown(self):
        # Remove the directory after the test
        shutil.rmtree(self.test_dir)
        log.yellow("tearDown done")

    def test_something(self):
        os.chdir(self.test_dir)
        stdout, _ = system_std(["git", "init"])
        stdout.close()
        with subprocess.Popen(["ls", "-a"]):
            pass
        with subprocess.Popen(["git", "status"]):
            pass


if __name__ == "__main__":
    unittest.main()
