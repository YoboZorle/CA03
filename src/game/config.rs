#[derive(Debug, Serialize, Deserialize)]
struct Config {
    refresh_rate:   i32,
    fullscreen:     bool,
    capture_cursor: bool,
    exit_on_esc:    bool,
    grid_size:      i32,
}
