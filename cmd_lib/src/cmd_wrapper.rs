use crate::json::command::Command as Cmd;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Child, Command, ExitStatus, Stdio},
    thread,
};

/// Wrapper struct to combine std::process::Command and json::command::Command
///
/// * `cmd` - std::process::Command
/// * `json_cmd` - cmd_lib::json::command::Command
pub struct CmdWrapper {
    cmd: Command,
    pub json_cmd: Cmd,
}

impl CmdWrapper {
    /// Constructs a new CmdWrapper by consuming json_cmd
    ///
    /// * `json_cmd` - cmd_lib::json::command::Command
    pub fn new(json_cmd: Cmd) -> Self {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(json_cmd.command.clone());
        CmdWrapper {
            cmd,
            json_cmd: json_cmd.clone(),
        }
    }

    /// Execute the interal std::process::Command
    ///
    /// * `show-verbose` - ```true``` for verbose output of command
    pub fn execute(&mut self, show_verbose: bool) -> ExitStatus {
        println!("{}", self.json_cmd.message);
        let mut proc_handle: Child;
        if show_verbose {
            //run command with full output
            self.cmd.stdout(Stdio::piped());
            self.cmd.stderr(Stdio::piped());
            proc_handle = self.cmd.spawn().unwrap();

            let stdout_reader = BufReader::new(proc_handle.stdout.take().unwrap());
            let stderr_reader = BufReader::new(proc_handle.stderr.take().unwrap());

            //spawn thread to handle stderr
            let mut error_lines: Vec<String> = Vec::new();
            let err_thread = thread::spawn(move || -> Vec<String> {
                stderr_reader.lines().for_each(|line| {
                    let line = line.unwrap();
                    println!("{}", line);
                    error_lines.push(line);
                });
                error_lines
            });

            //spawn thread to handle stdout
            let out_thread = thread::spawn(move || {
                stdout_reader
                    .lines()
                    .for_each(|line| println!("{}", line.unwrap()));
            });

            out_thread.join().unwrap();
            let error_lines = err_thread.join().unwrap();

            self.check_for_errors(proc_handle, Some(error_lines))
        } else {
            //run commands without verbose output
            proc_handle = self.cmd.stdout(Stdio::null()).spawn().unwrap();
            self.check_for_errors(proc_handle, None)
        }
    }

    /// Checks executed command for errors using Child handle
    ///
    /// * `handle` - ```Child``` handle
    /// * `lines` - lines from stderr ```Option<Vec<String>>```
    fn check_for_errors(&mut self, mut handle: Child, lines: Option<Vec<String>>) -> ExitStatus {
        let status = handle.wait().unwrap();
        if !status.success() {
            println!("Error running command, please view logs!");
            self.log_errors(lines);
        }
        status
    }

    /// Logs errors to error_log file
    /// * `lines` - lines from stderr ```Option<Vec<String>>```
    fn log_errors(&mut self, lines: Option<Vec<String>>) {
        let mut error_log = OpenOptions::new()
            .append(true)
            .create(true)
            .open("error_log")
            .unwrap();

        //write index of command to file
        writeln!(
            error_log,
            "Error executing command:  {}",
            self.json_cmd.message
        )
        .unwrap();

        //log stderr to file if available
        if let Some(lines) = lines {
            lines
                .into_iter()
                .for_each(|line| writeln!(error_log, "{}", line).unwrap())
        }
    }
}
