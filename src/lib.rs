//! # LIN LDF parser using Rust's `nom` parser combinator library.
//!
//! This library provides a parser for LIN (Local Interconnect Network) LDF (LIN Description File) files.
//!
//! ## Supported LIN versions (so far)
//!
//! - [ ] LIN 1.3
//! - [ ] LIN 2.0
//! - ✅  LIN 2.1
//! - [ ] LIN 2.2
//!
//! ## Supported LDF sections (so far)
//!  
//! - ✅  LIN_protocol_version
//! - ✅  LIN_language_version
//! - ✅  LIN_speed
//! - [ ] (Channel_name)
//! - ✅  Nodes
//! - [ ] (Node_composition)
//! - ✅  Signals
//! - ✅  (Diagnostic_signals)
//! - ✅  Frames
//! - [ ] (<Sporadic_frame_def>)
//! - [ ] (<Event_triggered_frame_def>)
//! - ✅  (Diagnostic_frames)
//! - ✅  <Node_attributes_def>
//! - ✅  <Schedule_table_def>
//! - [ ] (<Signal_groups_def>)
//! - ✅  (<Signal_encoding_type_def>)
//! - ✅  (<Signal_representation_def>)
//!
//! # Example
//!
//! ```
//! use lin_ldf::parse_ldf;
//!
//! let ldf = r#"
//! LIN_description_file ;
//! LIN_protocol_version = "2.1" ;
//! LIN_language_version = "2.1" ;
//! LIN_speed = 19.2 kbps ;
//!
//! Nodes {
//!     Master: Master, 5 ms, 0.1 ms ;
//!     Slaves: Slave1, Slave2, Slave3 ;
//! }
//!
//! Signals {
//!     Signal1: 10, 0, Master, Slave1 ;
//!     Signal2: 10, 0, Master, Slave1 ;
//!     Signal3: 10, 0, Slave1, Master ;
//!     Signal4: 10, 0, Slave1, Master ;
//!     Signal5: 2, 0, Slave1, Master ;
//!     Signal6: 1, 0, Slave1, Master ;
//! }
//!
//! Diagnostic_signals {
//!     MasterReqB0: 8, 0 ;
//!     MasterReqB1: 8, 0 ;
//!     MasterReqB2: 8, 0 ;
//!     MasterReqB3: 8, 0 ;
//!     MasterReqB4: 8, 0 ;
//!     MasterReqB5: 8, 0 ;
//! }
//!
//! Frames {
//!   Frame1: 0, Master, 8 {
//!     Signal1, 0 ;
//!     Signal2, 10 ;
//!   }
//!   Frame2: 1, Slave1, 8 {
//!       Signal3, 0 ;
//!       Signal4, 10 ;
//!   }
//! }
//!
//! Node_attributes {
//!   Slave1{
//!       LIN_protocol = "2.1" ;
//!       configured_NAD = 0xB ;
//!       initial_NAD = 0xB ;
//!       product_id = 0x123, 0x4567, 8 ;
//!       response_error = Signal1 ;
//!       P2_min = 100 ms ;
//!       ST_min = 0 ms ;
//!       N_As_timeout = 1000 ms ;
//!       N_Cr_timeout = 1000 ms ;
//!       configurable_frames {
//!           Frame1 ;
//!           Frame2 ;  
//!       }
//!   }
//!   Slave2{
//!       LIN_protocol = "2.1" ;
//!       configured_NAD = 0xC ;
//!       initial_NAD = 0xC ;
//!       product_id = 0x124, 0x4568, 6 ;
//!       response_error = Signal2 ;
//!       P2_min = 100 ms ;
//!       ST_min = 0 ms ;
//!       N_As_timeout = 1000 ms ;
//!       N_Cr_timeout = 1000 ms ;
//!       configurable_frames {
//!           Frame1 ;
//!           Frame2 ;
//!       }
//!   }
//! }
//!
//! Schedule_tables {
//!     AllFrames {
//!         Frame1 delay 10 ms ;
//!         Frame2 delay 10 ms ;
//!     }
//! }
//!
//! Signal_encoding_types {
//!     ENC_BOOL {
//!         logical_value, 0, "FALSE" ;
//!         logical_value, 1, "TRUE" ;
//!     }
//!     ENC_TEMP {
//!         physical_value, 0, 1023, 0.2, -40, "degC" ;
//!     }
//!     ENC_RPM {
//!         physical_value, 0, 1023, 10, 0, "rpm" ;
//!     }
//! }
//!
//! Signal_representation {
//!     ENC_BOOL: Signal1, Signal2 ;
//!     ENC_TEMP: Signal3, Signal4 ;
//!     ENC_RPM: Signal5, Signal6 ;
//! }
//! "#; // ... rest of the LDF file
//!
//! let parsed_ldf = parse_ldf(ldf).expect("Failed to parse LDF file");
//! let lin_protocol_version = parsed_ldf.header.lin_protocol_version; // "2.1"
//! let jitter = parsed_ldf.nodes.master.jitter; // "0.1 ms"
//!
//! ```

mod ldf;

pub use ldf::ldf_signal_encoding_types::LdfSignalEncodingTypeValue;
pub use ldf::LinLdf;

pub fn parse_ldf(ldf: &str) -> Result<LinLdf, &'static str> {
    ldf::LinLdf::parse(ldf)
}
