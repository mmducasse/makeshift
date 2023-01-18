use std::time::Duration;


pub fn min_sec_ms_string(dur: &Duration) -> String {
    let ms = (dur.subsec_millis() / 10) * 10;
    let total_s = dur.as_secs();
    let s = total_s % 60;
    let min = total_s / 60;

    format!("{:02}:{:02}.{}", min, s, ms)
}