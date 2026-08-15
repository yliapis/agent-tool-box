use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

use crate::model::{Artifact, ArtifactMeta, Catalog, Kind};

pub fn discover(root: &Path, kind: Kind) -> Result<Catalog> {
    let files = walk_files(root)?;
    let mut artifacts = match kind {
        Kind::Skill => discover_skills(&files)?,
        Kind::Command => discover_files(kind, &files, "commands"),
        Kind::Agent => discover_files(kind, &files, "agents"),
    };

    if artifacts.is_empty() {
        bail!(
            "no {} artifacts found (marker {}); search root should be ai-coding or ai-coding/plugins, not ai-coding/skills",
            kind.as_str(),
            kind.marker(),
        );
    }

    artifacts.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.source.cmp(&b.source)));
    check_duplicate_ids(kind, &artifacts)?;

    Ok(Catalog {
        kind,
        root: root.to_path_buf(),
        artifacts,
    })
}

fn walk_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(true) {
        let entry = entry.with_context(|| format!("walk {}", root.display()))?;
        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }
    Ok(files)
}

fn discover_skills(files: &[PathBuf]) -> Result<Vec<Artifact>> {
    let skill_mds: Vec<&Path> = files
        .iter()
        .map(PathBuf::as_path)
        .filter(|path| file_name_is(path, "SKILL.md"))
        .collect();

    let matches: Vec<&Path> = skill_mds
        .iter()
        .copied()
        .filter(|path| is_skill_marker(path))
        .collect();

    for path in &skill_mds {
        if is_skill_marker(path) {
            continue;
        }
        for marker in &matches {
            let skill_dir = marker.parent().expect("SKILL.md has a parent");
            if path.starts_with(skill_dir) {
                let id = skill_dir
                    .file_name()
                    .expect("skill dir has a name")
                    .to_string_lossy();
                bail!(
                    "nested SKILL.md at {} beneath skill `{id}` ({})",
                    path.display(),
                    skill_dir.display(),
                );
            }
        }
    }

    Ok(matches
        .into_iter()
        .map(|marker| {
            let source = marker
                .parent()
                .expect("SKILL.md has a parent")
                .to_path_buf();
            let id = source
                .file_name()
                .expect("skill dir has a name")
                .to_string_lossy()
                .into_owned();
            Artifact {
                kind: Kind::Skill,
                id,
                meta: read_frontmatter(marker),
                source,
            }
        })
        .collect())
}

fn discover_files(kind: Kind, files: &[PathBuf], dir_name: &str) -> Vec<Artifact> {
    files
        .iter()
        .filter(|path| is_direct_kind_file(path, dir_name))
        .map(|source| {
            let id = source
                .file_name()
                .expect("kind file has a name")
                .to_string_lossy()
                .into_owned();
            Artifact {
                kind,
                id,
                meta: read_frontmatter(source),
                source: source.clone(),
            }
        })
        .collect()
}

fn is_skill_marker(path: &Path) -> bool {
    file_name_is(path, "SKILL.md")
        && path
            .parent()
            .and_then(|skill_dir| skill_dir.parent())
            .is_some_and(|skills| file_name_is(skills, "skills"))
}

fn is_direct_kind_file(path: &Path, dir_name: &str) -> bool {
    path.extension().is_some_and(|ext| ext == "md")
        && path
            .parent()
            .is_some_and(|parent| file_name_is(parent, dir_name))
}

fn file_name_is(path: &Path, name: &str) -> bool {
    path.file_name().is_some_and(|n| n == name)
}

fn check_duplicate_ids(kind: Kind, artifacts: &[Artifact]) -> Result<()> {
    let mut first: HashMap<&str, &Path> = HashMap::new();
    for artifact in artifacts {
        if let Some(other) = first.get(artifact.id.as_str()) {
            bail!(
                "duplicate {} id `{}`: {} and {}",
                kind.as_str(),
                artifact.id,
                other.display(),
                artifact.source.display(),
            );
        }
        first.insert(&artifact.id, &artifact.source);
    }
    Ok(())
}

fn read_frontmatter(path: &Path) -> ArtifactMeta {
    let Ok(text) = fs::read_to_string(path) else {
        return ArtifactMeta::default();
    };
    let Some(yaml) = extract_frontmatter(&text) else {
        return ArtifactMeta::default();
    };
    ArtifactMeta {
        name: yaml_scalar(yaml, "name"),
        description: yaml_scalar(yaml, "description"),
    }
}

fn extract_frontmatter(text: &str) -> Option<&str> {
    let start = text.find("---")?;
    let after_open = &text[start + 3..];
    let after_open = after_open
        .strip_prefix('\n')
        .or_else(|| after_open.strip_prefix("\r\n"))
        .unwrap_or(after_open);
    let end = after_open
        .find("\n---")
        .or_else(|| after_open.find("\r\n---"))?;
    Some(&after_open[..end])
}

fn yaml_scalar(yaml: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}:");
    for line in yaml.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(&prefix) else {
            continue;
        };
        let rest = rest.trim();
        if rest.is_empty() {
            return None;
        }
        return Some(unquote(rest));
    }
    None
}

fn unquote(value: &str) -> String {
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        return unescape_double(&value[1..value.len() - 1]);
    }
    if value.len() >= 2 && value.starts_with('\'') && value.ends_with('\'') {
        return value[1..value.len() - 1].to_string();
    }
    value.to_string()
}

fn unescape_double(inner: &str) -> String {
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}
