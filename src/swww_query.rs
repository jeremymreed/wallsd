use std::process::Command;
use std::collections::HashMap;

pub fn get_current_wallpapers() -> HashMap<String, String> {
    tracing::info!("Querying swww for output initial state.");

    let output = Command::new("swww")
        .arg("query")
        .output()
        .expect("failed to execute process");

    process_output(&String::from_utf8_lossy(&output.stdout).to_string())
}

fn process_output(raw_output: &String) -> HashMap<String, String> {
    let lines = raw_output.lines().collect::<Vec<&str>>();
    let mut output_initial_state: HashMap<String, String> = HashMap::new();

    if lines.is_empty() {
        tracing::warn!("No lines found in swww query output.");
        panic!("No lines found in swww query output.");
    }

    for line in lines {
        tracing::debug!("line: {:?}", line);
        let (output_name, output_current_wallpaper) = parse_line(&line.to_string());
        tracing::debug!("output_name: {:?}, output_current_wallpaper: {:?}", output_name, output_current_wallpaper);
        output_initial_state.insert(output_name, output_current_wallpaper);
    }

    output_initial_state
}

fn parse_line(line: &String) -> (String, String) {
    let tokens = line.split(',').collect::<Vec<&str>>();

    if tokens.len() != 3 {
        tracing::warn!("Invalid line from swww query: {:?}", line);
        panic!("Invalid line from swww query: {:?}", line);
    }

    for token in tokens.as_slice() {
        tracing::debug!("token: {:?}", token);
    }



    (parse_output_name(&tokens[0].to_string()), parse_current_wallpaper(&tokens[2].to_string()))
}

fn parse_output_name(raw_name: &String) -> String {
    let tokens = raw_name.split(':').collect::<Vec<&str>>();

    if tokens.len() != 2 {
        tracing::warn!("Invalid name from swww query: {:?}", raw_name);
        panic!("Invalid name from swww query: {:?}", raw_name);
    }

    tokens[0].to_string()
}

fn parse_current_wallpaper(raw_current_wallpaper: &String) -> String {
    let tokens = raw_current_wallpaper.split(':').collect::<Vec<&str>>();

    if tokens.len() != 3 {
        tracing::warn!("Invalid current wallpaper from swww query: {:?}", raw_current_wallpaper);
        panic!("Invalid current wallpaper from swww query: {:?}", raw_current_wallpaper);
    }

    tokens[2].to_string().trim().to_string()
}
