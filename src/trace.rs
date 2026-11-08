use crate::adapter::Transport;
use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Open a trace file for appending; a new one is created readable by its owner only (it holds the VIN).
/// A file that cannot be opened is reported on stderr and skipped: a trace never stops a diagnosis.
pub fn open(path: &Path) -> Option<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options.open(path).map_err(|e| eprintln!("carlink: cannot write the trace file {} ({e}); the command goes on without it", path.display())).ok()
}

/// A transport that appends every command, its raw reply (or error) and how long it took to a file.
pub struct Trace<T: Transport> {
    inner: T,
    file: File,
    write_failed: bool,
}

impl<T: Transport> Trace<T> {
    pub fn new(inner: T, file: File, command: &str) -> Self {
        let mut t = Trace { inner, file, write_failed: false };
        let at = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let header = format!("# carlink {} at {at} (Unix time), simulated: {}, command: {command:?}\n", env!("CARGO_PKG_VERSION"), t.inner.simulated());
        t.write(&header);
        t
    }

    // A trace that cannot be written must never stop the diagnosis: warn once on stderr and go on.
    fn write(&mut self, line: &str) {
        if let Err(e) = self.file.write_all(line.as_bytes())
            && !self.write_failed
        {
            self.write_failed = true;
            eprintln!("carlink: the trace file could not be written ({e}); the command goes on without it");
        }
    }
}

impl<T: Transport> Transport for Trace<T> {
    fn exchange(&mut self, cmd: &str) -> Result<String> {
        self.write(&format!("> {cmd}\n"));
        let start = Instant::now();
        let r = self.inner.exchange(cmd);
        let secs = start.elapsed().as_secs_f64();
        let line = match &r {
            Ok(reply) => format!("< ({secs:.3} s) {reply:?}\n"),
            Err(e) => format!("! ({secs:.3} s) {}: {:?}\n", crate::problem::find(e).map_or("error", |p| p.kind), format!("{e:#}")),
        };
        self.write(&line);
        r
    }

    fn interrupt(&mut self) {
        self.write("! interrupt\n");
        self.inner.interrupt();
    }

    fn simulated(&self) -> bool {
        self.inner.simulated()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::Sim;

    fn traced(sim: Sim) -> (Trace<Sim>, tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trace.txt");
        (Trace::new(sim, open(&path).unwrap(), "probe"), dir, path)
    }

    #[test]
    fn each_run_starts_with_a_header_naming_the_command_and_whether_it_is_the_simulator() {
        let (_t, _dir, path) = traced(Sim::new("crv-2004").unwrap());
        let log = std::fs::read_to_string(&path).unwrap();
        assert!(log.starts_with("# carlink ") && log.contains("probe") && log.contains("simulated: true"), "{log}");
    }

    #[test]
    fn a_new_trace_file_is_readable_by_its_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let (_t, _dir, path) = traced(Sim::new("crv-2004").unwrap());
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
    }

    #[test]
    fn a_trace_file_that_cannot_be_opened_is_skipped() {
        assert!(open(std::path::Path::new("/nonexistent-dir/trace.txt")).is_none());
    }

    #[test]
    fn a_newline_in_the_command_cannot_forge_a_trace_record() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trace.txt");
        Trace::new(Sim::new("crv-2004").unwrap(), open(&path).unwrap(), "scan --note x\n> 0100");
        assert_eq!(std::fs::read_to_string(&path).unwrap().lines().count(), 1);
    }

    /// Reads the trace file while its exchange is still running.
    struct Peek {
        path: std::path::PathBuf,
        saw_request: bool,
    }

    impl Transport for Peek {
        fn exchange(&mut self, cmd: &str) -> Result<String> {
            self.saw_request = std::fs::read_to_string(&self.path).unwrap().contains(&format!("> {cmd}"));
            Ok("OK\r\r>".into())
        }
    }

    #[test]
    fn the_request_is_in_the_trace_before_the_adapter_answers() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trace.txt");
        let mut t = Trace::new(Peek { path: path.clone(), saw_request: false }, open(&path).unwrap(), "probe");
        t.exchange("ATI").unwrap();
        assert!(t.inner.saw_request);
    }

    #[test]
    fn a_trace_that_cannot_be_written_does_not_stop_the_exchange() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("trace.txt");
        std::fs::write(&path, "").unwrap();
        let read_only = std::fs::File::open(&path).unwrap();
        let mut t = Trace::new(Sim::new("crv-2004").unwrap(), read_only, "probe");
        assert!(t.exchange("ATI").is_ok());
        assert!(t.write_failed);
    }

    #[test]
    fn every_exchange_is_appended_with_its_raw_reply() {
        let (mut t, _dir, path) = traced(Sim::new("crv-2004").unwrap());
        t.exchange("ATI").unwrap();
        let log = std::fs::read_to_string(&path).unwrap();
        assert!(log.contains("> ATI") && log.contains(r#""ELM327 v1.4b\r\r>""#), "{log}");
    }

    #[test]
    fn a_failed_exchange_is_traced_with_its_error() {
        let mut sim = Sim::new("crv-2004").unwrap();
        sim.fail("0100", "adapter_timeout", 1);
        let (mut t, _dir, path) = traced(sim);
        assert!(t.exchange("0100").is_err());
        let log = std::fs::read_to_string(&path).unwrap();
        assert!(log.contains("> 0100") && log.contains("! ") && log.contains("adapter_timeout"), "{log}");
    }
}
