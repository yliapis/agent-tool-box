use anyhow::Result;

use crate::discover::{WalkMode, walk_files};
use crate::model::{Artifact, Catalog, FileOp, Kind, SyncPlan, Target};

pub trait Adapter {
    fn plan(&self, artifacts: &[Artifact], target: &Target) -> Result<Vec<FileOp>>;
}

pub struct CopyAdapter;

impl Adapter for CopyAdapter {
    fn plan(&self, artifacts: &[Artifact], target: &Target) -> Result<Vec<FileOp>> {
        let mut ops = Vec::new();
        for artifact in artifacts {
            match artifact.kind {
                Kind::Skill => ops.extend(skill_copies(artifact, target)?),
                Kind::Command | Kind::Agent => ops.push(FileOp::Copy {
                    from: artifact.source.clone(),
                    to: target.output.join(&artifact.id),
                }),
            }
        }
        Ok(ops)
    }
}

pub fn plan(catalog: &Catalog, targets: &[Target]) -> Result<Vec<SyncPlan>> {
    let adapter = CopyAdapter;
    targets
        .iter()
        .map(|target| {
            Ok(SyncPlan {
                target: target.clone(),
                ops: adapter.plan(&catalog.artifacts, target)?,
            })
        })
        .collect()
}

fn skill_copies(artifact: &Artifact, target: &Target) -> Result<Vec<FileOp>> {
    let dest_root = target.output.join(&artifact.id);
    let files = walk_files(&artifact.source, WalkMode::Expand)?;
    Ok(files
        .into_iter()
        .filter_map(|from| {
            let rel = from.strip_prefix(&artifact.source).ok()?;
            Some(FileOp::Copy {
                from: from.clone(),
                to: dest_root.join(rel),
            })
        })
        .collect())
}
