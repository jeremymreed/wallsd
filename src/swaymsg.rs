use std::process::Command;

pub fn get_outputs() -> Vec<String>{
    let output = Command::new("swaymsg")
        .arg("-r")
        .arg("-t")
        .arg("get_outputs")
        .output()
        .expect("failed to execute process");

    process_output(&String::from_utf8_lossy(&output.stdout).to_string())
}

fn process_output(raw_output: &String) -> Vec<String> {
    let mut outputs: Vec<String> = vec![];

    let json_output = json::parse(raw_output).unwrap();

    for index in 0..json_output.len() {
        outputs.push(json_output[index]["name"].to_string());
    }

    outputs
}