use std::path::Path;

use eyre::Context;

use crate::{TARGET_DIR, ui};

pub(crate) fn clean(project_dir: &Path) -> eyre::Result<()> {
    let target = project_dir.join(TARGET_DIR);
    if !target.exists() {
        ui::success("cleaned");
        return Ok(());
    }

    for entry in std::fs::read_dir(&target)
        .with_context(|| format!("could not read target dir {}", target.display()))?
    {
        let entry = entry.context("could not read target dir entry")?;
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str()) == Some(crate::utils::TARGET_LOCK_FILE_NAME) {
            continue;
        }

        let file_type = entry
            .file_type()
            .with_context(|| format!("could not inspect {}", path.display()))?;
        if file_type.is_dir() {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("could not remove {}", path.display()))?;
        } else {
            std::fs::remove_file(&path)
                .with_context(|| format!("could not remove {}", path.display()))?;
        }
    }

    ui::success("cleaned");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::clean;
    use std::{
        path::{Path, PathBuf},
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("mond-clean-test-{}-{nanos}", std::process::id()))
    }

    fn cleanup_temp_root(root: &Path) {
        for _ in 0..5 {
            match std::fs::remove_dir_all(root) {
                Ok(()) => return,
                Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(err) => panic!("cleanup temp root: {err}"),
            }
        }
        std::fs::remove_dir_all(root).expect("cleanup temp root");
    }

    #[test]
    fn clean_removes_target_contents_but_preserves_lock_file() {
        let root = unique_temp_root();
        let target = root.join(crate::TARGET_DIR);
        let debug_dir = target.join("debug");
        std::fs::create_dir_all(&debug_dir).expect("create target debug dir");
        std::fs::write(
            target.join(crate::utils::TARGET_LOCK_FILE_NAME),
            "lock placeholder",
        )
        .expect("create lock file");
        std::fs::write(debug_dir.join("artifact.beam"), "beam").expect("create artifact");
        std::fs::write(target.join("stale.txt"), "stale").expect("create stale file");

        clean(&root).expect("clean target");

        assert!(
            target.join(crate::utils::TARGET_LOCK_FILE_NAME).exists(),
            "lock file should be preserved"
        );
        assert!(!debug_dir.exists(), "debug dir should be removed");
        assert!(
            !target.join("stale.txt").exists(),
            "stale file should be removed"
        );

        cleanup_temp_root(&root);
    }
}
