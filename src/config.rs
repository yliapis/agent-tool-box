use std::path::PathBuf;

use crate::model::{Distribution, Kind, Target};

pub fn from_flags(src: PathBuf, dst: PathBuf, kind: Kind) -> Distribution {
    Distribution {
        kind,
        root: src,
        targets: vec![Target {
            tool: None,
            output: dst,
        }],
    }
}
