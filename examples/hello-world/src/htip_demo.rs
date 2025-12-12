//! # Full HTIP Pipeline Demo
//!
//! This demonstrates the complete dx-www stack:
//! 1. Server: Serialize to HTIP binary with Ed25519 signature
//! 2. Client: Deserialize and apply to DOM
//! 3. Measure: Benchmark performance

use dx_binary::deserializer::HtipStream;
use dx_binary::htip_bridge::HtipEngine;
use dx_binary::serializer::HtipWriter;
use ed25519_dalek::{SigningKey, VerifyingKey};
use wasm_bindgen::prelude::*;
use web_sys::{Performance, window};

#[wasm_bindgen]
pub struct HtipDemo {
    engine: HtipEngine,
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    perf: Performance,
}

#[wasm_bindgen]
impl HtipDemo {
    /// Initialize the demo
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<HtipDemo, JsValue> {
        console_error_panic_hook::set_once();

        // Generate Ed25519 keypair
        let signing_key = SigningKey::from_bytes(&[1u8; 32]); // Mock key for demo
        let verifying_key = signing_key.verifying_key();

        // Get performance API
        let window = window().ok_or("No window")?;
        let perf = window.performance().ok_or("No performance API")?;

        Ok(HtipDemo {
            engine: HtipEngine::new()?,
            signing_key,
            verifying_key,
            perf,
        })
    }

    /// Run the full HTIP pipeline demo
    pub fn run_demo(&mut self) -> Result<JsValue, JsValue> {
        let mut results = js_sys::Object::new();

        // Step 1: Create HTIP payload (server-side simulation)
        let t0 = self.perf.now();
        let binary = self
            .create_htip_payload()
            .map_err(|e| format!("Payload creation failed: {}", e))?;
        let t1 = self.perf.now();

        let serialization_time = t1 - t0;
        js_sys::Reflect::set(&results, &"serialization_ms".into(), &serialization_time.into())?;
        js_sys::Reflect::set(&results, &"payload_size".into(), &binary.len().into())?;

        // Step 2: Deserialize HTIP (client-side)
        let t2 = self.perf.now();
        let stream = HtipStream::new(&binary, &self.verifying_key)
            .map_err(|e| format!("Deserialization failed: {:?}", e))?;
        let t3 = self.perf.now();

        let deserialization_time = t3 - t2;
        js_sys::Reflect::set(&results, &"deserialization_ms".into(), &deserialization_time.into())?;

        // Step 3: Apply to DOM
        let t4 = self.perf.now();
        let document = window().unwrap().document().unwrap();
        let root = document
            .get_element_by_id("app")
            .ok_or("No #app element")?
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| "Failed to cast to HtmlElement")?;

        self.engine
            .process_stream(&stream, &root)
            .map_err(|e| format!("DOM update failed: {}", e))?;
        let t5 = self.perf.now();

        let dom_update_time = t5 - t4;
        js_sys::Reflect::set(&results, &"dom_update_ms".into(), &dom_update_time.into())?;

        // Total time
        let total_time = t5 - t0;
        js_sys::Reflect::set(&results, &"total_ms".into(), &total_time.into())?;

        // Log results
        web_sys::console::log_1(&"🚀 HTIP Pipeline Benchmark Results:".into());
        web_sys::console::log_1(&format!("  Payload Size: {} bytes", binary.len()).into());
        web_sys::console::log_1(&format!("  Serialization: {:.3}ms", serialization_time).into());
        web_sys::console::log_1(
            &format!("  Deserialization: {:.3}ms", deserialization_time).into(),
        );
        web_sys::console::log_1(&format!("  DOM Update: {:.3}ms", dom_update_time).into());
        web_sys::console::log_1(&format!("  TOTAL: {:.3}ms", total_time).into());

        Ok(results.into())
    }

    /// Create an HTIP payload (simulating server-side)
    fn create_htip_payload(&self) -> Result<Vec<u8>, String> {
        let mut writer = HtipWriter::new();

        // Define counter template
        let counter_html = r#"<div class="counter">
    <h1 id="count">0</h1>
    <div class="buttons">
        <button class="btn">-</button>
        <button class="btn">+</button>
    </div>
</div>"#;

        writer.write_template(0, counter_html, vec![]);

        // Instantiate template
        writer.write_instantiate(1, 0, 0);

        // Patch initial count
        writer.write_patch_text(1, 0, "42");

        // Batch operations for multiple updates
        writer.write_batch_start(1);
        writer.write_patch_attr(1, 0, "data-count", "42");
        writer.write_class_toggle(1, "active", true);
        writer.write_batch_commit(1);

        // Sign and return
        writer
            .finish_and_sign(&self.signing_key)
            .map_err(|e| format!("Failed to sign: {:?}", e))
    }

    /// Run stress test (1000 operations)
    pub fn run_stress_test(&mut self) -> Result<JsValue, JsValue> {
        let mut results = js_sys::Object::new();

        let t0 = self.perf.now();

        // Create large payload
        let mut writer = HtipWriter::new();

        // Define template once
        writer.write_template(0, "<div class='item'><!--SLOT_0--></div>", vec![]);

        // Instantiate 1000 times with different text
        for i in 0..1000 {
            writer.write_instantiate(i + 1, 0, 0);
            writer.write_patch_text(i + 1, 0, &format!("Item {}", i));
        }

        let binary = writer
            .finish_and_sign(&self.signing_key)
            .map_err(|e| format!("Failed: {:?}", e))?;
        let t1 = self.perf.now();

        let payload_size = binary.len();

        // Deserialize
        let stream = HtipStream::new(&binary, &self.verifying_key)
            .map_err(|e| format!("Failed: {:?}", e))?;
        let t2 = self.perf.now();

        // Apply to DOM
        let document = window().unwrap().document().unwrap();
        let root = document
            .get_element_by_id("app")
            .ok_or("No #app")?
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| "Failed to cast")?;

        self.engine
            .process_stream(&stream, &root)
            .map_err(|e| format!("Failed: {}", e))?;
        let t3 = self.perf.now();

        let serialization_time = t1 - t0;
        let deserialization_time = t2 - t1;
        let dom_time = t3 - t2;
        let total_time = t3 - t0;

        js_sys::Reflect::set(&results, &"operations".into(), &1000.into())?;
        js_sys::Reflect::set(&results, &"payload_size".into(), &payload_size.into())?;
        js_sys::Reflect::set(&results, &"serialization_ms".into(), &serialization_time.into())?;
        js_sys::Reflect::set(&results, &"deserialization_ms".into(), &deserialization_time.into())?;
        js_sys::Reflect::set(&results, &"dom_update_ms".into(), &dom_time.into())?;
        js_sys::Reflect::set(&results, &"total_ms".into(), &total_time.into())?;
        js_sys::Reflect::set(&results, &"ops_per_ms".into(), &(1000.0 / total_time).into())?;

        web_sys::console::log_1(&"⚡ Stress Test Results (1000 operations):".into());
        web_sys::console::log_1(
            &format!("  Payload: {} bytes ({:.2} KB)", payload_size, payload_size as f64 / 1024.0)
                .into(),
        );
        web_sys::console::log_1(&format!("  Serialization: {:.3}ms", serialization_time).into());
        web_sys::console::log_1(
            &format!("  Deserialization: {:.3}ms", deserialization_time).into(),
        );
        web_sys::console::log_1(&format!("  DOM Updates: {:.3}ms", dom_time).into());
        web_sys::console::log_1(
            &format!("  TOTAL: {:.3}ms ({:.0} ops/ms)", total_time, 1000.0 / total_time).into(),
        );

        Ok(results.into())
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"✨ dx-www HTIP Engine Ready".into());
    Ok(())
}
