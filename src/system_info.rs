use chrono::{DateTime, Local};
use colored::*;
use std::env;
use sysinfo::{Disks, System};

pub struct SystemInfo {
    pub user: String,
    pub hostname: String,
    pub datetime: DateTime<Local>,
    pub os_info: String,
    pub uptime: String,
    pub shell: String,
    pub displays: String,
    pub window_manager: String,
    pub terminal: String,
    pub font: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub storage: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let user = whoami::username().unwrap_or_else(|_| "Unknown".to_string());
        let hostname = whoami::hostname().unwrap_or_else(|_| "Unknown".to_string());
        let datetime = Local::now();

        // OS and Kernel info
        let os_info = format!(
            "{} {}",
            System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
            System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
        );

        // Uptime
        let uptime_secs = System::uptime();
        let uptime = format!(
            "{}d {}h {}m",
            uptime_secs / 86400,
            (uptime_secs % 86400) / 3600,
            (uptime_secs % 3600) / 60
        );

        // Shell
        let shell = env::var("SHELL")
            .unwrap_or_else(|_| "Unknown".to_string())
            .split('/')
            .next_back()
            .unwrap_or("Unknown")
            .to_string();

        // TODO: Displays detection
        let displays = "1".to_string();

        let window_manager = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());

        // Terminal
        let terminal = env::var("TERM").unwrap_or_else(|_| "Unknown".to_string());

        // TODO: Font detection
        let font = "Unknown".to_string();

        // CPU
        let cpu_info = if let Some(cpu) = sys.cpus().first() {
            format!("{} ({} cores)", cpu.brand().trim(), sys.cpus().len())
        } else {
            "Unknown".to_string()
        };

        // TODO: GPU detection
        let gpu = "Unknown".to_string();

        // Memory
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory = format!(
            "{}MB / {}MB",
            used_memory / 1024 / 1024,
            total_memory / 1024 / 1024
        );

        // Storage
        let mut total_storage = 0;
        let mut used_storage = 0;
        let disks = Disks::new_with_refreshed_list();
        for disk in disks.iter() {
            total_storage += disk.total_space();
            used_storage += disk.total_space() - disk.available_space();
        }
        let storage = format!(
            "{}GB / {}GB",
            used_storage / 1024 / 1024 / 1024,
            total_storage / 1024 / 1024 / 1024
        );

        SystemInfo {
            user,
            hostname,
            datetime,
            os_info,
            uptime,
            shell,
            displays,
            window_manager,
            terminal,
            font,
            cpu: cpu_info,
            gpu,
            memory,
            storage,
        }
    }

    pub fn as_vec(&self) -> Vec<String> {
        let mut lines = Vec::new();
        // Header
        lines.push(format!(
            "{}@{} ({})",
            self.user.bold().cyan(),
            self.hostname.bold().cyan(),
            self.datetime.format("%m/%d/%y %H:%M").to_string().dimmed()
        ));

        lines.push(String::new());

        lines.push(format!("{} {}", "OS:".bold().yellow(), self.os_info));
        lines.push(format!("{} {}", "Uptime:".bold().yellow(), self.uptime));
        lines.push(format!("{} {}", "Shell:".bold().yellow(), self.shell));
        lines.push(format!("{} {}", "Displays:".bold().yellow(), self.displays));
        lines.push(format!("{} {}", "WM:".bold().yellow(), self.window_manager));
        lines.push(format!("{} {}", "Terminal:".bold().yellow(), self.terminal));
        lines.push(format!("{} {}", "Font:".bold().yellow(), self.font));
        lines.push(format!("{} {}", "CPU:".bold().yellow(), self.cpu));
        lines.push(format!("{} {}", "GPU:".bold().yellow(), self.gpu));
        lines.push(format!("{} {}", "Memory:".bold().yellow(), self.memory));
        lines.push(format!("{} {}", "Storage:".bold().yellow(), self.storage));
        lines
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
