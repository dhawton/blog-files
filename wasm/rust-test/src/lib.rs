use log::{info, trace};
use proxy_wasm as wasm;
use wasm::{types::ContextType, traits::Context};

wasm::main! {{
    wasm::set_log_level(wasm::types::LogLevel::Trace);
    wasm::set_root_context(|_| -> Box<dyn wasm::traits::RootContext> {
        Box::new(RustTest)
    });
}}

struct RustTest;

struct HttpHeaders {
    context_id: u32,
}

impl wasm::traits::Context for RustTest {}

impl wasm::traits::RootContext for RustTest {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        info!("on_vm_start");
        true
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn wasm::traits::HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }
}


impl Context for HttpHeaders {}

impl wasm::traits::HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> wasm::types::Action {
        for (name, value) in &self.get_http_request_headers() {
            trace!("#{} - {} = {}", self.context_id, name, value);
        }

        match self.get_http_request_header(":path") {
            Some(path) if path == "/test" => {
                self.send_http_response(
                    200,
                    vec![("x-powered-by", "rust"), ("content-type", "text/plain")],
                    Some(b"Hello from Rust Wasm!"),
                );
                wasm::types::Action::Pause
            }
            _ => wasm::types::Action::Continue,
        }
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}