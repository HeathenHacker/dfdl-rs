use std::fmt::Display;

use serde::de::value::{BorrowedStrDeserializer, MapDeserializer};
use serde::de::{EnumAccess, IntoDeserializer, VariantAccess};
use serde::Deserializer;
use thiserror::Error;

use crate::{Data, Element};

#[derive(Error, Debug)]
pub enum DeserializationError {
    #[error("")]
    Custom(String),
    #[error("missmatched type: expected {expected}, found {infoset}")]
    TypeMismatch{infoset: &'static str, expected: &'static str},

    #[error("element not found")]
    ElementNotFound,

    #[error("not yet implemented")]
    NotImplemented,
}

impl serde::de::Error for DeserializationError {
    fn custom<T>(msg:T) -> Self 
        where T:Display 
    {
        Self::Custom(msg.to_string())
    }
}


#[derive(Debug)]
pub(crate) struct InfosetDeserializer<'is> {
    pub elements: &'is [Element<'is>],
    //pub name: Cow<'is, str>,
    pub name: &'is str,
}


macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond { Result::Err($err)?}
    };
    ($cond: expr) => {
        ensure!($cond, DeserializationError::Custom(format!("unspecified error at {}:{} in {}",line!(), column!(), file!() )))
    }
}
macro_rules! deserialize_builtin {
    ($self:expr, $var:pat, $ret: expr, $expect:literal) => { {
        ensure!($self.elements.len() == 1);
        let Element::SimpleElement(ref el) = $self.elements[0] else {return Err(DeserializationError::TypeMismatch{
            infoset: "<complex>", expected: $expect
        })};
        let Some(d) = &el.data else { return Err(DeserializationError::ElementNotFound)};
        match d {
            $var => $ret,
            _ => return Err(DeserializationError::TypeMismatch{
            infoset: d.typename(), expected: $expect
        }),
        }

    }}
}

impl<'de> Deserializer<'de> for InfosetDeserializer<'de> 
{
    type Error = DeserializationError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(DeserializationError::NotImplemented)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Boolean(data), data, "bool");
        visitor.visit_bool(*v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Byte(data), data, "i8");
        visitor.visit_i8(*v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Short(data), data, "i16");
        visitor.visit_i16(*v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Int(data), data, "i32");
        visitor.visit_i32(*v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Long(data), data, "i64");
        visitor.visit_i64(*v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::UnsignedByte(data), data, "u8");
        visitor.visit_u8(*v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::UnsignedShort(data), data, "u16");
        visitor.visit_u16(*v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::UnsignedInt(data), data, "u32");
        visitor.visit_u32(*v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::UnsignedLong(data), data, "u64");
        visitor.visit_u64(*v)
    }


    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Float(data), data, "f32");
        visitor.visit_f32(*v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::Double(data), data, "f64");
        visitor.visit_f64(*v)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::String(data), data, "char");
        let mut chars = v.chars();
        let Some(first) = chars.next() else {return Err(DeserializationError::TypeMismatch { 
            infoset: "string" , expected: "char"
        })};
        if chars.count() == 0 {
            visitor.visit_char(first)
        }
        else {
            Err(DeserializationError::TypeMismatch { infoset: "string", expected: "char" })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::String(data), data, "string");
        visitor.visit_borrowed_str(v)

    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let v = deserialize_builtin!(self, Data::HexBinary(data), data, "hexnumber");
        visitor.visit_borrowed_bytes(v)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len() == 1);
        let el = &self.elements[0];
        if el.is_some() {
            visitor.visit_some(self)
        }
        else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len() == 1);
        ensure!(self.elements[0].is_some());
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len() == 1);
        let Element::ComplexElement(elem) = &self.elements[0] else {
            return Err(DeserializationError::TypeMismatch { infoset: "<simple>", expected: "<complex>" })
        };
        let children = elem.children.iter()
            .skip_while(|e| e.name() != self.name)
            .take_while(|e| e.name() == self.name)
            .map(|e| {
                InfosetDeserializer {
                    elements: std::slice::from_ref(e),
                    name: self.name,
                }
            });
        let seq = serde::de::value::SeqDeserializer::new(children);
        visitor.visit_seq(seq)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len()== 1);
        let Element::ComplexElement(elem) = &self.elements[0] else {
            return Err(DeserializationError::TypeMismatch { infoset: "<simple>", expected: "<complex>" })
        };
        let pairs = elem.children.chunks_exact(2).map(|c| {
            let key = InfosetDeserializer {
                elements: &c[0..1],
                name: "key"
            };
            let value = InfosetDeserializer {
                elements: &c[1..2],
                name: "value",
            };
            (key, value)
        });
        visitor.visit_map(MapDeserializer::new(pairs))

    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len() == 1);
        let Element::ComplexElement(elem) = &self.elements[0] else {
            return Err(DeserializationError::TypeMismatch { infoset: "<simple>", expected: "<complex>" })
        };
        let fields = fields.iter().filter_map(|&field| {
            let c = elem.children.iter().find(|e| e.name() == field)?;
            let deser = if c.is_array() {
                InfosetDeserializer {
                    elements: self.elements,
                    //name: Cow::Borrowed(field),
                    name: field,
                }
            }
            else {
                InfosetDeserializer {
                    elements: std::slice::from_ref(c),
                    //name: Cow::Borrowed(field),
                    name: field,
                }
            };
            Some((field, deser))
        });
        let map = MapDeserializer::new(fields);
        visitor.visit_map(map)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        println!("variants: {variants:?}");
        println!("deser: {self:?}");
        ensure!(self.elements.len() == 1);
        let en_de = EnumDeserializer {
            element: &self.elements[0],
            variants,
        };
        visitor.visit_enum(en_de)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        ensure!(self.elements.len() == 1);
        visitor.visit_borrowed_str(self.elements[0].typename())
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        Err(DeserializationError::NotImplemented)
    }
}
/*impl<'e> serde::de::IntoDeserializer<'e, DeserializationError> for &'e Element<'e> {
    type Deserializer = InfosetDeserializer<'e>;

    fn into_deserializer(self) -> Self::Deserializer {
        InfosetDeserializer {
            elements: std::slice::from_ref(self),
            name: "",
        }
    }
}*/
impl<'de> IntoDeserializer<'de, DeserializationError> for InfosetDeserializer<'de> {
    type Deserializer = InfosetDeserializer<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

struct EnumDeserializer<'e> {
    element: &'e Element<'e>,
    variants: &'static [&'static str],

}

impl<'de> EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = DeserializationError;

    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de> {
        let Element::ComplexElement(el) = &self.element else {
            return Err(DeserializationError::TypeMismatch { infoset: "<simple>", expected: "<complex>" })
        };
        ensure!(!el.children.is_empty());
        let name = el.children[0].name();
        ensure!(self.variants.contains(&name));
        let var_de = VariantDeserializer {
            element: self.element,
            variant: name,
        };
        Ok((seed.deserialize(BorrowedStrDeserializer::new(name))?, var_de))
    }
}
#[derive(Debug)]
struct VariantDeserializer<'e> {
    element: &'e Element<'e>,
    variant: &'e str,
}
impl<'de> VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = DeserializationError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {
        let Element::ComplexElement(el) = &self.element else { unreachable!()};
        let deserializer = InfosetDeserializer {
            elements: &el.children,
            name: self.element.name(),
        };
        seed.deserialize(deserializer)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let deserializer = InfosetDeserializer {
            elements: std::slice::from_ref(self.element),
            name: self.variant,
        };
        deserializer.deserialize_seq(visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
        let Element::ComplexElement(el) = &self.element else { unreachable!()};
        let deserializer = InfosetDeserializer {
            elements: &el.children,
            name: self.variant,
        };
        println!("fields: {fields:?}");
        println!("data: {self:#?}");
        deserializer.deserialize_struct("", fields, visitor)
    }
}


