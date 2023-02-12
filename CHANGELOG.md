Release Notes
---

## [0.24.0](https://github.com/substrait-io/substrait/compare/v0.23.0...v0.24.0) (2023-02-12)


### Features

* add round function ([#322](https://github.com/substrait-io/substrait/issues/322)) ([57121c8](https://github.com/substrait-io/substrait/commit/57121c8ca6f1fe815e98eda8962f8f84736c58e2))

## [0.23.0](https://github.com/substrait-io/substrait/compare/v0.22.0...v0.23.0) (2023-01-22)


### Features

* add extended expression for expression only evaluation ([#405](https://github.com/substrait-io/substrait/issues/405)) ([d35f0ed](https://github.com/substrait-io/substrait/commit/d35f0ed98ccefe31a90d53ff887402636a74bbd1))
* **spec:** add physical plans for hashJoin and mergeJoin ([#336](https://github.com/substrait-io/substrait/issues/336)) ([431651e](https://github.com/substrait-io/substrait/commit/431651efbd64958d2611b035ffdb25f589b28477))


### Bug Fixes

* update extension yaml files to match type-syntax spec ([#423](https://github.com/substrait-io/substrait/issues/423)) ([0608878](https://github.com/substrait-io/substrait/commit/0608878b25e7f9b4b3ffe33662eea9ef0f016548))

## [0.22.0](https://github.com/substrait-io/substrait/compare/v0.21.1...v0.22.0) (2022-12-18)


### Features

* add bitwise NOT, AND, OR & XOR functions ([#370](https://github.com/substrait-io/substrait/issues/370)) ([81e34d4](https://github.com/substrait-io/substrait/commit/81e34d4054ff0dbde23ac749fbb8fcc232989c5d))

## [0.21.1](https://github.com/substrait-io/substrait/compare/v0.21.0...v0.21.1) (2022-12-04)


### Bug Fixes

* rename regex_string_split to regexp_string_split ([#393](https://github.com/substrait-io/substrait/issues/393)) ([f9f4967](https://github.com/substrait-io/substrait/commit/f9f4967e6785b49eccb64a42497b5b4aaffa63ff))

## [0.21.0](https://github.com/substrait-io/substrait/compare/v0.20.0...v0.21.0) (2022-11-27)


### Features

* add nested type constructor expressions ([#351](https://github.com/substrait-io/substrait/issues/351)) ([b64d30b](https://github.com/substrait-io/substrait/commit/b64d30b28077973dd94f1f49e5016662a35bcf56))
* add title to simple extensions schema ([#387](https://github.com/substrait-io/substrait/issues/387)) ([2819ecc](https://github.com/substrait-io/substrait/commit/2819ecc69175b96eefb8a73fb4b533431890f3da))

## [0.20.0](https://github.com/substrait-io/substrait/compare/v0.19.0...v0.20.0) (2022-11-20)


### ⚠ BREAKING CHANGES

* optional arguments are no longer allowed to be specified
as a part of FunctionArgument messages.  Instead they are now specified
separately as part of the function invocation.
* optional arguments are now specified separately from
required arguments in the YAML specification.

Co-authored-by: Benjamin Kietzman <bengilgit@gmail.com>

Co-authored-by: Benjamin Kietzman <bengilgit@gmail.com>

### Features

* add best effort filter to read rel and clarify that the pre-masked schema should be used ([#271](https://github.com/substrait-io/substrait/issues/271)) ([4beff87](https://github.com/substrait-io/substrait/commit/4beff877550ac4ac10199748acbba391aca172f6))
* optional args are now specified separately from required args ([#342](https://github.com/substrait-io/substrait/issues/342)) ([bd29ea3](https://github.com/substrait-io/substrait/commit/bd29ea3b06391ae9018de851055db11075fd0758))

## [0.19.0](https://github.com/substrait-io/substrait/compare/v0.18.0...v0.19.0) (2022-11-06)


### Features

* add functions for splitting strings ([#346](https://github.com/substrait-io/substrait/issues/346)) ([20a2f14](https://github.com/substrait-io/substrait/commit/20a2f14ee0f2c3186318543d7ff264c91f714967))


### Bug Fixes

* rename version fields which conflict with sysmacros ([#362](https://github.com/substrait-io/substrait/issues/362)) ([4170bf1](https://github.com/substrait-io/substrait/commit/4170bf12c0f86032d8649a0880c684c37a5065f7))

## [0.18.0](https://github.com/substrait-io/substrait/compare/v0.17.0...v0.18.0) (2022-10-09)


### Features

* attach Substrait version number to plans ([#347](https://github.com/substrait-io/substrait/issues/347)) ([2d1bb9d](https://github.com/substrait-io/substrait/commit/2d1bb9d9472409715f1667dfeae241677c6c5ec2))

## [0.17.0](https://github.com/substrait-io/substrait/compare/v0.16.0...v0.17.0) (2022-10-02)


### Features

* support non-struct type class structure ([#328](https://github.com/substrait-io/substrait/issues/328)) ([dd7f9f0](https://github.com/substrait-io/substrait/commit/dd7f9f01bdf11f5ac9db7713c5ff3d2f82ff5a78))

## [0.16.0](https://github.com/substrait-io/substrait/compare/v0.15.0...v0.16.0) (2022-09-25)


### Features

* add any_value aggregate function ([#321](https://github.com/substrait-io/substrait/issues/321)) ([6f603d3](https://github.com/substrait-io/substrait/commit/6f603d3b61ad26a2f7da1bc74e2a60dd246def16))
* support constant function arguments ([#305](https://github.com/substrait-io/substrait/issues/305)) ([6021030](https://github.com/substrait-io/substrait/commit/6021030a599134f959ebc0f36621b27127316356))

## [0.15.0](https://github.com/substrait-io/substrait/compare/v0.14.0...v0.15.0) (2022-09-18)


### ⚠ BREAKING CHANGES

* options were added to division and logarithmic functions.

### Features

* add options for behaviour when dividing by zero or calculating log zero ([#329](https://github.com/substrait-io/substrait/issues/329)) ([1c170c8](https://github.com/substrait-io/substrait/commit/1c170c8d984ffbee759f7d7371cbb93b1fd24db9))


### Bug Fixes

* **naming:** add missing arg names in functions_aggregate_*.yaml ([#316](https://github.com/substrait-io/substrait/issues/316)) ([fb92997](https://github.com/substrait-io/substrait/commit/fb9299735f4e67cffaa7b153f4dce885c9f7f93d))

## [0.14.0](https://github.com/substrait-io/substrait/compare/v0.13.0...v0.14.0) (2022-09-11)


### ⚠ BREAKING CHANGES

* option argument added to std_dev and variance aggregate functions

### Features

* add bool_and and bool_or aggregate functions ([#314](https://github.com/substrait-io/substrait/issues/314)) ([52fa523](https://github.com/substrait-io/substrait/commit/52fa5235c6bb2f43ccc2e25c6fe548a0f0215524))
* add corr and mode aggregation functions ([#296](https://github.com/substrait-io/substrait/issues/296)) ([96b13d7](https://github.com/substrait-io/substrait/commit/96b13d7ea4e9dc95c051d02521812e6011c47e20))
* add median and count_distinct aggregation functions ([#278](https://github.com/substrait-io/substrait/issues/278)) ([9be62e5](https://github.com/substrait-io/substrait/commit/9be62e5067c13858e8c545689891937c2dced4ee))
* add population option to variance and standard deviation functions ([#295](https://github.com/substrait-io/substrait/issues/295)) ([c47fffa](https://github.com/substrait-io/substrait/commit/c47fffa83af26f7278a5d7f6501d9eadbd365d30))
* add quantile aggregate function ([#279](https://github.com/substrait-io/substrait/issues/279)) ([de6bc9f](https://github.com/substrait-io/substrait/commit/de6bc9fad440880b6b5333cb0ee129d2c19e471c))
* add string_agg aggregate function ([#297](https://github.com/substrait-io/substrait/issues/297)) ([fbe5e09](https://github.com/substrait-io/substrait/commit/fbe5e0949b863334d02b5ad9ecac55ec8fc4debb))


### Bug Fixes

* mark string_agg aggregate as being sensitive to input order ([#312](https://github.com/substrait-io/substrait/issues/312)) ([683faaa](https://github.com/substrait-io/substrait/commit/683faaa37ce8cad444a5fe703a7653dc04d02486))
* **naming:** add missing arg names in functions_arithmetic.yaml ([#315](https://github.com/substrait-io/substrait/issues/315)) ([d433a06](https://github.com/substrait-io/substrait/commit/d433a06adc77d9d71db3a3b956d82b8318d220ed))
* **naming:** add missing arg names in functions_datetime.yaml ([#318](https://github.com/substrait-io/substrait/issues/318)) ([b7347d1](https://github.com/substrait-io/substrait/commit/b7347d15c62e67fbca2cb810c008c32460263d7b))
* **naming:** add missing arg names in functions_logarithmic.yaml and functions_set.yaml ([#319](https://github.com/substrait-io/substrait/issues/319)) ([1c14d27](https://github.com/substrait-io/substrait/commit/1c14d271557addb5980123778102f844359a749e))
* **naming:** add/replace arg names in functions_boolean.yaml ([#317](https://github.com/substrait-io/substrait/issues/317)) ([809a2f4](https://github.com/substrait-io/substrait/commit/809a2f42c2f2795bc7efd64b7ff4cef3d9abc807))
* revert addition of count_distinct aggregate function ([#311](https://github.com/substrait-io/substrait/issues/311)) ([90d7c0d](https://github.com/substrait-io/substrait/commit/90d7c0df9c729a3027988aeadfd74104f7385014))

## [0.13.0](https://github.com/substrait-io/substrait/compare/v0.12.0...v0.13.0) (2022-09-04)


### ⚠ BREAKING CHANGES

* nullability behavior of is_nan, is_finite, and is_infinite has changed
* compound name for concat has changed to concat:str and
concat:vchar (one argument) to make it 1+ variadic

### Features

* add center function ([#282](https://github.com/substrait-io/substrait/issues/282)) ([7697d39](https://github.com/substrait-io/substrait/commit/7697d397aaf53999d6eca7799bb4535f30af4e45))
* add coalesce function ([#301](https://github.com/substrait-io/substrait/issues/301)) ([63c5da0](https://github.com/substrait-io/substrait/commit/63c5da0173369ce3d7667da6a30c9581057fa890))
* add dwrf file format ([#304](https://github.com/substrait-io/substrait/issues/304)) ([0f7c2ea](https://github.com/substrait-io/substrait/commit/0f7c2eae469f8bf92905230bbed0d6e88dff7f40))
* add exp function ([#299](https://github.com/substrait-io/substrait/issues/299)) ([7ed31f6](https://github.com/substrait-io/substrait/commit/7ed31f60e58aeff0b5e17af1be3fa0fba24b7ae1))
* add factorial scalar function ([#300](https://github.com/substrait-io/substrait/issues/300)) ([a4d6f35](https://github.com/substrait-io/substrait/commit/a4d6f35f3d12c50d45e15ac974f5cc366e4aa905))
* add hyperbolic functions ([#290](https://github.com/substrait-io/substrait/issues/290)) ([4252824](https://github.com/substrait-io/substrait/commit/4252824264025f69352cf16cd6f886cd4b30df48))
* add log1p function ([#273](https://github.com/substrait-io/substrait/issues/273)) ([55e8275](https://github.com/substrait-io/substrait/commit/55e827519d70b466e748e5c3fef3a568733a9076))
* add regexp_match_substring, regexp_strpos, and regexp_count_substring ([#293](https://github.com/substrait-io/substrait/issues/293)) ([6b8191f](https://github.com/substrait-io/substrait/commit/6b8191f304d28171dfc8edb5a82c1e254284cd5b))
* add regexp_replace function ([#281](https://github.com/substrait-io/substrait/issues/281)) ([433d049](https://github.com/substrait-io/substrait/commit/433d0493b66d67c048f5e41017c6fdcd521b92d9))
* add string transform functions ([#267](https://github.com/substrait-io/substrait/issues/267)) ([ff2f7f1](https://github.com/substrait-io/substrait/commit/ff2f7f1da8ea38452a3760ccc8a232cd8f59cfee))
* clarify behavior of is_null, is_not_null, is_nan, is_finite, and is_infinite for nulls ([#285](https://github.com/substrait-io/substrait/issues/285)) ([cb25124](https://github.com/substrait-io/substrait/commit/cb25124d2d12f629a2f6335bb4f2563c1745758f))

## [0.12.0](https://github.com/substrait-io/substrait/compare/v0.11.0...v0.12.0) (2022-08-28)


### Features

* add between function ([#287](https://github.com/substrait-io/substrait/issues/287)) ([aad6f63](https://github.com/substrait-io/substrait/commit/aad6f637a19c96f02effc7bd5068f4c2d11525c4))
* add case_sensitivity option to string functions ([#289](https://github.com/substrait-io/substrait/issues/289)) ([4c354de](https://github.com/substrait-io/substrait/commit/4c354de568ac5448053b1b11a6373fe0b7e7a229))

## [0.11.0](https://github.com/substrait-io/substrait/compare/v0.10.0...v0.11.0) (2022-08-21)


### Features

* add nullif function ([#291](https://github.com/substrait-io/substrait/issues/291)) ([dc677c2](https://github.com/substrait-io/substrait/commit/dc677c226623489786f0def03db2a8c1e0d0116f))
* **set:** add basic set membership operations ([#280](https://github.com/substrait-io/substrait/issues/280)) ([1bd1bd1](https://github.com/substrait-io/substrait/commit/1bd1bd1aa01e11bf769bfc68fbccb81920a46677))

## [0.10.0](https://github.com/substrait-io/substrait/compare/v0.9.0...v0.10.0) (2022-08-14)


### Features

* add and_not boolean function ([#276](https://github.com/substrait-io/substrait/issues/276)) ([8af3fe0](https://github.com/substrait-io/substrait/commit/8af3fe0e874d8006430699628adfc755d4a1a1b0))
* add is_finite and is_infinite ([#286](https://github.com/substrait-io/substrait/issues/286)) ([01d5428](https://github.com/substrait-io/substrait/commit/01d54287f69635b463832c8b84a75a8fa90f9f3f))
* add support for DDL and INSERT/DELETE/UPDATE operations ([#252](https://github.com/substrait-io/substrait/issues/252)) ([cbb6c26](https://github.com/substrait-io/substrait/commit/cbb6c26e16bced5187c779eaa7027c90461e3e2e))

## [0.9.0](https://github.com/substrait-io/substrait/compare/v0.8.0...v0.9.0) (2022-07-31)


### ⚠ BREAKING CHANGES

* **arithmetic:** Options SILENT, SATURATE, ERROR are no longer valid for use with floating point
arguments to add, subtract, multiply or divide
* function argument bindings were open to interpretation
before, and were often produced incorrectly; therefore, this change
semantically shifts some responsibilities from the consumers to the
producers.
* the grouping set index column now only exists if there is more
than one grouping set.
* Existing plans that are modeling `cast` with the `cast`
function (as opposed to the `cast` expression) will no longer be valid. All
producers/consumers should use the `cast` expression type.

### Features

* add functions for arithmetic, rounding, logarithmic, and string transformations ([#245](https://github.com/substrait-io/substrait/issues/245)) ([f7c5da5](https://github.com/substrait-io/substrait/commit/f7c5da5625f50514ba70b9e8a32cb2e7085c24f1))
* add standard deviation functions ([#257](https://github.com/substrait-io/substrait/issues/257)) ([1339534](https://github.com/substrait-io/substrait/commit/13395340f6971f705e43f304005ea540d04780ce))
* add string containment functions ([#256](https://github.com/substrait-io/substrait/issues/256)) ([d6b9b34](https://github.com/substrait-io/substrait/commit/d6b9b344f0f0865573a79feb6ec818c146b47f62))
* add string trimming and padding functions ([#248](https://github.com/substrait-io/substrait/issues/248)) ([8a8f65d](https://github.com/substrait-io/substrait/commit/8a8f65d3860ce8fc09424947b4fb45b8dd21cef0))
* add trigonometry functions ([#241](https://github.com/substrait-io/substrait/issues/241)) ([d83d566](https://github.com/substrait-io/substrait/commit/d83d566851a0fb5d35c2b23ed8aa549b88d6a63b))
* add variance function ([#263](https://github.com/substrait-io/substrait/issues/263)) ([b6c3772](https://github.com/substrait-io/substrait/commit/b6c377216687a6e253d4b7ec77b48a886cfb501a))
* **arithmetic:** add abs and sign to scalar function extensions ([#244](https://github.com/substrait-io/substrait/issues/244)) ([1b9a45f](https://github.com/substrait-io/substrait/commit/1b9a45fd4f4ea9f9db3d3e7132c5db4d06c05e77))
* support window functions ([#224](https://github.com/substrait-io/substrait/issues/224)) ([4b2072a](https://github.com/substrait-io/substrait/commit/4b2072a40447a4f1a3f6875fa0476cc57145ba30))


### Bug Fixes

* **message:** commit lint issue ([#250](https://github.com/substrait-io/substrait/issues/250)) ([34ec8f5](https://github.com/substrait-io/substrait/commit/34ec8f570b7782c1d26bc6d237d461f211dd8078))
* removes cast function definition ([#253](https://github.com/substrait-io/substrait/issues/253)) ([66a3476](https://github.com/substrait-io/substrait/commit/66a347603bd0a2cba27d749864a9bdb1164eb1dd)), closes [#88](https://github.com/substrait-io/substrait/issues/88) [#152](https://github.com/substrait-io/substrait/issues/152)
* specify how function arguments are to be bound ([#231](https://github.com/substrait-io/substrait/issues/231)) ([d4cfbe0](https://github.com/substrait-io/substrait/commit/d4cfbe014e9c126ac008094323a2baca9f47c42d))


### Documentation

* better explain aggregate relations ([#260](https://github.com/substrait-io/substrait/issues/260)) ([42d9ca3](https://github.com/substrait-io/substrait/commit/42d9ca31a032e1fac0248a998501241eaa27b56f))


### Code Refactoring

* **arithmetic:** specify FP overflow and domain options for remaining ops ([#269](https://github.com/substrait-io/substrait/issues/269)) ([de64a3c](https://github.com/substrait-io/substrait/commit/de64a3c8879c6e0219dd405ce18659219ead926a))

## [0.8.0](https://github.com/substrait-io/substrait/compare/v0.7.0...v0.8.0) (2022-07-17)


### ⚠ BREAKING CHANGES

* The signature of divide functions for multiple types now specify an enumeration prior to specifying operands.

### Bug Fixes

* add overflow behavior to integer division ([#223](https://github.com/substrait-io/substrait/issues/223)) ([cf552d7](https://github.com/substrait-io/substrait/commit/cf552d7c76da9a91bce992391356c6ffb5a969ac))

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
