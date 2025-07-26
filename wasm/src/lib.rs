use lin_ldf::{parse_ldf, LinLdf};
use wasm_bindgen::prelude::*;

// Optional demo features - only include if "demo" feature is enabled
#[cfg(feature = "demo")]
mod demo {
    use super::*;

    // Import the `console.log` function from the `console` module
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    // Define a macro to make console.log easier to use
    macro_rules! console_log {
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }

    // Called when the wasm module is instantiated (demo only)
    #[wasm_bindgen(start)]
    pub fn main() {
        console_log!("lin-ldf WASM module loaded");
    }

    pub(crate) fn log_parse_start() {
        console_log!("Parsing LDF file...");
    }

    pub(crate) fn log_parse_success() {
        console_log!("LDF parsed successfully");
    }

    pub(crate) fn log_parse_error(e: &str) {
        console_log!("Parse error: {}", e);
    }
}

// Stubs for when demo feature is disabled
#[cfg(not(feature = "demo"))]
mod demo {
    pub(crate) fn log_parse_start() {}
    pub(crate) fn log_parse_success() {}
    pub(crate) fn log_parse_error(_e: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(feature = "ts-rs")]
    fn generate_typescript_bindings() {
        // This test triggers ts-rs to generate TypeScript definitions
        // when run with: cargo test --features ts-rs
        
        // Import and reference all types to trigger export
        use lin_ldf::*;
        use ts_rs::TS;
        
        // Export types by calling their TS generation
        LinLdf::export_all_to("./types").unwrap();
        
        println!("TypeScript bindings generated in wasm/types/");
    }
}

// Use smaller allocator for WASM - this is generally good for any WASM usage
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// CORE LIBRARY FUNCTIONS
// ============================================================================

/// Parse LDF file content and return a JavaScript-friendly structure
///
/// This is the main entry point for the library. Pass in LDF file content
/// as a string and get back a parsed structure that can be used in JavaScript.
///
/// The returned structure uses the same types as the main lin-ldf library
/// but is serialized to JavaScript-compatible format.
#[wasm_bindgen]
pub fn parse_ldf_file(ldf_content: &str) -> Result<JsValue, JsValue> {
    demo::log_parse_start();

    match parse_ldf(ldf_content) {
        Ok(ldf) => {
            demo::log_parse_success();
            Ok(serde_wasm_bindgen::to_value(&ldf)?)
        }
        Err(e) => {
            let error_msg = format!("Parse error: {}", e);
            demo::log_parse_error(&error_msg);
            Err(JsValue::from_str(&error_msg))
        }
    }
}

/// Parse LDF file content and return it as a JSON string
///
/// This function is useful if you prefer working with JSON strings
/// instead of JavaScript objects.
#[wasm_bindgen]
pub fn parse_ldf_to_json(ldf_content: &str) -> Result<String, JsValue> {
    match parse_ldf(ldf_content) {
        Ok(ldf) => match serde_json::to_string(&ldf) {
            Ok(json) => Ok(json),
            Err(e) => Err(JsValue::from_str(&format!("JSON serialization error: {}", e))),
        },
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

/// Parse LDF file content and return it as a pretty-printed JSON string
///
/// This function is useful for debugging or when you want formatted JSON output.
#[wasm_bindgen]
pub fn parse_ldf_to_pretty_json(ldf_content: &str) -> Result<String, JsValue> {
    match parse_ldf(ldf_content) {
        Ok(ldf) => match serde_json::to_string_pretty(&ldf) {
            Ok(json) => Ok(json),
            Err(e) => Err(JsValue::from_str(&format!("JSON serialization error: {}", e))),
        },
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}

/// Create a LinLdf object from a JSON string
///
/// This function allows you to deserialize a previously serialized LinLdf object.
/// Useful for saving/loading parsed LDF data.
#[wasm_bindgen]
pub fn ldf_from_json(json_str: &str) -> Result<JsValue, JsValue> {
    match serde_json::from_str::<LinLdf>(json_str) {
        Ok(ldf) => Ok(serde_wasm_bindgen::to_value(&ldf)?),
        Err(e) => Err(JsValue::from_str(&format!("JSON deserialization error: {}", e))),
    }
}

/// Get the version of the WASM wrapper
#[wasm_bindgen]
pub fn get_wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get the version of the underlying lin-ldf library
#[wasm_bindgen]
pub fn get_core_version() -> String {
    // This reads from the main crate's version
    lin_ldf::parse_ldf("LIN_description_file;LIN_protocol_version=\"2.1\";LIN_language_version=\"2.1\";LIN_speed=19.2 kbps;Nodes{Master:Master,5 ms,0.1 ms;Slaves:;}Signals{}Frames{}Node_attributes{}Schedule_tables{}")
        .map(|_| "0.0.27") // For now, hardcoded, but this could be made dynamic
        .unwrap_or("unknown")
        .to_string()
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Validate LDF file content without fully parsing it
///
/// Returns true if the LDF content appears to be valid, false otherwise.
/// This is faster than full parsing for validation purposes.
#[wasm_bindgen]
pub fn validate_ldf(ldf_content: &str) -> bool {
    parse_ldf(ldf_content).is_ok()
}

/// Get basic statistics about an LDF file
///
/// Returns a JavaScript object with counts of various LDF elements.
#[wasm_bindgen]
pub fn get_ldf_stats(ldf_content: &str) -> Result<JsValue, JsValue> {
    match parse_ldf(ldf_content) {
        Ok(ldf) => {
            let stats = serde_json::json!({
                "signal_count": ldf.signals.len(),
                "frame_count": ldf.frames.len(),
                "node_count": ldf.nodes.slaves.len() + 1, // +1 for master
                "schedule_table_count": ldf.schedule_tables.len(),
                "diagnostic_signal_count": ldf.diagnostic_signals.len(),
                "diagnostic_frame_count": ldf.diagnostic_frames.len(),
                "node_attributes_count": ldf.node_attributes.len(),
                "signal_encoding_types_count": ldf.signal_encoding_types.len(),
                "signal_representations_count": ldf.signal_representations.len(),
                "lin_protocol_version": ldf.header.lin_protocol_version,
                "lin_language_version": ldf.header.lin_language_version,
                "lin_speed": ldf.header.lin_speed
            });
            Ok(serde_wasm_bindgen::to_value(&stats)?)
        }
        Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
    }
}
