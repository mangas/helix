use exports::helix::plugin::editor::Document;

pub mod plugins;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "host",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: Myhost,
        "helix:plugin/editor": Editor,
    },
});

struct Editor;

impl crate::exports::helix::plugin::editor::Guest for Editor {
    fn current_document() -> Option<Document> {
        todo!()
    }

    fn get_git_remote() -> Result<wit_bindgen::rt::string::String, wit_bindgen::rt::string::String>
    {
        todo!()
    }

    fn clipboard_copy(
        content: wit_bindgen::rt::string::String,
    ) -> Result<wit_bindgen::rt::string::String, wit_bindgen::rt::string::String> {
        todo!()
    }
}

struct MyHost;
