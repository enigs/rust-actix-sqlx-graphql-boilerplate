use async_graphql::{Context, MaybeUndefined, InputObject, Result};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

use crate::{Core, Errors, Validator, Response};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::S3, error = "S3Error")]
#[serde(rename_all = "camelCase")]
pub struct S3Form {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_key_id: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub secret_access_key: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub bucket: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub path: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub region: MaybeUndefined<String>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_small_size: MaybeUndefined<i32>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_thumbnail_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_small_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_small_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_medium_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_large_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_width_xxxl_size: MaybeUndefined<i32>,

    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui322i32)]
    #[error(String)]
    pub image_landscape_height_xxxl_size: MaybeUndefined<i32>,
}

impl S3Form {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = S3Error {
            access_key_id: Validator::new(locale, "s3-access-key-id")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.access_key_id)
                .validate_string(),
            secret_access_key: Validator::new(locale, "s3-secret-access-key")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.secret_access_key)
                .validate_string(),
            path: Validator::new(locale, "s3-path")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.path)
                .validate_string(),
            bucket: Validator::new(locale, "s3-bucket")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.bucket)
                .validate_string(),
            region: Validator::new(locale, "s3-region")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.region)
                .validate_string(),
            image_thumbnail_small_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_thumbnail_small_size)
                .validate_i32(),
            image_thumbnail_medium_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_thumbnail_medium_size)
                .validate_i32(),
            image_thumbnail_large_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_thumbnail_large_size)
                .validate_i32(),
            image_thumbnail_xl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_thumbnail_xl_size)
                .validate_i32(),
            image_landscape_width_small_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_small_size)
                .validate_i32(),
            image_landscape_height_small_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_small_size)
                .validate_i32(),
            image_landscape_width_medium_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_medium_size)
                .validate_i32(),
            image_landscape_height_medium_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_medium_size)
                .validate_i32(),
            image_landscape_width_large_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_large_size)
                .validate_i32(),
            image_landscape_height_large_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_large_size)
                .validate_i32(),
            image_landscape_width_xl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_xl_size)
                .validate_i32(),
            image_landscape_height_xl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_xl_size)
                .validate_i32(),
            image_landscape_width_xxl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_xxl_size)
                .validate_i32(),
            image_landscape_height_xxl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_xxl_size)
                .validate_i32(),
            image_landscape_width_xxxl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_width_xxxl_size)
                .validate_i32(),
            image_landscape_height_xxxl_size: Validator::new(locale, "s3-image-size")
                .set_min(1)
                .set_max(2080)
                .set_as_required(true)
                .set_i32_value(&data.image_landscape_height_xxxl_size)
                .validate_i32(),
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => Ok(data),
            false => Err(Errors::to(response, error))
        }
    }
}