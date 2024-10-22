use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_while},
    IResult,
};

/// `Diagnostic_frames` section of a LIN Description File (LDF)
/// ```text
/// Diagnostic_frames {
///   MasterReq: 0x3C {
///     MasterReqB0, 0 ;
///     MasterReqB1, 8 ;
///     MasterReqB2, 16 ;
///     MasterReqB3, 24 ;
///     MasterReqB4, 32 ;
///     MasterReqB5, 40 ;
///     MasterReqB6, 48 ;
///     MasterReqB7, 56 ;
///   }
///   SlaveResp: 0x3D {
///     SlaveRespB0, 0 ;
///     SlaveRespB1, 8 ;
///     SlaveRespB2, 16 ;
///     SlaveRespB3, 24 ;
///     SlaveRespB4, 32 ;
///     SlaveRespB5, 40 ;
///     SlaveRespB6, 48 ;
///     SlaveRespB7, 56 ;
///   }
/// }
pub struct LdfDiagnosticFrame {
    /// Frame name
    pub frame_name: String,

    /// Frame ID
    pub frame_id: u8,

    /// Frame signals
    pub signals: Vec<LdfDiagnosticFrameSignal>,
}

/// One signal section of a Frame in a LIN Description File (LDF).
/// ```text
/// MasterReqB0, 0 ;
/// ```
pub struct LdfDiagnosticFrameSignal {
    /// Signal name
    pub signal_name: String,

    /// Signal start bit
    pub start_bit: u8,
}

/*
Diagnostic_frames {
  MasterReq: 0x3C {
    MasterReqB0, 0 ;
    MasterReqB1, 8 ;
    MasterReqB2, 16 ;
    MasterReqB3, 24 ;
    MasterReqB4, 32 ;
    MasterReqB5, 40 ;
    MasterReqB6, 48 ;
    MasterReqB7, 56 ;
  }
  SlaveResp: 0x3D {
    SlaveRespB0, 0 ;
    SlaveRespB1, 8 ;
    SlaveRespB2, 16 ;
    SlaveRespB3, 24 ;
    SlaveRespB4, 32 ;
    SlaveRespB5, 40 ;
    SlaveRespB6, 48 ;
    SlaveRespB7, 56 ;
  }
}
*/

pub fn parse_ldf_diagnostic_frames(s: &str) -> IResult<&str, Vec<LdfDiagnosticFrame>> {
    // `Diagnostic_frames {` or `Diagnostic_frames{` or ...
    // - May be any number of spaces before and after the "Diagnostic_frames" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Diagnostic_frames")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    let (s, _) = skip_whitespace(s)?;

    let mut diagnostic_frames = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `MasterReq: 0x3C {` or `MasterReq: 0x3C{` or ...
        // - May be any number of spaces before and after the frame name
        // - May be any number of spaces before and after the colon
        // - May be any number of spaces before and after the frame ID
        // - May be any number of spaces before and after the opening curly brace
        let (s, frame_name) = take_while(|c: char| c != ':' && c != '{')(remaining)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("0x")(s)?;
        let (s, frame_id) = take_while(|c: char| c != ' ' && c != '{')(s)?;
        let frame_id = u8::from_str_radix(frame_id, 16).unwrap();
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("{")(s)?;
        let (s, _) = skip_whitespace(s)?;

        let mut signals = Vec::new();
        remaining = s;

        while !remaining.starts_with('}') {
            // `MasterReqB0, 0 ;` or `MasterReqB0, 0;` or ...
            // - May be any number of spaces before and after the signal name
            // - May be any number of spaces before and after the comma
            // - May be any number of spaces before and after the start bit
            // - May be any number of spaces before and after the semicolon
            let (s, signal_name) = take_while(|c: char| c != ',' && c != ';')(remaining)?;
            let (s, _) = tag(",")(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, start_bit) = take_while(|c: char| c.is_numeric())(s)?;
            let start_bit = u8::from_str_radix(start_bit, 10).unwrap();
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag(";")(s)?;

            signals.push(LdfDiagnosticFrameSignal {
                signal_name: signal_name.to_string(),
                start_bit,
            });
            let (s, _) = skip_whitespace(s)?;
            remaining = s;
        }

        let (s, _) = tag("}")(remaining)?;
        let (s, _) = skip_whitespace(s)?;

        diagnostic_frames.push(LdfDiagnosticFrame {
            frame_name: frame_name.to_string(),
            frame_id,
            signals,
        });

        remaining = s;
    }
    let (remaining, _) = tag("}")(remaining)?;

    Ok((remaining, diagnostic_frames))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_diagnostic_frames() {
        let input = r#"
        Diagnostic_frames {
            MasterReq: 0x3C {
                MasterReqB0, 0 ;
                MasterReqB1, 8 ;
                MasterReqB2, 16 ;
                MasterReqB3, 24 ;
                MasterReqB4, 32 ;
                MasterReqB5, 40 ;
                MasterReqB6, 48 ;
                MasterReqB7, 56 ;
            }
            SlaveResp: 0x3D {
                SlaveRespB0, 0 ;
                SlaveRespB1, 8 ;
                SlaveRespB2, 16 ;
                SlaveRespB3, 24 ;
                SlaveRespB4, 32 ;
                SlaveRespB5, 40 ;
                SlaveRespB6, 48 ;
                SlaveRespB7, 56 ;
            }
        }
        "#;

        let (_, diagnostic_frames) = parse_ldf_diagnostic_frames(input).unwrap();
        assert_eq!(diagnostic_frames.len(), 2);
        assert_eq!(diagnostic_frames[0].frame_name, "MasterReq");
        assert_eq!(diagnostic_frames[0].frame_id, 0x3C);
        assert_eq!(diagnostic_frames[0].signals.len(), 8);
        assert_eq!(diagnostic_frames[0].signals[0].signal_name, "MasterReqB0");
        assert_eq!(diagnostic_frames[0].signals[0].start_bit, 0);
        assert_eq!(diagnostic_frames[0].signals[7].signal_name, "MasterReqB7");
        assert_eq!(diagnostic_frames[0].signals[7].start_bit, 56);
        assert_eq!(diagnostic_frames[1].frame_name, "SlaveResp");
        assert_eq!(diagnostic_frames[1].frame_id, 0x3D);
        assert_eq!(diagnostic_frames[1].signals.len(), 8);
        assert_eq!(diagnostic_frames[1].signals[0].signal_name, "SlaveRespB0");
        assert_eq!(diagnostic_frames[1].signals[0].start_bit, 0);
        assert_eq!(diagnostic_frames[1].signals[7].signal_name, "SlaveRespB7");
        assert_eq!(diagnostic_frames[1].signals[7].start_bit, 56);
    }
}
