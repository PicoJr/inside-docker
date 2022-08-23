//! # inside-docker
//!
//! Detect if code is running inside a docker container.
//!
//! ## How does it work
//!
//! Check the filesystem type of `/`, if it is [OverlayFS](https://en.wikipedia.org/wiki/OverlayFS) assume the code
//! is running inside a Docker container.
//!
//! ## Quick Start
//!
//! ```
//! use inside_docker::inside_docker;
//! let inside: bool = inside_docker() == Some(true);
//! ```

use std::path::Path;
use nix::sys::statfs::OVERLAYFS_SUPER_MAGIC;

/// Detect if inside a Docker container by checking `/` filesystem type using statfs.
///
/// Return Some(bool) if filesystem was performed successfully, None otherwise.
/// ```
/// use inside_docker::inside_docker;
/// let inside: bool = inside_docker() == Some(true);
/// ```
///
pub fn inside_docker() -> Option<bool> {
    let root_fs_statfs = nix::sys::statfs::statfs(Path::new("/"));
    match root_fs_statfs {
        Ok(statfs) => Some(statfs.filesystem_type() == OVERLAYFS_SUPER_MAGIC),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use crate::inside_docker;

    #[test]
    fn test_inside_docker() {
        assert_eq!(inside_docker(), Some(false)); // may fail if test is run inside a Docker container
    }
}
