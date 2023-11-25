extern crate windows;

pub mod bindings;
use std::future::Future;

pub use bindings::*;

pub mod client;
pub use client::*;

pub mod error;

pub mod query;
use error::{Error, FabricErrorCode};
pub use query::*;

pub mod service;
pub use service::*;

pub mod types;
pub use types::*;

use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use tokio_retry::{strategy::FixedInterval, RetryIf};

lazy_static! {
    static ref RETRYABLE_ERRORS: Vec<FabricErrorCode> = vec![
        FabricErrorCode::FabricHealthEntityNotFound,
        FabricErrorCode::PLBNotReady,
        FabricErrorCode::InvalidReplicaStateForReplicaOperation,
        FabricErrorCode::ObjectClosed,
        FabricErrorCode::ServiceNotFound,
        FabricErrorCode::AlreadyAuxiliaryReplica,
        FabricErrorCode::AlreadyInstance,
        FabricErrorCode::AlreadySecondaryReplica,
        FabricErrorCode::AlreadyPrimaryReplica,
        FabricErrorCode::FabricVersionAlreadyExists,
        FabricErrorCode::FabricUpgradeInProgress,
        FabricErrorCode::FabricAlreadyInTargetVersion,
        FabricErrorCode::StopInProgress
    ];
}

pub async fn run_with_retry<T, F, Fut>(op_name: &str, f: F) -> Result<T, Error>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    // Try 10 times and wait 200ms between each try.
    let strategy = FixedInterval::from_millis(200).take(10);
    RetryIf::spawn(strategy, f, |err: &Error| is_retryable_error(op_name, err)).await
}

fn is_retryable_error(op_name: &str, err: &Error) -> bool {
    if let Error::Windows(e) = err {
        let err = FabricErrorCode::from_u32(e.code().0 as u32);
        let retryable = err
            .as_ref()
            .map(|code| RETRYABLE_ERRORS.contains(code))
            .unwrap_or(false);
        if retryable {
            log::warn!("Retrying {} due to error: {:?}", op_name, err);
        }

        retryable
    } else {
        false
    }
}
