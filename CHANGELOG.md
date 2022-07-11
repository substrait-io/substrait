Release Notes
---

## [0.7.0](https://github.com/substrait-io/substrait/compare/v0.6.0...v0.7.0) (2022-07-11)


### Features

* introduce compound (parameterizable) extension types and variations ([#196](https://github.com/substrait-io/substrait/issues/196)) ([a79eb07](https://github.com/substrait-io/substrait/commit/a79eb07a15cfa157e795f028a83f746967c98805))

## [0.6.0](https://github.com/substrait-io/substrait/compare/v0.5.0...v0.6.0) (2022-06-26)


### Features

* add contains, starts_with and ends_with functions definitions ([#228](https://github.com/substrait-io/substrait/issues/228)) ([a5fa851](https://github.com/substrait-io/substrait/commit/a5fa85153ffbf7005b9039e06f502a9cc8a732f0))


### Bug Fixes

* fix binary serialization idl link ([#229](https://github.com/substrait-io/substrait/issues/229)) ([af0b452](https://github.com/substrait-io/substrait/commit/af0b45247692dc4bb8fbd25c7f8ec59ff49dbc36))

## [0.5.0](https://github.com/substrait-io/substrait/compare/v0.4.0...v0.5.0) (2022-06-12)


### ⚠ BREAKING CHANGES

* The `substrait/ReadRel/LocalFiles/format` field is deprecated. This will cause a hard break in compatibility. Newer consumers will not be able to read older files. Older consumers will not be able to read newer files. One should now express format concepts using the file_format oneof field.

Co-authored-by: Jacques Nadeau <jacques@apache.org>

### Features

* add aggregate function min/max support ([#219](https://github.com/substrait-io/substrait/issues/219)) ([48b6b12](https://github.com/substrait-io/substrait/commit/48b6b12ebf74c3cc38d4381b950e2caaeb4eef78))
* add Arrow and Orc file formats ([#169](https://github.com/substrait-io/substrait/issues/169)) ([43be00a](https://github.com/substrait-io/substrait/commit/43be00a73abd90fe8f0cafef2b8da9b078d1f243))
* support nullable and non-default variation user-defined types ([#217](https://github.com/substrait-io/substrait/issues/217)) ([5851b02](https://github.com/substrait-io/substrait/commit/5851b02d29aafe44cd804f4248b95b0593878c0a))

## [0.4.0](https://github.com/substrait-io/substrait/compare/v0.3.0...v0.4.0) (2022-06-05)


### ⚠ BREAKING CHANGES

* there was an accidental inclusion of a binary `not` function with unspecified behavior. This function was removed. Use the unary `not` function to return the compliment of an input argument.

### Bug Fixes

* remove not function that expects two arguments ([#182](https://github.com/substrait-io/substrait/issues/182)) ([e06067c](https://github.com/substrait-io/substrait/commit/e06067c991ddc34b2720408ed7e1ca5152774a29))

## [0.3.0](https://github.com/substrait-io/substrait/compare/v0.2.0...v0.3.0) (2022-05-22)


### Features

* support type function arguments in protobuf ([#161](https://github.com/substrait-io/substrait/issues/161)) ([df98816](https://github.com/substrait-io/substrait/commit/df988163a5afcebe8823b9e466c3e1923c3b9e79))
* define APPROX_COUNT_DISTINCT in new yaml for approximate aggregate functions ([#204](https://github.com/substrait-io/substrait/issues/204)) ([8e206b9](https://github.com/substrait-io/substrait/commit/8e206b9594880886c513c8437663fac15e0dfe59))
* literals for extension types ([#197](https://github.com/substrait-io/substrait/issues/197)) ([296c266](https://github.com/substrait-io/substrait/commit/296c2661de007a2d8f41d3fe242a1f4b6e60c9e1))
* support fractional seconds for interval_day literals ([#199](https://github.com/substrait-io/substrait/issues/199)) ([129e52f](https://github.com/substrait-io/substrait/commit/129e52f2519db00d6cef35f3faa3bc9e1ff1e890))

## [0.2.0](https://github.com/substrait-io/substrait/compare/v0.1.2...v0.2.0) (2022-05-15)


### Features

* add flag FailureBehavior in Cast expression ([#186](https://github.com/substrait-io/substrait/issues/186)) ([a3d3b2f](https://github.com/substrait-io/substrait/commit/a3d3b2f5ccc6e8375a950290eda09489c7fb30e7))
* add invocation property to AggregateFunction message for specifying distinct vs all ([#191](https://github.com/substrait-io/substrait/issues/191)) ([373b33f](https://github.com/substrait-io/substrait/commit/373b33f62b1e8f026718bc3b55cbe267421a1abb))

### [0.1.2](https://github.com/substrait-io/substrait/compare/v0.1.1...v0.1.2) (2022-05-01)


### Bug Fixes

* **docs:** use conventionalcommits to show breaking changes first ([#181](https://github.com/substrait-io/substrait/issues/181)) ([b7f2587](https://github.com/substrait-io/substrait/commit/b7f2587f492071bed2250eb6f04c0b8123e715e1))

## [0.1.1](https://github.com/substrait-io/substrait/compare/v0.1.0...v0.1.1) (2022-04-28)


### Bug Fixes

* **ci:** cd into buf-configured proto directory ([#180](https://github.com/substrait-io/substrait/issues/180)) ([78c0781](https://github.com/substrait-io/substrait/commit/78c0781f72cae2f4445a708ae3ccf0c2c3eb9725))

# [0.1.0](https://github.com/substrait-io/substrait/compare/v0.0.0...v0.1.0) (2022-04-28)


### Bug Fixes

* add missing switch expression ([#160](https://github.com/substrait-io/substrait/issues/160)) ([4db2a9f](https://github.com/substrait-io/substrait/commit/4db2a9fb7e7849c73adcd21d1b06fb7e8df73fae))


### Features

* add subquery representation ([#134](https://github.com/substrait-io/substrait/issues/134)) ([3670518](https://github.com/substrait-io/substrait/commit/3670518d37c53660d496860f81c761ccb0afbce0))
