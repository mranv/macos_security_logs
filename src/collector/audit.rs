use crate::models::log_entry::AuditLogEntry;
use anyhow::Result;
use tokio::process::Command;

pub async fn collect_audit_logs() -> Result<Vec<AuditLogEntry>> {
    let output = Command::new("praudit")
        .arg("-l")
        .arg("/var/audit/current")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to collect audit logs"));
    }

    // Parse audit log output into structured format
    let log_string = String::from_utf8_lossy(&output.stdout);
    let mut audit_logs = Vec::new();

    for line in log_string.lines() {
        // Basic parsing - you might want to enhance this based on your needs
        let now = chrono::Utc::now();
        audit_logs.push(AuditLogEntry {
            timestamp: now,
            event_type: "audit".to_string(),
            message: line.to_string(),
            raw_data: line.to_string(),
        });
    }

    Ok(audit_logs)
}