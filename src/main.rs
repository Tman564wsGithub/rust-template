use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    {{crate_name}}::greet("{{project-name}}");

    Ok(())
}
