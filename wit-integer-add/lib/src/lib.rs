use example::Payload;

wit_bindgen_rust::export!("../wit/example.wit");

struct Example {}
impl example::Example for Example {
    fn add(p: Payload) -> u32 {
        p.a + p.b
    }
}
