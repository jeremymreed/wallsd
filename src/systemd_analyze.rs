use std::process::Command;
use std::error::Error;
use std::time;
use chrono::{DateTime, Local};

// Use systemd-timer OnCalendar syntax.
// See: https://man.archlinux.org/man/systemd.time.7 or 'man 7 systemd.timer'.
pub fn get_next_event(oncalendar_string: &String) -> Result<DateTime<Local>, String> {
    tracing::debug!("Getting next event datetime string");
    let now = time::Instant::now();

    let output = Command::new("systemd-analyze")
        .arg("calendar")
        .arg(oncalendar_string)
        .output()
        .expect("Failed to execute systemd-analyze calendar command");

    //tracing::tracing::debug!("output: ${:?}", output);
    let elapsed = now.elapsed();
    tracing::debug!("Elapsed time: {:#?}", elapsed);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if stderr.is_empty() {

        let next_event: DateTime<Local> = process_output(&stdout);

        Ok(next_event)
    } else {
        tracing::error!("Error: {:#?}", stderr);
        Err(format!("Error {:#?}", stderr))
    }
}

fn process_output(raw_output: &String) -> DateTime<Local> {
    tracing::debug!("raw_output: {:?}", raw_output);

    let next_event = parse_raw_output(raw_output);

    let offset = get_offset();

    tracing::debug!("next_event: {:?}", next_event);
    tracing::debug!("offset: {:?}", offset);

    let dt = next_event.parse::<DateTime<Local>>().unwrap();

    tracing::debug!("final dt: {:?}", dt);

    dt
}

fn parse_raw_output(raw_output: &str) -> String {
    let lines = raw_output.lines();
    let mut raw_next_event: String = String::new();
    tracing::debug!("lines: {:?}", lines);

    for line in lines {
        if line.contains("Next elapse: ") {
            raw_next_event = line.to_string();
            break;
        }
    }

    tracing::debug!("next_event: {:?}", raw_next_event);

    parse_next_string(&raw_next_event)
}

fn parse_next_string(raw_next_event: &str) -> String {
    let mut next_event: String = String::new();
    let mut datetime_str: &str = "";
    let parts = raw_next_event.split(": ");
    for part in parts {
        tracing::debug!("part: {:?}", part);
        datetime_str = part;
    }

    tracing::debug!("datetime_str: {:?}", datetime_str);

    let parts: Vec<&str> = datetime_str.split(' ').collect();

    let offset = get_offset();

    let date = parts[1];
    let time = parts[2];
    
    next_event.push_str(date);
    next_event.push('T');
    next_event.push_str(time);
    next_event.push_str(offset.as_str());

    next_event
}

fn get_offset() -> String {
    let dt = Local::now();
    let offset = dt.offset().to_string();

    offset
}
