//! # LIN LDF parser using Rust's `nom` parser combinator library.
//!
//! This library provides a parser for LIN (Local Interconnect Network) LDF (LIN Description File) files.
//!
//! ## Supported LDF sections (so far)
//!  
//! - [x] LIN_protocol_version
//! - [x] LIN_language_version
//! - [x] LIN_speed
//! - [x] (Channel_name)
//! - [x] Nodes
//! - [ ] (Node_composition)
//! - [x] Signals
//! - [x] (Diagnostic_signals)
//! - [x] Frames
//! - [ ] (Sporadic_frame)
//! - [ ] (Event_triggered_frame)
//! - [x] (Diagnostic_frames)
//! - [x] Node_attributes
//! - [x] Schedule_table
//! - [ ] (Signal_groups)
//! - [x] (Signal_encoding_type)
//! - [x] (Signal_representation)
//!
//! (optional sections are in parentheses)
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
//! /* PARSING IGNORES BLOCK COMMENTS */
//!
//! Nodes {
//!     Master: Master, 5 ms, 0.1 ms ;
//!     Slaves: Slave1, Slave2, Slave3 ;
//! }
//!
//! Signals {
//!     Signal1: 10, 0, Master, Slave1 , Slave2 ;
//!     Signal2: 10, 0, Master, Slave1 ;
//!     Signal3: 10, 0, Slave1, Master ;
//!     Signal4: 10, 0, Slave1, Master ;
//!     Signal5: 2, 0, Slave1, Master ;
//!     Signal6: 1, 0, Slave1, Master ;
//! }
//!
//! Frames {
//!     Frame1: 0, Master, 8 {
//!         Signal1, 0 ;
//!         Signal2, 10 ;
//!     }
//!     Frame2: 0x16, Slave1, 8 {
//!         Signal3, 0 ;
//!         Signal4, 10 ;
//!     }
//! }
//!
//! Node_attributes {
//!    Slave1 {
//!        LIN_protocol = "2.1" ;
//!        configured_NAD = 0xB ;
//!        initial_NAD = 0xB ;
//!        product_id = 0x123, 0x4567, 8 ;
//!        response_error = Signal1 ;
//!        P2_min = 100 ms ;
//!        ST_min = 0 ms ;
//!        N_As_timeout = 1000 ms ;
//!        N_Cr_timeout = 1000 ms ;
//!        configurable_frames {
//!            Frame1 ;
//!            Frame2 ;  
//!        }
//!    }
//!    Slave2 {
//!        LIN_protocol = "2.1" ;
//!        configured_NAD = 0xC ;
//!        initial_NAD = 0xC ;
//!        product_id = 0x124, 0x4568, 0x66 ;
//!        response_error = Signal2 ;
//!        P2_min = 100 ms ;
//!        ST_min = 0 ms ;
//!        N_As_timeout = 1000 ms ;
//!        N_Cr_timeout = 1000 ms ;
//!        configurable_frames {
//!            Frame1 ;
//!            Frame2 ;
//!        }
//!    }
//! }
//!
//! Schedule_tables {
//!     AllFrames {
//!         Frame1 delay 10 ms ;
//!         Frame2 delay 10 ms ;
//!     }
//! }
//! "#;
//!
//! let parsed_ldf = parse_ldf(ldf).expect("Failed to parse LDF file");
//! let total_signal_count = parsed_ldf.signals.len(); // 6
//!
//! for frame in parsed_ldf.frames {
//!     println!("Frame: {}", frame.frame_name);
//!     for signal in frame.signals {
//!         println!("\tSignal: {}", signal.signal_name);
//!     }
//! }
//! ```

mod ldf;

pub use ldf::ldf_signal_encoding_types::LdfSignalEncodingTypeValue;
pub use ldf::LinLdf;

pub fn parse_ldf(ldf: &str) -> Result<LinLdf, &'static str> {
    ldf::LinLdf::parse(ldf)
}
