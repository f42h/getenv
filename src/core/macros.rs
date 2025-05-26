#[macro_export]
macro_rules! get_os {
    () => {
        if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else {
            "Unknown"
        }
    };
}

#[macro_export]
macro_rules! gb {
    ($bytes:expr) => {
        $bytes as f64 / 1_073_741_824.0
    };
}