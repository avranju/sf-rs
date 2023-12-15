use std::{ffi::OsString, os::windows::ffi::OsStrExt, ptr, slice};

use tokio::sync::mpsc;
use windows::core::{implement, ComInterface, GUID};

use crate::{
    agile::AgileRef, channel_send, error::Error, run_with_retry, IFabricAsyncOperationCallback,
    IFabricAsyncOperationCallback_Impl, IFabricAsyncOperationContext, IFabricQueryClient12,
    MakeClient, PartitionQueryResultItem, FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION,
};

#[derive(Debug, Clone)]
pub struct QueryClient {
    client: AgileRef<IFabricQueryClient12>,
}

impl MakeClient for QueryClient {
    type Interface = IFabricQueryClient12;

    fn make(client: Self::Interface) -> Result<Self, Error> {
        Ok(Self {
            client: AgileRef::new(client.cast()?)?,
        })
    }
}

impl QueryClient {
    pub fn new(client: IFabricQueryClient12) -> Result<Self, Error> {
        Ok(Self {
            client: AgileRef::new(client.cast()?)?,
        })
    }

    pub async fn get_partition_list(
        &self,
        service_name: &str,
        timeout_ms: u32,
    ) -> Result<Vec<PartitionQueryResultItem>, Error> {
        let client = self.client.clone();
        run_with_retry("get_partition_list", move || {
            let client = client.clone();
            async move {
                let mut rx = try_get_partition_list(client.resolve()?, service_name, timeout_ms)?;
                rx.recv()
                    .await
                    .ok_or(Error::Abandoned("GetPartitionList"))?
            }
        })
        .await
    }
}

fn try_get_partition_list(
    client: IFabricQueryClient12,
    service_name: &str,
    timeout_ms: u32,
) -> Result<mpsc::Receiver<Result<Vec<PartitionQueryResultItem>, Error>>, Error> {
    let service_name = OsString::from(service_name);
    let mut service_name = service_name.encode_wide().collect::<Vec<_>>();

    let query_desc = FABRIC_SERVICE_PARTITION_QUERY_DESCRIPTION {
        ServiceName: service_name.as_mut_ptr(),
        PartitionIdFilter: GUID::zeroed(),
        Reserved: ptr::null_mut(),
    };
    let (tx, rx) = mpsc::channel(1);
    let callback = GetPartitionListCallback {
        tx,
        client: client.clone(),
    };
    let op_callback: IFabricAsyncOperationCallback = callback.into();

    let _context =
        unsafe { client.BeginGetPartitionList(&query_desc, timeout_ms, Some(&op_callback)) }?;

    Ok(rx)
}

#[implement(IFabricAsyncOperationCallback)]
struct GetPartitionListCallback {
    tx: mpsc::Sender<Result<Vec<PartitionQueryResultItem>, Error>>,
    client: IFabricQueryClient12,
}

impl IFabricAsyncOperationCallback_Impl for GetPartitionListCallback {
    fn Invoke(&self, context: Option<&IFabricAsyncOperationContext>) {
        match unsafe { self.client.EndGetPartitionList(context) } {
            Ok(res) => {
                let res = unsafe { res.get_PartitionList() };
                let count = unsafe { (*res).Count };
                let items = unsafe { slice::from_raw_parts((*res).Items, count as usize) };
                let items = items
                    .iter()
                    .map(PartitionQueryResultItem::try_from)
                    .collect::<Result<Vec<_>, Error>>();

                let _ = channel_send(self.tx.clone(), items);
            }
            Err(e) => {
                let _ = channel_send(self.tx.clone(), Err(e.into()));
            }
        }
    }
}
