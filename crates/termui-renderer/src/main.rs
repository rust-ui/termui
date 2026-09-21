use std::env;
use termui_renderer::render_demo;

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => output.push(' '),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn main() {
    let mut first = true;
    print!("{{");
    for name in env::args().skip(1) {
        if !first {
            print!(",");
        }
        first = false;
        print!("{}:[", json_string(&name));
        for (index, line) in render_demo(&name).iter().enumerate() {
            if index > 0 {
                print!(",");
            }
            print!("{}", json_string(line));
        }
        print!("]");
    }
    println!("}}");
}
