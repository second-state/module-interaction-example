use example::Person;

wit_bindgen_rust::export!("../wit/example.wit");

struct Example {}
impl example::Example for Example {
    fn hello(p: Person) -> String {
        format!("Hello, {} {}", p.first_name, p.last_name)
    }
}
