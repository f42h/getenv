use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;

pub struct Resolution {
    pub width: u16,
    pub height: u16
}

impl Resolution {
    pub fn new() -> Self {
        let (conn, screen_num) = RustConnection::connect(None).expect("Unable to connect to X server");
        let screen = conn.setup().roots[screen_num].clone();

        Self { 
            width: screen.width_in_pixels, 
            height: screen.height_in_pixels
        }
    }
}