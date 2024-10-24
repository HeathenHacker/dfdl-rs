use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("schema error: ")]
    SchemaError(#[from] SchemaError),
}

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("xml error: ")]
    XmlError(#[from] roxmltree::Error),
    #[error("xml document is not a schema document")]
    NotASchema,
}

impl From<roxmltree::Error> for Error 
{
    fn from(value: roxmltree::Error) -> Self {
        Self::SchemaError(value.into())
    }
}

