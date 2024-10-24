
use serde::{Deserialize, Serialize};

use crate::{Element, Infoset};

mod serialize;
pub use serialize::SerializationError;

mod deserialize;
pub use deserialize::DeserializationError;



/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn to_infoset<S: Serialize>(item: &S) -> Result<Infoset<'static>, SerializationError> {
    let base_name = "root";
    let base_namespace = "";
    let elem = serialize::ElementSerializer {
        name: std::borrow::Cow::Borrowed(base_name),
        namespace: std::borrow::Cow::Borrowed(base_namespace),
        array: false,
    };
    let (elem, _) = item.serialize(elem)?;
    Ok(Infoset { dfdl_version: "1.0".to_string(), root_element: Some(elem) })
}


/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn from_infoset<'de, 'is, D: Deserialize<'de>>(infoset: &'is Infoset<'is>) -> Result<D, DeserializationError> 
    where 'is: 'de
{
    let elements: &[Element] = infoset.root_element.as_ref().map_or(&[], std::slice::from_ref);
    let deserializer = deserialize::InfosetDeserializer {
        elements,
        //name: Cow::Borrowed("root"),
        name: "root",
    };
    D::deserialize(deserializer)
}

