use anyhow::Result;
use xsalsa20poly1305::aead::{ Aead, KeyInit };
use xsalsa20poly1305::aead::generic_array::{ GenericArray, typenum };
use xsalsa20poly1305::XSalsa20Poly1305;

const NONCE_LENGTH: usize = 24;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cipher {
    is_encrypted: bool,
    content: CipherContent
}

#[derive(Debug, Clone, PartialEq)]
enum CipherContent {
    String(String),
    Vec(Vec<u8>)
}

impl Default for CipherContent {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl std::fmt::Display for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.b64encode()
            .map_err(|_| std::fmt::Error)?)
    }
}

impl sqlx::Type<sqlx::Postgres> for Cipher {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("TEXT")
    }
}

impl From<Option<String>> for Cipher {
    fn from(content: Option<String>) -> Self {
        if let Some(content) = content {
            return Self::from(content)
                .set_as_encrypted()
                .decrypt()
                .unwrap_or_else(|_| Self::default())
                .set_as_string();
        }

        Self {
            is_encrypted: false,
            content: CipherContent::String(String::new())
        }
    }
}

impl<'r> sqlx::Encode<'r, sqlx::Postgres> for Cipher {
    fn encode_by_ref(&self, buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'r>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        let content = self.clone();

        if let Ok(encrypted) = content.encrypt() {
            if let Ok(string) = encrypted.b64encode() {
                buf.extend_from_slice(string.as_bytes());
                return sqlx::encode::IsNull::No;
            }
        }

        sqlx::encode::IsNull::Yes
    }
}

impl From<String> for Cipher {
    fn from(content: String) -> Self {
        Self::from(content.as_str())
    }
}

impl From<&String> for Cipher {
    fn from(content: &String) -> Self {
        Self::from(content.as_str())
    }
}

impl From<&str> for Cipher {
    fn from(content: &str) -> Self {
        Self {
            is_encrypted: true,
            content: CipherContent::String(content.to_string())
        }
    }
}

impl From<Vec<u8>> for Cipher {
    fn from(content: Vec<u8>) -> Self {
        Self {
            is_encrypted: true,
            content: CipherContent::Vec(content)
        }
    }
}

impl Cipher {
    pub fn decrypt(self) -> Result<Self> {
        // Check if self is already decrypted
        if !self.is_encrypted {
            return Ok(self);
        }

        // Create bindings and then generate cipher instance
        let bindings = base64_url::decode(&std::env::var("MASTER_KEY")?)?;
        let key = GenericArray::from_slice(&bindings);
        let cipher = XSalsa20Poly1305::new(key);

        // Retrieve content
        let content = match self.content.clone() {
            CipherContent::String(content) => base64_url::decode(&content)?,
            CipherContent::Vec(content) => content
        };

        // Check content length
        if content.len() <= NONCE_LENGTH {
            return Err(anyhow::anyhow!("Invalid content length"));
        }

        // Split content
        let (nonce, content) = content.split_at(NONCE_LENGTH);

        // Set nonce & content
        let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

        // Decrypt content
        match cipher.decrypt(nonce, content) {
            Ok(content) => Ok(Self {
                is_encrypted: false,
                content: CipherContent::Vec(content)
            }),
            Err(_) => Err(anyhow::anyhow!("Unable to decrypt content"))
        }
    }

    pub fn encrypt(&self) -> Result<Self> {
        // Check if self is already encrypted
        if self.is_encrypted {
            return Ok(self.clone());
        }

        // Create bindings and then generate cipher instance
        let bindings = base64_url::decode(&std::env::var("MASTER_KEY")?)?;
        let key = GenericArray::from_slice(&bindings);
        let cipher = XSalsa20Poly1305::new(key);

        // Set nonce
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);

        // Retrieve content
        let content = match self.content.clone() {
            CipherContent::String(content) => content,
            CipherContent::Vec(content) => String::from_utf8_lossy(content.as_slice()).to_string()
        };

        // Encrypt content
        if let Ok(content) = cipher.encrypt(&nonce, content.as_bytes()) {
            return Ok(Self{
                is_encrypted: true,
                content: CipherContent::Vec([&nonce[..], &content[..]].concat())
            });
        }

        Err(anyhow::anyhow!("Unable to encrypt content"))
    }

    pub fn is_controller(&self) -> bool {
        if let Ok(env) = std::env::var("MASTER_KEY") {
            if let Ok(key) = base64_url::decode(&env) {
                if let Ok(cipher) = self.clone().decrypt() {
                    if let Ok(cipher) = cipher.to_vec() {
                        return key == cipher;
                    }
                }
            }
        }

        false
    }

    pub fn set_as_decrypted(&self) -> Self {
        let mut data = self.clone();
        data.is_encrypted = false;
        data
    }

    pub fn set_as_encrypted(&self) -> Self {
        let mut data = self.clone();
        data.is_encrypted = true;
        data
    }

    pub fn set_as_string(&self) -> Self {
        let mut data = self.clone();
        data.content = CipherContent::String(data.to_string().unwrap_or_default());
        data
    }

    pub fn to_string(&self) -> Result<String> {
        // Retrieve content
        let content = match &self.content {
            CipherContent::String(content) => content.clone(),
            CipherContent::Vec(content) => String::from_utf8_lossy(content).to_string()
        };

        // Return content
        Ok(content)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        // Retrieve content
        let content = match &self.content {
            CipherContent::String(content) => content.as_bytes().to_vec(),
            CipherContent::Vec(content) => content.clone()
        };

        // Return content
        Ok(content)
    }

    pub fn b64encode(&self) -> Result<String> {
        match &self.content {
            CipherContent::String(content) => Ok(base64_url::encode(content)),
            CipherContent::Vec(content) => Ok(base64_url::encode(content.as_slice()))
        }
    }
}