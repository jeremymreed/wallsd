use std::process::Command;

pub fn get_current_wallpapers() {
    tracing::info!("Querying swww for output initial state.");

    let output = Command::new("swww")
        .arg("query")
        .output()
        .expect("failed to execute process");

    process_output(&String::from_utf8_lossy(&output.stdout).to_string());
}

fn process_output(raw_output: &String) {
    tracing::info!("raw_output: {:?}", raw_output);
}
