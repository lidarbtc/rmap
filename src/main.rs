use clap::Parser;
use colored::*;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{sleep, Duration};

mod utils;
use utils::{hasher, read_lines_to_vec};

mod scan;
use scan::visit;

#[derive(Parser)]
#[command(
    name = "rmap",
    version = "0.1.0",
    author = "lidar. <lidarbtc@protonmail.com>",
    about = "Fast http/https single host scanner for find original IP behind CDN",
    long_about = None,
)]
struct Cli {
    #[arg(short = 'i', long = "input", help = "Path to the IP file")]
    ip_file_path: Option<String>,

    #[arg(
        short = 'f',
        long = "favicon",
        help = "Path to the favicon file; saves IP if the website contains a matching favicon"
    )]
    favicon_path: Option<String>,

    #[arg(
        short = 't',
        long = "trigger",
        help = "Trigger word to identify matching websites; saves IP if the website contains this word"
    )]
    trigger_word: Option<String>,

    #[arg(
        short = 'd',
        long = "delay",
        help = "Delay time (in milliseconds) between task creations"
    )]
    delay: Option<u64>,

    #[arg(
        short = 'o',
        long = "output",
        help = "Path to the output file where results will be saved"
    )]
    output_path: Option<String>,

    #[arg(short = 'w', long = "host", help = "Host (domain) to search for")]
    host: Option<String>,

    #[arg(
        short = 's',
        long = "https",
        help = "Set to true to use HTTPS, false for HTTP",
        default_value = "false"
    )]
    use_https: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.output_path.is_none() {
        println!(
            "{}: Output path not provided. Results will not be saved.",
            "Warning".yellow()
        );
    }

    if cli.favicon_path.is_none() && cli.trigger_word.is_none() {
        println!(
            "{}: No favicon path or trigger word provided. Output will not be generated.",
            "Warning".yellow()
        );
    }

    let host = match cli.host {
        Some(host) => host,
        None => {
            println!(
                "{}: Host not provided; Please refer to the --help command",
                "Error".red()
            );
            return Ok(());
        }
    };

    let delay = match cli.delay {
        Some(delay) => delay,
        None => {
            println!(
                "{}: Delay(ms) not provided; Please refer to the --help command",
                "Error".red()
            );
            return Ok(());
        }
    };

    let path = match cli.ip_file_path {
        Some(file_path) => file_path,
        None => {
            println!(
                "{}: IP file path not provided; Please refer to the --help command",
                "Error".red()
            );
            return Ok(());
        }
    };

    let lines = match read_lines_to_vec(&path).await {
        Ok(lines) => lines,
        Err(e) => {
            println!("{}: {}", "Error reading file".red(), e);
            return Err(e);
        }
    };

    let favicon_hash: Option<String> = match cli.favicon_path {
        Some(path) => {
            let file = File::open(path).await;
            let mut file = match file {
                Ok(file) => file,
                Err(e) => {
                    println!("Error reading file: {}", e);
                    return Err(e);
                }
            };

            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await?;

            let hashed = hasher(contents);
            Some(hashed)
        }
        None => None,
    };

    let total_tasks = lines.len();

    let mut current_time = 0;

    let progress_success = Arc::new(Mutex::new(0));
    let progress_triggered = Arc::new(Mutex::new(0));
    let progress_for_timer_success = Arc::clone(&progress_success);
    let progress_for_timer_triggered = Arc::clone(&progress_triggered);

    let timer_task = task::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;

            current_time += 1;

            let success = progress_for_timer_success.lock().await;
            let triggered = progress_for_timer_triggered.lock().await;

            let process_percent = *success / total_tasks * 100;
            let total = *success;
            let average = *success / current_time;
            let f_total = *triggered;

            // println!("{process_percent}% ({remain} left); send: {total} {per_second} ({average} avg); found: {f_total} {f_per_second} ({f_avg} avg); hits: {hit_percent}");
            println!("{process_percent}% send: {total} ({average} p/s avg); found: {f_total}");

            if *success == total_tasks {
                break;
            }
        }
    });

    let mut tasks = Vec::new();
    for ip in lines {
        let progress_success_clone = Arc::clone(&progress_success);
        let progress_triggered_clone = Arc::clone(&progress_triggered);
        let trigger_word = cli.trigger_word.clone();
        let output_path = cli.output_path.clone();
        let favicon_hash = favicon_hash.clone();
        let host = host.clone();
        let https = cli.use_https.clone();

        let task = task::spawn(async move {
            let result = visit(trigger_word, ip, output_path, favicon_hash, host, https).await;

            if result.is_ok_and(|x| x) {
                let mut triggered = progress_triggered_clone.lock().await;
                *triggered += 1;
            }

            let mut progress = progress_success_clone.lock().await;
            *progress += 1;
        });
        tasks.push(task);
        sleep(Duration::from_millis(delay)).await;
    }

    for task in tasks {
        let _ = task.await?;
    }

    timer_task.abort();

    Ok(())
}
