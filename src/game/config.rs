#[derive(Debug, Serialize, Deserialize)]
struct Config {
    frame_rate:     u64,
    fullscreen:     bool,
    capture_cursor: bool,
    exit_on_esc:    bool,
    grid_size:      f64,
}
