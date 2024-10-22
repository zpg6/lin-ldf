use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    IResult,
};

/// `Nodes` section of a LIN Description File (LDF) for LIN 2.1
/// ```text
/// Nodes {
///   Master: Master, 5 ms, 0.1 ms ;
///   Slaves: Slave1, Slave2, Slave3 ;
/// }
/// ```
pub struct LdfNodes {
    pub master: MasterNode,
    pub slaves: Vec<Node>,
}

/// Master node in the `Nodes` section of a LIN Description File (LDF) for LIN 2.1
pub struct MasterNode {
    // Identifier after the Master reserved word specifies the master node.
    // All identifiers must be unique within the LDF file.
    pub name: String,

    // `<time_base> ms`
    // The time_base value specifies the used time base in the master node to generate the
    // maximum allowed frame transfer time. The time base shall be specified in milliseconds.
    // Defined in section (2.4.1 TIME DEFINITIONS) of the LIN 2.1 specification.
    pub time_base: String,

    // `<jitter> ms`
    // Specifies the differences between the maximum and minimum delay from time base tick
    // to the header sending start point (falling edge of break field).
    // Defined in section (2.4.1 TIME DEFINITIONS) of the LIN 2.1 specification.
    pub jitter: String,
}

/// Slave node in the `Nodes` section of a LIN Description File (LDF) for LIN 2.1
pub struct Node {
    // All identifiers must be unique within the LDF file.
    pub name: String,
}

/*
Nodes {
  Master: Master, 5 ms, 0.1 ms ;
  Slaves: Slave1, Slave2, Slave3 ;
}
*/

pub fn parse_ldf_nodes(s: &str) -> IResult<&str, LdfNodes> {
    // `Nodes {` or `Nodes{` or ...
    // - May be any number of spaces before and after the "Nodes" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Nodes")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    // [Assumes no comments between the nodes tags]

    // `Master: Master, 5 ms, 0.1 ms ;` or `Master: Master, 5 ms, 0.1 ms;` or ...
    // - May be any number of spaces before and after the "Master" tag
    // - May be any number of spaces before and after the colon
    // - May be any number of spaces before and after the node name
    // - May be any number of spaces before and after the comma
    // - May be any number of spaces before and after the time value
    // - May be any number of spaces before and after the comma
    // - May be any number of spaces before and after the time value
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Master")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, master_node_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, time_base) = take_while(|c: char| c.is_numeric() || c == '.')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("ms")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, jitter) = take_while(|c: char| c.is_numeric() || c == '.')(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("ms")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(";")(s)?;

    let master = MasterNode {
        name: master_node_name.to_string(),
        time_base: time_base.to_string() + " ms",
        jitter: jitter.to_string() + " ms",
    };

    // [Assumes no comments between the nodes tags]

    // `Slaves: Slave1, Slave2, Slave3 ;` or `Slaves: Slave1, Slave2, Slave3;` or ...
    // - May be any number of spaces before and after the "Slaves" tag
    // - May be any number of spaces before and after the colon
    // - May be any number of spaces before and after each node name
    // - May be any number of spaces before and after the comma separating each node name
    // - May be any number of spaces before and after the semicolon
    // - May be any number of nodes (isolate that part first, then return a Vec<Node>)
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Slaves")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, slaves) = take_until(";")(s)?;
    let (s, _) = tag(";")(s)?;

    let slaves = slaves
        .split(",")
        .map(|slave| Node {
            name: slave.trim().to_string(),
        })
        .collect();

    // `}` or `} ;` or ...
    // - May be any number of spaces before and after the closing curly brace
    // - May be any number of spaces before and after the semicolon
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("}")(s)?;
    let (s, _) = skip_whitespace(s)?;

    Ok((s, LdfNodes { master, slaves }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = r#"
            Nodes {
                Master: Master, 5 ms, 0.1 ms ;
                Slaves: Slave1, Slave2, Slave3 ;
            }
        "#;

        let (_, header) = parse_ldf_nodes(s).unwrap();
        assert_eq!(header.master.name, "Master");
        assert_eq!(header.master.time_base, "5 ms");
        assert_eq!(header.master.jitter, "0.1 ms");
        assert_eq!(header.slaves.len(), 3);
        assert_eq!(header.slaves[0].name, "Slave1");
        assert_eq!(header.slaves[1].name, "Slave2");
        assert_eq!(header.slaves[2].name, "Slave3");
    }
}
