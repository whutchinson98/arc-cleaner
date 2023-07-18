use std::env;
use std::process::{Command, Output};
use std::str;

fn main() {
    let home = env::var("HOME").expect("You do not have a $HOME env var");
    let arc_path = home + "/Library/Application Support/Arc";

    let file_patterns: Vec<&str> = vec![
        "StorableWindows.*.json",
        "StorableSidebar.*.json",
        "StorableArchive*.json",
    ];

    file_patterns
        .iter()
        .map(|file_pattern| {
            println!("Cleaning up file_pattern {}", file_pattern);

            return (*file_pattern, cleanup_arc(&arc_path, file_pattern));
        })
        .collect::<Vec<(&str, Output)>>()
        .iter()
        .for_each(|result| {
            let (file_path, output) = result;
            println!(
                "file pattern {}, result {}",
                *file_path,
                output.status.success()
            );
        });
}

fn cleanup_arc(arc_path: &str, remove_pattern: &str) -> Output {
    return Command::new("sh")
        .arg("-c")
        .current_dir(arc_path)
        .arg(format!(
            "rm -rf {remove_pattern}",
            remove_pattern = remove_pattern
        ))
        .output()
        .expect("error executing ls");
}
