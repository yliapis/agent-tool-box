use walkdir::WalkDir;

use crate::model::{Artifact, Catalog, FileOp, Kind, SyncPlan, Target};

pub trait Adapter {
    fn plan(&self, artifacts: &[Artifact], target: &Target) -> Vec<FileOp>;
}

pub struct CopyAdapter;

impl Adapter for CopyAdapter {
    fn plan(&self, artifacts: &[Artifact], target: &Target) -> Vec<FileOp> {
        let mut ops = Vec::new();
        for artifact in artifacts {
            match artifact.kind {
                Kind::Skill => ops.extend(skill_copies(artifact, target)),
                Kind::Command | Kind::Agent => ops.push(FileOp::Copy {
                    from: artifact.source.clone(),
                    to: target.output.join(&artifact.id),
                }),
            }
        }
        ops
    }
}

pub fn plan(catalog: &Catalog, targets: &[Target]) -> Vec<SyncPlan> {
    let adapter = CopyAdapter;
    targets
        .iter()
        .map(|target| SyncPlan {
            target: target.clone(),
            ops: adapter.plan(&catalog.artifacts, target),
        })
        .collect()
}

fn skill_copies(artifact: &Artifact, target: &Target) -> Vec<FileOp> {
    let dest_root = target.output.join(&artifact.id);
    WalkDir::new(&artifact.source)
        .follow_links(true)
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let rel = entry.path().strip_prefix(&artifact.source).ok()?;
            Some(FileOp::Copy {
                from: entry.path().to_path_buf(),
                to: dest_root.join(rel),
            })
        })
        .collect()
}
