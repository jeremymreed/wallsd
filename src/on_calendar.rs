use chrono::{DateTime, Local};

pub fn is_time_after_target(target: DateTime<Local>, current_time: DateTime<Local>) -> bool {
    tracing::debug!("target: {:?}", target);
    tracing::debug!("current_time: {:?}", current_time);

    let delta = target - current_time;

    tracing::debug!("delta:        {:?}", delta);

    match delta.num_seconds() < 0 {
        true => {
            tracing::debug!("current_time <= target");
            true
        },
        false => {
            tracing::debug!("current_time != target");
            false
        },
    }
}