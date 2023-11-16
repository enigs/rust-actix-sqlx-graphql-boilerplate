use library::prelude::CustomStatus;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[derive(async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    Inactive,
    #[default]
    None
}

impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let variant_str = match self {
            Status::Active => "ACTIVE",
            Status::Inactive => "INACTIVE",
            Status::None => "NONE",
        };

        serializer.serialize_str(variant_str)
    }
}

impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;

        match variant.to_lowercase().as_str() {
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            "none" => Ok(Status::None),
            _ => Err(serde::de::Error::unknown_variant(
                &variant,
                &["ACTIVE", "INACTIVE", "NONE"]
            )),
        }
    }
}

impl CustomStatus for Status {
    fn from_str(s: &str) -> Self {
        Status::from(s)
    }

    fn to_string(&self) -> String {
        match self {
            Status::Active => String::from("ACTIVE"),
            Status::Inactive => String::from("INACTIVE"),
            Status::None => String::from("NONE"),
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => Status::Active,
            "inactive" => Status::Inactive,
            _ => Status::None,
        }
    }
}

impl From<&String> for Status {
    fn from(s: &String) -> Self {
        Status::from(s.to_string())
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        Status::from(s.to_string())
    }
}

impl From<Option<String>> for Status {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Status::from(s),
            None => Status::None,
        }
    }
}