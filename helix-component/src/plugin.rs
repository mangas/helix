// wit_bindgen::generate!("plugin");

use wasmtime::component::bindgen;

use self::{document::Document, editor::Editor};

bindgen!("plugin");

struct HostExports {}

impl Document for HostExports {
    fn line_number(&mut self) -> wasmtime::component::__internal::anyhow::Result<u64> {
        todo!()
    }

    fn filename(&mut self) -> wasmtime::component::__internal::anyhow::Result<Option<String>> {
        todo!()
    }
}

impl Editor for HostExports {
    fn clipboard_copy(
        &mut self,
        content: String,
    ) -> wasmtime::component::__internal::anyhow::Result<()> {
        todo!()
    }

    fn run_command(
        &mut self,
        cmd: editor::Command,
    ) -> wasmtime::component::__internal::anyhow::Result<()> {
        todo!()
    }
}
