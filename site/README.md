## Substrait Site

This directory contains the source for the Substrait site.

* Site structure is maintained in mkdocs.yml
* Pages are maintained in markdown in the `docs/` folder
* Links use bare page names: `[link text](target-page)`

### Installation

The site is built using mkdocs. We are using [Pixi](https://pixi.prefix.dev) to manage the build environment.

```

```

### Local Changes

To see changes locally before committing, use mkdocs to run a local server from this directory.

```
pixi run mkdocs serve
```

### Publishing

Publishing is done automatically by the `site.yml` Github Actions workflow in the `.github/workflows` directory.
