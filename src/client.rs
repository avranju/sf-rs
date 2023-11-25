use windows::{
    core::{ComInterface, Error as WindowsError, GUID, HRESULT, PCSTR},
    s,
    Win32::{
        Foundation::HANDLE,
        System::LibraryLoader::{GetProcAddress, LoadLibraryExA, LOAD_WITH_ALTERED_SEARCH_PATH},
    },
};

use crate::{error::Error, ServiceManagementClient};
use crate::{IFabricPropertyManagementClient2, IFabricServiceManagementClient7};

pub struct FabricClient {
    client: IFabricPropertyManagementClient2,
}

impl FabricClient {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            client: unsafe {
                fabric_create_local_client(
                    s!("FabricClient.dll"),
                    &IFabricPropertyManagementClient2::IID,
                )?
            },
        })
    }

    pub fn service_management_client(&self) -> Result<ServiceManagementClient, Error> {
        Ok(ServiceManagementClient::new(
            self.client.cast::<IFabricServiceManagementClient7>()?,
        ))
    }
}

type FabricCreateLocalClient =
    unsafe extern "system" fn(iid: *const GUID, ppv: *mut *mut std::ffi::c_void) -> HRESULT;

unsafe fn fabric_create_local_client<T: ComInterface>(
    lib: PCSTR,
    riid: *const GUID,
) -> Result<T, Error> {
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

    Err(WindowsError::from_win32().into())
}
