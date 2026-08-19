// Copyright 2026 Oxide Computer Company

#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

// Alias the crate under its external name so the unit tests can use the
// documented attribute recipes verbatim.
#[cfg(test)]
extern crate self as json_serde;

use serde_core::{
    Deserialize, Deserializer, Serializer,
    de::Error,
    ser::{Impossible, SerializeSeq},
};

/// Deserializer function that always produces `Some(T)` if a value is present.
///
/// It is useful when one wants to distinguish between a field that's absent
/// and a field that's present with a `null` value. For example, the annotation
/// below may be used for a field that may be absent, but may not be `null`.
///
/// ```
/// # #[derive(serde::Deserialize, serde::Serialize)]
/// # struct Foo {
///     #[serde(
///         default,
///         deserialize_with = "::json_serde::deserialize_some",
///         skip_serializing_if = "Option::is_none",
///     )]
///     field: Option<String>,
/// # }
/// ```
///
/// It can also be used with a "double-Option" to determine whether a field
/// was absent, `null`, or had a value:
/// ```
/// # #[derive(serde::Deserialize, serde::Serialize)]
/// # struct Foo {
///     #[serde(
///         default,
///         deserialize_with = "::json_serde::deserialize_some",
///         skip_serializing_if = "Option::is_none",
///     )]
///     field: Option<Option<String>>,
/// # }
/// ```
///
/// In the first case, a `null` value results in an error because a `String`
/// cannot be deserialized from `null`. In the second case, a `null` value
/// results in `field` having a value of `Some(None)` since `Option<String>`
/// *can* be deserialized from `null`.
pub fn deserialize_some<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

/// Serializer used to flatten sequences into other sequences.
///
/// Wrap an in-progress [`SerializeSeq`] and serialize a sequence-shaped
/// value into the wrapper: the value's elements are appended to the
/// enclosing sequence. Values that do not serialize as a sequence are an
/// error. Ending the flattened sequence leaves the enclosing serializer
/// open for further elements.
///
/// The value must serialize as a serde *seq* (e.g. `Vec<T>`); fixed-size
/// tuples and arrays serialize via `serialize_tuple` and are rejected.
pub struct FlattenedSequenceSerializer<'a, S>(&'a mut S);

impl<'a, S> FlattenedSequenceSerializer<'a, S>
where
    S: serde_core::ser::SerializeSeq,
{
    /// Wrap the in-progress sequence serializer `seq_serializer`.
    pub fn new(seq_serializer: &'a mut S) -> Self {
        Self(seq_serializer)
    }

    fn wrong_type_error<T>(kind: &str) -> Result<T, S::Error> {
        Err(serde_core::ser::Error::custom(format!(
            "FlattenedSequenceSerializer only supports sequence values, \
             not {kind}",
        )))
    }
}

impl<S> std::fmt::Debug for FlattenedSequenceSerializer<'_, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlattenedSequenceSerializer")
            .finish_non_exhaustive()
    }
}

impl<S> Serializer for FlattenedSequenceSerializer<'_, S>
where
    S: serde_core::ser::SerializeSeq,
{
    type Ok = ();
    type Error = S::Error;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Self::wrong_type_error("tuple")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Self::wrong_type_error("tuple struct")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Self::wrong_type_error("tuple variant")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Self::wrong_type_error("map")
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Self::wrong_type_error("struct")
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Self::wrong_type_error("struct variant")
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("bool")
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("i8")
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("i16")
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("i32")
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("i64")
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("u8")
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("u16")
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("u32")
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("u64")
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("f32")
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("f64")
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("char")
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("str")
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("bytes")
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("None")
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error("Some")
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("unit")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("unit struct")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error("unit variant")
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error("newtype struct")
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error("newtype variant")
    }
}

impl<S> SerializeSeq for FlattenedSequenceSerializer<'_, S>
where
    S: serde_core::ser::SerializeSeq,
{
    type Ok = ();

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        self.0.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Deserializer used to extract flattened sequences from the end of another
/// sequence.
///
/// Wrap an in-progress [`SeqAccess`](serde_core::de::SeqAccess) and
/// deserialize a sequence-shaped value from the wrapper: the value
/// consumes the remaining elements of the enclosing sequence. Target
/// types that do not expect a sequence are an error.
///
/// The target must deserialize as a serde *seq* (e.g. `Vec<T>`);
/// fixed-size tuples and arrays deserialize via `deserialize_tuple` and
/// are rejected.
pub struct FlattenedSequenceDeserializer<'a, S>(&'a mut S);

impl<'a, S> FlattenedSequenceDeserializer<'a, S> {
    /// Wrap the in-progress sequence access `seq_access`.
    pub fn new<'de>(seq_access: &'a mut S) -> Self
    where
        S: serde_core::de::SeqAccess<'de>,
    {
        Self(seq_access)
    }
}

impl<S> std::fmt::Debug for FlattenedSequenceDeserializer<'_, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlattenedSequenceDeserializer")
            .finish_non_exhaustive()
    }
}

impl<'de, S> Deserializer<'de> for FlattenedSequenceDeserializer<'_, S>
where
    S: serde_core::de::SeqAccess<'de>,
{
    type Error = S::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, S::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        Err(S::Error::custom(
            "FlattenedSequenceDeserializer only supports sequence values",
        ))
    }

    serde_core::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum identifier ignored_any
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        visitor.visit_seq(self.0)
    }
}

/// Always returns `true`; a predicate for `#[serde(skip_serializing_if)]`.
///
/// Use `#[serde(skip_serializing_if = "::json_serde::always")]` in place of
/// `#[serde(skip_serializing)]` on fields that must never serialize when
/// the containing type also derives `JsonSchema` (either schemars version).
/// The two attribute forms serialize identically, but the schemas differ:
/// schemars 0.8 (through 0.8.22) incorrectly marks `default` +
/// `skip_serializing` fields as required, and schemars 1.x decorates them
/// with `writeOnly`; conditionally-skipped fields avoid both. See
/// [`Absent`].
#[must_use]
#[inline]
pub fn always<T>(_: &T) -> bool {
    true
}

/// Type for a value that *must* be absent.
///
/// This should be accompanied by serde attributes to indicate that:
/// - the default value should be taken `#[serde(default)]`
/// - it should never be serialized `#[serde(skip_serializing)]` (or
///   `#[serde(skip_serializing_if = "::json_serde::always")]`; see
///   [`always`])
///
/// Deserialization always fails--the field must not be present--and
/// serialization fails if it is ever invoked, hence the attributes above.
/// With the `schemars08` and `schemars1` features, `Absent`'s `JsonSchema`
/// implementation is the `false` schema, which no value satisfies.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Absent;

impl serde_core::Serialize for Absent {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde_core::ser::Error;
        Err(S::Error::custom(
            "field must be annotated with `skip_serializing` (or \
             `skip_serializing_if = \"json_serde::always\"`)",
        ))
    }
}

impl<'de> Deserialize<'de> for Absent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_core::de::Error;
        // Chew up any inputs.
        let _ = serde_core::de::IgnoredAny::deserialize(deserializer)?;
        Err(D::Error::custom("field must be absent"))
    }
}

#[cfg(feature = "schemars08")]
impl schemars08::JsonSchema for Absent {
    fn schema_name() -> String {
        "Absent".to_string()
    }

    fn json_schema(_: &mut schemars08::r#gen::SchemaGenerator) -> schemars08::schema::Schema {
        schemars08::schema::Schema::Bool(false)
    }

    fn is_referenceable() -> bool {
        false
    }
}

#[cfg(feature = "schemars1")]
impl schemars1::JsonSchema for Absent {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Absent")
    }

    fn json_schema(_: &mut schemars1::SchemaGenerator) -> schemars1::Schema {
        schemars1::Schema::from(false)
    }

    fn inline_schema() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize, ser::SerializeSeq};

    use crate::{Absent, FlattenedSequenceDeserializer, FlattenedSequenceSerializer};

    #[test]
    fn test_deserialize_some() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            #[serde(
                default,
                deserialize_with = "::json_serde::deserialize_some",
                skip_serializing_if = "Option::is_none"
            )]
            field: Option<String>,
        }

        // An absent field yields None.
        let de = serde_json::from_str::<Test>("{}").unwrap();
        assert_eq!(de.field, None);

        // A null value is an error: a String cannot be deserialized from
        // null.
        assert!(serde_json::from_str::<Test>(r#"{ "field": null }"#).is_err());

        // A present value yields Some.
        let de = serde_json::from_str::<Test>(r#"{ "field": "value" }"#).unwrap();
        assert_eq!(de.field, Some("value".to_string()));
    }

    #[test]
    fn test_deserialize_some_double_option() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Test {
            #[serde(
                default,
                deserialize_with = "::json_serde::deserialize_some",
                skip_serializing_if = "Option::is_none"
            )]
            field: Option<Option<String>>,
        }

        // An absent field yields None.
        let de = serde_json::from_str::<Test>("{}").unwrap();
        assert_eq!(de.field, None);

        // A null value yields Some(None).
        let de = serde_json::from_str::<Test>(r#"{ "field": null }"#).unwrap();
        assert_eq!(de.field, Some(None));

        // A present value yields Some(Some(..)).
        let de = serde_json::from_str::<Test>(r#"{ "field": "value" }"#).unwrap();
        assert_eq!(de.field, Some(Some("value".to_string())));
    }

    #[test]
    fn test_flatten_tuple_vec() {
        #[derive(Debug, Eq, PartialEq)]
        struct TestType(u32, String, Vec<u32>);

        impl Serialize for TestType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(None)?;

                seq.serialize_element(&self.0)?;
                seq.serialize_element(&self.1)?;

                self.2
                    .serialize(FlattenedSequenceSerializer::new(&mut seq))?;

                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for TestType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = TestType;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a flattened tuple vec")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let v_0 = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(0, &"a tuple of size 2")
                        })?;
                        let v_1 = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(1, &"a tuple of size 2")
                        })?;

                        let rest =
                            Deserialize::deserialize(FlattenedSequenceDeserializer::new(&mut seq))?;

                        Ok(TestType(v_0, v_1, rest))
                    }
                }
                deserializer.deserialize_seq(Visitor)
            }
        }

        let value = TestType(42, "Hello".to_string(), vec![1, 2, 3]);
        let serialized = serde_json::to_string(&value).unwrap();

        assert_eq!(serialized, "[42,\"Hello\",1,2,3]");

        let de_value = serde_json::from_str::<TestType>(&serialized).unwrap();

        assert_eq!(value, de_value);

        let value = TestType(7, "World".to_string(), vec![]);
        let serialized = serde_json::to_string(&value).unwrap();

        assert_eq!(serialized, "[7,\"World\"]");

        let de_value = serde_json::from_str::<TestType>(&serialized).unwrap();

        assert_eq!(value, de_value);

        let input = "[1, \"Two\", \"Three\", 4, 5, 6]";
        let de_result = serde_json::from_str::<TestType>(input);
        assert!(de_result.is_err());

        let input = "[100]";
        let de_result = serde_json::from_str::<TestType>(input);
        let e = de_result.unwrap_err().to_string();
        assert!(
            e.starts_with("invalid length 1, expected a tuple of size 2"),
            "{e}",
        );

        let input = "[1, \"Two\", \"Three\"]";
        let de_result = serde_json::from_str::<TestType>(input);
        let e = de_result.unwrap_err().to_string();
        assert!(e.starts_with("invalid type"), "{e}");
    }

    /// Serialize `value` into a flattening serializer wrapped around a
    /// JSON array and return the resulting error message.
    fn flatten_ser_err(value: impl Serialize) -> String {
        struct Wrapper<T>(T);

        impl<T: Serialize> Serialize for Wrapper<T> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(None)?;
                self.0
                    .serialize(FlattenedSequenceSerializer::new(&mut seq))?;
                seq.end()
            }
        }

        serde_json::to_string(&Wrapper(value))
            .unwrap_err()
            .to_string()
    }

    #[test]
    fn test_flatten_serializer_rejects_scalar() {
        assert_eq!(
            flatten_ser_err(42u32),
            "FlattenedSequenceSerializer only supports sequence values, \
             not u32",
        );
    }

    #[test]
    fn test_flatten_serializer_rejects_map() {
        let map = std::collections::BTreeMap::from([("key", "value")]);
        assert_eq!(
            flatten_ser_err(map),
            "FlattenedSequenceSerializer only supports sequence values, \
             not map",
        );
    }

    #[test]
    fn test_flatten_deserializer_rejects_non_seq() {
        #[derive(Debug)]
        struct TestType;

        impl<'de> Deserialize<'de> for TestType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = TestType;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        // A non-seq target: u32 forwards to deserialize_any.
                        let _ = u32::deserialize(FlattenedSequenceDeserializer::new(&mut seq))?;
                        Ok(TestType)
                    }
                }
                deserializer.deserialize_seq(Visitor)
            }
        }

        let e = serde_json::from_str::<TestType>("[1, 2, 3]")
            .unwrap_err()
            .to_string();
        assert!(
            e.starts_with("FlattenedSequenceDeserializer only supports sequence values"),
            "{e}",
        );
    }

    #[test]
    fn test_absent_serialize_requires_skip() {
        // Without skip_serializing (or the always predicate), serializing
        // a struct containing Absent is an error.
        #[derive(Serialize)]
        struct Test {
            absent: Absent,
        }

        let e = serde_json::to_string(&Test { absent: Absent })
            .unwrap_err()
            .to_string();
        assert_eq!(
            e,
            "field must be annotated with `skip_serializing` (or \
             `skip_serializing_if = \"json_serde::always\"`)",
        );
    }

    #[test]
    fn test_absent() {
        #[derive(Serialize, Deserialize)]
        struct Test {
            #[serde(default, skip_serializing)]
            absent: Absent,
        }

        let test = Test { absent: Absent };

        assert_eq!(serde_json::to_string(&test).unwrap(), "{}");

        let de = serde_json::from_str::<Test>("{}").unwrap();
        let Absent = de.absent;
        assert!(serde_json::from_str::<Test>(r#"{ "absent": null }"#).is_err());
    }

    #[cfg(feature = "schemars08")]
    #[test]
    fn test_absent_schema() {
        // The `always` helper is necessary due to a bug present in schemars
        // 0.8.22 where default + skip_serializing yields a required
        // property. It is fixed in schemars 1.x.
        #[derive(Serialize, Deserialize, schemars08::JsonSchema)]
        #[schemars(crate = "schemars08")]
        struct Test {
            #[serde(skip_serializing_if = "crate::always")]
            #[serde(default)]
            absent: Absent,
        }

        let test = Test { absent: Absent };

        assert_eq!(serde_json::to_string(&test).unwrap(), "{}");

        let de = serde_json::from_str::<Test>("{}").unwrap();
        let Absent = de.absent;
        assert!(serde_json::from_str::<Test>(r#"{ "absent": null }"#).is_err());

        let schema = schemars08::schema_for!(Test);
        let expected = serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Test",
            "type": "object",
            "properties": {
                "absent": false
            }
        });

        assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
    }

    #[cfg(feature = "schemars1")]
    #[test]
    fn test_absent_schema_v1() {
        // Unlike schemars 0.8.22, schemars 1.x correctly treats default +
        // skip_serializing as an optional property, so no workaround akin to
        // the `always` helper is needed here.
        #[derive(Serialize, Deserialize, schemars1::JsonSchema)]
        #[schemars(crate = "schemars1")]
        struct Test {
            #[serde(default, skip_serializing)]
            absent: Absent,
        }

        let test = Test { absent: Absent };

        assert_eq!(serde_json::to_string(&test).unwrap(), "{}");

        let de = serde_json::from_str::<Test>("{}").unwrap();
        let Absent = de.absent;
        assert!(serde_json::from_str::<Test>(r#"{ "absent": null }"#).is_err());

        let schema = schemars1::schema_for!(Test);
        // schemars 1.x marks skip_serializing fields as `writeOnly`; to
        // attach that keyword it rewrites the `false` schema as its object
        // form, `{"not": {}}`, which is equivalent.
        let expected = serde_json::json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Test",
            "type": "object",
            "properties": {
                "absent": {
                    "not": {},
                    "writeOnly": true
                }
            }
        });

        assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
    }

    #[cfg(feature = "schemars1")]
    #[test]
    fn test_absent_schema_v1_always() {
        // The `always` form is the recommended annotation: a conditionally
        // skipped field gets no `writeOnly` decoration, so the `false`
        // schema survives intact.
        #[derive(Serialize, Deserialize, schemars1::JsonSchema)]
        #[schemars(crate = "schemars1")]
        struct Test {
            #[serde(default, skip_serializing_if = "crate::always")]
            absent: Absent,
        }

        let test = Test { absent: Absent };

        assert_eq!(serde_json::to_string(&test).unwrap(), "{}");

        let de = serde_json::from_str::<Test>("{}").unwrap();
        let Absent = de.absent;
        assert!(serde_json::from_str::<Test>(r#"{ "absent": null }"#).is_err());

        let schema = schemars1::schema_for!(Test);
        let expected = serde_json::json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Test",
            "type": "object",
            "properties": {
                "absent": false
            }
        });

        assert_eq!(serde_json::to_value(&schema).unwrap(), expected);
    }
}
