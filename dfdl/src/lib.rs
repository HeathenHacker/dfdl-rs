#![warn(missing_docs)]
//! A DFDL Implementation for Rust
//! ==============================
//! <div class="warning">Still In early development, no acutual functionality provided yet</div>
//!
//! The [Data Format Description Language (DFDL)](https://ogf.org/ogf/doku.php/standards/dfdl/dfdl) is a
//! language developed by the Open Grid Forum (OGF), to describe binary and textual data formats
//! via an extention to XML Schemas.
//!
//! This crate aims to be a rust implementation of the language specification, providing facilities
//! to parse and unparse DFDL described data formats to a DFDL Infoset, and to provide ways of
//! interacting with these Infosets (e.g. via [`serde`])
//!
//! Aims:
//! - [ ] implementation of a parser & unparser conforming to the core DFDL specification 
//! - [ ] ability to serialize/deserialize to/from the DFDL Infoset
//! - [ ] generating structs & parsers for a DFDL schema at compile time
//! - [ ] generating schemas & parsers for existing rust structs
//!
//! [`serde`]: https://serde.rs


use std::borrow::Cow;
/// XML namespace for DFDL
pub const DFDL_NAMESPACE: &str = r"http://www.ogf.org/dfdl/dfdl-1.0/";
/// XML namespace for DFDL Annotations
pub const DFDL_ANNOTATION_SOURCE: &str = r"http://www.ogf.org/dfdl/";
/// XML namespace for XML Schema (XSD)
pub const XSD_NAMESPACE: &str = r"http://www.w3.org/2001/XMLSchema";

#[cfg(feature = "serde")]
/// utilities relating to serde (serializing, deserializing)
pub mod serde;


#[derive(Clone, Debug)]
pub struct Infoset<'i> {
    dfdl_version: String,
    root_element: Option<Element<'i>>
}
impl Infoset<'_> {
    /// Returns the DFDL version of this [`Infoset`].
    #[must_use]
    pub fn version(&self) -> &str {
        &self.dfdl_version
    }
    /// Turns this [`Infoset`] onto a version of itself that owns all its data
    /// specifically, any borrowed string and/or binary sequence is turned into
    /// an owned version of itself
    pub fn into_owned(self) -> Infoset<'static> {
        Infoset { dfdl_version: self.dfdl_version, root_element: self.root_element.map(Element::into_owned) }
    }
}

#[derive(Clone, Debug)]
enum Element<'e> {
    SimpleElement(SimpleElement<'e>),
    ComplexElement(ComplexElement<'e>),
}

impl Element<'_> {
    fn is_some(&self) -> bool {
        match self {
            Self::SimpleElement(SimpleElement { nilled, .. }) |
            Self::ComplexElement(ComplexElement { nilled, .. }) => !nilled,
        }
    }
    /*fn is_none(&self) -> bool {
        match self {
            Self::SimpleElement(SimpleElement { nilled, .. }) => *nilled,
            Self::ComplexElement(ComplexElement { nilled, .. }) => *nilled,
        }
    }*/
    fn name(&self) -> &str {
        match self {
            Self::SimpleElement(SimpleElement {name, ..}) |
            Self::ComplexElement(ComplexElement {name, ..}) => name,
        }
    }
    fn is_array(&self) -> bool {
        match self {
            Self::SimpleElement(SimpleElement {array, ..}) |
            Self::ComplexElement(ComplexElement {array, ..}) => *array,
        }
    }
    fn typename(&self) -> &str {
        match self {
            Element::SimpleElement(SimpleElement {data: Some(data), ..}) => data.typename(),
            Element::SimpleElement(SimpleElement {data: None, ..}) => todo!(),
            Element::ComplexElement(_complex_element) => "",
        }
    }
    fn into_owned(self) -> Element<'static> {
        match self {
            Element::SimpleElement(simple_element) => Element::SimpleElement(simple_element.into_owned()),
            Element::ComplexElement(complex_element) => Element::ComplexElement(complex_element.into_owned()),
        }
    }
}


#[derive(Clone, Debug)]
struct SimpleElement<'e> {
    schema: String,
    namespace: String,
    name: String,
    nilled: bool,
    array: bool,
    valid: bool,
    union_member_schema: String,
    data: Option<Data<'e>>,
}
impl SimpleElement<'_> {
    fn into_owned(self) -> SimpleElement<'static> {
        SimpleElement {
            schema: self.schema,
            namespace: self.namespace,
            name: self.name,
            nilled: self.nilled,
            array: self.array,
            valid: self.valid,
            union_member_schema: self.union_member_schema,
            data: self.data.map(Data::into_owned),
        }
    }
}

#[derive(Clone, Debug)]
enum Data<'d> {
    Double(f64),
    Float(f32),
    Decimal(/*TODO*/),
    Integer(i32), //TODO: Generic Int?
    NonNegativeInteger(u32), //TODO:: Generic Int?
    Long(i64),
    Int(i32),
    Short(i16),
    Byte(i8),
    UnsignedLong(u64),
    UnsignedInt(u32),
    UnsignedShort(u16),
    UnsignedByte(u8),

    String(Cow<'d, str>),
    //TODO: Calendar Types
    DateTime,
    Date,
    Time,

    HexBinary(Cow<'d, [u8]>),

    Boolean(bool),

}

impl Data<'_> {
    fn typename(&self) -> &'static str {
        match self {
            Data::Double(_) => "f64",
            Data::Float(_) => "f32",
            Data::Decimal() => "decimal",
            Data::Integer(_) => "integer",
            Data::NonNegativeInteger(_) => "unsigned",
            Data::Long(_) => "i64",
            Data::Int(_) => "i32",
            Data::Short(_) => "i16",
            Data::Byte(_) => "i8",
            Data::UnsignedLong(_) => "u64",
            Data::UnsignedInt(_) => "u32",
            Data::UnsignedShort(_) => "u16",
            Data::UnsignedByte(_) => "u8",
            Data::String(_) => "string",
            Data::DateTime => "datetime",
            Data::Date => "date",
            Data::Time => "time",
            Data::HexBinary(_) => "hexnumber",
            Data::Boolean(_) => "bool",
        }
    }
    fn into_owned(self) -> Data<'static> {
        match self {
            Data::Double(data) => Data::Double(data),
            Data::Float(data) => Data::Float(data),
            Data::Decimal() => Data::Decimal(),
            Data::Integer(data) => Data::Integer(data),
            Data::NonNegativeInteger(data) => Data::NonNegativeInteger(data),
            Data::Long(data) => Data::Long(data),
            Data::Int(data) => Data::Int(data),
            Data::Short(data) => Data::Short(data),
            Data::Byte(data) => Data::Byte(data),
            Data::UnsignedLong(data) => Data::UnsignedLong(data),
            Data::UnsignedInt(data) => Data::UnsignedInt(data),
            Data::UnsignedShort(data) => Data::UnsignedShort(data),
            Data::UnsignedByte(data) => Data::UnsignedByte(data),
            Data::String(cow) => Data::String(Cow::Owned(cow.into_owned())),
            Data::DateTime => Data::DateTime,
            Data::Date => Data::Date,
            Data::Time => Data::Time,
            Data::HexBinary(cow) => Data::HexBinary(Cow::Owned(cow.into_owned())),
            Data::Boolean(data) => Data::Boolean(data),
        }
    }
}

#[derive(Clone, Debug)]
struct ComplexElement<'e> {
    #[allow(dead_code)]
    schema: String,
    #[allow(dead_code)]
    namespace: String,
    name: String,
    nilled: bool,
    array: bool,
    #[allow(dead_code)]
    valid: bool,
    #[allow(dead_code)]
    union_member_schema: String,
    children: Vec<Element<'e>>,
}

impl ComplexElement<'_> {
    fn into_owned(self) -> ComplexElement<'static> {
        let children = self.children.into_iter().map(Element::into_owned).collect();
        ComplexElement {
            schema: self.schema,
            namespace: self.namespace,
            name: self.name,
            nilled: self.nilled,
            array: self.array,
            valid: self.valid,
            union_member_schema: self.union_member_schema,
            children,
        }
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
