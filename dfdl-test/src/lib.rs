use roxmltree::ExpandedName;

use self::error::Error;
use self::properties::Format;


pub mod error;
mod properties;

pub const DFDL_NAMESPACE: &str = r"http://www.ogf.org/dfdl/dfdl-1.0/";
pub const DFDL_ANNOTATION_SOURCE: &str = r"http://www.ogf.org/dfdl/";
pub const XSD_NAMESPACE: &str = r"http://www.w3.org/2001/XMLSchema";

pub struct Schema {
}


impl Schema {
    pub fn new(xml: &str) -> Result<Self, Error> {
        let tree = roxmltree::Document::parse(xml)?;
        
        
        println!("{tree:#?}");
        let root = tree.root_element();
        println!("{root:#?}");
        if root.tag_name() != ExpandedName::from_static(XSD_NAMESPACE, "schema") { Err(error::SchemaError::NotASchema)?; };


        for c in root.children() {
            let tag = c.tag_name();
            if tag == ExpandedName::from_static(XSD_NAMESPACE, "annotation") {
                
                let format = Format::from_annotation(c);
                println!("format: {format:#?}");
            }
            println!("child: {c:?}");
        }
        


        Ok(Self {
        })
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
