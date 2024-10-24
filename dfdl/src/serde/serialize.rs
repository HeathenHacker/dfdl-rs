use std::borrow::Cow;
use std::fmt::Display;

use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use serde::Serializer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("")]
    Custom(String),
    #[error("no associated key found")]
    NoKey,
}
impl serde::ser::Error for SerializationError {
    fn custom<T>(msg:T) -> Self 
        where T:Display 
    {
        Self::Custom(msg.to_string())
    }
}


use crate::{ComplexElement, Data, Element, SimpleElement};

#[derive(Clone, Debug)]
pub(crate) struct ElementSerializer<'e> {
    pub(crate) name: Cow<'e, str>,
    pub(crate) namespace: Cow<'e, str>,
    pub(crate) array: bool,
}
impl<'e> Serializer for ElementSerializer<'e> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    type SerializeSeq = SequenceSerializer<'e>;

    type SerializeTuple = TupleSerializer<'e>;

    type SerializeTupleStruct = TupleStructSerializer<'e>;

    type SerializeTupleVariant = TupleVariantSerializer<'e>;

    type SerializeMap = MapSerializer<'e>;

    type SerializeStruct = StructSerializer<'e>;

    type SerializeStructVariant = StructVariantSerializer<'e>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Boolean(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Byte(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Short(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Int(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Long(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::UnsignedByte(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }),true))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::UnsignedShort(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::UnsignedInt(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::UnsignedLong(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Float(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::Double(v));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }),true))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::String(Cow::Owned(v.to_string())));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::String(Cow::Owned(v.to_string())));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let data = Some(Data::HexBinary(Cow::Owned(v.to_vec())));
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let data = None;
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: true,
            valid: true,
            data,
        }), true))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        let data = None;
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        let data = None;
        Ok((Element::SimpleElement( SimpleElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            data,
        }), true))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let child = Element::SimpleElement(SimpleElement { 
            schema: String::new(),
            namespace: self.namespace.to_string(),
            name: variant.to_string(),
            nilled: false,
            array: false,
            valid: true,
            union_member_schema: String::new(),
            data: None });
        Ok((Element::ComplexElement( ComplexElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            children: vec![child],
        }), true))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let namespace = self.namespace.to_string();
        let child_serializer = ElementSerializer {
            name: Cow::Borrowed(variant),
            namespace: self.namespace,
            array: false,
        };
        let (child, _) = value.serialize(child_serializer)?;
        
        Ok((Element::ComplexElement( ComplexElement {
            name: self.name.to_string(),
            namespace,
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            children: vec![child],
        }), true))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SequenceSerializer {
            ref_element: self,
            sequence_components: len.map_or_else(Vec::new, Vec::with_capacity)
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleSerializer { 
            ref_elem: self,
            tuple_components: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TupleStructSerializer { 
            ref_elem: self,
            tuple_components: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let element = ComplexElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            children: Vec::with_capacity(len),
        };
        Ok(TupleVariantSerializer { 
            ref_elem: ElementSerializer {
                namespace: self.namespace,
                name: Cow::Borrowed(variant),
                array: true,
            },
            variant_element: element,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            ref_elem: self,
            pairs: len.map_or_else(Vec::new, Vec::with_capacity),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            ref_elem: self,
            children: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let choice = ComplexElement {
            name: self.name.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: self.array,
            nilled: false,
            valid: true,
            children: Vec::with_capacity(1),
        };
        let fields = ComplexElement {
            name: variant.to_string(),
            namespace: self.namespace.to_string(),
            schema: String::new(),
            union_member_schema: String::new(),
            array: false,
            nilled: false,
            valid: true,
            children: Vec::with_capacity(len),
        };
        Ok(StructVariantSerializer {
            ref_elem: self,
            choice,
            fields,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SequenceSerializer<'e> {
    ref_element: ElementSerializer<'e>,
    sequence_components: Vec<Element<'static>>,
}
impl SerializeSeq for SequenceSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let mut element_serializer = self.ref_element.clone();
        element_serializer.array = true;
        let (elem, _) = value.serialize(element_serializer)?;
        self.sequence_components.push(elem);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(ComplexElement { 
            schema: String::new(),
            namespace: self.ref_element.namespace.to_string(),
            name: self.ref_element.name.to_string(),
            nilled: false,
            array: self.ref_element.array,
            valid: true,
            union_member_schema: String::new(),
            children: self.sequence_components, 
        }), true))
    }
}



pub(crate) struct TupleSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    tuple_components: Vec<Element<'static>>,
}
impl SerializeTuple for TupleSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let mut element_serializer = self.ref_elem.clone();
        element_serializer.array = true;
        let (elem, _) = value.serialize(element_serializer)?;
        self.tuple_components.push(elem);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(crate::ComplexElement { 
            schema: String::new(),
            namespace: self.ref_elem.namespace.to_string(),
            name: self.ref_elem.name.to_string(),
            nilled: false,
            array: self.ref_elem.array,
            valid: true,
            union_member_schema: String::new(),
            children: self.tuple_components, 
        }), true))
    }
} 

pub(crate) struct TupleStructSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    tuple_components: Vec<Element<'static>>,
}
impl SerializeTupleStruct for TupleStructSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let mut element_serializer = self.ref_elem.clone();
        element_serializer.array = true;
        let (elem, _) = value.serialize(element_serializer)?;
        self.tuple_components.push(elem);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(crate::ComplexElement { 
            schema: String::new(),
            namespace: self.ref_elem.namespace.to_string(),
            name: self.ref_elem.name.to_string(),
            nilled: false,
            array: self.ref_elem.array,
            valid: true,
            union_member_schema: String::new(),
            children: self.tuple_components, 
        }), true))
    }
} 

pub(crate) struct TupleVariantSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    variant_element: ComplexElement<'static>,
}
impl SerializeTupleVariant for TupleVariantSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let mut element_serializer = self.ref_elem.clone();
        element_serializer.array = true;
        let (elem, _) = value.serialize(element_serializer)?;
        self.variant_element.children.push(elem);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(self.variant_element), true))
    }
} 

pub(crate) struct MapSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    pairs: Vec<Element<'static>>,
}
impl SerializeMap for MapSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let key_ser = ElementSerializer {
            name: Cow::Borrowed("key"),
            namespace: self.ref_elem.namespace.clone(),
            array: false,
        };
        let (key, _) = key.serialize(key_ser)?;
        self.pairs.push(key);
        //self.key = Some(key);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let value_ser = ElementSerializer {
            name: Cow::Borrowed("value"),
            namespace: self.ref_elem.namespace.clone(),
            array: false,
        };
        let (value, _) = value.serialize(value_ser)?;
        //let key = self.key.take().ok_or(SerializationError::NoKey)?;
        /*let pair = Element::ComplexElement(ComplexElement {
            schema: String::new(),
            namespace: self.ref_elem.namespace.to_string(),
            name: self.ref_elem.name.to_string(),
            nilled: false,
            array: true,
            valid: true,
            union_member_schema: String::new(),
            children: vec![key, value],
        });*/
        self.pairs.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(ComplexElement {
            schema: String::new(),
            namespace: self.ref_elem.namespace.to_string(),
            name: self.ref_elem.name.to_string(),
            nilled: false,
            array: self.ref_elem.array,
            valid: true,
            union_member_schema: String::new(),
            children: self.pairs,
        }), false))

    }
} 

pub(crate) struct StructSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    children: Vec<Element<'static>>,
}
impl SerializeStruct for StructSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let elem = ElementSerializer {
            name: Cow::Borrowed(key),
            namespace: self.ref_elem.namespace.clone(),
            array: false,
        };
        let (child, can_flatten) = value.serialize(elem)?;
        if can_flatten && matches!(child, Element::ComplexElement(ComplexElement {
            ref children,
    ..
        }) if children.first().is_some_and(|child| matches!(child, 
            Element::ComplexElement(ComplexElement{array: true, ..})
            | Element::SimpleElement(SimpleElement {array: true, ..})
        ))) {
            println!("flattening");
            let Element::ComplexElement(child) = child else {unreachable!()};
            self.children.extend(child.children);
        }
        else {
            self.children.push(child);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok((Element::ComplexElement(ComplexElement {
            array: self.ref_elem.array,
            schema: String::new(),
            name: self.ref_elem.name.into_owned(),
            namespace: self.ref_elem.namespace.into_owned(),
            nilled: false,
            valid: true,
            union_member_schema: String::new(),
            children: self.children,
        }), true))
    }
} 

pub(crate) struct StructVariantSerializer<'e> {
    ref_elem: ElementSerializer<'e>,
    choice: ComplexElement<'static>,
    fields: ComplexElement<'static>,
}
impl SerializeStructVariant for StructVariantSerializer<'_> {
    type Ok = (Element<'static>, bool);

    type Error = SerializationError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        let elem = ElementSerializer {
            name: Cow::Borrowed(key),
            namespace: self.ref_elem.namespace.clone(),
            array: false,
        };
        let (child, can_flatten) = value.serialize(elem)?;
        if can_flatten && matches!(child, Element::ComplexElement(ComplexElement {
            ref children,
    ..
        }) if children.first().is_some_and(|child| matches!(child, 
            Element::ComplexElement(ComplexElement{array: true, ..})
            | Element::SimpleElement(SimpleElement {array: true, ..})
        ))) {
            let Element::ComplexElement(child) = child else {unreachable!()};
            self.fields.children.extend(child.children);
        }
        else {
            self.fields.children.push(child);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut choice = self.choice;
        choice.children.push(Element::ComplexElement(self.fields));
        Ok((Element::ComplexElement(choice), true))
    }
} 






