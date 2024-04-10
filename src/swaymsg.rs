use std::process::Command;
use crate::mode;
use crate::resolution;
use crate::output;

pub fn get_outputs() -> Vec<output::Output>{
    tracing::info!("Getting outputs");

    let output = Command::new("swaymsg")
        .arg("-r")
        .arg("-t")
        .arg("get_outputs")
        .output()
        .expect("failed to execute process");

    process_output(&String::from_utf8_lossy(&output.stdout).to_string())
}

fn process_output(raw_output: &String) -> Vec<output::Output> {
    let mut outputs: Vec<output::Output> = vec![];

    let json_output = json::parse(raw_output).unwrap();

    for index in 0..json_output.len() {
        let output = output::Output {
            name: json_output[index]["name"].to_string(),
            resolution: resolution::Resolution {
                width: json_output[index]["rect"]["width"].as_u32().unwrap(),
                height: json_output[index]["rect"]["height"].as_u32().unwrap(),
            },
            mode: mode::Mode::ONESHOT,
            images: Vec::new(),
        };
        outputs.push(output);
    }

    if outputs.is_empty() {
        panic!("No outputs found");
    }

    outputs
}