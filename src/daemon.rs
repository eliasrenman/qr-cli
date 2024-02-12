use std::{env, process};

pub struct Daemon {}
// An argument that can be passed into the program to signal that it should daemonize itself. This
// can be anything as long as it is unlikely to be passed in by the user by mistake.
pub const DAEMONIZE_ARG: &str = "__internal_daemonize";

impl Daemon {
    pub fn in_daemon() -> bool {
        let args: Vec<String> = std::env::args().collect();
        return args.iter().any(|arg| arg == DAEMONIZE_ARG);
    }

    pub fn should_run_in_daemon() -> bool {
        cfg!(target_os = "linux") && !Daemon::in_daemon()
    }

    pub fn run_in_daemon() {
        let args: Vec<String> = std::env::args().collect();
        let mut child_process = process::Command::new(env::current_exe().unwrap());

        // Pass all the original arguments to the child process
        for arg in args.iter().skip(1) {
            child_process.arg(arg);
        }

        let _ = child_process
            .arg(DAEMONIZE_ARG)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn();
    }
}
