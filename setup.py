from setuptools import setup, find_packages
import os
from setuptools.command.install import install
from setuptools.command.develop import develop
from subprocess import run

VERSION = "0.0.19"
DESCRIPTION = "Enumerate git status"
LONG_DESCRIPTION = "A cli tools that indexes the output of git status to use in subsequent git commands."


def _post_install():
    home = str(os.getenv("HOME"))
    install_path = os.path.join(home, ".local", "bin")
    install_file = os.path.join(install_path, "gitnu")
    run(["touch", install_file])
    with open(install_file, "w") as f:
        f.writelines(["#!/usr/bin/env bash\n", "python3 -m gitnu"])
    run(["chmod", "+x", install_file])


class PostDevelopCommand(install):
    def run(self):
        install.run(self)
        _post_install()


class PostInstallCommand(install):
    def run(self):
        install.run(self)
        _post_install()


# Setting up
setup(
    name="gitnu",
    version=VERSION,
    author="Nguyen Vu Khang",
    author_email="<brew4k@gmail.com>",
    description=DESCRIPTION,
    long_description_content_type="text/markdown",
    long_description=LONG_DESCRIPTION,
    packages=find_packages(),
    install_requires=[],
    keywords=["python", "git"],
    classifiers=[
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3",
        "Operating System :: Unix",
        "Operating System :: MacOS :: MacOS X",
    ],
    cmdclass={
        "install": PostInstallCommand,
        "develop": PostDevelopCommand,
    },
)
