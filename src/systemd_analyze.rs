use std::process::Command;
use std::time;
use chrono::{DateTime, Local};

// Use systemd-timer OnCalendar syntax.
// See: https://man.archlinux.org/man/systemd.time.7 or 'man 7 systemd.timer'.
pub fn get_next_event(oncalendar_string: &String) -> DateTime<Local> {
    println!("Getting next event datetime string");
    let now = time::Instant::now();

    let output = Command::new("systemd-analyze")
        .arg("calendar")
        .arg(oncalendar_string)
        .output()
        .expect("failed to execute process");

    //println!("output: ${:?}", output);
    let elapsed = now.elapsed();
    println!("Elapsed time: {:#?}", elapsed);

    let next_event: DateTime<Local> = process_output(&String::from_utf8_lossy(&output.stdout).to_string());

    next_event
}

fn process_output(raw_output: &String) -> DateTime<Local> {
    println!("{:?}", raw_output);

    let next_event = parse_raw_output(raw_output);

    let offset = get_offset();

    println!("next_event: {:?}", next_event);
    println!("offset: {:?}", offset);

    let dt = next_event.parse::<DateTime<Local>>().unwrap();

    println!("final dt: {:?}", dt);

    dt
}

fn parse_raw_output(raw_output: &str) -> String {
    let lines = raw_output.lines();
    let mut raw_next_event: String = String::new();
    for line in lines {
        if line.contains("Next elapse: ") {
            raw_next_event = line.to_string();
            break;
        }
    }

    println!("next_event: {:?}", raw_next_event);

    parse_next_string(&raw_next_event)
}

fn parse_next_string(raw_next_event: &str) -> String {
    let mut next_event: String = String::new();
    let mut datetime_str: &str = "";
    let parts = raw_next_event.split(": ");
    for part in parts {
        println!("part: {:?}", part);
        datetime_str = part;
    }

    println!("datetime_str: {:?}", datetime_str);

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
