Procedural macro library for core
=================================

This library contains some `#[derive]` macros for the core library,
specifically for the types generated by `prost-build`. This is needed because
`prost-build` on its own doesn't generate any introspection-like information
for the protobuf structures, such as message type names as strings, which we
want to be able to use in our parse tree.