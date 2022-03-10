Substrait query plan validator
==============================

This crate implements a validator for [Substrait](https://substrait.io/) query
plans.

```
[dependencies]
substrait-validator = "0.0.1"
```

YAML file resolution
--------------------

One of the complexities of validating Substrait plans is resolving the YAML
extension files. By default, the crate only supports `file://...` URLs, but
often, the YAML files will be stored remotely. To make handling this easier,
you can enable [curl](https://crates.io/crates/curl) as an optional
dependency:

```
[dependencies]
substrait-validator = { version = "0.0.1", features = ["curl"] }
```

This adds the `substrait_validator::Config::add_curl_yaml_uri_resolver()`
method, which will use `libcurl` to resolve the files, thus supporting all the
common protocols (http, https, ftp, etc.). The downside is that the curl crate
depends on system libraries.
