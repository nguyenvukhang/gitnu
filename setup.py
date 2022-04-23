from setuptools import setup, find_packages

VERSION = '0.0.1'
DESCRIPTION = 'Enumerate git status'
LONG_DESCRIPTION = 'A cli tools that indexes the output of git status to use in subsequent git commands.'

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
    install_requires=['opencv-python', 'pyautogui', 'pyaudio'],
    keywords=['python', 'git'],
    classifiers=[
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3",
        "Operating System :: Unix",
        "Operating System :: MacOS :: MacOS X",
    ]
)
