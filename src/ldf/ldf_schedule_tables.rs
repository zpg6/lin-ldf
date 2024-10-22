use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_while},
    IResult,
};

/// `Schedule_tables` section of a LIN Description File (LDF)
/// ```text
/// Schedule_tables {
///   AllFrames {
///      Frame1 delay 10 ms ;
///      Frame2 delay 10 ms ;
///   }
/// }
/// ```
pub struct LdfScheduleTable {
    /// Schedule table name.
    /// All schedule_table_name identifiers shall be unique within the schedule table identifier set
    pub schedule_table_name: String,

    /// Frame delays
    pub frame_delays: Vec<LdfFrameDelay>,
}

/// Frame delay in a schedule table in a LIN Description File (LDF).
/// ```text
/// Frame1 delay 10 ms ;
/// ```
pub struct LdfFrameDelay {
    /// Frame name
    pub frame_name: String,

    /// Frame delay in milliseconds
    pub frame_time: f32,
}

/*
Schedule_tables {
  AllFrames {
     Frame1 delay 10 ms ;
     Frame2   delay 10.0 ms ;
  }
}
*/

pub fn parse_ldf_schedule_tables(s: &str) -> IResult<&str, Vec<LdfScheduleTable>> {
    // `Schedule_tables {` or `Schedule_tables{` or ...
    // - May be any number of spaces before and after the "Schedule_tables" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Schedule_tables")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    let mut schedule_tables = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `AllFrames: {` or `AllFrames:{` or ...
        // - May be any number of spaces before and after the "AllFrames" tag
        // - May be any number of spaces before and after the opening curly brace
        let (s, _) = skip_whitespace(remaining)?;
        let (s, schedule_table_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("{")(s)?;

        let mut frame_delays = Vec::new();
        remaining = s;

        while !remaining.starts_with('}') {
            // `Frame1 delay 10 ms ;` or `Frame1 delay 10 ms;` or ...
            // - May be any number of spaces before and after the frame name
            // - May be any number of spaces before and after the "delay" tag
            // - May be any number of spaces before and after the frame delay
            // - May be any number of spaces before and after the "ms" tag
            // - May be any number of spaces before and after the semicolon
            let (s, _) = skip_whitespace(remaining)?;
            let (s, frame_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag("delay")(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, frame_time) = take_while(|c: char| c.is_numeric() || c == '.')(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag("ms")(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag(";")(s)?;
            let (s, _) = skip_whitespace(s)?;

            frame_delays.push(LdfFrameDelay {
                frame_name: frame_name.to_string(),
                frame_time: frame_time.parse().unwrap(),
            });

            remaining = s;
        }

        let (s, _) = tag("}")(remaining)?;
        let (s, _) = skip_whitespace(s)?;

        schedule_tables.push(LdfScheduleTable {
            schedule_table_name: schedule_table_name.to_string(),
            frame_delays,
        });

        remaining = s;
    }

    let (remaining, _) = tag("}")(remaining)?;

    Ok((remaining, schedule_tables))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_schedule_tables() {
        let input = r#"
            Schedule_tables {
                AllFrames {
                    Frame1 delay 10 ms ;
                    Frame2   delay 10.0 ms ;
                }
            }
        "#;

        let (_, schedule_tables) = parse_ldf_schedule_tables(input).unwrap();

        assert_eq!(schedule_tables.len(), 1);

        let schedule_table = &schedule_tables[0];
        assert_eq!(schedule_table.schedule_table_name, "AllFrames");
        assert_eq!(schedule_table.frame_delays.len(), 2);

        let frame_delay = &schedule_table.frame_delays[0];
        assert_eq!(frame_delay.frame_name, "Frame1");
        assert_eq!(frame_delay.frame_time, 10.0);

        let frame_delay = &schedule_table.frame_delays[1];
        assert_eq!(frame_delay.frame_name, "Frame2");
        assert_eq!(frame_delay.frame_time, 10.0);
    }
}
