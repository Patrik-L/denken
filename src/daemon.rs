use daemonize::Daemonize;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::{fs, fs::File, path::Path, thread, time};

pub fn is_started() -> bool {
    let pidexits = Path::new("/tmp/denken.pid").exists();
    if !pidexits {
        return false;
    }

    let pid = fs::read_to_string("/tmp/denken.pid").expect("Could not read pid file");
    let is_daemon_running = Path::new(&format!("/proc/{}/status", pid.trim())).exists();
    return is_daemon_running;
}

pub fn start_daemon() {
    let stdout = File::create("/tmp/denken.out").unwrap();
    let stderr = File::create("/tmp/denken.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/denken.pid")
        .chown_pid_file(false)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");
            loop {
                println!("juuuuuuuuuh, toimii");
                thread::sleep(time::Duration::from_secs(1))
            }
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}

pub fn stop_daemon() {
    let pid: String = fs::read_to_string("/tmp/denken.pid").expect("Unable to read file");

    let res = nix::sys::signal::kill(
        Pid::from_raw(pid.trim().parse().expect("Denken daemon pid not found")),
        Signal::SIGTERM,
    );

    match res {
        Ok(_) => {}
        Err(e) => println!("Failed printing process, {}", e),
    }
}
