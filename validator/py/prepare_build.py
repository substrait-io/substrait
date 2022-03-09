#!/usr/bin/python
# SPDX-License-Identifier: Apache-2.0

import os
import sys
import substrait_validator_build

def eprint(*args):
    print(*args, file=sys.stderr)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        eprint('Usage: {} [populate|clean]'.format(sys.argv[0]))
        eprint()
        eprint('Populates or removes local copies of Substrait files needed for the build')
        eprint('that are stored outside of this subdirectory.')
        sys.exit(1)

    if sys.argv[1] == 'populate':
        os.chdir(os.path.dirname(os.path.abspath(__file__)))
        substrait_validator_build.populate()
        sys.exit(0)

    if sys.argv[1] == 'clean':
        os.chdir(os.path.dirname(os.path.abspath(__file__)))
        substrait_validator_build.clean()
        sys.exit(0)

    eprint('Unknown command: {}'.format(sys.argv[1]))
    sys.exit(1)
