# dfdl

A DFDL Implementation for Rust
==============================
<div class="warning">Still In early development, no acutual functionality provided yet</div>

The [Data Format Description Language (DFDL)](https://ogf.org/ogf/doku.php/standards/dfdl/dfdl) is a
language developed by the Open Grid Forum (OGF), to describe binary and textual data formats
via an extention to XML Schemas.

This crate aims to be a rust implementation of the language specification, providing facilities
to parse and unparse DFDL described data formats to a DFDL Infoset, and to provide ways of
interacting with these Infosets (e.g. via [`serde`])

Aims:
- [ ] implementation of a parser & unparser conforming to the core DFDL specification
- [ ] ability to serialize/deserialize to/from the DFDL Infoset
- [ ] generating structs & parsers for a DFDL schema at compile time
- [ ] generating schemas & parsers for existing rust structs

[`serde`]: https://serde.rs

License: MIT
