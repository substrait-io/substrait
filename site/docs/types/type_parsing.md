# Type Syntax Parsing

In many places, it is useful to have a human readable string representation of data types. Substrait has a custom syntax for type declaration. The basic structure of a type declaration is:

```
name?[variation]<param0,...,paramN>
```

The components of this expression are:

| Component              | Description                                                  | Required                              |
| ---------------------- | ------------------------------------------------------------ | ------------------------------------- |
| Name                   | Each type has a name. A type is expressed by providing a name. This name can be expressed in arbitrary case (e.g. `varchar` and `vArChAr` are equivalent). |                                       |
| Nullability Indicator  | A type is either non-nullable or nullable. To express nullability, a type name is appended with a question mark. | Optional, defaults to non-nullable    |
| Variation              | When expressing a type, a user can define the type based on a type variation. Some systems use type variations to describe different underlying representations of the same data type. This is expressed as a bracketed integer such as [2]. | Optional, defaults to [0]             |
| Parameters             | Compound types may have one or more configurable properties. The two main types of properties are integer and type properties. The parameters for each type correspond to a list of known properties associated with a type as declared in the order defined in the type specification. For compound types (types that contain types), the data type syntax will include nested type declarations. The one exception is structs, which are further outlined below. | Required where parameters are defined |

### Grammars

It is relatively easy in most languages to produce simple parser & emitters for the type syntax. To make that easier, Substrait also includes an ANTLR [impl pending] grammar to ease consumption and production of types.

### Structs & Named Structs

Structs are unique from other types because they have an arbitrary number of parameters. The parameters can also include one or two subproperties. Struct parsing is thus declared in the following two ways:

```
# Struct
struct?[variation]<type0, type1,..., typeN>

# Named Struct
nstruct?[variation]<name0:type0, name1:type1,..., nameN:typeN>
```

In the normal (non-named) form, struct declares a set of types that are fields within that struct. In the named struct form, the parameters are formed by tuples of names + types, delineated by a colon. Names that are composed only of numbers and letters can be left unquoted. For other characters, names should be quoted with double quotes and use backslash for double-quote escaping.

Note, in core Substrait algebra, fields are unnamed and references are always based on zero-index ordinal positions. However, data inputs must declare name-to-ordinal mappings and outputs must declare ordinal-to-name mappings. As such, Substrait also provides a named struct which is a pseudo-type that is useful for human consumption. Outside of these places, most structs in a substrait plan are structs, not named-structs. The two cannot be used interchangably.
