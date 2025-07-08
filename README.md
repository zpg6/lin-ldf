# lin-ldf

<br>
<a href="https://crates.io/crates/lin-ldf">
    <img src="https://img.shields.io/crates/v/lin-ldf.svg" alt="Crates.io">
</a>
<a href="https://docs.rs/lin-ldf">
    <img src="https://docs.rs/lin-ldf/badge.svg" alt="Documentation">
</a>
<a href="">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg">
</a>
<br><br>

LIN Description File (.ldf) parser using Rust's `nom` parser combinator library. LIN is an automotive serial protocol used for communication between ECUs in a vehicle. The LDF file is used to describe the network configuration, including the different nodes and signals sent between them.

> [!WARNING]
> This crate may not be suitable for production use. It was written as hands-on learning exercise of a well-documented specification. It may not cover all edge cases or vendor-specific implementations. Please use with caution.

This parser attempts to be a simple reflection of the well-documented instructions from the LIN specification: https://www.lin-cia.org/fileadmin/microsites/lin-cia.org/resources/documents/LIN_2.2A.pdf

## Alternatives

There are some existing alternatives that have been around for years if you need something more robust:

- https://github.com/c4deszes/ldfparser (Python) (**most popular**)
- https://github.com/uCAN-LIN/LinUSBConverter/tree/master/python_lib (Python)
- https://github.com/TrippW/LDF-Parser (Python)
- https://bitbucket.org/tobylorenz/lin/src/master/ (C++)

Here are more recent alternatives:

- https://github.com/dragonlock2/autodbconv (Rust)

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

## Other features

- [x] Serde
- [ ] WASM sub-crate
- [ ] WASM NPM package

> [!TIP]
> It would be difficult to plan for all edge cases in vendor-specific implementations, so this just tries to follow the specification. CONTRIBUTIONS ARE WELCOMED! You can always open an issue or a PR if you find something that doesn't work as expected - but be sure to anonymize the data if it's proprietary (or just don't share it).

# Example

Here's how you can parse an LDF file and access the parsed data for your use case:

```rust
use lin_ldf::parse_ldf;

let ldf = r#"
LIN_description_file ;
LIN_protocol_version = "2.1" ;
LIN_language_version = "2.1" ;
LIN_speed = 19.2 kbps ;

/* PARSING IGNORES BLOCK COMMENTS */
// AND LINE COMMENTS

Nodes {
    Master: Master, 5 ms, 0.1 ms ;
    Slaves: Slave1, Slave2, Slave3 ;
}

Signals {
    Signal1: 10, 0, Master, Slave1 , Slave2 ;
    Signal2: 10, 0, Master, Slave1 ;
    Signal3: 10, 0, Slave1, Master ;
    Signal4: 10, 0, Slave1, Master ;
    Signal5: 2, 0, Slave1, Master ;
    Signal6: 1, 0, Slave1, Master ;
}

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

Node_attributes {
   Slave1 {
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
   Slave2 {
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
"#;

let parsed_ldf = parse_ldf(ldf).expect("Failed to parse LDF file");

println!("LIN Version: {}", parsed_ldf.lin_protocol_version); // 2.1
println!("LIN Speed: {}", parsed_ldf.lin_speed); // 19200

for frame in parsed_ldf.frames {
    println!("Frame: `{}` is {} bytes long", frame.frame_name, frame.frame_size);
    for signal in frame.signals {
        println!("\tSignal: `{}` at bit position {}", signal.signal_name, signal.start_bit);
    }
}
```
