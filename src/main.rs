use std::collections::HashMap;
use sysinfo::{DiskExt, System, SystemExt};
use termion::{color, style, terminal_size};

fn main() {
    let s = System::new_all();
    let mut disks_info = HashMap::new();
    let mut max_partition_name_len = 0;
    let mut max_mount_point_len = 0;

    for disk in s.disks() {
        let disk_name = disk.name().to_string_lossy().into_owned();
        let disk_mount_point = disk.mount_point().to_string_lossy().into_owned();
        let disk_info = disks_info.entry(disk_name.clone()).or_insert_with(Vec::new);
        disk_info.push(disk);

        if disk_name.len() > max_partition_name_len {
            max_partition_name_len = disk_name.len();
        }

        if disk_mount_point.len() > max_mount_point_len {
            max_mount_point_len = disk_mount_point.len();
        }
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
                "   [{}]{} ",
                partition.mount_point().to_string_lossy(),
                " ".repeat(max_mount_point_len - partition.mount_point().to_string_lossy().len()),
            );

            let storage_info = format!(
                " [{}/{}]",
                format_size(used_space),
                format_size(total_space)
            );

            let percent_str = if percent_used < 100.0 {
                format!("{:>5.2}%", percent_used)
            } else {
                format!("{:>3}%", percent_used)
            };

            let bar_width = (term_width as usize - partition_info.len() - storage_info.len() - 3) & !1;
            let fifty_percent = (bar_width / 2).saturating_sub(percent_str.len() / 2);

            let space_used = (percent_used / 100.0 * bar_width as f64).ceil() as usize;
            let space_free = (bar_width).saturating_sub(space_used);

            let disk_space_bar = if space_used < (fifty_percent + percent_str.len()) {
                format!(
                    "[{}{}{}{}{}{}{}{}]",
                    bar_color,
                    "█".repeat(space_used),
                    "░".repeat(fifty_percent.saturating_sub(space_used)),
                    style::Reset,
                    percent_str,
                    bar_color,
                    "░".repeat(fifty_percent),
                    style::Reset
                )
            } else {
                format!(
                    "[{}{}{}{}{}{}{}{}]",
                    bar_color,
                    "█".repeat(fifty_percent),
                    style::Reset,
                    percent_str,
                    bar_color,
                    "█".repeat(space_used.saturating_sub(fifty_percent + percent_str.len())),
                    "░".repeat(space_free),
                    style::Reset
                )
            };

            let output = format!(
                "{}{}{}",
                partition_info,
                disk_space_bar,
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