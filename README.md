# LIN LDF parser using Rust's `nom` parser combinator library.

This library provides a parser for LIN (Local Interconnect Network) LDF (LIN Description File) files.

## Supported LDF sections (so far)

- [x] LIN_protocol_version
- [x] LIN_language_version
- [x] LIN_speed
- [x] (Channel_name)
- [x] Nodes
- [ ] (Node_composition)
- [x] Signals
- [x] (Diagnostic_signals)
- [x] Frames
- [ ] (Sporadic_frame)
- [ ] (Event_triggered_frame)
- [x] (Diagnostic_frames)
- [x] Node_attributes
- [x] Schedule_table
- [ ] (Signal_groups)
- [x] (Signal_encoding_type)
- [x] (Signal_representation)

(optional sections are in parentheses)

# Example

```rust
use lin_ldf::parse_ldf;

let ldf = r#"
LIN_description_file ;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;
Channel_name = "DB" ;

Nodes {
    Master: Master, 5 ms, 0.1 ms ;
    Slaves: Slave1, Slave2, Slave3 ;
}

Signals {
    Signal1: 10, 0, Master, Slave1 ;
    Signal2: 10, 0, Master, Slave1 ;
    Signal3: 10, 0, Slave1, Master ;
    Signal4: 10, 0, Slave1, Master ;
    Signal5: 2, 0, Slave1, Master ;
    Signal6: 1, 0, Slave1, Master ;
}
"#; // ... rest of the LDF file

let parsed_ldf = parse_ldf(ldf).expect("Failed to parse LDF file");
let lin_protocol_version = parsed_ldf.header.lin_protocol_version; // "2.1"
let jitter = parsed_ldf.nodes.master.jitter; // "0.1 ms"
```
