# LIN LDF parser using Rust's `nom` parser combinator library.

This library provides a parser for LIN (Local Interconnect Network) LDF (LIN Description File) files.

## Supported LIN versions (so far)

- [ ] LIN 1.3
- [ ] LIN 2.0
- ✅ LIN 2.1
- [ ] LIN 2.2

## Supported LDF sections (so far)

- ✅ LIN_protocol_version
- ✅ LIN_language_version
- ✅ LIN_speed
- [ ] (Channel_name)
- ✅ Nodes
- [ ] (Node_composition)
- ✅ Signals
- ✅ (Diagnostic_signals)
- ✅ Frames
- [ ] (Sporadic_frame)
- [ ] (Event_triggered_frame)
- ✅ (Diagnostic_frames)
- ✅ Node_attributes
- ✅ Schedule_table
- [ ] (Signal_groups)
- ✅ (Signal_encoding_type)
- ✅ (Signal_representation)

(optional sections are in parentheses)

# Example

```rust
use lin_ldf::parse_ldf;

let ldf = r#"
LIN_description_file ;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;

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
