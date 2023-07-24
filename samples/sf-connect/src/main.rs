use anyhow::Result;
use sf_rs::IFabricQueryClient;
use windows::{Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED, CoCreateInstance, CLSCTX_INPROC_SERVER}, core::ComInterface};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        let _runtime: IFabricQueryClient = CoCreateInstance(&IFabricQueryClient::IID, None, CLSCTX_INPROC_SERVER)?;

        Ok(())
    }
}
