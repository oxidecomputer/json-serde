# json-serde

[![json-serde on crates.io](https://img.shields.io/crates/v/json-serde)](https://crates.io/crates/json-serde)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest%20version-brightgreen.svg)](https://docs.rs/json-serde)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/oxidecomputer/json-serde/blob/main/LICENSE)

Runtime serde helpers for esoteric JSON semantics

## Overview

Serde's derived implementations cover typical serialization and deserialization
for native Rust types. There are type serializations, however, specified by
JSON Schema that can't be modeled by those derived implementations.
`json-serde` provides helpers for those cases. It exists to be referenced by
generated code--in particular from
[typify](https://github.com/oxidecomputer/typify) and
[progenitor](https://github.com/oxidecomputer/progenitor) code generators (via
[typespace](https://github.com/oxidecomputer/typespace)) that translate JSON
Schema and OpenAPI (respectively) into Rust code.

`json-serde` depends only on `serde_core` (plus optional schemars 0.8 and/or
1.x via the `schemars08` and `schemars1` features).

## Helpers

### Absent vs. `null` for `Option<T>`

A longstanding design decision of `serde` is that an `Option<T>` field may
either be absent or have a `null` value. Schemas may be more specific, allowing
a field to be `null` or absent or both. `deserialize_some` deserializes
`Option<T>` fields such that a present value always produces `Some`; combined
with `#[serde(default)]` it distinguishes absent from `null`. Applied to an
`Option<T>` field, absent is fine but `null` is an error; applied to a double
`Option<Option<T>>`, absent, `null`, and a value each map to a distinct state:

```rust
#[derive(serde::Deserialize, serde::Serialize)]
struct Data {
    /// may be absent, but may not be null
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = "Option::is_none",
    )]
    required_field: Option<String>,

    /// distinct states for absent, null, and a value
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = "Option::is_none"
    )]
    tri_state_field: Option<Option<String>>,
}
```

In addition, the `OptionalNullable` trait can be implemented for types that
model this tri-state of absent, null, or a value. It has a default
implementation for `Option<Option<T>>`.

```rust
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OptionField<T> {
    #[default]
    #[serde(skip)]
    Absent,
    Null,
    Present(T),
}

impl<T> json_serde::OptionalNullable for OptionField<T> {
    type Target = T;

    fn is_absent(&self) -> bool {
        matches!(self, OptionField::Absent)
    }

    fn null() -> Self {
        Self::Null
    }

    fn value(value: Self::Target) -> Self {
        Self::Present(value)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Data {
    /// default constructs an absent value; deserialize accepts a null or value
    #[serde(
        default,
        skip_serializing_if = "::json_serde::OptionalNullable::is_absent"
    )]
    optional_option: OptionField<String>,
}
```

### "Flattened" sequences

`serde` allows a struct to be "flattened" (included) in another struct. It
doesn't allow a sequence (e.g. `Vec<T>`) to be "flattened" into, say, a tuple.
JSON Schema allows such constructions. `FlattenedSequenceSerializer` and
`FlattenedSequenceDeserializer` flatten one sequence into the tail of an
enclosing sequence--e.g. a tuple struct with a "rest" field whose elements
share the enclosing JSON array--for use within custom `Serialize` and
`Deserialize` impls.

### Absent

With its `deny_unknown_fields`, `serde` disallows unspecified properties from
appearing in an object. JSON Schema, however, is more granular: in some cases,
specific, named properties may be disallowed. To handle these cases, the
`Absent` type disallows a specific field from appearing during deserialization.

With the `schemars1` or `schemars08` feature enabled, its `JsonSchema` impl
emits the `false`--unsatisfiable--schema.

On types deriving `JsonSchema` (either version), use
`#[serde(skip_serializing_if = "::json_serde::always")]` instead of
`skip_serializing`. The `always` predicate serializes identically and produces
better schemas: schemars 0.8 (through 0.8.22) incorrectly marks `default` +
`skip_serializing` fields as required (fix proposed in
[GREsau/schemars#532](https://github.com/GREsau/schemars/pull/532)), and
schemars 1.0 (as of 1.2.2) spuriously annotates the field `writeOnly` (when,
in fact, no value can be written!; fix proposed in
[GREsau/schemars#533](https://github.com/GREsau/schemars/pull/533)).

## Features

- `schemars08`: implements the schemars 0.8 `JsonSchema` trait for
  `Absent`.
- `schemars1`: implements the schemars 1.x `JsonSchema` trait for `Absent`.

The two features are independent and may be enabled together. Both are
derive-less for consumers.

## Notes

- Early alpha; API unstable.
- Part of the typify/progenitor code-generation stack.
