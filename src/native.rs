use crate::view::*;
use nddev_device_sync_adapter_io::{CancellationToken, IoError, ProcessIo, ProcessRequest};

use std::{path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct Cleaner {
    io: ProcessIo,
    executable: PathBuf,
}

impl Cleaner {
    pub fn new(io: ProcessIo, executable: PathBuf) -> Result<Self, IoError> {
        if !executable.is_absolute() {
            return Err(IoError::InvalidConfiguration);
        }
        Ok(Self { io, executable })
    }

    fn request(&self, args: &[&str]) -> ProcessRequest {
        ProcessRequest {
            executable: self.executable.clone(),
            arguments: args.iter().map(Into::into).collect(),
            directory: None,
            timeout: Duration::from_secs(30),
            max_output_bytes: 1_048_576,
        }
    }

    pub async fn diagnose(&self, cancellation: CancellationToken) -> Result<Diagnostics, IoError> {
        let (report, exit): (Diagnostics, _) = self
            .io
            .json(self.request(&["doctor", "--json"]), &[0, 1], cancellation)
            .await?;
        if !valid_owner(&report.tool, &report.version)
            || report.checks.len() > 256
            || report.errors
                != report
                    .checks
                    .iter()
                    .filter(|check| check.state == CheckState::Error)
                    .count()
            || report.warnings
                != report
                    .checks
                    .iter()
                    .filter(|check| check.state == CheckState::Warning)
                    .count()
            || exit != i32::from(report.errors > 0)
        {
            return Err(IoError::InvalidResponse);
        }
        Ok(report)
    }

    pub async fn preview(
        &self,
        cancellation: CancellationToken,
    ) -> Result<CleanerRunSummary, IoError> {
        self.report(&["run", "--dry-run", "--json"], true, cancellation)
            .await
    }

    pub async fn last_run(
        &self,
        cancellation: CancellationToken,
    ) -> Result<CleanerRunSummary, IoError> {
        self.report(&["status"], false, cancellation).await
    }

    async fn report(
        &self,
        args: &[&str],
        preview: bool,
        cancellation: CancellationToken,
    ) -> Result<CleanerRunSummary, IoError> {
        let (report, _): (CleanerRunSummary, _) =
            self.io.json(self.request(args), &[0], cancellation).await?;
        if !valid_owner(&report.tool, &report.version) || report.dry_run != preview {
            return Err(IoError::InvalidResponse);
        }
        Ok(report)
    }
}

fn valid_owner(tool: &str, version: &str) -> bool {
    tool == "rldyour-cleaner" && !version.is_empty() && version.len() <= 64
}
