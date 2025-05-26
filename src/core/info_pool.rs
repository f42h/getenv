use gethostname::gethostname;
use sysinfo::{Disks, Networks, System};

use crate::{gb, get_os};
use super::screen::Resolution;

pub struct InfoPool {
    sys: System
}

impl InfoPool {
    pub fn new() -> Self {
        Self {
            sys: System::new_all()
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_all();
    } 

    pub fn print_info<Inf>(&self, mut inf: Inf)
    where Inf: FnMut(String) {
        inf("################ GENERAL ################".to_string());

        let mut os = String::new();

        if let Some(name) = System::name() {
            os.push_str(&format!("{} ", name));
        }

        if let Some(version) = System::os_version() {
            os.push_str(&format!("{} ", version));
        }

        if let Some(kernel_version) = System::name() {
            os.push_str(&format!("{}", kernel_version));
        }

        let os_sum = format!("{} {}", get_os!(), os).replace("\"", "");
        inf(format!("Operating System       : {}", os_sum));

        let hostname = gethostname().into_string();
        if let Ok(hostname) = hostname {
            inf(format!("Hostname               : {}", hostname));
        }

        let username = whoami::username();
        inf(format!("Username               : {}", username));

        let resolution = Resolution::new();
        inf(format!("Screen Resolution      : {}x{}", resolution.width, resolution.height));

        inf("################ MEMORY #################".to_string());

        let mem_total = self.sys.total_memory();
        inf(format!("Total Memory           : {:.2} GB ({} bytes)", gb!(mem_total), mem_total));

        let mem_used = self.sys.used_memory();
        inf(format!("Used Memory            : {:.2} GB ({} bytes)", gb!(mem_used), mem_used));

        let swap_total = self.sys.total_swap();
        inf(format!("total swap             : {:.2} GB ({} bytes)", gb!(swap_total), swap_total));

        let used_swap = self.sys.used_swap();
        inf(format!("used swap              : {:.2} GB ({} bytes)", gb!(used_swap), used_swap));

        inf("################  CPU   #################".to_string());

        let architecture = whoami::arch();
        inf(format!("Architecture           : {}", architecture));

        let cpus = self.sys.cpus().len();
        inf(format!("Cores                  : {}", cpus));

        inf("################ DISKS ##################".to_string());

        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            inf(format!("{disk:?}"));
        }

        inf("############### NETWORK #################".to_string());

        let networks = Networks::new_with_refreshed_list();
        for (interface_name, data) in &networks {
            inf(format!("{interface_name}: {} B (down) / {} B (up)", data.total_received(), data.total_transmitted()));
        }
    }
}