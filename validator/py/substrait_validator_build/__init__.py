# SPDX-License-Identifier: Apache-2.0

from maturin import *
import shutil
import os


_LOCALIZED_PATHS = ('proto', 'text', 'LICENSE')
_ADDITIONAL_PATHS_TO_CLEAN = ('protoc_out', 'substrait_validator/substrait', 'substrait_validator/__pycache__')


def clean():
    for path in _LOCALIZED_PATHS + _ADDITIONAL_PATHS_TO_CLEAN:
        if os.path.isdir(path):
            shutil.rmtree(path)
        elif os.path.isfile(path):
            os.unlink(path)


def populate():
    clean()
    for path in _LOCALIZED_PATHS:
        source = os.path.join('..', '..', path)
        if os.path.isdir(source):
            shutil.copytree(source, path)
        else:
            shutil.copyfile(source, path)


def _prepare():
    # If the local_dependencies directory exists, pip is building the package
    # from a source distribution. In that case, the build environment is
    # already as it should be.
    if not os.path.isdir('local_dependencies'):
        populate()


_maturin_prepare_metadata_for_build_wheel = prepare_metadata_for_build_wheel
def prepare_metadata_for_build_wheel(*args, **kwargs):
    _prepare()
    return _maturin_prepare_metadata_for_build_wheel(*args, **kwargs)


_maturin_build_wheel = build_wheel
def build_wheel(*args, **kwargs):
    _prepare()
    return _maturin_build_wheel(*args, **kwargs)


_maturin_build_sdist = build_sdist
def build_sdist(*args, **kwargs):
    _prepare()
    return _maturin_build_sdist(*args, **kwargs)

