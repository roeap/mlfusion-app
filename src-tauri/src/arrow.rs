use crate::models::{DataType, Signal};
use arrow::datatypes::{DataType as ArrowDataType, Field};

impl From<ArrowDataType> for DataType {
    fn from(datatype: ArrowDataType) -> Self {
        DataType::from(&datatype)
    }
}

impl From<&ArrowDataType> for DataType {
    fn from(datatype: &ArrowDataType) -> Self {
        match datatype {
            ArrowDataType::Boolean => DataType::Unspecified,
            ArrowDataType::Int16 => DataType::Int16,
            ArrowDataType::Int32 => DataType::Int32,
            ArrowDataType::Int64 => DataType::Int64,
            ArrowDataType::UInt16 => DataType::Uint16,
            ArrowDataType::UInt32 => DataType::Uint32,
            ArrowDataType::UInt64 => DataType::Uint64,
            ArrowDataType::Float32 => DataType::Float32,
            ArrowDataType::Float64 => DataType::Float64,
            ArrowDataType::Utf8 | ArrowDataType::LargeUtf8 => DataType::String,
            _ => DataType::Unspecified,
        }
    }
}

impl From<Field> for Signal {
    fn from(field: Field) -> Self {
        let data_type: DataType = field.data_type().into();
        Self {
            name: field.name().to_owned(),
            data_type: data_type.into(),
            description: "from arrow".to_string(),
            nullable: field.is_nullable(),
            ..Self::default()
        }
    }
}

impl From<&Field> for Signal {
    fn from(field: &Field) -> Self {
        let data_type: DataType = field.data_type().into();
        Self {
            name: field.name().to_owned(),
            data_type: data_type.into(),
            description: "from arrow".to_string(),
            nullable: field.is_nullable(),
            ..Self::default()
        }
    }
}
