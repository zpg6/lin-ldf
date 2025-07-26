use nom::{
    bytes::complete::{tag, take_until, take_while},
    IResult,
};

use crate::ldf::ldf_comment::skip_whitespace;

/// Header of a LIN Description File (LDF) for LIN
/// ```text
/// LIN_description_file;
/// LIN_protocol_version = "2.1" ;
/// LIN_language_version = "2.1" ;
/// LIN_speed = 19.2 kbps ;
/// Channel_name = "DB";
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct LdfHeader {
    /// LIN protocol version number (e.g. 2.1).
    /// Shall be in the range of "0.01" to "99.99".
    pub lin_protocol_version: String,

    /// LIN language version number (e.g. 2.1)
    /// Shall be in the range of "0.01" to "99.99".
    pub lin_language_version: String,

    /// LIN speed in bits per second (e.g. 19.2 kbps = 19200).
    /// This sets the nominal bit rate for the cluster. It shall be in the range of 1 to 20 kbit/second.
    pub lin_speed: u16,

    /// Channel_name is optional and was added in LIN 2.2 version.
    pub channel_name: Option<String>,
}

/*
LIN_description_file;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;
Channel_name = "DB";
*/

pub fn parse_ldf_header(s: &str) -> IResult<&str, LdfHeader> {
    // Skip anything before "LIN_description_file"
    let (s, _) = take_until("LIN_description_file")(s)?;

    // `LIN_description_file;` or `LIN_description_file ;` or ...
    // - May be any number of spaces before and after the "LIN_description_file" tag
    // - May be any number of spaces before and after the semicolon
    let (s, _) = tag("LIN_description_file")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    // `LIN_protocol_version = "2.1" ;` or `LIN_protocol_version = "2.1";` or ...
    // - May be any number of spaces before and after the "LIN_protocol_version" tag
    // - May be any number of spaces before and after the equal sign
    // - May be any number of spaces before and after the version number
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("LIN_protocol_version")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, protocol_major) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = tag(".")(s)?;
    let (s, protocol_minor) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    let lin_protocol_version = format!("{}.{}", protocol_major, protocol_minor);

    // `LIN_language_version = "2.1" ;` or `LIN_language_version = "2.1";` or ...
    // - May be any number of spaces before and after the "LIN_language_version" tag
    // - May be any number of spaces before and after the equal sign
    // - May be any number of spaces before and after the version number
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("LIN_language_version")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, language_major) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = tag(".")(s)?;
    let (s, language_minor) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    let lin_language_version = format!("{}.{}", language_major, language_minor);

    // LIN_speed = 19.2 kbps ;
    // - May be any number of spaces before and after the "LIN_speed" tag
    // - May be any number of spaces before and after the equal sign
    // - May be any number of spaces before and after the speed number
    // - May be any number of spaces before and after the speed unit
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("LIN_speed")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, lin_speed) = take_while(|c: char| c.is_numeric() || c == '.')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("kbps")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    let lin_speed = (lin_speed.parse::<f32>().unwrap() * 1000.0) as u16;

    let (s, channel_name) = parse_channel_name(s).unwrap_or((s, None));

    Ok((
        s,
        LdfHeader {
            lin_protocol_version,
            lin_language_version,
            lin_speed,
            channel_name,
        },
    ))
}

pub fn parse_channel_name(s: &str) -> IResult<&str, Option<String>> {
    // Channel_name = "DB";
    // - May be any number of spaces before and after the "Channel_name" tag
    // - May be any number of spaces before and after the equal sign
    // - May be any number of spaces before and after the channel name
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Channel_name")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, channel_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    let (s, _) = tag("\"")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    Ok((s, Some(channel_name.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = r#"
            LIN_description_file ; 
            LIN_protocol_version = "2.1" ; 
            LIN_language_version = "2.1" ; 
            LIN_speed = 19.2 kbps ;
            Channel_name = "DB";
        "#;

        let (_, header) = parse_ldf_header(s).unwrap();
        assert_eq!(header.lin_protocol_version, "2.1");
        assert_eq!(header.lin_language_version, "2.1");
        assert_eq!(header.lin_speed, 19200);
        assert_eq!(header.channel_name, Some("DB".to_string()));
    }

    #[test]
    fn test_parse_channel_name() {
        let s = r#"Channel_name = "DB";"#;
        let (_, channel_name) = parse_channel_name(s).unwrap();
        assert_eq!(channel_name, Some("DB".to_string()));
    }

    #[test]
    fn test_parse_header_no_channel_name() {
        let s = r#"
            LIN_description_file ; 
            LIN_protocol_version = "2.1" ; 
            LIN_language_version = "2.1" ; 
            LIN_speed = 19.2 kbps ;
        "#;

        let (_, header) = parse_ldf_header(s).unwrap();
        assert_eq!(header.lin_protocol_version, "2.1");
        assert_eq!(header.lin_language_version, "2.1");
        assert_eq!(header.lin_speed, 19200);
        assert_eq!(header.channel_name, None);
    }
}
