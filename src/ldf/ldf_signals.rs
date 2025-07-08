use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    IResult,
};

/// The init_value specifies the signal value that shall be used by all subscriber nodes.
/// The init_value_scalar is used for scalar signals and the init_value_array is used for byte array signals.
#[derive(Debug, PartialEq)]
pub enum LdfSignalInitValue {
    Scalar(u8),
    Array(Vec<u8>),
}

/// `Signals` section of a LIN Description File (LDF) for LIN 2.1
/// ```text
/// Signals {
///  Signal1: 10, 0, Master, Slave1 ;
///  Signal2: 10, 0, Master, Slave1 ;
///  Signal3: 10, 0, Master, Slave1 ;
///  Signal4: 10, 0, Slave1, Master ;
///  Signal5: 2, 0, Slave1, Master ;
///  Signal6: 1, 0, Slave1, Master ;
/// }
/// ```
/// ---
/// Signal in the `Signals` section of a LIN Description File (LDF) for LIN 2.1
/// ```text
/// Signal1: 10, 0, Master, Slave1 ;
/// ```
/// Reads as:
/// ```text
/// <signal_name>: <signal_size>, <init_value>, <published_by> [, <subscribed_by>] ;
/// ```
pub struct LdfSignal {
    /// All identifiers must be unique within the LDF file.
    pub name: String,

    /// The signal_size specifies the size of the signal. It shall be in the range 1 to 16 bits for
    /// scalar signals and 8, 16, 24, 32, 40, 48, 56 or 64 for byte array signals.
    pub signal_size: u8,

    /// ```text
    /// <init_value> ::= <init_value_scalar> | <init_value_array>
    /// <init_value_scalar> ::= integer
    /// <init_value_array> ::= {integer ([, integer])}
    /// ```
    ///
    /// The init_value specifies the signal value that shall be used by all subscriber nodes
    /// until the frame containing the signal is received. The init_value_scalar is used for
    /// scalar signals and the init_value_array is used for byte array signals. The initial_value for
    /// byte arrays shall be arranged in big-endian order (i.e. with the most significant byte
    /// first).
    ///
    /// The only way to describe if a signal with size 8 or 16 is a byte array with one or two
    /// elements or a scalar signal is by analyzing the init_value, i.e. the curly parenthesis are
    /// very important to distinguish between arrays and scalar values.
    pub init_value: LdfSignalInitValue,

    /// The published_by specifies the node that is publishing the signal.
    /// The published_by identifier shall exist in the node identifier set.
    pub published_by: String,

    /// The subscribed_by specifies the node(s) that is subscribing to the signal.
    /// The subscribed_by identifiers shall exist in the node identifier set.
    pub subscribed_by: Vec<String>,
}

/*
Signals {
    Signal1: 10, 0, Master, Slave1 ;
    Signal2: 10, 0, Master, Slave1 ;
    Signal3: 10, 0, Slave1, Master ;
    Signal4: 10, 0, Slave1, Master ;
    Signal5: 2, 0, Slave1, Master ;
    Signal6: 1, 0, Slave1, Master ;
}
*/

pub fn parse_ldf_signals(s: &str) -> IResult<&str, Vec<LdfSignal>> {
    // `Signals {` or `Signals{` or ...
    // - May be any number of spaces before and after the "Signals" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Signals")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    let mut signals = Vec::new();
    let mut remaining = s;
    while !remaining.starts_with('}') {
        // `Signal1: 10, 0, Master, Slave1 ;` or `Signal1: 10, 0, Master, Slave1;` or ...
        // - May be any number of spaces before and after the signal name
        // - May be any number of spaces before and after the colon
        // - May be any number of spaces before and after the signal size
        // - May be any number of spaces before and after the comma
        // - May be any number of spaces before and after the init value
        // - May be any number of spaces before and after the comma
        // - May be any number of spaces before and after the published_by node
        // - May be any number of spaces before and after the comma
        // - May be any number of spaces before and after the subscribed_by node
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
        let (s, _) = tag(",")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, published_by) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        
        let (s, symbol) = take_while(|c: char| c == ',' || c == ';')(s)?;
        let mut subscribed_by = Vec::new();
        let s = match symbol {
            "," => {
                // There is at least one subscribed_by node
                let (s, subscribed_by_str) = take_until(";")(s)?;
                subscribed_by = subscribed_by_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                let (s, _) = tag(";")(s)?;
                s
            }
            ";" => {
                // There are no subscribed_by nodes
                s
            }
            _ => {
                // This should not happen, but we can handle it gracefully
                s
            }
        };
        let (s, _) = skip_whitespace(s)?;

        remaining = s;

        let signal = LdfSignal {
            name: signal_name.to_string(),
            signal_size: signal_size.parse().unwrap(),
            init_value: LdfSignalInitValue::Scalar(init_value.parse().unwrap()),
            published_by: published_by.to_string(),
            subscribed_by,
        };

        signals.push(signal);
    }

    // `}` or `} ;` or ...
    // - May be any number of spaces before and after the closing curly brace
    let (s, _) = skip_whitespace(remaining)?;
    let (s, _) = tag("}")(s)?;

    Ok((s, signals))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_signals() {
        let input = r#"
            Signals {
                Signal1: 10, 0, Master, Slave1 , Slave2 ;
                Signal2: 10, 0, Master, Slave1 ;
                Signal3: 10, 0, Slave1, Master ;
                Signal4: 10, 0, Slave1, Master ;
                Signal5: 2, 0, Slave1, Master ;
                Signal6: 1, 0, Slave1, Master ;
            }
        "#;

        let (_, signals) = parse_ldf_signals(input).unwrap();
        assert_eq!(signals.len(), 6);
        assert_eq!(signals[0].name, "Signal1");
        assert_eq!(signals[0].signal_size, 10);
        assert_eq!(signals[0].published_by, "Master");
        assert_eq!(signals[0].subscribed_by, vec!["Slave1", "Slave2"]);
        assert_eq!(signals[1].name, "Signal2");
    }
}
