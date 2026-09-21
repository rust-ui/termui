use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use termui_registry::{registered_demo_names, render_demo};
use termui_renderer::render_example_preview;

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{1b}' => output.push_str("\\u001b"),
            character if character.is_control() => output.push(' '),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn main() -> Result<(), Box<dyn Error>> {
    let names = env::args()
        .skip(1)
        .chain(registered_demo_names().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let mut first = true;
    print!("{{");
    for name in names {
        let frame = if name.starts_with("rust/") {
            render_demo(&name)
                .ok_or_else(|| format!("No Rust demo renderer registered for `{name}`"))?
        } else {
            render_example_preview(&name)
        };
        if !first {
            print!(",");
        }
        first = false;
        print!("{}:[", json_string(&name));
        for (index, line) in frame.iter().enumerate() {
            if index > 0 {
                print!(",");
            }
            print!("{}", json_string(line));
        }
        print!("]");
    }
    println!("}}");
    Ok(())
}
