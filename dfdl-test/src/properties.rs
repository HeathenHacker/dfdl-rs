use roxmltree::ExpandedName;

use crate::{DFDL_ANNOTATION_SOURCE, DFDL_NAMESPACE};

#[derive(Debug, Default)]
pub struct Format {
    pub representation: Option<String>,
    pub binaryNumberRep: Option<String>,
    pub binaryFloatRep: Option<String>,
    pub byteOrder: Option<String>,
    pub lengthKind: Option<String>,
    pub lengthUnits: Option<String>,
    pub alignmentUnits: Option<String>,
    pub leadingSkip: Option<String>,
    pub trailingSkip: Option<String>,
    pub alignment: Option<String>,
    pub encoding: Option<String>,
    pub initiator: Option<String>,
    pub terminator: Option<String>,
    pub separator: Option<String>,
    pub ignoreCase: Option<String>,
    pub sequenceKind: Option<String>,
    pub textOutputMinLength: Option<String>,
    pub initiatedContent: Option<String>,
    pub truncateSpecifiedLengthString: Option<String>,
    pub textPadKind: Option<String>,
    pub fillByte: Option<String>,
    pub textBidi: Option<String>,
    pub floating: Option<String>,
    pub escapeSchemeRef: Option<String>,
    pub encodingErrorPolicy: Option<String>,
    pub bitOrder: Option<String>,
    pub occursCountKind: Option<String>,
}

impl Format {
    pub fn from_annotation(annotation: roxmltree::Node) -> Result<Self, ()> {

        let mut format = Format::default();
        for c in annotation.children() {
            // dfdl annotation in non-short form is inside an appinfo block
            if c.tag_name() != ExpandedName::from_static(crate::XSD_NAMESPACE, "appinfo") {continue;};
            if c.attribute("source") != Some(DFDL_ANNOTATION_SOURCE) {continue;}
            println!("annotation");
            for c in c.children() {
                if c.tag_name() == ExpandedName::from_static(DFDL_NAMESPACE, "format") {
                    for attr in c.attributes() {
                            format.merge_in_place(Format::from_property( attr.name(), attr.value())?);
                        
                    }
                }
            }
            
        }


        Ok(format)
    }
    fn from_property(name: &str, value: &str) -> Result<Self, ()> {
        let mut format = Self::default();
        match name {
            "representation" => format.representation = Some(value.to_string()),
            "binaryNumberRep" => format.binaryNumberRep = Some(value.to_string()),
            "binaryFloatRep" => format.binaryFloatRep = Some(value.to_string()),
            "byteOrder" => format.byteOrder = Some(value.to_string()),
            "lengthKind" => format.lengthKind = Some(value.to_string()),
            "lengthUnits" => format.lengthUnits = Some(value.to_string()),
            "alignmentUnits" => format.alignmentUnits = Some(value.to_string()),
            "leadingSkip" => format.leadingSkip = Some(value.to_string()),
            "trailingSkip" => format.trailingSkip = Some(value.to_string()),
            "alignment" => format.alignment = Some(value.to_string()),
            "encoding" => format.encoding = Some(value.to_string()),
            "initiator" => format.initiator = Some(value.to_string()),
            "terminator" => format.terminator = Some(value.to_string()),
            "separator" => format.separator = Some(value.to_string()),
            "ignoreCase" => format.ignoreCase = Some(value.to_string()),
            "sequenceKind" => format.sequenceKind = Some(value.to_string()),
            "textOutputMinLength" => format.textOutputMinLength = Some(value.to_string()),
            "initiatedContent" => format.initiatedContent = Some(value.to_string()),
            "truncateSpecifiedLengthString" => format.truncateSpecifiedLengthString = Some(value.to_string()),
            "textPadKind" => format.textPadKind = Some(value.to_string()),
            "fillByte" => format.fillByte = Some(value.to_string()),
            "textBidi" => format.textBidi = Some(value.to_string()),
            "floating" => format.floating = Some(value.to_string()),
            "escapeSchemeRef" => format.escapeSchemeRef = Some(value.to_string()),
            "encodingErrorPolicy" => format.encodingErrorPolicy = Some(value.to_string()),
            "bitOrder" => format.bitOrder = Some(value.to_string()),
            "occursCountKind" => format.occursCountKind = Some(value.to_string()),
            _ => return Err(()),

        }
        Ok(format)

    }
    fn merge_in_place(&mut self, other: Self) {
        self.representation = other.representation.or(self.representation.take());
        self.binaryNumberRep = other.binaryNumberRep.or(self.binaryNumberRep.take());
        self.binaryFloatRep = other.binaryFloatRep.or(self.binaryFloatRep.take());
        self.byteOrder = other.byteOrder.or(self.byteOrder.take());
        self.lengthKind = other.lengthKind.or(self.lengthKind.take());
        self.lengthUnits = other.lengthUnits.or(self.lengthUnits.take());
        self.alignmentUnits = other.alignmentUnits.or(self.alignmentUnits.take());
        self.leadingSkip = other.leadingSkip.or(self.leadingSkip.take());
        self.trailingSkip = other.trailingSkip.or(self.trailingSkip.take());
        self.alignment = other.alignment.or(self.alignment.take());
        self.encoding = other.encoding.or(self.encoding.take());
        self.initiator = other.initiator.or(self.initiator.take());
        self.terminator = other.terminator.or(self.terminator.take());
        self.separator = other.separator.or(self.separator.take());
        self.ignoreCase = other.ignoreCase.or(self.ignoreCase.take());
        self.sequenceKind = other.sequenceKind.or(self.sequenceKind.take());
        self.textOutputMinLength = other.textOutputMinLength.or(self.textOutputMinLength.take());
        self.initiatedContent = other.initiatedContent.or(self.initiatedContent.take());
        self.truncateSpecifiedLengthString = other.truncateSpecifiedLengthString.or(self.truncateSpecifiedLengthString.take());
        self.textPadKind = other.textPadKind.or(self.textPadKind.take());
        self.fillByte = other.fillByte.or(self.fillByte.take());
        self.textBidi = other.textBidi.or(self.textBidi.take());
        self.floating = other.floating.or(self.floating.take());
        self.escapeSchemeRef = other.escapeSchemeRef.or(self.escapeSchemeRef.take());
        self.encodingErrorPolicy = other.encodingErrorPolicy.or(self.encodingErrorPolicy.take());
        self.bitOrder = other.bitOrder.or(self.bitOrder.take());
        self.occursCountKind = other.occursCountKind.or(self.occursCountKind.take());
        
    }
    fn merge(mut self, other: Self) -> Self {
        self.merge_in_place(other);
        self
    }
}

#[derive(Debug, Default)]
pub struct Assert {
    pub test_kind: AssertTestKind,
    pub test: Option<DfdlExpression>,
    pub test_pattern: Option<String>,
    pub message: String,
    pub failure_type: AssertFailureType,
}




#[derive(Debug, Default)]
pub enum AssertTestKind {
    #[default]
    Expression,
    Pattern,
}
#[derive(Debug, Default)]
pub enum AssertFailureType {
    #[default]
    ProcessingError,
    RecoverableError,
}







#[derive(Debug, Default)]
pub struct DfdlExpression(String);
#[derive(Debug, Default)]
pub struct DfdlRegex(String);
