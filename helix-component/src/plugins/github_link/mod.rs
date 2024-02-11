use self::helix::plugin::editor;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "plugin",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: GithubLinkPlugin,
    },
});

pub struct GithubLinkPlugin {}

impl Guest for GithubLinkPlugin {
    fn handle_command(
        command: wit_bindgen::rt::string::String,
    ) -> Result<wit_bindgen::rt::string::String, wit_bindgen::rt::string::String> {
        let document = editor::current_document().ok_or("unable to get current document")?;
        let remote = editor::get_git_remote()?;

        Ok("".to_string())
    }

    fn commands(
        command: wit_bindgen::rt::string::String,
    ) -> wit_bindgen::rt::vec::Vec<wit_bindgen::rt::string::String> {
        todo!()
    }
}
