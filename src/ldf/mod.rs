pub mod ldf_comment;
pub mod ldf_diagnostic_frames;
pub mod ldf_diagnostic_signals;
pub mod ldf_frames;
pub mod ldf_header;
pub mod ldf_node_attributes;
pub mod ldf_nodes;
pub mod ldf_schedule_tables;
pub mod ldf_signal_encoding_types;
pub mod ldf_signal_representation;
pub mod ldf_signals;

use crate::ldf::ldf_comment::skip_whitespace;
use crate::ldf::ldf_diagnostic_frames::{parse_ldf_diagnostic_frames, LdfDiagnosticFrame};
use crate::ldf::ldf_diagnostic_signals::{parse_ldf_diagnostic_signals, LdfDiagnosticSignal};
use crate::ldf::ldf_frames::{parse_ldf_frames, LdfFrame};
use crate::ldf::ldf_header::{parse_ldf_header, LdfHeader};
use crate::ldf::ldf_node_attributes::{parse_ldf_node_attributes, LdfNodeAttributes};
use crate::ldf::ldf_nodes::{parse_ldf_nodes, LdfNodes};
use crate::ldf::ldf_schedule_tables::{parse_ldf_schedule_tables, LdfScheduleTable};
use crate::ldf::ldf_signal_encoding_types::{parse_ldf_signal_encoding_types, LdfSignalEncodingType};
use crate::ldf::ldf_signal_representation::{parse_ldf_signal_representation, LdfSignalRepresentation};
use crate::ldf::ldf_signals::{parse_ldf_signals, LdfSignal};

pub struct LinLdf {
    pub header: LdfHeader,
    pub nodes: LdfNodes,
    pub signals: Vec<LdfSignal>,
    pub diagnostic_signals: Vec<LdfDiagnosticSignal>,
    pub frames: Vec<LdfFrame>,
    pub diagnostic_frames: Vec<LdfDiagnosticFrame>,
    pub node_attributes: Vec<LdfNodeAttributes>,
    pub schedule_tables: Vec<LdfScheduleTable>,
    pub signal_encoding_types: Vec<LdfSignalEncodingType>,
    pub signal_representations: Vec<LdfSignalRepresentation>,
}

impl LinLdf {
    /// <LIN_description_file> ::=
    /// ```text
    /// LIN_description_file ;
    /// <LIN_protocol_version_def>
    /// <LIN_language_version_def>
    /// <LIN_speed_def>
    /// (<Channel_name_def>)
    /// <Node_def>
    /// (<Node_composition_def>)
    /// <Signal_def>
    /// (<Diag_signal_def>)
    /// <Frame_def>
    /// (<Sporadic_frame_def>)
    /// (<Event_triggered_frame_def>)
    /// (<Diag_frame_def>)
    /// <Node_attributes_def>
    /// <Schedule_table_def>
    /// (<Signal_groups_def>)
    /// (<Signal_encoding_type_def>)
    /// (<Signal_representation_def>)
    /// ```
    pub fn parse(s: &str) -> Result<LinLdf, &'static str> {
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, header) = parse_ldf_header(s).map_err(|_| "Failed to parse header")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, nodes) = parse_ldf_nodes(s).map_err(|_| "Failed to parse Nodes section")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, signals) = parse_ldf_signals(s).map_err(|_| "Failed to parse Signals section (required)")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, diagnostic_signals) = parse_ldf_diagnostic_signals(s).unwrap_or((s, Vec::new()));
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, frames) = parse_ldf_frames(s).map_err(|_| "Failed to parse Frames section")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, diagnostic_frames) = parse_ldf_diagnostic_frames(s).unwrap_or((s, Vec::new()));
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, node_attributes) =
            parse_ldf_node_attributes(s).map_err(|_| "Failed to parse Node_attributes section")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, schedule_tables) =
            parse_ldf_schedule_tables(s).map_err(|_| "Failed to parse Schedule_tables section")?;
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (s, signal_encoding_types) = parse_ldf_signal_encoding_types(s).unwrap_or((s, Vec::new()));
        let (s, _) = skip_whitespace(s).map_err(|_| "Failed to skip whitespace and comments")?;
        let (_, signal_representations) = parse_ldf_signal_representation(s).unwrap_or((s, Vec::new()));

        Ok(LinLdf {
            header,
            nodes,
            signals,
            diagnostic_signals,
            frames,
            diagnostic_frames,
            node_attributes,
            schedule_tables,
            signal_encoding_types,
            signal_representations,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ldf::ldf_signals::LdfSignalInitValue;

    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
            /* MY BLOCK COMMENT */
            
            LIN_description_file ; 
            LIN_protocol_version = "2.1" ; 
            LIN_language_version = "2.1" ; 
            LIN_speed = 19.2 kbps ;
            
            Nodes {
                Master: Master, 5 ms, 0.1 ms ;
                Slaves: Slave1, Slave2, Slave3 ;
            }

            // MY LINE COMMENT

            Signals {
                Signal1: 10, 0, Master, Slave1 ;
                Signal2: 10, 0, Master, Slave1 ;
                Signal3: 10, 0, Master, Slave1 ;
                Signal4: 10, 0, Slave1, Master ;
                Signal5: 2, 0, Slave1, Master ;
                Signal6: 1, 0, Slave1, Master ;
            }

            Diagnostic_signals {
                MasterReqB0: 8, 0 ;   /* MID SECTION COMMENT */
                MasterReqB1: 8, 0 ;
                MasterReqB2: 8, 0 ;
                MasterReqB3: 8, 0 ;
                MasterReqB4: 8, 0 ;
                MasterReqB5: 8, 0 ;   /* MID SECTION COMMENT */
            }

            Frames {
                Frame1: 0, Master, 8 {
                    Signal1, 0 ;      /* MID SECTION COMMENT */
                    Signal2, 10 ;
                }
                Frame2: 0x16, Slave1, 8 {
                    Signal3, 0 ;
                    Signal4, 10 ;
                }
            }

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

            Schedule_tables {
                AllFrames {
                    Frame1 delay 10 ms ;
                    Frame2 delay 10 ms ;
                }
            }

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

            Signal_representation {
                ENC_BOOL: Signal1, Signal2 ;
                ENC_TEMP: Signal3, Signal4 ;
                ENC_RPM: Signal5, Signal6 ;
            }
        "#;

        let ldf = LinLdf::parse(input).unwrap();

        // Header
        assert_eq!(ldf.header.lin_protocol_version, "2.1");
        assert_eq!(ldf.header.lin_language_version, "2.1");
        assert_eq!(ldf.header.lin_speed, "19.2");

        // Nodes
        assert_eq!(ldf.nodes.master.name, "Master");
        assert_eq!(ldf.nodes.master.time_base, "5 ms");
        assert_eq!(ldf.nodes.master.jitter, "0.1 ms");
        assert_eq!(ldf.nodes.slaves.len(), 3);
        assert_eq!(ldf.nodes.slaves[0].name, "Slave1");
        assert_eq!(ldf.nodes.slaves[1].name, "Slave2");
        assert_eq!(ldf.nodes.slaves[2].name, "Slave3");

        // Signals
        assert_eq!(ldf.signals.len(), 6);
        assert_eq!(ldf.signals[0].name, "Signal1");
        assert_eq!(ldf.signals[0].signal_size, 10);
        assert_eq!(ldf.signals[0].init_value, LdfSignalInitValue::Scalar(0));
        assert_eq!(ldf.signals[0].published_by, "Master");
        assert_eq!(ldf.signals[0].subscribed_by, "Slave1");
        assert_eq!(ldf.signals[5].name, "Signal6");
        assert_eq!(ldf.signals[5].signal_size, 1);
        assert_eq!(ldf.signals[5].init_value, LdfSignalInitValue::Scalar(0));
        assert_eq!(ldf.signals[5].published_by, "Slave1");
        assert_eq!(ldf.signals[5].subscribed_by, "Master");

        // Diagnostic signals
        assert_eq!(ldf.diagnostic_signals.len(), 6);
        assert_eq!(ldf.diagnostic_signals[0].name, "MasterReqB0");
        assert_eq!(ldf.diagnostic_signals[0].length, 8);
        assert_eq!(ldf.diagnostic_signals[0].init_value, 0);
        assert_eq!(ldf.diagnostic_signals[5].name, "MasterReqB5");
        assert_eq!(ldf.diagnostic_signals[5].length, 8);
        assert_eq!(ldf.diagnostic_signals[5].init_value, 0);

        // Frames
        assert_eq!(ldf.frames.len(), 2);
        assert_eq!(ldf.frames[0].frame_name, "Frame1");
        assert_eq!(ldf.frames[0].frame_id, 0);
        assert_eq!(ldf.frames[0].published_by, "Master");
        assert_eq!(ldf.frames[0].frame_size, 8);
        assert_eq!(ldf.frames[0].signals.len(), 2);
        assert_eq!(ldf.frames[0].signals[0].signal_name, "Signal1");
        assert_eq!(ldf.frames[0].signals[0].start_bit, 0);
        assert_eq!(ldf.frames[0].signals[1].signal_name, "Signal2");
        assert_eq!(ldf.frames[0].signals[1].start_bit, 10);
        assert_eq!(ldf.frames[1].frame_name, "Frame2");
        assert_eq!(ldf.frames[1].frame_id, 0x16);
        assert_eq!(ldf.frames[1].published_by, "Slave1");
        assert_eq!(ldf.frames[1].frame_size, 8);
        assert_eq!(ldf.frames[1].signals.len(), 2);
        assert_eq!(ldf.frames[1].signals[0].signal_name, "Signal3");
        assert_eq!(ldf.frames[1].signals[0].start_bit, 0);
        assert_eq!(ldf.frames[1].signals[1].signal_name, "Signal4");
        assert_eq!(ldf.frames[1].signals[1].start_bit, 10);

        // Diagnostic frames
        assert_eq!(ldf.diagnostic_frames.len(), 2);
        assert_eq!(ldf.diagnostic_frames[0].frame_name, "MasterReq");
        assert_eq!(ldf.diagnostic_frames[0].frame_id, 0x3C);
        assert_eq!(ldf.diagnostic_frames[0].signals.len(), 8);
        assert_eq!(ldf.diagnostic_frames[0].signals[0].signal_name, "MasterReqB0");
        assert_eq!(ldf.diagnostic_frames[0].signals[0].start_bit, 0);
        assert_eq!(ldf.diagnostic_frames[0].signals[7].signal_name, "MasterReqB7");
        assert_eq!(ldf.diagnostic_frames[0].signals[7].start_bit, 56);
        assert_eq!(ldf.diagnostic_frames[1].frame_name, "SlaveResp");
        assert_eq!(ldf.diagnostic_frames[1].frame_id, 0x3D);
        assert_eq!(ldf.diagnostic_frames[1].signals.len(), 8);
        assert_eq!(ldf.diagnostic_frames[1].signals[0].signal_name, "SlaveRespB0");
        assert_eq!(ldf.diagnostic_frames[1].signals[0].start_bit, 0);
        assert_eq!(ldf.diagnostic_frames[1].signals[7].signal_name, "SlaveRespB7");
        assert_eq!(ldf.diagnostic_frames[1].signals[7].start_bit, 56);

        // Node attributes
        assert_eq!(ldf.node_attributes.len(), 2);
        assert_eq!(ldf.node_attributes[0].node_name, "Slave1");
        assert_eq!(ldf.node_attributes[0].lin_protocol, "2.1");
        assert_eq!(ldf.node_attributes[0].configured_nad, 0xB);
        assert_eq!(ldf.node_attributes[0].initial_nad, 0xB);
        assert_eq!(ldf.node_attributes[0].supplier_id, 0x123);
        assert_eq!(ldf.node_attributes[0].function_id, 0x4567);
        assert_eq!(ldf.node_attributes[0].variant, 8);
        assert_eq!(ldf.node_attributes[0].response_error, "Signal1");
        assert_eq!(ldf.node_attributes[0].p2_min, "100 ms");
        assert_eq!(ldf.node_attributes[0].st_min, "0 ms");
        assert_eq!(ldf.node_attributes[0].n_as_timeout, "1000 ms");
        assert_eq!(ldf.node_attributes[0].n_cr_timeout, "1000 ms");
        assert_eq!(ldf.node_attributes[0].configurable_frames.len(), 2);
        assert_eq!(ldf.node_attributes[0].configurable_frames[0], "Frame1");
        assert_eq!(ldf.node_attributes[0].configurable_frames[1], "Frame2");
        assert_eq!(ldf.node_attributes[1].node_name, "Slave2");
        assert_eq!(ldf.node_attributes[1].lin_protocol, "2.1");
        assert_eq!(ldf.node_attributes[1].configured_nad, 0xC);
        assert_eq!(ldf.node_attributes[1].initial_nad, 0xC);
        assert_eq!(ldf.node_attributes[1].supplier_id, 0x124);
        assert_eq!(ldf.node_attributes[1].function_id, 0x4568);
        assert_eq!(ldf.node_attributes[1].variant, 0x66);
        assert_eq!(ldf.node_attributes[1].response_error, "Signal2");
        assert_eq!(ldf.node_attributes[1].p2_min, "100 ms");
        assert_eq!(ldf.node_attributes[1].st_min, "0 ms");
        assert_eq!(ldf.node_attributes[1].n_as_timeout, "1000 ms");
        assert_eq!(ldf.node_attributes[1].n_cr_timeout, "1000 ms");
        assert_eq!(ldf.node_attributes[1].configurable_frames.len(), 2);
        assert_eq!(ldf.node_attributes[1].configurable_frames[0], "Frame1");
        assert_eq!(ldf.node_attributes[1].configurable_frames[1], "Frame2");

        // Schedule tables
        assert_eq!(ldf.schedule_tables.len(), 1);
        assert_eq!(ldf.schedule_tables[0].schedule_table_name, "AllFrames");
        assert_eq!(ldf.schedule_tables[0].frame_delays.len(), 2);
        assert_eq!(ldf.schedule_tables[0].frame_delays[0].frame_name, "Frame1");
        assert_eq!(ldf.schedule_tables[0].frame_delays[0].frame_time, 10);
        assert_eq!(ldf.schedule_tables[0].frame_delays[1].frame_name, "Frame2");
        assert_eq!(ldf.schedule_tables[0].frame_delays[1].frame_time, 10);

        // Signal encoding types
        assert_eq!(ldf.signal_encoding_types.len(), 3);
        assert_eq!(ldf.signal_encoding_types[0].encoding_type_name, "ENC_BOOL");
        assert_eq!(ldf.signal_encoding_types[0].encoding_type_values.len(), 2);

        // Signal representations
        assert_eq!(ldf.signal_representations.len(), 3);
        assert_eq!(ldf.signal_representations[0].encoding_type_name, "ENC_BOOL");
        assert_eq!(ldf.signal_representations[0].signal_names.len(), 2);
        assert_eq!(ldf.signal_representations[0].signal_names[0], "Signal1");
        assert_eq!(ldf.signal_representations[0].signal_names[1], "Signal2");
    }
}
