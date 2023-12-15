use windows::{
    core::{ComInterface, Error as WindowsError, IUnknown, GUID, HRESULT, PCSTR},
    s,
    Win32::{
        Foundation::HANDLE,
        System::LibraryLoader::{GetProcAddress, LoadLibraryExA, LOAD_WITH_ALTERED_SEARCH_PATH},
    },
};

use crate::IFabricPropertyManagementClient2;
use crate::{agile::AgileRef, error::Error};

pub trait MakeClient: Sized {
    type Interface: ComInterface;

    fn make(client: Self::Interface) -> Result<Self, Error>;
}

#[derive(Debug)]
pub struct FabricLocalClient {
    client: AgileRef<IFabricPropertyManagementClient2>,
}

impl FabricLocalClient {
    pub fn new() -> Result<Self, Error> {
        let client: IUnknown = unsafe {
            fabric_create_local_client(
                s!("FabricClient.dll"),
                &IFabricPropertyManagementClient2::IID,
            )?
        };

        Ok(Self {
            client: AgileRef::new(client)?,
        })
    }

    pub fn make_client<T: MakeClient>(&self) -> Result<T, Error> {
        T::make(self.client.resolve()?.cast()?)
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
