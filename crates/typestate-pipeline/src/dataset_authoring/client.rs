use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use serde_json::value::RawValue;

use crate::dataset_authoring::primitives::{Hash, JobId, Name, Namespace, Version};

/// Trivial in-memory stand-in for `amp_client_admin::Client`. Keeps a
/// monotonic job-id counter and a registry of registered manifests so
/// `tag_version` and `edit_existing` can pretend to fetch.
#[derive(Debug, Default)]
pub struct Client {
    next_job_id: AtomicU64,
    state: Mutex<ClientState>,
}

#[derive(Debug, Default)]
struct ClientState {
    /// Last manifest hash observed for `(namespace, name)`. Used by
    /// `edit_existing` to hand back the most recent manifest body.
    manifests: BTreeMap<(Namespace, Name), (String, Box<RawValue>)>,
    versions: BTreeMap<(Namespace, Name), BTreeSet<Version>>,
}

impl Client {
    pub async fn allocate_job_id(&self) -> JobId {
        JobId(self.next_job_id.fetch_add(1, Ordering::SeqCst) + 1)
    }

    pub async fn register(
        &self,
        namespace: Namespace,
        name: Name,
        manifest: Box<RawValue>,
        kind: &'static str,
    ) -> Hash {
        // The "hash" is just a stable string of the body for this example.
        let hash = Hash(format!("h:{}", manifest.get().len()));
        self.state
            .lock()
            .unwrap()
            .manifests
            .insert((namespace, name), (kind.to_owned(), manifest));
        hash
    }

    pub async fn tag(&self, namespace: Namespace, name: Name, version: Version) {
        self.state
            .lock()
            .unwrap()
            .versions
            .entry((namespace, name))
            .or_default()
            .insert(version);
    }

    pub async fn fetch_latest(&self, namespace: &Namespace, name: &Name) -> Option<Version> {
        self.state
            .lock()
            .unwrap()
            .versions
            .get(&(namespace.clone(), name.clone()))
            .and_then(|versions| versions.last().copied())
    }

    pub async fn fetch_manifest(
        &self,
        namespace: &Namespace,
        name: &Name,
    ) -> Option<(String, Box<RawValue>)> {
        self.state
            .lock()
            .unwrap()
            .manifests
            .get(&(namespace.clone(), name.clone()))
            .map(|(k, m)| (k.clone(), m.clone()))
    }
}
