// Copyright (C) 2026 Slicer Project Generators contributors
// Adapted 2026-10-08 from onshape-export's MIT-licensed neutral settings v2
// contract at 793ac0d5ab8e1db1ff6183fc651a568e5bacfe34. See neutral/LICENSE-MIT.
use std::collections::HashSet;

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

use crate::{
    cache_key,
    generator_protocol::{InputManifest, InputRole},
};

pub const MAX_SETTINGS_BYTES: usize = 1_048_576;
pub const SCHEMA: &str = include_str!("../neutral/generator-settings-v2.schema.json");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Blocker {
    pub object_identity: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Placement {
    pub object_identity: String,
    pub matrix: [f64; 16],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub schema_version: u32,
    pub blockers: Vec<Blocker>,
    pub placements: Vec<Placement>,
}

pub fn schema_identity() -> Result<String> {
    cache_key::hash_json(
        "onshape-export-generator-settings-schema-v2",
        &serde_json::from_str::<serde_json::Value>(SCHEMA)?,
    )
}

impl Settings {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        ensure!(
            bytes.len() <= MAX_SETTINGS_BYTES,
            "settings exceed byte limit"
        );
        let mut settings: Self = serde_json::from_slice(bytes)?;
        for placement in &mut settings.placements {
            for value in &mut placement.matrix {
                if *value == 0.0 {
                    *value = 0.0;
                }
            }
        }
        settings.validate()?;
        Ok(settings)
    }

    pub fn identity(&self) -> Result<String> {
        let normalized = Self::parse(&serde_json::to_vec(self)?)?;
        cache_key::hash_json("onshape-export-generator-settings-v2", &normalized)
    }

    fn validate(&self) -> Result<()> {
        ensure!(self.schema_version == 2, "unsupported settings version");
        ensure!(
            self.blockers.len() <= 256 && self.placements.len() <= 256,
            "settings exceed object limit"
        );
        let mut blockers = HashSet::new();
        for blocker in &self.blockers {
            super::generator_protocol::validate_identity(&blocker.object_identity, "blocker")?;
            ensure!(
                blockers.insert(&blocker.object_identity),
                "duplicate blocker"
            );
            ensure!(
                !blocker.targets.is_empty() && blocker.targets.len() <= 64,
                "invalid blocker target cardinality"
            );
            let mut targets = HashSet::new();
            for target in &blocker.targets {
                super::generator_protocol::validate_identity(target, "target")?;
                ensure!(targets.insert(target), "duplicate blocker target");
            }
        }
        let mut placements = HashSet::new();
        for placement in &self.placements {
            super::generator_protocol::validate_identity(&placement.object_identity, "placement")?;
            ensure!(
                placements.insert(&placement.object_identity),
                "duplicate placement"
            );
            ensure!(
                placement.matrix.iter().all(|v| v.is_finite())
                    && placement.matrix[12..] == [0.0, 0.0, 0.0, 1.0],
                "invalid affine placement"
            );
        }
        Ok(())
    }

    pub fn validate_manifest(&self, manifest: &InputManifest) -> Result<()> {
        self.validate()?;
        ensure!(
            self.placements.len() == manifest.objects.len(),
            "placement cardinality mismatch"
        );
        for (placement, object) in self.placements.iter().zip(&manifest.objects) {
            ensure!(
                placement.object_identity == object.object_identity,
                "placement identity or order mismatch"
            );
        }
        let auxiliary: HashSet<_> = manifest
            .objects
            .iter()
            .filter(|o| o.role == InputRole::AuxiliaryGeometry)
            .map(|o| &o.object_identity)
            .collect();
        ensure!(
            auxiliary == self.blockers.iter().map(|b| &b.object_identity).collect(),
            "blocker role mismatch"
        );
        let printable: HashSet<_> = manifest
            .objects
            .iter()
            .filter(|o| o.role == InputRole::RawGeometry)
            .map(|o| &o.object_identity)
            .collect();
        for blocker in &self.blockers {
            ensure!(
                blocker.targets.iter().all(|t| printable.contains(t)),
                "unknown or wrong-role target"
            );
        }
        Ok(())
    }
}
