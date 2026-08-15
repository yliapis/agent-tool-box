use std::fs;

use anyhow::{Context, Result};

use crate::model::{FileOp, SyncPlan};

pub fn apply(plans: &[SyncPlan]) -> Result<()> {
    for plan in plans {
        for op in &plan.ops {
            let FileOp::Copy { from, to } = op;
            println!("{} -> {}", from.display(), to.display());
        }
    }

    for plan in plans {
        for op in &plan.ops {
            let FileOp::Copy { from, to } = op;
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("create {}", parent.display()))?;
            }
            fs::copy(from, to)
                .with_context(|| format!("copy {} -> {}", from.display(), to.display()))?;
        }
    }
    Ok(())
}
