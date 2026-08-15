use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use agent_tool_box::{FileOp, Kind, Target, apply, discover, plan};

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn skill_tree(root: &Path) {
    write(
        &root.join("plugins/core/skills/alpha/SKILL.md"),
        "---\nname: alpha\ndescription: \"Alpha skill\"\n---\n\n# Alpha\n",
    );
    write(
        &root.join("plugins/extra/skills/beta/SKILL.md"),
        "---\nname: beta\ndescription: \"Beta skill\"\n---\n\n# Beta\n",
    );
    write(
        &root.join("plugins/extra/skills/beta/references/note.md"),
        "# note\n",
    );
}

fn dests(plans: &[agent_tool_box::SyncPlan]) -> Vec<PathBuf> {
    plans
        .iter()
        .flat_map(|p| &p.ops)
        .map(|FileOp::Copy { to, .. }| to.clone())
        .collect()
}

#[test]
fn discover_plan_apply_two_skills() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    skill_tree(&root);

    let catalog = discover(&root, Kind::Skill).unwrap();
    let ids: Vec<&str> = catalog.artifacts.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["alpha", "beta"]);

    let plans = plan(
        &catalog,
        &[Target {
            tool: None,
            output: dst.clone(),
        }],
    );
    let to = dests(&plans);
    assert!(to.contains(&dst.join("alpha/SKILL.md")));
    assert!(to.contains(&dst.join("beta/SKILL.md")));

    apply(&plans).unwrap();
    assert_eq!(
        fs::read_to_string(dst.join("alpha/SKILL.md")).unwrap(),
        "---\nname: alpha\ndescription: \"Alpha skill\"\n---\n\n# Alpha\n",
    );
    assert_eq!(
        fs::read_to_string(dst.join("beta/SKILL.md")).unwrap(),
        "---\nname: beta\ndescription: \"Beta skill\"\n---\n\n# Beta\n",
    );
    assert_eq!(
        fs::read_to_string(dst.join("beta/references/note.md")).unwrap(),
        "# note\n",
    );
}

#[test]
fn discover_and_plan_command() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("src");
    let dst = tmp.path().join("dst");
    write(
        &root.join("plugins/writing/commands/critique.md"),
        "---\ndescription: \"Critique a thing\"\n---\n\nDo the critique.\n",
    );

    let catalog = discover(&root, Kind::Command).unwrap();
    let ids: Vec<&str> = catalog.artifacts.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["critique.md"]);

    let plans = plan(
        &catalog,
        &[Target {
            tool: None,
            output: dst.clone(),
        }],
    );
    assert!(dests(&plans).contains(&dst.join("critique.md")));
}

#[test]
fn discover_zero_matches_names_marker_and_search_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("empty");
    fs::create_dir_all(&root).unwrap();
    write(&root.join("readme.md"), "nothing here\n");

    let err = discover(&root, Kind::Skill).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("**/skills/*/SKILL.md"), "{msg}");
    assert!(
        msg.contains("ai-coding") && msg.contains("ai-coding/plugins"),
        "{msg}"
    );
    assert!(msg.contains("not ai-coding/skills"), "{msg}");
}

#[test]
fn discover_duplicate_ids_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("src");
    write(
        &root.join("plugins/a/skills/foo/SKILL.md"),
        "---\nname: foo\n---\n\n# A\n",
    );
    write(
        &root.join("plugins/b/skills/foo/SKILL.md"),
        "---\nname: foo\n---\n\n# B\n",
    );

    let err = discover(&root, Kind::Skill).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("duplicate"), "{msg}");
    assert!(msg.contains("foo"), "{msg}");
}

#[test]
fn discover_nested_skill_md_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("src");
    write(
        &root.join("skills/foo/SKILL.md"),
        "---\nname: foo\n---\n\n# Foo\n",
    );
    write(
        &root.join("skills/foo/nested/SKILL.md"),
        "---\nname: nested\n---\n\n# Nested\n",
    );

    let err = discover(&root, Kind::Skill).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("nested"), "{msg}");
    assert!(msg.contains("SKILL.md"), "{msg}");
}

#[test]
fn config_flag_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_atb"))
        .args(["sync", "--config", "sync.yaml"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--config"), "{stderr}");
}
