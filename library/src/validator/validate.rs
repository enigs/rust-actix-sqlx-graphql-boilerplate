use crate::Validator;

impl Validator {
    pub fn validate_list_string(&self) -> Option<String> {
        // Check if string is empty
        if self.is_required && self.string_value.is_empty() {
            return Some(self.locales.lookup(format!("{}-empty", self.field)));
        }

        // Check if list has value
        if let Some(list) = self.option_list_string.clone() {
            match self.is_case_sensitive {
                true => if self.is_required && !list.contains(&self.string_value) {
                    return Some(self.locales.lookup(format!("{}-invalid", self.field)));
                }
                false => {
                    let list = list.iter()
                        .map(|value| value.to_lowercase())
                        .collect::<Vec<String>>();

                    if self.is_required && !list.contains(&self.string_value.to_lowercase()) {
                        return Some(self.locales.lookup(format!("{}-invalid", self.field)));
                    }
                }
            }
        }

        None
    }

    pub fn validate_i32(&self) -> Option<String> {
        // Check if i32 is empty
        if self.is_required && self.i32_value == 0 {
            return Some(self.locales.lookup(format!("{}-empty", self.field)));
        }

        // Check if min & max has value
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if self.is_required && self.i32_value < min as i32 && self.i32_value > max as i32 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-min-max", self.field),
                    &[
                        ("min", min.to_string().as_str()),
                        ("max", max.to_string().as_str())
                    ]
                ));
            }
        }

        // Check if min has value
        if let Some(min) = self.min {
            if self.is_required && self.i32_value < min as i32 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-min", self.field),
                    &[("min", min.to_string().as_str())]
                ));
            }
        }

        // Check if max has value
        if let Some(max) = self.max {
            if self.is_required && self.i32_value > max as i32 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-max", self.field),
                    &[("max", max.to_string().as_str())]
                ));
            }
        }

        None
    }

    pub fn validate_i64(&self) -> Option<String> {
        // Check if i64 is empty
        if self.is_required && self.i64_value == 0 {
            return Some(self.locales.lookup(format!("{}-empty", self.field)));
        }

        // Check if min & max has value
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if self.is_required && self.i64_value < min as i64 && self.i64_value > max as i64 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-min-max", self.field),
                    &[
                        ("min", min.to_string().as_str()),
                        ("max", max.to_string().as_str())
                    ]
                ));
            }
        }

        // Check if min has value
        if let Some(min) = self.min {
            if self.is_required && self.i64_value < min as i64 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-min", self.field),
                    &[("min", min.to_string().as_str())]
                ));
            }
        }

        // Check if max has value
        if let Some(max) = self.max {
            if self.is_required && self.i64_value > max as i64 {
                return Some(self.locales.lookup_with_args(
                    format!("{}-max", self.field),
                    &[("max", max.to_string().as_str())]
                ));
            }
        }

        None
    }

    pub fn validate_string(&self) -> Option<String> {
        // Check if string is empty
        if self.string_value.is_empty() {
            return Some(self.locales.lookup(format!("{}-empty", self.field)));
        }

        match () {
            _ if self.min.is_some() && self.max.is_some() => {
                let min = self.min.unwrap();
                let max = self.max.unwrap();
                let len = self.string_value.len();

                match () {
                    _ if len < min && len > max => {
                        let error = self.locales.lookup_with_args(
                            format!("{}-min-max", self.field),
                            &[
                                ("min", min.to_string().as_str()),
                                ("max", max.to_string().as_str())
                            ]
                        );

                        Some(error)
                    },
                    _ if len < min => {
                        let error = self.locales.lookup_with_args(
                            format!("{}-min", self.field),
                            &[("min", min.to_string().as_str())]
                        );

                        Some(error)
                    },
                    _ if len > max => {
                        let error = self.locales.lookup_with_args(
                            format!("{}-max", self.field),
                            &[("max", max.to_string().as_str())]
                        );

                        Some(error)
                    },
                    _ => None
                }
            },
            _ if self.min.is_some() && self.max.is_none() => {
                let min = self.min.unwrap();
                let len = self.string_value.len();

                if len < min {
                    let error = self.locales.lookup_with_args(
                        format!("{}-min", self.field),
                        &[("min", min.to_string().as_str())]
                    );

                    return Some(error);
                }

                None
            },
            _ if self.min.is_none() && self.max.is_some() => {
                let max = self.max.unwrap();
                let len = self.string_value.len();

                if len > max {
                    let error = self.locales.lookup_with_args(
                        format!("{}-max", self.field),
                        &[("max", max.to_string().as_str())]
                    );

                    return Some(error)
                };

                None
            },
            _ => None
        }
    }
}