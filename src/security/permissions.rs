use std::os::unix::fs::PermissionsExt;
use anyhow::{Result, Context};

pub struct SecurityContext {
    uid: u32,
    gid: u32,
}

impl SecurityContext {
    pub fn new() -> Result<Self> {
        Ok(SecurityContext {
            uid: unsafe { libc::getuid() },
            gid: unsafe { libc::getgid() },
        })
    }

    pub fn has_sufficient_permissions(&self) -> Result<bool> {
        // Check if we're root
        if self.uid == 0 {
            return Ok(true);
        }

        // Check if we have read access to necessary log files
        let log_permissions = std::fs::metadata("/var/log")
            .context("Failed to get /var/log metadata")?
            .permissions();

        // Verify we have at least read permissions
        Ok(log_permissions.mode() & 0o444 != 0)
    }
}