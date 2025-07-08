use crate::ldf::ldf_comment::skip_whitespace;
use nom::{
    bytes::complete::{tag, take_while},
    AsChar, IResult,
};

/// `Node_attributes` section of a LIN Description File (LDF)
/// ```text
/// Node_attributes {
///   Slave1{
///     LIN_protocol = "2.1" ;
///     configured_NAD = 0xB ;
///     initial_NAD = 0xB ;
///     product_id = 0x123, 0x4567, 8 ;
///     response_error = Signal1 ;
///     P2_min = 100 ms ;
///     ST_min = 0 ms ;
///     N_As_timeout = 1000 ms ;
///     N_Cr_timeout = 1000 ms ;
///     configurable_frames {
///        Frame1 ;
///        Frame2 ;  
///     }
///   }
///   Slave2{
///     LIN_protocol = "2.1" ;
///     configured_NAD = 0xC ;
///     initial_NAD = 0xC ;
///     product_id = 0x124, 0x4568, 0x66 ;
///     response_error = Signal2 ;
///     P2_min = 100 ms ;
///     ST_min = 0 ms ;
///     N_As_timeout = 1000 ms ;
///     N_Cr_timeout = 1000 ms ;
///     configurable_frames {
///        Frame1 ;
///        Frame2 ;
///     }
///   }
/// }
/// ```
pub struct LdfNodeAttributes {
    /// Node name
    pub node_name: String,

    /// LIN protocol version
    pub lin_protocol: String,

    /// Configured NAD
    pub configured_nad: u8,

    /// Initial NAD
    pub initial_nad: u8,

    /// Part of the product ID
    pub supplier_id: u16,

    /// Part of the product ID
    pub function_id: u16,

    /// Variant of the product ID
    pub variant: u8,

    /// Response error
    pub response_error: String,

    /// P2_min
    pub p2_min: String,

    /// ST_min
    pub st_min: String,

    /// N_As_timeout
    pub n_as_timeout: String,

    /// N_Cr_timeout
    pub n_cr_timeout: String,

    /// Configurable frames
    pub configurable_frames: Vec<String>,
}

pub fn parse_ldf_node_attributes(s: &str) -> IResult<&str, Vec<LdfNodeAttributes>> {
    // `Node_attributes {` or `Node_attributes{` or ...
    // - May be any number of spaces before and after the "Node_attributes" tag
    // - May be any number of spaces before and after the opening curly brace
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("Node_attributes")(s)?;
    let (s, _) = skip_whitespace(s)?;
    let (s, _) = tag("{")(s)?;

    let mut node_attributes = Vec::new();
    let mut remaining = s;

    while !remaining.starts_with('}') {
        // `Slave1{` or `Slave1 {` or ...
        // - May be any number of spaces before and after the node name
        // - May be any number of spaces before and after the opening curly brace
        let (s, _) = skip_whitespace(remaining)?;
        let (s, node_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("{")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("LIN_protocol")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("\"")(s)?;
        let (s, lin_protocol) = take_while(|c: char| c.is_numeric() || c == '.')(s)?;
        let (s, _) = tag("\"")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("configured_NAD")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, configured_nad) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("initial_NAD")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, initial_nad) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("product_id")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, supplier_id) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(",")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, function_id) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(",")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, variant) = take_while(|c: char| c.is_hex_digit() || c == 'x')(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        // response_error is the optional section
        let mut response_error = "";
        let res;
        if s.starts_with("response_error") {
            let (s, _) = tag("response_error")(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag("=")(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, resp_err) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
            response_error = resp_err;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag(";")(s)?;
            res = s;
        } else {
            res = s;
        }
        let s = res;
        
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("P2_min")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, p2_min) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("ms")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("ST_min")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, st_min) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("ms")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("N_As_timeout")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, n_as_timeout) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("ms")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("N_Cr_timeout")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("=")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, n_cr_timeout) = take_while(|c: char| c.is_numeric())(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("ms")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag(";")(s)?;

        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("configurable_frames")(s)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("{")(s)?;
        let (s, _) = skip_whitespace(s)?;

        remaining = s;
        let mut configurable_frames = Vec::new();

        while !remaining.starts_with('}') {
            let (s, _) = skip_whitespace(remaining)?;
            let (s, frame_name) = take_while(|c: char| c.is_alphanumeric() || c == '_')(s)?;
            let (s, _) = skip_whitespace(s)?;
            let (s, _) = tag(";")(s)?;
            let (s, _) = skip_whitespace(s)?;

            configurable_frames.push(frame_name.to_string());

            remaining = s;
        }

        let (s, _) = tag("}")(remaining)?;
        let (s, _) = skip_whitespace(s)?;
        let (s, _) = tag("}")(s)?;
        let (s, _) = skip_whitespace(s)?;

        let node = LdfNodeAttributes {
            node_name: node_name.to_string(),
            lin_protocol: lin_protocol.to_string(),
            configured_nad: u8::from_str_radix(&configured_nad[2..], 16).unwrap(),
            initial_nad: u8::from_str_radix(&initial_nad[2..], 16).unwrap(),
            supplier_id: u16::from_str_radix(&supplier_id[2..], 16).unwrap(),
            function_id: u16::from_str_radix(&function_id[2..], 16).unwrap(),
            variant: {
                if variant.starts_with("0x") {
                    u8::from_str_radix(&variant[2..], 16).unwrap()
                } else {
                    variant.parse().unwrap()
                }
            },
            response_error: response_error.to_string(),
            p2_min: p2_min.to_string() + " ms",
            st_min: st_min.to_string() + " ms",
            n_as_timeout: n_as_timeout.to_string() + " ms",
            n_cr_timeout: n_cr_timeout.to_string() + " ms",
            configurable_frames: configurable_frames,
        };

        node_attributes.push(node);
        remaining = s;
    }

    let (remaining, _) = tag("}")(remaining)?;

    Ok((remaining, node_attributes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ldf_node_attributes() {
        let input = r#"
            Node_attributes {
                Slave1{
                    LIN_protocol = "2.1" ;
                    configured_NAD = 0xB ;
                    initial_NAD = 0xB ;
                    product_id = 0x123, 0x4567, 8 ;
                    response_error = Signal1 ;
                    P2_min = 100 ms ;
                    ST_min = 0 ms ;
                    N_As_timeout = 1000 ms ;
                    N_Cr_timeout = 1000 ms ;
                    configurable_frames {
                        Frame1 ;
                        Frame2 ;  
                    }
                }
                Slave2{
                    LIN_protocol = "2.1" ;
                    configured_NAD = 0xC ;
                    initial_NAD = 0xC ;
                    product_id = 0x124, 0x4568, 0x66 ;
                    response_error = Signal2 ;
                    P2_min = 100 ms ;
                    ST_min = 0 ms ;
                    N_As_timeout = 1000 ms ;
                    N_Cr_timeout = 1000 ms ;
                    configurable_frames {
                        Frame1 ;
                        Frame2 ;
                    }
                }
            }
        "#;

        let (_, node_attributes) = parse_ldf_node_attributes(input).unwrap();

        assert_eq!(node_attributes.len(), 2);

        assert_eq!(node_attributes[0].node_name, "Slave1");
        assert_eq!(node_attributes[0].lin_protocol, "2.1");
        assert_eq!(node_attributes[0].configured_nad, 0xB);
        assert_eq!(node_attributes[0].initial_nad, 0xB);
        assert_eq!(node_attributes[0].supplier_id, 0x123);
        assert_eq!(node_attributes[0].function_id, 0x4567);
        assert_eq!(node_attributes[0].variant, 8);
        assert_eq!(node_attributes[0].response_error, "Signal1");
        assert_eq!(node_attributes[0].p2_min, "100 ms");
        assert_eq!(node_attributes[0].st_min, "0 ms");
        assert_eq!(node_attributes[0].n_as_timeout, "1000 ms");
        assert_eq!(node_attributes[0].n_cr_timeout, "1000 ms");
        assert_eq!(node_attributes[0].configurable_frames.len(), 2);
        assert_eq!(node_attributes[0].configurable_frames[0], "Frame1");
        assert_eq!(node_attributes[0].configurable_frames[1], "Frame2");

        assert_eq!(node_attributes[1].node_name, "Slave2");
        assert_eq!(node_attributes[1].lin_protocol, "2.1");
        assert_eq!(node_attributes[1].configured_nad, 0xC);
        assert_eq!(node_attributes[1].initial_nad, 0xC);
        assert_eq!(node_attributes[1].supplier_id, 0x124);
        assert_eq!(node_attributes[1].function_id, 0x4568);
        assert_eq!(node_attributes[1].variant, 0x66);
        assert_eq!(node_attributes[1].response_error, "Signal2");
        assert_eq!(node_attributes[1].p2_min, "100 ms");
        assert_eq!(node_attributes[1].st_min, "0 ms");
        assert_eq!(node_attributes[1].n_as_timeout, "1000 ms");
        assert_eq!(node_attributes[1].n_cr_timeout, "1000 ms");
        assert_eq!(node_attributes[1].configurable_frames.len(), 2);
        assert_eq!(node_attributes[1].configurable_frames[0], "Frame1");
        assert_eq!(node_attributes[1].configurable_frames[1], "Frame2");
    }
}
