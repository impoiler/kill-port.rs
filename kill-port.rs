use std::env;
use std::process::Command;
use std::process;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if port number is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <port_number>", args[0]);
        process::exit(1);
    }

    // Parse port number
    let port = match args[1].parse::<u16>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Please provide a valid port number");
            process::exit(1);
        }
    };

    // Find process using the port
    let lsof_output = Command::new("lsof")
        .args(["-i", &format!("tcp:{}", port)])
        .output()
        .expect("Failed to execute lsof command");

    if !lsof_output.status.success() {
        println!("No process found running on port {}", port);
        return;
    }

    // Parse the output to get PID
    let output = String::from_utf8_lossy(&lsof_output.stdout);
    let lines: Vec<&str> = output.lines().collect();
    
    if lines.len() < 2 {
        println!("No process found running on port {}", port);
        return;
    }

    // Extract PIDs (skip the header line)
    for line in lines.iter().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let pid = parts[1];
            
            // Kill the process
            let kill_output = Command::new("kill")
                .arg("-9")
                .arg(pid)
                .output()
                .expect("Failed to execute kill command");

            if kill_output.status.success() {
                println!("Successfully killed process {} running on port {}", pid, port);
            } else {
                eprintln!("Failed to kill process {}", pid);
            }
        }
    }
}
