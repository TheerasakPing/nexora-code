//! Conservative cross-team file ownership preflight for the native scheduler.
//!
//! Scope declarations are coordination hints, NOT a filesystem sandbox.
//! Unsafe/malformed or symlinked scopes fail closed for concurrent writers.
use std::fs;
use std::path::Path;

use super::Worker;

/// Compare two workers' effective write targets by their actual workspace roots.
/// Unlike root equality, this catches a parent checkout and a nested checkout
/// reserving the same physical files. No side effects or lease mutations.
pub(super) fn conflicts_between(
    first_root: &Path,
    first: &Worker,
    second_root: &Path,
    second: &Worker,
) -> bool {
    if first.read_only || second.read_only {
        return false;
    }
    let (Some(first_paths), Some(second_paths)) = (
        targets(first_root, &first.files),
        targets(second_root, &second.files),
    ) else {
        // Cannot prove disjoint ownership: never admit concurrent writers.
        return true;
    };
    first_paths
        .iter()
        .any(|first| second_paths.iter().any(|second| overlaps(first, second)))
}

fn targets(root: &Path, scopes: &[String]) -> Option<Vec<String>> {
    // Workspace roots must already exist and be real checkout directories.
    // Canonicalizing resolves a registered root symlink, if any.
    let root = root.canonicalize().ok()?;
    if !root.is_dir() {
        return None;
    }
    if scopes.len() > 24 {
        return None;
    }
    if scopes.is_empty() {
        return Some(vec![fold(&root)]);
    }
    let mut targets = Vec::with_capacity(scopes.len());
    for scope in scopes {
        if !valid_scope(scope) {
            return None;
        }
        let mut current = root.clone();
        let mut ancestor_missing = false;
        for segment in scope.split('/') {
            current.push(segment);
            if ancestor_missing {
                // Preserve ALL remaining path components even when this
                // future file/directory does not exist yet. An absent parent
                // is not evidence that sibling scopes are identical.
                continue;
            }
            // Existing symlinks can alias another tree or escape the scope.
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_symlink() => return None,
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    ancestor_missing = true;
                }
                Err(_) => return None,
            }
        }
        targets.push(fold(&current));
    }
    Some(targets)
}

fn valid_scope(scope: &str) -> bool {
    !scope.is_empty()
        && scope.len() <= 512
        && !scope.contains(['\\', '\0', ':', '*', '?', '[', ']'])
        && !scope.starts_with('/')
        && scope
            .split('/')
            .all(|part| !matches!(part, "" | "." | ".."))
}

fn fold(path: &Path) -> String {
    // Lowercase conservatively even on case-sensitive hosts. May serialize
    // independent paths, never permits concurrent conflicting writes.
    let text = path.to_string_lossy().replace('\\', "/").to_lowercase();
    text.trim_end_matches('/').to_owned()
}

fn overlaps(first: &str, second: &str) -> bool {
    first == second
        || first
            .strip_prefix(second)
            .is_some_and(|suffix| suffix.starts_with('/'))
        || second
            .strip_prefix(first)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn fixture() -> TempDir {
        let path = std::env::temp_dir().join(format!("nexora-ownership-{}", crate::storage::id()));
        fs::create_dir(&path).unwrap();
        TempDir(path)
    }

    fn worker(scopes: &[&str], read_only: bool) -> Worker {
        Worker {
            id: "owner".into(),
            session_id: "worker-session".into(),
            name: "Worker".into(),
            profile_id: "coder".into(),
            provider: "codex".into(),
            model: "default".into(),
            reasoning_effort: None,
            status: "running".into(),
            assignment: "edit scoped files".into(),
            summary: String::new(),
            error: None,
            depends_on: Vec::new(),
            files: scopes.iter().map(|s| (*s).into()).collect(),
            read_only,
            attempt: 0,
            created_at: 0,
            updated_at: 0,
            keys: Vec::new(),
            abandoned: false,
            result_revision: 0,
            checkpoint: None,
            timed_out: false,
        }
    }
    fn check(
        first_root: &Path,
        first_scopes: &[&str],
        second_root: &Path,
        second_scopes: &[&str],
    ) -> bool {
        conflicts_between(
            first_root,
            &worker(first_scopes, false),
            second_root,
            &worker(second_scopes, false),
        )
    }

    #[test]
    fn nested_checkouts_conflict_only_for_overlapping_effective_paths() {
        let folder = fixture();
        let top = folder.path();
        let nested = top.join("app");
        fs::create_dir(&nested).unwrap();
        assert!(check(top, &["app/src"], &nested, &["src/main.rs"]));
        assert!(check(top, &[], &nested, &["src/main.rs"]));
        assert!(check(top, &["app"], &nested, &[]));
        assert!(!check(top, &["docs"], &nested, &["src"]));
        assert!(!check(top, &["app/alpha"], &nested, &["alphabet"]));
    }

    #[test]
    fn physically_separate_worktrees_remain_parallel() {
        let folder = fixture();
        let a = folder.path().join("repo-one");
        let b = folder.path().join("repo-two");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        assert!(!check(&a, &[], &b, &[]));
        assert!(!check(&a, &["src/main.rs"], &b, &["src/main.rs"]));
    }

    #[test]
    fn case_variants_and_ancestor_scopes_fail_closed() {
        let folder = fixture();
        let root = folder.path();
        assert!(check(root, &["SRC"], root, &["src/main.rs"]));
        assert!(check(root, &["src/main.rs"], root, &["SRC/MAIN.RS"]));
        assert!(!check(root, &["src/a"], root, &["src/ab"]));
    }

    #[test]
    fn read_only_access_does_not_reserve_write_ownership() {
        let folder = fixture();
        let root = folder.path();
        let writer = worker(&[], false);
        let reader = worker(&[], true);
        assert!(!conflicts_between(root, &writer, root, &reader));
        assert!(!conflicts_between(root, &reader, root, &writer));
        assert!(!conflicts_between(root, &reader, root, &reader));
    }

    #[test]
    fn malformed_or_missing_scopes_cannot_bypass_writer_conflicts() {
        let folder = fixture();
        let root = folder.path();
        for bad in [
            "../out",
            "src/../out",
            ".",
            "/tmp/out",
            "C:drive",
            "src\\win",
            "src/*",
            "src//bad",
        ] {
            assert!(check(root, &[bad], root, &["docs"]));
        }
        assert!(check(&root.join("missing"), &["src"], root, &["docs"]));
        assert!(check(root, &["src"], &root.join("missing"), &["docs"]));
    }

    #[cfg(unix)]
    #[test]
    fn symlink_scopes_are_not_considered_safe_for_parallel_writers() {
        use std::os::unix::fs::symlink;
        let folder = fixture();
        let root = folder.path();
        fs::create_dir(root.join("actual")).unwrap();
        symlink(root.join("actual"), root.join("link")).unwrap();
        assert!(check(root, &["link/file.rs"], root, &["docs"]));
    }

    #[test]
    fn more_than_24_or_oversized_legacy_scopes_fail_closed() {
        let folder = fixture();
        let root = folder.path();
        let mut claim = worker(&["src"], false);
        claim.files = (0..25).map(|i| format!("src/file-{i}")).collect();
        assert!(conflicts_between(
            root,
            &claim,
            root,
            &worker(&["docs"], false)
        ));
        claim.files = vec!["a".repeat(513)];
        assert!(conflicts_between(
            root,
            &claim,
            root,
            &worker(&["docs"], false)
        ));
    }
}
