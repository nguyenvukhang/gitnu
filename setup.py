from setuptools import setup

import gitn.main as gitn
import test.test_gitn as test

setup(name='gitn',
      version='0.0.1',
      description='enumerate git status',
      author='Khang',
      author_email='brew4k@gmail.com',
      url='https://www.nguyenvukhang.com',
      packages=['gitn', 'test'],
     )

# the existence of this file is what allows
# test files to import from gitn

# because yes
