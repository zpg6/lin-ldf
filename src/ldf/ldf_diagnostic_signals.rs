use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_while},
    IResult,
};

/// `Diagnostic_signals` section of a LIN Description File (LDF) for LIN 2.1
/// ```text
/// Diagnostic_signals {
///  MasterReqB0: 8, 0 ;
///  MasterReqB1: 8, 0 ;
///  MasterReqB2: 8, 0 ;
///  MasterReqB3: 8, 0 ;
///  MasterReqB4: 8, 0 ;
///  MasterReqB5: 8, 0 ;
///  MasterReqB6: 8, 0 ;
///  MasterReqB7: 8, 0 ;
///  SlaveRespB0: 8, 0 ;
///  SlaveRespB1: 8, 0 ;
///  SlaveRespB2: 8, 0 ;
///  SlaveRespB3: 8, 0 ;
///  SlaveRespB4: 8, 0 ;
///  SlaveRespB5: 8, 0 ;
///  SlaveRespB6: 8, 0 ;
///  SlaveRespB7: 8, 0 ;
/// }
/// ```

/// Diagnostic signal in the `Diagnostic_signals` section of a LIN Description File (LDF) for LIN 2.1
/// ```text
/// MasterReqB0: 8, 0 ;
/// ```
pub struct LdfDiagnosticSignal {
    /// All identifiers must be unique within the LDF file.
    pub name: String,

    /// Length of the signal in bits.
    pub length: u8,

    /// Initial value of the signal.
    pub init_value: u8,
}

pub fn parse_ldf_diagnostic_signals(s: &str) -> IResult<&str, Vec<LdfDiagnosticSignal>> {
    // `Diagnostic_signals {` or `Diagnostic_signals{` or ...
    // - May be any number of spaces before and after the "Diagnostic_signals" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Diagnostic_signals")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = skip_whitespace(s)?;

    let mut diagnostic_signals = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `MasterReqB0: 8, 0 ;` or `MasterReqB0: 8, 0;` or ...
        // - May be any number of spaces before and after the signal name
        // - May be any number of spaces before and after the colon
        // - May be any number of spaces before and after the signal size
        // - May be any number of spaces before and after the comma
        // - May be any number of spaces before and after the init value
        // - May be any number of spaces before and after the semicolon
        let (s, _) = skip_whitespace(remaining)?;
        let (s, signal_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, signal_size) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(",")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, init_value) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;
        let (s, _) = skip_whitespace(s)?;

        diagnostic_signals.push(LdfDiagnosticSignal {
            name: signal_name.to_string(),
            length: signal_size.parse().unwrap(),
            init_value: init_value.parse().unwrap(),
        });

        remaining = s;
    }

    let (remaining, _) = tag("}")(remaining)?;

    Ok((remaining, diagnostic_signals))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_diagnostic_signals() {
        let input = r#"
            Diagnostic_signals {
                MasterReqB0: 8, 0 ;
                MasterReqB1: 8, 0 ;
                MasterReqB2: 8, 0 ;
                MasterReqB3: 8, 0 ;
                MasterReqB4: 8, 0 ;
                MasterReqB5: 8, 0 ;
                MasterReqB6: 8, 0 ;
                MasterReqB7: 8, 0 ;
                SlaveRespB0: 8, 0 ;
                SlaveRespB1: 8, 0 ;
                SlaveRespB2: 8, 0 ;
                SlaveRespB3: 8, 0 ;
                SlaveRespB4: 8, 0 ;
                SlaveRespB5: 8, 0 ;
                SlaveRespB6: 8, 0 ;
                SlaveRespB7: 8, 0 ;
            }
        "#;

        let (_, diagnostic_signals) = parse_ldf_diagnostic_signals(input).unwrap();
        assert_eq!(diagnostic_signals.len(), 16);
        assert_eq!(diagnostic_signals[0].name, "MasterReqB0");
        assert_eq!(diagnostic_signals[0].length, 8);
        assert_eq!(diagnostic_signals[0].init_value, 0);

        assert_eq!(diagnostic_signals[15].name, "SlaveRespB7");
        assert_eq!(diagnostic_signals[15].length, 8);
        assert_eq!(diagnostic_signals[15].init_value, 0);
    }
}
