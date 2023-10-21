use std::collections::HashMap;
use sysinfo::{DiskExt, System, SystemExt};
use termion::{color, style, terminal_size};

fn main() {
    let s = System::new_all();
    let mut disks_info = HashMap::new();

    for disk in s.disks() {
        let disk_name = disk.name().to_string_lossy().into_owned();
        let disk_info = disks_info.entry(disk_name.clone()).or_insert_with(Vec::new);
        disk_info.push(disk);
    }

    let (term_width, _) = terminal_size().unwrap_or((80, 24));

    for (disk_name, partitions) in disks_info {
        println!("[{}]", disk_name);
        for partition in partitions {
            let total_space = partition.total_space();
            let available_space = partition.available_space();
            let used_space = total_space - available_space;
            let percent_used = used_space as f64 / total_space as f64 * 100.0;

            let bar_color: Box<dyn std::fmt::Display> = if percent_used > 90.0 {
                Box::new(color::Fg(color::Red))
            } else {
                Box::new(color::Fg(color::Cyan))
            };

            let partition_info = format!(
                " [{}] [{}] ",
                partition.name().to_string_lossy(),
                partition.mount_point().to_string_lossy(),
            );
            let storage_info = format!(
                " {}/{}",
                format_size(used_space),
                format_size(total_space)
            );
            let used_width = term_width as usize - partition_info.len() - storage_info.len() - 4; // -4 accounts for spaces and brackets
            
            let bar_width = (percent_used / 100.0 * used_width as f64) as usize;
            let space_width = used_width - bar_width;

            let progress_bar = format!(
                "[{}{}{}{}] {:.2}%",
                bar_color,
                "█".repeat(bar_width),
                style::Reset,
                ":".repeat(space_width),
                percent_used
            );

            let output = format!(
                "{}{} {}",
                partition_info,
                progress_bar,
                storage_info
            );

            println!("{:<width$}", output, width = term_width as usize);
        }
    }
}

fn format_size(size: u64) -> String {
    const KILO: u64 = 1024;
    if size < KILO {
        return format!("{} B", size);
    }
    let kb = size as f64 / KILO as f64;
    if kb < KILO as f64 {
        return format!("{:.2} KB", kb);
    }
    let mb = kb / KILO as f64;
    if mb < KILO as f64 {
        return format!("{:.2} MB", mb);
    }
    let gb = mb / KILO as f64;
    format!("{:.2} GB", gb)
}
