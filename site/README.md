## Substrait Site

This directory contains the source for the Substrait site.

* Site structure is maintained in mkdocs.yml
* Pages are maintained in markdown in the `docs/` folder
* Links use bare page names: `[link text](target-page)`

### Installation

The site is built using mkdocs. To install mkdocs and the theme, run:

```
# Activate the virtual environment (if installed)
cd site/
. venv/bin/activate
# Install or update the dependencies
pip install -r ./requirements.txt
```

It is easier to use `virtualenv` to keep the Python dependencies for `site/`
separate from your other projects and/or distinct from system managed Python
dependencies.

* To use `virtualenv`, you need Python 3.7/3.8 installed locally.
  * For Ubuntu: `apt-get install python3 virtualenv`
  * For MacOS/brew: `brew install python pyenv-virtualenv`
* Install the virtual environment:
  ```
  # cd to the site/ directory
  cd site/
  # setup the virtual environment (only needed once)
  virtualenv -p $(which python3) venv
  # activate the virtual environment
  . venv/bin/activate
  # Install or update the dependencies as usual
  pip install -r ./requirements.txt
  ```

### Local Changes

To see changes locally before committing, use mkdocs to run a local server from this directory.

```
mkdocs serve
```

### Publishing

TBD
