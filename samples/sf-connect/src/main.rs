#![allow(non_snake_case, clippy::missing_safety_doc)]

use anyhow::Result;
use sf_rs::FabricClient;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }

    let client = FabricClient::new()?;
    let service_client = client.service_management_client()?;

    let res = service_client
        .resolve_service_partition(
            "fabric:/ZZZ/BlockStorageSFPkg",
            sf_rs::PartitionKeyType::Int64,
            1,
            1000,
        )
        .await?;

    println!("{:#?}", res);

    Ok(())
}
