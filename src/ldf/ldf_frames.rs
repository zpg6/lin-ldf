use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_while},
    AsChar, IResult,
};

/// `Frames` section of a LIN Description File (LDF)
/// ```text
/// Frames {
///    Frame1: 0, Master, 8 {
///       Signal1, 0 ;
///       Signal2, 1 ;
///    }
///    Frame2: 0x16, Slave1, 8 {
///       Signal3, 0 ;
///       Signal4, 1 ;
///    }
/// }
/// ```
pub struct LdfFrame {
    /// Frame name
    pub frame_name: String,

    /// Frame ID
    pub frame_id: u8,

    /// Frame publisher
    pub published_by: String,

    /// Frame length in bytes
    pub frame_size: u8,

    /// Frame signals
    pub signals: Vec<LdfFrameSignal>,
}

/// One signal section of a Frame in a LIN Description File (LDF).
/// ```text
/// Frames {
///    Frame1: 0, Master, 8 {
///       Signal1, 0 ;
///       Signal2, 1 ;
///    }
///    Frame2: 0x16, Slave1, 8 {
///       Signal3, 0 ;
///       Signal4, 1 ;
///    }
/// }
/// ```
pub struct LdfFrameSignal {
    /// Signal name
    pub signal_name: String,

    /// Signal start bit
    pub start_bit: u8,
}

/*
Frames {
   Frame1: 0, Master, 8 {
      Signal1, 0 ;
      Signal2, 1 ;
   }
   Frame2: 0x16, Slave1, 8 {
      Signal3, 0 ;
      Signal4, 1 ;
   }
}
*/

pub fn parse_ldf_frames(s: &str) -> IResult<&str, Vec<LdfFrame>> {
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Frames")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = skip_whitespace(s)?;

    let mut frames = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with("}") {
        let (s, frame) = parse_ldf_frame(remaining)?;
        let (s, _) = skip_whitespace(s)?;
        remaining = s;
        frames.push(frame);
    }

    let (remaining, _) = tag("}")(remaining)?;
    Ok((remaining, frames))
}

fn parse_ldf_frame(s: &str) -> IResult<&str, LdfFrame> {
    let (s, _) = skip_whitespace(s)?;
    let (s, frame_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, frame_id) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, published_by) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, frame_size) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;
    let (s, _) = skip_whitespace(s)?;

    let mut signals = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with("}") {
        let (s, signal) = parse_ldf_frame_signal(remaining)?;
        let (s, _) = skip_whitespace(s)?;
        remaining = s;
        signals.push(signal);
    }
    let (remaining, _) = tag("}")(remaining)?;

    Ok((
        remaining,
        LdfFrame {
            frame_name: frame_name.to_string(),
            frame_id: {
                if frame_id.starts_with("0x") {
                    u8::from_str_radix(&frame_id[2..], 16).unwrap()
                } else {
                    frame_id.parse().unwrap()
                }
            },
            published_by: published_by.to_string(),
            frame_size: frame_size.parse().unwrap(),
            signals,
        },
    ))
}

fn parse_ldf_frame_signal(s: &str) -> IResult<&str, LdfFrameSignal> {
    let (s, _) = skip_whitespace(s)?;
    let (s, signal_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, start_bit) = take_while(|c: char| c.is_numeric())(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    Ok((
        s,
        LdfFrameSignal {
            signal_name: signal_name.to_string(),
            start_bit: start_bit.parse().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_frames() {
        let input = r#"
        Frames {
            Frame1: 0, Master, 8 {
                Signal1, 0 ;
                Signal2, 10 ;
            }
            Frame2: 0x16, Slave1, 8 {
                Signal3, 0 ;
                Signal4, 10 ;
            }
        }
        "#;

        let (_, frames) = parse_ldf_frames(input).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].frame_name, "Frame1");
        assert_eq!(frames[0].frame_id, 0);
        assert_eq!(frames[0].published_by, "Master");
        assert_eq!(frames[0].frame_size, 8);
        assert_eq!(frames[0].signals.len(), 2);
        assert_eq!(frames[0].signals[0].signal_name, "Signal1");
        assert_eq!(frames[0].signals[0].start_bit, 0);
        assert_eq!(frames[0].signals[1].signal_name, "Signal2");
        assert_eq!(frames[0].signals[1].start_bit, 10);

        assert_eq!(frames[1].frame_name, "Frame2");
        assert_eq!(frames[1].frame_id, 0x16);
        assert_eq!(frames[1].published_by, "Slave1");
        assert_eq!(frames[1].frame_size, 8);
        assert_eq!(frames[1].signals.len(), 2);
        assert_eq!(frames[1].signals[0].signal_name, "Signal3");
        assert_eq!(frames[1].signals[0].start_bit, 0);
        assert_eq!(frames[1].signals[1].signal_name, "Signal4");
        assert_eq!(frames[1].signals[1].start_bit, 10);
    }
}
