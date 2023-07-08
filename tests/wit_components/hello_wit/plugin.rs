wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "test-world",
});

struct Plugin;

impl TestWorld for Plugin {
    fn add_hello(text: String) -> String {
        format!("Hello {text}")
    }

    fn add_abc(text: String) -> String {
        let text = text + "a";
        let text = add_b(&text);
        text + "c"
    }
}

export_test_world!(Plugin);
