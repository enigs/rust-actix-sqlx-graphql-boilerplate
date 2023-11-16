use async_graphql::Context;
use library::prelude::CustomRole;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[derive(async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Controller,
    Admin,
    #[default]
    Guest
}

impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let variant_str = match self {
            Role::Controller => "CONTROLLER",
            Role::Admin => "ADMIN",
            Role::Guest => "GUEST",
        };

        serializer.serialize_str(variant_str)
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;

        match variant.to_lowercase().as_str() {
            "controller" => Ok(Role::Controller),
            "admin" => Ok(Role::Admin),
            "guest" => Ok(Role::Guest),
            _ => Err(serde::de::Error::unknown_variant(
                &variant,
                &["CONTROLLER", "ADMIN", "GUEST"]
            )),
        }
    }
}

impl CustomRole for Role {
    fn get(ctx: &Context<'_>) -> Self {
        if let Some(role) = ctx.data_opt::<Self>() {
            return *role;
        }

        Role::Guest
    }

    fn get_controller() -> Self {
        Self::Controller
    }

    fn get_admin() -> Self {
        Self::Admin
    }

    fn get_guest() -> Self {
        Self::Guest
    }

    fn from_str(s: &str) -> Self {
        Role::from(s)
    }

    fn to_string(&self) -> String {
        match self {
            Role::Controller => String::from("CONTROLLER"),
            Role::Admin => String::from("ADMIN"),
            Role::Guest => String::from("GUEST"),
        }
    }
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "controller" => Role::Controller,
            "admin" => Role::Admin,
            "guest" => Role::Guest,
            _ => Role::Guest,
        }
    }
}

impl From<&String> for Role {
    fn from(s: &String) -> Self {
        Role::from(s.to_string())
    }
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        Role::from(s.to_string())
    }
}

impl From<Option<String>> for Role {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Role::from(s),
            None => Role::Guest,
        }
    }
}