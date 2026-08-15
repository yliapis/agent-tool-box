mod adapter;
mod apply;
pub mod config;
mod discover;
mod model;

pub use adapter::{Adapter, CopyAdapter, plan};
pub use apply::apply;
pub use discover::discover;
pub use model::{
    Artifact, ArtifactMeta, Catalog, Distribution, FileOp, Kind, SyncPlan, Target, Tool,
};
