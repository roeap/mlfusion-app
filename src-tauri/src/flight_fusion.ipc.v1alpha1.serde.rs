impl serde::Serialize for ActionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACTION_STATUS_UNSPECIFIED",
            Self::Success => "ACTION_STATUS_SUCCESS",
            Self::Failure => "ACTION_STATUS_FAILURE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ActionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_STATUS_UNSPECIFIED",
            "ACTION_STATUS_SUCCESS",
            "ACTION_STATUS_FAILURE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ActionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ActionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACTION_STATUS_UNSPECIFIED" => Ok(ActionStatus::Unspecified),
                    "ACTION_STATUS_SUCCESS" => Ok(ActionStatus::Success),
                    "ACTION_STATUS_FAILURE" => Ok(ActionStatus::Failure),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AreaSourceDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceDetails", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaSourceDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaSourceDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaSourceDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaSourceDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AreaSourceDetails {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaSourceMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_versioned {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.properties.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceMetadata", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_versioned {
            struct_ser.serialize_field("isVersioned", &self.is_versioned)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.properties.is_empty() {
            struct_ser.serialize_field("properties", &self.properties)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaSourceMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "isVersioned",
            "source",
            "tags",
            "properties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            IsVersioned,
            Source,
            Tags,
            Properties,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "isVersioned" => Ok(GeneratedField::IsVersioned),
                            "source" => Ok(GeneratedField::Source),
                            "tags" => Ok(GeneratedField::Tags),
                            "properties" => Ok(GeneratedField::Properties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaSourceMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaSourceMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaSourceMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_versioned__ = None;
                let mut source__ = None;
                let mut tags__ = None;
                let mut properties__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsVersioned => {
                            if is_versioned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVersioned"));
                            }
                            is_versioned__ = Some(map.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(AreaSourceMetadata {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    is_versioned: is_versioned__.unwrap_or_default(),
                    source: source__,
                    tags: tags__.unwrap_or_default(),
                    properties: properties__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaSourceReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceReference", len)?;
        if let Some(v) = self.table.as_ref() {
            match v {
                area_source_reference::Table::Location(v) => {
                    struct_ser.serialize_field("location", v)?;
                }
                area_source_reference::Table::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                area_source_reference::Table::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaSourceReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
            "id",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
            Id,
            Uri,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "location" => Ok(GeneratedField::Location),
                            "id" => Ok(GeneratedField::Id),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaSourceReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaSourceReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaSourceReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            table__ = Some(area_source_reference::Table::Location(map.next_value()?));
                        }
                        GeneratedField::Id => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            table__ = Some(area_source_reference::Table::Id(map.next_value()?));
                        }
                        GeneratedField::Uri => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            table__ = Some(area_source_reference::Table::Uri(map.next_value()?));
                        }
                    }
                }
                Ok(AreaSourceReference {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaSourceReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaTableId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaTableId", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaTableId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaTableId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaTableId")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaTableId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AreaTableId {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaTableId", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaTableLocation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.areas.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaTableLocation", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.areas.is_empty() {
            struct_ser.serialize_field("areas", &self.areas)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaTableLocation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "areas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Areas,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "areas" => Ok(GeneratedField::Areas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaTableLocation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaTableLocation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaTableLocation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut areas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Areas => {
                            if areas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("areas"));
                            }
                            areas__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AreaTableLocation {
                    name: name__.unwrap_or_default(),
                    areas: areas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaTableLocation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AreaTableUri {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.AreaTableUri", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AreaTableUri {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AreaTableUri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.AreaTableUri")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AreaTableUri, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AreaTableUri {
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.AreaTableUri", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchStatistics {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.record_count != 0 {
            len += 1;
        }
        if self.total_byte_size != 0 {
            len += 1;
        }
        if !self.column_statistics.is_empty() {
            len += 1;
        }
        if self.is_exact {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.BatchStatistics", len)?;
        if self.record_count != 0 {
            struct_ser.serialize_field("recordCount", ToString::to_string(&self.record_count).as_str())?;
        }
        if self.total_byte_size != 0 {
            struct_ser.serialize_field("totalByteSize", ToString::to_string(&self.total_byte_size).as_str())?;
        }
        if !self.column_statistics.is_empty() {
            struct_ser.serialize_field("columnStatistics", &self.column_statistics)?;
        }
        if self.is_exact {
            struct_ser.serialize_field("isExact", &self.is_exact)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchStatistics {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recordCount",
            "totalByteSize",
            "columnStatistics",
            "isExact",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RecordCount,
            TotalByteSize,
            ColumnStatistics,
            IsExact,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "recordCount" => Ok(GeneratedField::RecordCount),
                            "totalByteSize" => Ok(GeneratedField::TotalByteSize),
                            "columnStatistics" => Ok(GeneratedField::ColumnStatistics),
                            "isExact" => Ok(GeneratedField::IsExact),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchStatistics;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.BatchStatistics")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchStatistics, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut record_count__ = None;
                let mut total_byte_size__ = None;
                let mut column_statistics__ = None;
                let mut is_exact__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RecordCount => {
                            if record_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordCount"));
                            }
                            record_count__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::TotalByteSize => {
                            if total_byte_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalByteSize"));
                            }
                            total_byte_size__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::ColumnStatistics => {
                            if column_statistics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnStatistics"));
                            }
                            column_statistics__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsExact => {
                            if is_exact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExact"));
                            }
                            is_exact__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchStatistics {
                    record_count: record_count__.unwrap_or_default(),
                    total_byte_size: total_byte_size__.unwrap_or_default(),
                    column_statistics: column_statistics__.unwrap_or_default(),
                    is_exact: is_exact__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.BatchStatistics", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnStatistics {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.null_count != 0 {
            len += 1;
        }
        if !self.max_value.is_empty() {
            len += 1;
        }
        if !self.min_value.is_empty() {
            len += 1;
        }
        if self.distinct_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.ColumnStatistics", len)?;
        if self.null_count != 0 {
            struct_ser.serialize_field("nullCount", ToString::to_string(&self.null_count).as_str())?;
        }
        if !self.max_value.is_empty() {
            struct_ser.serialize_field("maxValue", &self.max_value)?;
        }
        if !self.min_value.is_empty() {
            struct_ser.serialize_field("minValue", &self.min_value)?;
        }
        if self.distinct_count != 0 {
            struct_ser.serialize_field("distinctCount", ToString::to_string(&self.distinct_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnStatistics {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nullCount",
            "maxValue",
            "minValue",
            "distinctCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NullCount,
            MaxValue,
            MinValue,
            DistinctCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nullCount" => Ok(GeneratedField::NullCount),
                            "maxValue" => Ok(GeneratedField::MaxValue),
                            "minValue" => Ok(GeneratedField::MinValue),
                            "distinctCount" => Ok(GeneratedField::DistinctCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnStatistics;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.ColumnStatistics")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ColumnStatistics, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut null_count__ = None;
                let mut max_value__ = None;
                let mut min_value__ = None;
                let mut distinct_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NullCount => {
                            if null_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullCount"));
                            }
                            null_count__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::MaxValue => {
                            if max_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValue"));
                            }
                            max_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinValue => {
                            if min_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minValue"));
                            }
                            min_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::DistinctCount => {
                            if distinct_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("distinctCount"));
                            }
                            distinct_count__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(ColumnStatistics {
                    null_count: null_count__.unwrap_or_default(),
                    max_value: max_value__.unwrap_or_default(),
                    min_value: min_value__.unwrap_or_default(),
                    distinct_count: distinct_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.ColumnStatistics", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandDropSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandDropSource", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandDropSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandDropSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandDropSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandDropSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandDropSource {
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandDropSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandExecuteQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandExecuteQuery", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.context.as_ref() {
            match v {
                command_execute_query::Context::Source(v) => {
                    struct_ser.serialize_field("source", v)?;
                }
                command_execute_query::Context::Frame(v) => {
                    struct_ser.serialize_field("frame", v)?;
                }
                command_execute_query::Context::Collection(v) => {
                    struct_ser.serialize_field("collection", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandExecuteQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "source",
            "frame",
            "collection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            Source,
            Frame,
            Collection,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "query" => Ok(GeneratedField::Query),
                            "source" => Ok(GeneratedField::Source),
                            "frame" => Ok(GeneratedField::Frame),
                            "collection" => Ok(GeneratedField::Collection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandExecuteQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandExecuteQuery")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandExecuteQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map.next_value()?);
                        }
                        GeneratedField::Source => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            context__ = Some(command_execute_query::Context::Source(map.next_value()?));
                        }
                        GeneratedField::Frame => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frame"));
                            }
                            context__ = Some(command_execute_query::Context::Frame(map.next_value()?));
                        }
                        GeneratedField::Collection => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collection"));
                            }
                            context__ = Some(command_execute_query::Context::Collection(map.next_value()?));
                        }
                    }
                }
                Ok(CommandExecuteQuery {
                    query: query__.unwrap_or_default(),
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandExecuteQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandKqlOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandKqlOperation", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandKqlOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "serviceName",
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Query,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serviceName" => Ok(GeneratedField::ServiceName),
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandKqlOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandKqlOperation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandKqlOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut query__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandKqlOperation {
                    service_name: service_name__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandKqlOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandListSources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.recursive {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandListSources", len)?;
        if self.recursive {
            struct_ser.serialize_field("recursive", &self.recursive)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandListSources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recursive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recursive,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "recursive" => Ok(GeneratedField::Recursive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandListSources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandListSources")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandListSources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recursive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Recursive => {
                            if recursive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recursive"));
                            }
                            recursive__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandListSources {
                    recursive: recursive__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandListSources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandReadDataset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.column_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandReadDataset", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.column_names.is_empty() {
            struct_ser.serialize_field("columnNames", &self.column_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandReadDataset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "columnNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            ColumnNames,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "columnNames" => Ok(GeneratedField::ColumnNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandReadDataset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandReadDataset")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandReadDataset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut column_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::ColumnNames => {
                            if column_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnNames"));
                            }
                            column_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandReadDataset {
                    source: source__,
                    column_names: column_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandReadDataset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandSetMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.meta.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandSetMetadata", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.meta.as_ref() {
            struct_ser.serialize_field("meta", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandSetMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "meta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Meta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "meta" => Ok(GeneratedField::Meta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandSetMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandSetMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandSetMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut meta__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::Meta => {
                            if meta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            meta__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandSetMetadata {
                    source: source__,
                    meta: meta__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandSetMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandWriteIntoDataset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.save_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.CommandWriteIntoDataset", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if self.save_mode != 0 {
            let v = SaveMode::from_i32(self.save_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.save_mode)))?;
            struct_ser.serialize_field("saveMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandWriteIntoDataset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "saveMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            SaveMode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "saveMode" => Ok(GeneratedField::SaveMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandWriteIntoDataset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.CommandWriteIntoDataset")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandWriteIntoDataset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut save_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::SaveMode => {
                            if save_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveMode"));
                            }
                            save_mode__ = Some(map.next_value::<SaveMode>()? as i32);
                        }
                    }
                }
                Ok(CommandWriteIntoDataset {
                    source: source__,
                    save_mode: save_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.CommandWriteIntoDataset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaCreateOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.save_mode != 0 {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.DeltaCreateOperation", len)?;
        if self.save_mode != 0 {
            let v = SaveMode::from_i32(self.save_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.save_mode)))?;
            struct_ser.serialize_field("saveMode", &v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaCreateOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saveMode",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SaveMode,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "saveMode" => Ok(GeneratedField::SaveMode),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaCreateOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.DeltaCreateOperation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaCreateOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut save_mode__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SaveMode => {
                            if save_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveMode"));
                            }
                            save_mode__ = Some(map.next_value::<SaveMode>()? as i32);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeltaCreateOperation {
                    save_mode: save_mode__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.DeltaCreateOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaOperationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.DeltaOperationRequest", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.operation.as_ref() {
            match v {
                delta_operation_request::Operation::Create(v) => {
                    struct_ser.serialize_field("create", v)?;
                }
                delta_operation_request::Operation::Write(v) => {
                    struct_ser.serialize_field("write", v)?;
                }
                delta_operation_request::Operation::Read(v) => {
                    struct_ser.serialize_field("read", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaOperationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "create",
            "write",
            "read",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Create,
            Write,
            Read,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "create" => Ok(GeneratedField::Create),
                            "write" => Ok(GeneratedField::Write),
                            "read" => Ok(GeneratedField::Read),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaOperationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.DeltaOperationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaOperationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut operation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::Create => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("create"));
                            }
                            operation__ = Some(delta_operation_request::Operation::Create(map.next_value()?));
                        }
                        GeneratedField::Write => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("write"));
                            }
                            operation__ = Some(delta_operation_request::Operation::Write(map.next_value()?));
                        }
                        GeneratedField::Read => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            operation__ = Some(delta_operation_request::Operation::Read(map.next_value()?));
                        }
                    }
                }
                Ok(DeltaOperationRequest {
                    source: source__,
                    operation: operation__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.DeltaOperationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaOperationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.DeltaOperationResponse", len)?;
        if !self.stats.is_empty() {
            struct_ser.serialize_field("stats", &self.stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaOperationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stats,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stats" => Ok(GeneratedField::Stats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaOperationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.DeltaOperationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaOperationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeltaOperationResponse {
                    stats: stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.DeltaOperationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaReadOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.timestamp.is_empty() {
            len += 1;
        }
        if !self.predicate.is_empty() {
            len += 1;
        }
        if !self.column_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.DeltaReadOperation", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", ToString::to_string(&self.version).as_str())?;
        }
        if !self.timestamp.is_empty() {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        if !self.predicate.is_empty() {
            struct_ser.serialize_field("predicate", &self.predicate)?;
        }
        if !self.column_names.is_empty() {
            struct_ser.serialize_field("columnNames", &self.column_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaReadOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "timestamp",
            "predicate",
            "columnNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Timestamp,
            Predicate,
            ColumnNames,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "predicate" => Ok(GeneratedField::Predicate),
                            "columnNames" => Ok(GeneratedField::ColumnNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaReadOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.DeltaReadOperation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaReadOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut timestamp__ = None;
                let mut predicate__ = None;
                let mut column_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate__ = Some(map.next_value()?);
                        }
                        GeneratedField::ColumnNames => {
                            if column_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnNames"));
                            }
                            column_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeltaReadOperation {
                    version: version__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    predicate: predicate__.unwrap_or_default(),
                    column_names: column_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.DeltaReadOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaWriteOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.save_mode != 0 {
            len += 1;
        }
        if !self.partition_by.is_empty() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.DeltaWriteOperation", len)?;
        if self.save_mode != 0 {
            let v = SaveMode::from_i32(self.save_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.save_mode)))?;
            struct_ser.serialize_field("saveMode", &v)?;
        }
        if !self.partition_by.is_empty() {
            struct_ser.serialize_field("partitionBy", &self.partition_by)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaWriteOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saveMode",
            "partitionBy",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SaveMode,
            PartitionBy,
            Predicate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "saveMode" => Ok(GeneratedField::SaveMode),
                            "partitionBy" => Ok(GeneratedField::PartitionBy),
                            "predicate" => Ok(GeneratedField::Predicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaWriteOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.DeltaWriteOperation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaWriteOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut save_mode__ = None;
                let mut partition_by__ = None;
                let mut predicate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SaveMode => {
                            if save_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveMode"));
                            }
                            save_mode__ = Some(map.next_value::<SaveMode>()? as i32);
                        }
                        GeneratedField::PartitionBy => {
                            if partition_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionBy"));
                            }
                            partition_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeltaWriteOperation {
                    save_mode: save_mode__.unwrap_or_default(),
                    partition_by: partition_by__.unwrap_or_default(),
                    predicate: predicate__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.DeltaWriteOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityReferenceTrait {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.level.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.EntityReferenceTrait", len)?;
        if !self.level.is_empty() {
            struct_ser.serialize_field("level", &self.level)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityReferenceTrait {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "level" => Ok(GeneratedField::Level),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityReferenceTrait;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.EntityReferenceTrait")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EntityReferenceTrait, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EntityReferenceTrait {
                    level: level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.EntityReferenceTrait", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.ExpressionReference", len)?;
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "expression",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            Expression,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uid" => Ok(GeneratedField::Uid),
                            "expression" => Ok(GeneratedField::Expression),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.ExpressionReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExpressionReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut expression__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExpressionReference {
                    uid: uid__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.ExpressionReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FILE_FORMAT_UNSPECIFIED",
            Self::Parquet => "FILE_FORMAT_PARQUET",
            Self::Avro => "FILE_FORMAT_AVRO",
            Self::Csv => "FILE_FORMAT_CSV",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FileFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FILE_FORMAT_UNSPECIFIED",
            "FILE_FORMAT_PARQUET",
            "FILE_FORMAT_AVRO",
            "FILE_FORMAT_CSV",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(FileFormat::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(FileFormat::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FILE_FORMAT_UNSPECIFIED" => Ok(FileFormat::Unspecified),
                    "FILE_FORMAT_PARQUET" => Ok(FileFormat::Parquet),
                    "FILE_FORMAT_AVRO" => Ok(FileFormat::Avro),
                    "FILE_FORMAT_CSV" => Ok(FileFormat::Csv),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FileReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FileReference", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if self.format != 0 {
            let v = FileFormat::from_i32(self.format)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Format,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FileReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map.next_value::<FileFormat>()? as i32);
                        }
                    }
                }
                Ok(FileReference {
                    path: path__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FileReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlightActionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FlightActionRequest", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                flight_action_request::Action::Drop(v) => {
                    struct_ser.serialize_field("drop", v)?;
                }
                flight_action_request::Action::SetMeta(v) => {
                    struct_ser.serialize_field("setMeta", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlightActionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drop",
            "setMeta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Drop,
            SetMeta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "drop" => Ok(GeneratedField::Drop),
                            "setMeta" => Ok(GeneratedField::SetMeta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlightActionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FlightActionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlightActionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Drop => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drop"));
                            }
                            action__ = Some(flight_action_request::Action::Drop(map.next_value()?));
                        }
                        GeneratedField::SetMeta => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setMeta"));
                            }
                            action__ = Some(flight_action_request::Action::SetMeta(map.next_value()?));
                        }
                    }
                }
                Ok(FlightActionRequest {
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FlightActionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlightActionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FlightActionResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                flight_action_response::Payload::Status(v) => {
                    struct_ser.serialize_field("status", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlightActionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlightActionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FlightActionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlightActionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            payload__ = Some(flight_action_response::Payload::Status(map.next_value()?));
                        }
                    }
                }
                Ok(FlightActionResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FlightActionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlightDoGetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.command.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FlightDoGetRequest", len)?;
        if let Some(v) = self.command.as_ref() {
            match v {
                flight_do_get_request::Command::Kql(v) => {
                    struct_ser.serialize_field("kql", v)?;
                }
                flight_do_get_request::Command::Read(v) => {
                    struct_ser.serialize_field("read", v)?;
                }
                flight_do_get_request::Command::Query(v) => {
                    struct_ser.serialize_field("query", v)?;
                }
                flight_do_get_request::Command::Delta(v) => {
                    struct_ser.serialize_field("delta", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlightDoGetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kql",
            "read",
            "query",
            "delta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kql,
            Read,
            Query,
            Delta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kql" => Ok(GeneratedField::Kql),
                            "read" => Ok(GeneratedField::Read),
                            "query" => Ok(GeneratedField::Query),
                            "delta" => Ok(GeneratedField::Delta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlightDoGetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FlightDoGetRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlightDoGetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut command__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kql => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kql"));
                            }
                            command__ = Some(flight_do_get_request::Command::Kql(map.next_value()?));
                        }
                        GeneratedField::Read => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read"));
                            }
                            command__ = Some(flight_do_get_request::Command::Read(map.next_value()?));
                        }
                        GeneratedField::Query => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            command__ = Some(flight_do_get_request::Command::Query(map.next_value()?));
                        }
                        GeneratedField::Delta => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delta"));
                            }
                            command__ = Some(flight_do_get_request::Command::Delta(map.next_value()?));
                        }
                    }
                }
                Ok(FlightDoGetRequest {
                    command: command__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FlightDoGetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlightDoPutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.command.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FlightDoPutRequest", len)?;
        if let Some(v) = self.command.as_ref() {
            match v {
                flight_do_put_request::Command::Storage(v) => {
                    struct_ser.serialize_field("storage", v)?;
                }
                flight_do_put_request::Command::Delta(v) => {
                    struct_ser.serialize_field("delta", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlightDoPutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "storage",
            "delta",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Storage,
            Delta,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "storage" => Ok(GeneratedField::Storage),
                            "delta" => Ok(GeneratedField::Delta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlightDoPutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FlightDoPutRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlightDoPutRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut command__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Storage => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            command__ = Some(flight_do_put_request::Command::Storage(map.next_value()?));
                        }
                        GeneratedField::Delta => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delta"));
                            }
                            command__ = Some(flight_do_put_request::Command::Delta(map.next_value()?));
                        }
                    }
                }
                Ok(FlightDoPutRequest {
                    command: command__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FlightDoPutRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlightDoPutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.FlightDoPutResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                flight_do_put_response::Payload::Update(v) => {
                    struct_ser.serialize_field("update", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlightDoPutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "update",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Update,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "update" => Ok(GeneratedField::Update),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlightDoPutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.FlightDoPutResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FlightDoPutResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Update => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            payload__ = Some(flight_do_put_response::Payload::Update(map.next_value()?));
                        }
                    }
                }
                Ok(FlightDoPutResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.FlightDoPutResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.ModelReference", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.ModelReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelReference {
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.ModelReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResultActionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.ResultActionStatus", len)?;
        if self.status != 0 {
            let v = ActionStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResultActionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResultActionStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.ResultActionStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResultActionStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<ActionStatus>()? as i32);
                        }
                    }
                }
                Ok(ResultActionStatus {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.ResultActionStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResultDoPutUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.statistics.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.ResultDoPutUpdate", len)?;
        if let Some(v) = self.statistics.as_ref() {
            struct_ser.serialize_field("statistics", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResultDoPutUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "statistics",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Statistics,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "statistics" => Ok(GeneratedField::Statistics),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResultDoPutUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.ResultDoPutUpdate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResultDoPutUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut statistics__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Statistics => {
                            if statistics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statistics"));
                            }
                            statistics__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResultDoPutUpdate {
                    statistics: statistics__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.ResultDoPutUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SaveMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SAVE_MODE_UNSPECIFIED",
            Self::Append => "SAVE_MODE_APPEND",
            Self::Overwrite => "SAVE_MODE_OVERWRITE",
            Self::ErrorIfExists => "SAVE_MODE_ERROR_IF_EXISTS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SaveMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SAVE_MODE_UNSPECIFIED",
            "SAVE_MODE_APPEND",
            "SAVE_MODE_OVERWRITE",
            "SAVE_MODE_ERROR_IF_EXISTS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SaveMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SaveMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SaveMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SAVE_MODE_UNSPECIFIED" => Ok(SaveMode::Unspecified),
                    "SAVE_MODE_APPEND" => Ok(SaveMode::Append),
                    "SAVE_MODE_OVERWRITE" => Ok(SaveMode::Overwrite),
                    "SAVE_MODE_ERROR_IF_EXISTS" => Ok(SaveMode::ErrorIfExists),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SensitiveDataTrait {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.level.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.SensitiveDataTrait", len)?;
        if !self.level.is_empty() {
            struct_ser.serialize_field("level", &self.level)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SensitiveDataTrait {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "level" => Ok(GeneratedField::Level),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SensitiveDataTrait;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.SensitiveDataTrait")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SensitiveDataTrait, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SensitiveDataTrait {
                    level: level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.SensitiveDataTrait", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Signal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.traits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.Signal", len)?;
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.traits.is_empty() {
            struct_ser.serialize_field("traits", &self.traits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Signal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "name",
            "description",
            "traits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            Name,
            Description,
            Traits,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uid" => Ok(GeneratedField::Uid),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "traits" => Ok(GeneratedField::Traits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Signal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.Signal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Signal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut traits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Traits => {
                            if traits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traits"));
                            }
                            traits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Signal {
                    uid: uid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    traits: traits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.Signal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignalFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.providers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.SignalFrame", len)?;
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.providers.is_empty() {
            struct_ser.serialize_field("providers", &self.providers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignalFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "name",
            "description",
            "providers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            Name,
            Description,
            Providers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uid" => Ok(GeneratedField::Uid),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "providers" => Ok(GeneratedField::Providers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignalFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.SignalFrame")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignalFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut providers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Providers => {
                            if providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providers"));
                            }
                            providers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SignalFrame {
                    uid: uid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    providers: providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.SignalFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignalProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.signals.is_empty() {
            len += 1;
        }
        if !self.inputs.is_empty() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.SignalProvider", len)?;
        if !self.uid.is_empty() {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.signals.is_empty() {
            struct_ser.serialize_field("signals", &self.signals)?;
        }
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("inputs", &self.inputs)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                signal_provider::Source::Table(v) => {
                    struct_ser.serialize_field("table", v)?;
                }
                signal_provider::Source::Expression(v) => {
                    struct_ser.serialize_field("expression", v)?;
                }
                signal_provider::Source::Model(v) => {
                    struct_ser.serialize_field("model", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignalProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uid",
            "name",
            "description",
            "signals",
            "inputs",
            "table",
            "expression",
            "model",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uid,
            Name,
            Description,
            Signals,
            Inputs,
            Table,
            Expression,
            Model,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uid" => Ok(GeneratedField::Uid),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "signals" => Ok(GeneratedField::Signals),
                            "inputs" => Ok(GeneratedField::Inputs),
                            "table" => Ok(GeneratedField::Table),
                            "expression" => Ok(GeneratedField::Expression),
                            "model" => Ok(GeneratedField::Model),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignalProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.SignalProvider")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignalProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut signals__ = None;
                let mut inputs__ = None;
                let mut source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signals => {
                            if signals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signals"));
                            }
                            signals__ = Some(map.next_value()?);
                        }
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            inputs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Table => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            source__ = Some(signal_provider::Source::Table(map.next_value()?));
                        }
                        GeneratedField::Expression => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            source__ = Some(signal_provider::Source::Expression(map.next_value()?));
                        }
                        GeneratedField::Model => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            source__ = Some(signal_provider::Source::Model(map.next_value()?));
                        }
                    }
                }
                Ok(SignalProvider {
                    uid: uid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    signals: signals__.unwrap_or_default(),
                    inputs: inputs__.unwrap_or_default(),
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.SignalProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignalTrait {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#trait.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.SignalTrait", len)?;
        if let Some(v) = self.r#trait.as_ref() {
            match v {
                signal_trait::Trait::Sensitive(v) => {
                    struct_ser.serialize_field("sensitive", v)?;
                }
                signal_trait::Trait::TimeSeries(v) => {
                    struct_ser.serialize_field("timeSeries", v)?;
                }
                signal_trait::Trait::EntityReference(v) => {
                    struct_ser.serialize_field("entityReference", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignalTrait {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sensitive",
            "timeSeries",
            "entityReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sensitive,
            TimeSeries,
            EntityReference,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sensitive" => Ok(GeneratedField::Sensitive),
                            "timeSeries" => Ok(GeneratedField::TimeSeries),
                            "entityReference" => Ok(GeneratedField::EntityReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignalTrait;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.SignalTrait")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignalTrait, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#trait__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sensitive => {
                            if r#trait__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensitive"));
                            }
                            r#trait__ = Some(signal_trait::Trait::Sensitive(map.next_value()?));
                        }
                        GeneratedField::TimeSeries => {
                            if r#trait__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeSeries"));
                            }
                            r#trait__ = Some(signal_trait::Trait::TimeSeries(map.next_value()?));
                        }
                        GeneratedField::EntityReference => {
                            if r#trait__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityReference"));
                            }
                            r#trait__ = Some(signal_trait::Trait::EntityReference(map.next_value()?));
                        }
                    }
                }
                Ok(SignalTrait {
                    r#trait: r#trait__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.SignalTrait", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignalType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGNAL_TYPE_UNSPECIFIED",
            Self::Observation => "SIGNAL_TYPE_OBSERVATION",
            Self::Constant => "SIGNAL_TYPE_CONSTANT",
            Self::Expression => "SIGNAL_TYPE_EXPRESSION",
            Self::Model => "SIGNAL_TYPE_MODEL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SignalType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNAL_TYPE_UNSPECIFIED",
            "SIGNAL_TYPE_OBSERVATION",
            "SIGNAL_TYPE_CONSTANT",
            "SIGNAL_TYPE_EXPRESSION",
            "SIGNAL_TYPE_MODEL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignalType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SignalType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SignalType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGNAL_TYPE_UNSPECIFIED" => Ok(SignalType::Unspecified),
                    "SIGNAL_TYPE_OBSERVATION" => Ok(SignalType::Observation),
                    "SIGNAL_TYPE_CONSTANT" => Ok(SignalType::Constant),
                    "SIGNAL_TYPE_EXPRESSION" => Ok(SignalType::Expression),
                    "SIGNAL_TYPE_MODEL" => Ok(SignalType::Model),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SourceCollection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.SourceCollection", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceCollection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sources" => Ok(GeneratedField::Sources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.SourceCollection")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SourceCollection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sources__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SourceCollection {
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.SourceCollection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.TableReference", len)?;
        if let Some(v) = self.table.as_ref() {
            match v {
                table_reference::Table::File(v) => {
                    struct_ser.serialize_field("file", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "file" => Ok(GeneratedField::File),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.TableReference")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TableReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            table__ = Some(table_reference::Table::File(map.next_value()?));
                        }
                    }
                }
                Ok(TableReference {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.TableReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.Tag", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.Tag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Tag {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.Tag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeSeriesTrait {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.level.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flight_fusion.ipc.v1alpha1.TimeSeriesTrait", len)?;
        if !self.level.is_empty() {
            struct_ser.serialize_field("level", &self.level)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeSeriesTrait {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Level,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "level" => Ok(GeneratedField::Level),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeSeriesTrait;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flight_fusion.ipc.v1alpha1.TimeSeriesTrait")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TimeSeriesTrait, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut level__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TimeSeriesTrait {
                    level: level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flight_fusion.ipc.v1alpha1.TimeSeriesTrait", FIELDS, GeneratedVisitor)
    }
}
