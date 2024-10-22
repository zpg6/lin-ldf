use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    IResult,
};

/// `Signal_encoding_types` section of a LIN Description File (LDF)
/// ```text
/// Signal_encoding_types {
///   ENC_BOOL {
///     logical_value, 0, "FALSE" ;
///     logical_value, 1, "TRUE" ;
///   }
///   ENC_TEMP {
///     physical_value, 0, 1023, 0.2, -40, "degC" ;
///   }
///   ENC_RPM {
///     physical_value, 0, 1023, 10, 0, "rpm" ;
///   }
/// }
/// ```
pub struct LdfSignalEncodingType {
    /// Signal encoding type name
    pub encoding_type_name: String,

    /// Signal encoding type values
    pub encoding_type_values: Vec<LdfSignalEncodingTypeValue>,
}

/// Signal encoding type value in the `Signal_encoding_types` section of a LIN Description File (LDF)
pub enum LdfSignalEncodingTypeValue {
    LogicalValue {
        /// Value
        value: u32,
        /// Value description
        value_description: String,
    },
    PhysicalValue {
        /// Minimum value
        min_value: u32,
        /// Maximum value
        max_value: u32,
        /// Scaling factor
        scaling_factor: f32,
        /// Offset
        offset: f32,
        /// Unit
        unit: String,
    },
}

/*
Signal_encoding_types {
    ENC_BOOL {
        logical_value, 0, "FALSE" ;
        logical_value, 1, "TRUE" ;
    }
    ENC_TEMP {
        physical_value, 0, 1023, 0.2, -40, "degC" ;
    }
    ENC_RPM {
        physical_value, 0, 1023, 10, 0, "rpm" ;
    }
}
*/

pub fn parse_ldf_signal_encoding_types(s: &str) -> IResult<&str, Vec<LdfSignalEncodingType>> {
    // `Signal_encoding_types {` or `Signal_encoding_types{` or ...
    // - May be any number of spaces before and after the "Signal_encoding_types" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Signal_encoding_types")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = skip_whitespace(s)?;

    let mut signal_encoding_types = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `ENC_BOOL {` or `ENC_BOOL{` or ...
        // - May be any number of spaces before and after the signal encoding type name
        // - May be any number of spaces before and after the opening curly brace
        let (s, _) = skip_whitespace(remaining)?;
        let (s, encoding_type_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("{")(s)?;

        let mut encoding_type_values = Vec::new();
        remaining = s;

        while !remaining.starts_with('}') {
            // `logical_value, 0, "FALSE" ;` or `logical_value, 0, "FALSE";` or ...
            // - May be any number of spaces before and after the value type
            // - May be any number of spaces before and after the comma
            // - May be any number of spaces before and after the value
            // - May be any number of spaces before and after the comma
            // - May be any number of spaces before and after the value description
            // - May be any number of spaces before and after the semicolon
            let (s, _) = skip_whitespace(remaining)?;
            let (s, value_type) = take_until(",")(s)?;
            let (s, _) = tag(",")(s)?;
            let (s, _) = skip_whitespace(s)?;

            let (s, encoding_type_value) = match value_type {
                "logical_value" => {
                    let (s, value) = take_while(|c: char| c.is_numeric())(s)?;
                    let (s, _) = tag(",")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag("\"")(s)?;
                    let (s, value_description) = take_until("\"")(s)?;
                    let (s, _) = tag("\"")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(";")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    (
                        s,
                        LdfSignalEncodingTypeValue::LogicalValue {
                            value: value.parse().unwrap(),
                            value_description: value_description.to_string(),
                        },
                    )
                }
                "physical_value" => {
                    let (s, min_value) = take_while(|c: char| c.is_numeric())(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(",")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, max_value) = take_while(|c: char| c.is_numeric())(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(",")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, scaling_factor) = take_while(|c: char| c.is_numeric() || c == '.' || c == '-')(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(",")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, offset) = take_while(|c: char| c.is_numeric() || c == '.' || c == '-')(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(",")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag("\"")(s)?;
                    let (s, unit) = take_until("\"")(s)?;
                    let (s, _) = tag("\"")(s)?;
                    let (s, _) = skip_whitespace(s)?;
                    let (s, _) = tag(";")(s)?;
                    let (s, _) = skip_whitespace(s)?;

                    (
                        s,
                        LdfSignalEncodingTypeValue::PhysicalValue {
                            min_value: min_value.parse().unwrap(),
                            max_value: max_value.parse().unwrap(),
                            scaling_factor: scaling_factor.parse().unwrap(),
                            offset: offset.parse().unwrap(),
                            unit: unit.to_string(),
                        },
                    )
                }
                _ => panic!("Unknown value type: {}", value_type),
            };

            encoding_type_values.push(encoding_type_value);
            remaining = s;
        }

        let (s, _) = tag("}")(remaining)?;
        let (s, _) = skip_whitespace(s)?;

        signal_encoding_types.push(LdfSignalEncodingType {
            encoding_type_name: encoding_type_name.to_string(),
            encoding_type_values,
        });

        remaining = s;
    }

    let (remaining, _) = tag("}")(remaining)?;

    Ok((remaining, signal_encoding_types))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_signal_encoding_types() {
        let input = r#"
            Signal_encoding_types {
                ENC_BOOL {
                    logical_value, 0, "FALSE" ;
                    logical_value, 1, "TRUE" ;
                }
                ENC_TEMP {
                    physical_value, 0, 1023, 0.2, -40, "degC" ;
                }
                ENC_RPM {
                    physical_value, 0, 1023, 10, 0, "rpm" ;
                }
            }
        "#;
        let (_, signal_encoding_types) = parse_ldf_signal_encoding_types(input).unwrap();
        assert_eq!(signal_encoding_types.len(), 3);

        let enc_bool = &signal_encoding_types[0];
        assert_eq!(enc_bool.encoding_type_name, "ENC_BOOL");
        assert_eq!(enc_bool.encoding_type_values.len(), 2);
        match &enc_bool.encoding_type_values[0] {
            LdfSignalEncodingTypeValue::LogicalValue {
                value,
                value_description,
            } => {
                assert_eq!(*value, 0);
                assert_eq!(value_description, "FALSE");
            }
            _ => panic!("Expected LogicalValue"),
        }
        match &enc_bool.encoding_type_values[1] {
            LdfSignalEncodingTypeValue::LogicalValue {
                value,
                value_description,
            } => {
                assert_eq!(*value, 1);
                assert_eq!(value_description, "TRUE");
            }
            _ => panic!("Expected LogicalValue"),
        }

        let enc_temp = &signal_encoding_types[1];
        assert_eq!(enc_temp.encoding_type_name, "ENC_TEMP");
        assert_eq!(enc_temp.encoding_type_values.len(), 1);
        match &enc_temp.encoding_type_values[0] {
            LdfSignalEncodingTypeValue::PhysicalValue {
                min_value,
                max_value,
                scaling_factor,
                offset,
                unit,
            } => {
                assert_eq!(*min_value, 0);
                assert_eq!(*max_value, 1023);
                assert_eq!(*scaling_factor, 0.2);
                assert_eq!(*offset, -40.0);
                assert_eq!(unit, "degC");
            }
            _ => panic!("Expected PhysicalValue"),
        }

        let enc_rpm = &signal_encoding_types[2];
        assert_eq!(enc_rpm.encoding_type_name, "ENC_RPM");
        assert_eq!(enc_rpm.encoding_type_values.len(), 1);
        match &enc_rpm.encoding_type_values[0] {
            LdfSignalEncodingTypeValue::PhysicalValue {
                min_value,
                max_value,
                scaling_factor,
                offset,
                unit,
            } => {
                assert_eq!(*min_value, 0);
                assert_eq!(*max_value, 1023);
                assert_eq!(*scaling_factor, 10.0);
                assert_eq!(*offset, 0.0);
                assert_eq!(unit, "rpm");
            }
            _ => panic!("Expected PhysicalValue"),
        }
    }
}
