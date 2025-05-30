#![allow(missing_docs)]
use crate::commit_engine::{HunkHeader, RejectionReason};
use bstr::BString;
use gitbutler_serde::BStringForFrontend;
use serde::{Deserialize, Serialize};

/// The JSON serializable type of [super::DiffSpec].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffSpec {
    /// lossless version of `previous_path` if this was a rename.
    pub previous_path_bytes: Option<BString>,
    /// lossless version of `path`.
    pub path_bytes: BString,
    /// The headers of the hunks to use, or empty if all changes are to be used.
    pub hunk_headers: Vec<HunkHeader>,
}

impl From<DiffSpec> for super::DiffSpec {
    fn from(
        DiffSpec {
            path_bytes,
            hunk_headers,
            previous_path_bytes,
        }: DiffSpec,
    ) -> Self {
        super::DiffSpec {
            previous_path: previous_path_bytes,
            path: path_bytes,
            hunk_headers,
        }
    }
}

impl From<super::DiffSpec> for DiffSpec {
    fn from(
        super::DiffSpec {
            path,
            hunk_headers,
            previous_path,
        }: super::DiffSpec,
    ) -> Self {
        DiffSpec {
            previous_path_bytes: previous_path,
            path_bytes: path,
            hunk_headers,
        }
    }
}

/// The JSON serializable type of [super::CreateCommitOutcome].
// TODO(ST): this type should contain mappings from old to new commits so that the UI knows what state to update, maybe.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommitOutcome {
    /// Paths that contained at least one rejected hunk, for instance, a change that didn't apply, along with the reason for the rejection.
    pub paths_to_rejected_changes: Vec<(RejectionReason, BStringForFrontend)>,
    /// The newly created commit, if there was one. It maybe that a couple of paths were rejected, but the commit was created anyway.
    #[serde(with = "gitbutler_serde::object_id_opt")]
    pub new_commit: Option<gix::ObjectId>,
}

impl From<super::CreateCommitOutcome> for CreateCommitOutcome {
    fn from(
        super::CreateCommitOutcome {
            rejected_specs,
            new_commit,
            changed_tree_pre_cherry_pick: _,
            references: _,
            rebase_output: _,
            index: _,
        }: super::CreateCommitOutcome,
    ) -> Self {
        CreateCommitOutcome {
            paths_to_rejected_changes: rejected_specs
                .into_iter()
                .map(|(reason, spec)| (reason, spec.path.into()))
                .collect(),
            new_commit,
        }
    }
}
