#![allow(non_snake_case, clippy::missing_safety_doc)]

use anyhow::Result;
use sf_rs::{IFabricPropertyManagementClient2, IFabricServiceManagementClient7, IFabricQueryClient12};
use windows::{
    core::{ComInterface, Error, GUID, HRESULT, PCSTR},
    s,
    Win32::{
        Foundation::HANDLE,
        System::{
            Com::{CoInitializeEx, COINIT_MULTITHREADED},
            LibraryLoader::{GetProcAddress, LoadLibraryExA, LOAD_WITH_ALTERED_SEARCH_PATH},
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        let prop_client: IFabricPropertyManagementClient2 = fabric_create_local_client(
            s!("C:\\Program Files\\Microsoft Service Fabric\\bin\\Fabric\\Fabric.Code\\FabricClient.dll"),
            &IFabricPropertyManagementClient2::IID,
        )?;
        println!("prop_client : {:?}", prop_client);

        let service_client = prop_client.cast::<IFabricServiceManagementClient7>()?;
        println!("service_client : {:?}", service_client);

        let query_client = prop_client.cast::<IFabricQueryClient12>()?;
        println!("query_client : {:?}", query_client);

        Ok(())
    }
}

type FabricCreateLocalClient =
    unsafe extern "system" fn(iid: *const GUID, ppv: *mut *mut std::ffi::c_void) -> HRESULT;

pub unsafe fn fabric_create_local_client<T: ComInterface>(
    lib: PCSTR,
    riid: *const GUID,
) -> Result<T> {
    let instance = LoadLibraryExA(lib, HANDLE::default(), LOAD_WITH_ALTERED_SEARCH_PATH)?;
    if !instance.is_invalid() {
        if let Some(farproc) = GetProcAddress(instance, s!("FabricCreateLocalClient")) {
            let fabric_create_local_client: FabricCreateLocalClient = std::mem::transmute(farproc);
            let mut client: Option<T> = None;
            if fabric_create_local_client(riid, &mut client as *mut _ as *mut _).is_ok() {
                return Ok(client.unwrap());
            }
        }
    }

    Err(Error::from_win32().into())
}
