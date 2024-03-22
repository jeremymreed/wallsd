use chrono::{DateTime, Local};

pub fn is_time_after_target(target: DateTime<Local>, current_time: DateTime<Local>) -> bool {
    println!("target: {:?}", target);
    println!("current_time: {:?}", current_time);

    let delta = target - current_time;

    println!("delta:        {:?}", delta);

    match delta.num_seconds() < 0 {
        true => {
            println!("current_time <= target");
            true
        },
        false => {
            println!("current_time != target");
            false
        },
    }
}