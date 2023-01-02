# Powered by Substrait

In addition to the work maintained in repositories within the
[substrait-io GitHub organization](https://github.com/substrait-io), a growing
list of other open source projects have adopted Substrait.


[//]: # (Maintain this list in alphabetical order)

[Acero](https://arrow.apache.org/docs/cpp/streaming_execution.html)
: Acero is a query execution engine implemented as a part of the Apache Arrow
  C++ library. Acero provides a Substrait consumer interface.

[ADBC](https://arrow.apache.org/adbc/)
: ADBC (Arrow Database Connectivity) is an API specification for Apache
  Arrow-based database access. ADBC allows applications to pass queries either
  as SQL strings or Substrait plans.

[Arrow Flight SQL](https://arrow.apache.org/docs/format/FlightSql.html)
: Arrow Flight SQL is a client-server protocol for interacting with databases
  and query engines using the Apache Arrow in-memory columnar format and the
  [Arrow Flight RPC](https://arrow.apache.org/docs/format/Flight.html)
  framework. Arrow Flight SQL allows clients to send queries as SQL strings or
  Substrait plans.

[DataFusion](https://arrow.apache.org/datafusion/)
: DataFusion is an extensible query planning, optimization, and execution
  framework, written in Rust, that uses Apache Arrow as its in-memory format.
  DataFusion provides a
  [Substrait producer and consumer](https://github.com/datafusion-contrib/datafusion-substrait)
  that can convert DataFusion logical plans to and from Substrait plans.

[DuckDB](https://duckdb.org)
: DuckDB is an in-process SQL OLAP database management system. DuckDB provides
  a [Substrait extension](https://duckdb.org/docs/extensions/substrait)
  that allows users to produce and consume Substrait plans through DuckDB's
  SQL, Python, and R APIs.

[Gluten](https://github.com/oap-project/gluten)
: Gluten is a plugin for Apache Spark that allows computation to be offloaded
  to engines that have better performance or efficiency than Spark's built-in
  JVM-based engine. Gluten converts Spark physical plans to Substrait plans.

[Ibis](https://ibis-project.org/)
: Ibis is a Python library that provides a lightweight, universal interface 
  for data wrangling. It includes a dataframe API for Python with support for
  more than 10 query execution engines, plus a 
  [Substrait producer](https://github.com/ibis-project/ibis-substrait)
  to enable support for Substrait-consuming execution engines.

[Substrait R Interface](https://github.com/voltrondata/substrait-r)
: The Substrait R interface package allows users to construct Substrait plans
  from R for evaluation by Substrait-consuming execution engines. The package
  provides a [dplyr](https://dplyr.tidyverse.org) backend as well as
  lower-level interfaces for creating Substrait plans and integrations with
  Acero and DuckDB.

[Velox](https://velox-lib.io)
: Velox is a unified execution engine aimed at accelerating data management
  systems and streamlining their development. Velox provides a Substrait
  consumer interface.


To add your project to this list, please open a
[pull request](https://github.com/substrait-io/substrait/edit/main/site/docs/community/powered_by.md) 
