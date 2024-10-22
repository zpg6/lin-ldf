use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    IResult,
};

/// `Signal_representation` section of a LIN Description File (LDF)
/// ```text
/// Signal_representation {
///   ENC_BOOL: Signal1, Signal2 ;
///   ENC_TEMP: Signal3, Signal4 ;
///   ENC_RPM: Signal5, Signal6 ;
/// }
pub struct LdfSignalRepresentation {
    /// Signal encoding type name
    pub encoding_type_name: String,

    /// Signal names
    pub signal_names: Vec<String>,
}

/*
Signal_representation {
    ENC_BOOL: Signal1, Signal2 ;
    ENC_TEMP: Signal3, Signal4 ;
    ENC_RPM: Signal5, Signal6 ;
}
*/

pub fn parse_ldf_signal_representation(s: &str) -> IResult<&str, Vec<LdfSignalRepresentation>> {
    // `Signal_representation {` or `Signal_representation{` or ...
    // - May be any number of spaces before and after the "Signal_representation" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Signal_representation")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    // [Assumes no comments between the signal representation tags]

    let mut signal_representations = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `ENC_BOOL: Signal1, Signal2 ;` or `ENC_BOOL: Signal1, Signal2;` or ...
        // - May be any number of spaces before and after the "ENC_BOOL" tag
        // - May be any number of spaces before and after the colon
        // - May be any number of spaces before and after the semicolon
        let (s, _) = skip_whitespace(remaining)?;
        let (s, encoding_type_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, signal_names) = take_until(";")(s)?;
        let (s, _) = tag(";")(s)?;
        let (s, _) = skip_whitespace(s)?;

        let signal_names = signal_names
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        signal_representations.push(LdfSignalRepresentation {
            encoding_type_name: encoding_type_name.trim().to_string(),
            signal_names,
        });

        remaining = s;
    }

    let (s, _) = tag("}")(remaining)?;

    Ok((s, signal_representations))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_signal_representation() {
        let input = r#"
        Signal_representation {
            ENC_BOOL: Signal1, Signal2 ;
            ENC_TEMP: Signal3, Signal4 ;
            ENC_RPM: Signal5, Signal6 ;
        }
        "#;

        let (_, signal_representations) = parse_ldf_signal_representation(input).unwrap();

        assert_eq!(signal_representations.len(), 3);

        assert_eq!(signal_representations[0].encoding_type_name, "ENC_BOOL");
        assert_eq!(signal_representations[0].signal_names.len(), 2);
        assert_eq!(signal_representations[0].signal_names[0], "Signal1");
        assert_eq!(signal_representations[0].signal_names[1], "Signal2");

        assert_eq!(signal_representations[1].encoding_type_name, "ENC_TEMP");
        assert_eq!(signal_representations[1].signal_names.len(), 2);
        assert_eq!(signal_representations[1].signal_names[0], "Signal3");
        assert_eq!(signal_representations[1].signal_names[1], "Signal4");

        assert_eq!(signal_representations[2].encoding_type_name, "ENC_RPM");
        assert_eq!(signal_representations[2].signal_names.len(), 2);
        assert_eq!(signal_representations[2].signal_names[0], "Signal5");
        assert_eq!(signal_representations[2].signal_names[1], "Signal6");
    }
}
