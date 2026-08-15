use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tool {
    Claude,
    Cursor,
    Codex,
    OpenCode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    Skill,
    Command,
    Agent,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Skill => "skill",
            Kind::Command => "command",
            Kind::Agent => "agent",
        }
    }

    pub fn marker(self) -> &'static str {
        match self {
            Kind::Skill => "**/skills/*/SKILL.md",
            Kind::Command => "**/commands/*.md",
            Kind::Agent => "**/agents/*.md",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    pub kind: Kind,
    pub id: String,
    pub source: PathBuf,
    pub meta: ArtifactMeta,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ArtifactMeta {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Catalog {
    pub kind: Kind,
    pub root: PathBuf,
    pub artifacts: Vec<Artifact>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Target {
    pub tool: Option<Tool>,
    pub output: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Distribution {
    pub kind: Kind,
    pub root: PathBuf,
    pub targets: Vec<Target>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileOp {
    Copy { from: PathBuf, to: PathBuf },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncPlan {
    pub target: Target,
    pub ops: Vec<FileOp>,
}
