from setuptools import setup

import gitnu.main as gitnu
import test.test_gitnu as test

setup(name='gitnu',
      version='0.0.1',
      description='enumerate git status',
      author='Khang',
      author_email='brew4k@gmail.com',
      url='https://www.nguyenvukhang.com',
      packages=['gitnu', 'test'],
     )

# the existence of this file is what allows
# test files to import from gitnu

# because yes
