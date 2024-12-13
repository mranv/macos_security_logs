use crate::models::log_entry::LogEntry;
use crate::utils::error::CollectorError;
use anyhow::Result;
use tokio::process::Command;

pub struct UnifiedLogCollector {
    predicate: String,
    max_entries: usize,
}

impl UnifiedLogCollector {
    pub fn new(predicate: &str, max_entries: usize) -> Self {
        Self {
            predicate: predicate.to_string(),
            max_entries,
        }
    }

    pub async fn collect_logs(&self) -> Result<Vec<LogEntry>> {
        let output = Command::new("log")
            .arg("show")
            .arg("--predicate")
            .arg(&self.predicate)
            .arg("--style")
            .arg("json")
            .arg("--last")
            .arg(self.max_entries.to_string())
            .output()
            .await?;

        if !output.status.success() {
            return Err(CollectorError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ).into());
        }

        let logs: Vec<LogEntry> = serde_json::from_slice(&output.stdout)?;
        Ok(logs)
    }
}