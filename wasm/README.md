# lin-ldf

WebAssembly parser for LIN Description Files (.ldf) that describe automotive LIN bus networks.

## Demo

🚀 **[Try the live demo](https://zpg6.github.io/lin-ldf/)** - Interactive LDF parser in your browser!

## Installation

```bash
npm install lin-ldf
```

## API

- `parse_ldf_file(content: string)` - Parse LDF content and return JavaScript object
- `parse_ldf_to_json(content: string)` - Parse LDF content and return JSON string
- `parse_ldf_to_pretty_json(content: string)` - Parse LDF content and return formatted JSON
- `validate_ldf(content: string)` - Validate LDF content without full parsing
- `get_ldf_stats(content: string)` - Get statistics about the LDF file
- `ldf_from_json(json: string)` - Create LDF object from JSON string

## Usage

```javascript
import * as linLdf from "lin-ldf";

const ldfContent = `/* MY BLOCK COMMENT */
            
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
    Signal1: 10, 0, Master, Slave1 , Slave2 ;
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
}`;

try {
    // Parse LDF file and get JavaScript object
    const parsed = linLdf.parse_ldf_file(ldfContent);
    console.log("Parsed LDF:", parsed);

    // Or get as JSON string
    const jsonString = linLdf.parse_ldf_to_json(ldfContent);
    console.log("LDF as JSON:", jsonString);

    // Validate LDF content
    const isValid = linLdf.validate_ldf(ldfContent);
    console.log("Is valid:", isValid);

    // Get statistics
    const stats = linLdf.get_ldf_stats(ldfContent);
    console.log("LDF stats:", stats);
} catch (error) {
    console.error("Parse error:", error);
}
```

## License

MIT
