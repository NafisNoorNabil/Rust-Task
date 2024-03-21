use std::env;
use std::fs::{self, File};
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::time::{Duration};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
struct Monitor {
    monitor_id: Option<u32>,
    name: String,
    script: Option<String>,
    result: Option<Result>, 
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[3];
    let mut file = File::open(path).unwrap(); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut monitors: Monitors = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
    let start_time=SystemTime::now();
    let duration_limit = Duration::from_secs(300);
    loop {
        let elapsed = start_time.elapsed().unwrap();
        if elapsed >= duration_limit {
            println!("ran for 5 minutes");
            break;
        } else {
            process_monitors(&mut monitors);
        }
    }
}
fn process_monitors(monitors: &mut Monitors) {
    let current_time = SystemTime::now(); 
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let processed_at = since_epoch.as_secs();
    update_monitors(monitors,processed_at); 
    
    loop {
        let elapsed_time = match current_time.elapsed() {
            Ok(elapsed) => elapsed,
            Err(_) => {
                eprintln!("Error getting elapsed time");
                return; 
            }
        };
        
        if elapsed_time >= Duration::from_secs(60) {
            store_monitors(monitors,processed_at); 
            break; 
        } else {
            thread::sleep(Duration::from_millis(1))
        }
    }
}
fn update_monitors(monitors: &mut Monitors,processed_at: u64) {
        for monitor in &mut monitors.monitors {
            let value = rand::thread_rng().gen_range(0..100);

            
            let result = Result {
                value,
                processed_at: processed_at as i64, 
            };
            monitor.result = Some(result);
        }
        thread::sleep(Duration::from_secs(30));

}

fn store_monitors(monitors: &Monitors,processed_at: u64) {


        let filename = format!("{}_monitors.json", processed_at);

        let json_output = match serde_json::to_string_pretty(&monitors) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Error serializing to JSON: {}", e);
                return;
            }
        };
        match fs::write(&filename, json_output) {
            Ok(_) => println!("Monitors stored in file: {}", filename),
            Err(e) => eprintln!("Error writing to file {}: {}", filename, e),
    }
    // thread::sleep(Duration::from_secs(10));
}

