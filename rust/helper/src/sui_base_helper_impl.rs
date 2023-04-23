// sui-base helper library
//
// This is the implementation. See lib.rs for the public API and documentation.

use sui_sdk::types::base_types::{ObjectID, SuiAddress};

use crate::error::SuiBaseError;
use crate::sui_base_root::SuiBaseRoot;
use crate::sui_base_workdir::SuiBaseWorkdir;

pub struct SuiBaseHelperImpl {
    root: SuiBaseRoot,               // for most features related to ~/sui-base
    workdir: Option<SuiBaseWorkdir>, // for *one* selected workdir under ~/sui-base/workdirs
}

impl Default for SuiBaseHelperImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl SuiBaseHelperImpl {
    // Steps to get started with the API:
    //
    //  (1) Check if is_installed()
    //
    //  (2) Call select_workdir()
    //
    //  (3) You can now call any other API functions (in any order).
    //      Most calls will relate to the selected workdir.

    pub fn new() -> SuiBaseHelperImpl {
        SuiBaseHelperImpl {
            root: SuiBaseRoot::new(),
            workdir: None,
        }
    }

    // Check first if sui-base is installed, otherwise
    // most of the other calls will fail in some ways.
    pub fn is_installed(self: &mut SuiBaseHelperImpl) -> Result<bool, SuiBaseError> {
        Ok(self.root.is_installed())
    }

    // Select an existing workdir by name.
    //
    // Possible values are:
    //   "active", "cargobin", "localnet", "devnet", "testnet", "mainnet" and
    //    other custom names might be supported in future.
    //
    // Note: "active" is special. It will resolve the active workdir at the moment of the
    //       call. Example: if "localnet" is the active, then this call is equivalent to
    //       to be done for "localnet". The selection does not change even if the user
    //       externally change the active after this call.
    //
    pub fn select_workdir(
        self: &mut SuiBaseHelperImpl,
        workdir_name: &str,
    ) -> Result<(), SuiBaseError> {
        let mut new_wd = SuiBaseWorkdir::new();
        new_wd.init_from_existing(&mut self.root, workdir_name)?;
        self.workdir = Some(new_wd);
        Ok(())
    }

    // Get the name of the selected workdir.
    pub fn get_workdir_name(&self) -> Result<String, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_name()?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get the pathname of the file keystore (when available).
    //
    // Context: Selected Workdir by this API.
    pub fn get_keystore_pathname(&mut self) -> Result<String, SuiBaseError> {
        // TODO Implement this better with sui-base.yaml and/or ENV variables.
        //      See https://github.com/sui-base/sui-base/issues/6
        match &self.workdir {
            Some(wd) => Ok(wd.get_keystore_pathname(&mut self.root)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get the ObjectID of the last successfully published "package_name".
    //
    // package_name is the "name" field specified in the "Move.toml".
    //
    // Related path: ~/sui-base/workdirs/<workdir_name>/published-data/<package_name>/
    pub fn get_package_id(
        self: &mut SuiBaseHelperImpl,
        package_name: &str,
    ) -> Result<ObjectID, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_package_id(&mut self.root, package_name)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get the ObjectID of the objects that were created when the package was published.
    //
    // object_type format is the Sui Move "package::module::type".
    //
    // Example:
    //
    //    module acme::Tools {
    //       struct Anvil has key, drop { ... }
    //       ...
    //       fun init(ctx: &mut TxContext) {
    //          Anvil::new(ctx);
    //          ...
    //       }
    //    }
    //
    // The object_type is "acme::Tools::Anvil"
    //
    // Related path: ~/sui-base/workdirs/<workdir_name>/published-data/<package_name>/
    pub fn get_published_new_objects(
        self: &mut SuiBaseHelperImpl,
        object_type: &str,
    ) -> Result<Vec<ObjectID>, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_published_new_objects(&mut self.root, object_type)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get an address by name.
    //
    // Sui-base localnet/devnet/testnet workdir are created with a set of pre-defined client addresses.
    //
    // These addresses are useful for testing. In particular, with localnet they are prefunded.
    //
    // Their names are:
    //   sb-[1-5]-[ed25519|scp256k1|scp256r1]
    //
    // Example of valid names: "sb-1-ed25519", "sb-3-scp256r1", "sb-5-scp256k1" ...
    //
    pub fn get_client_address(
        self: &mut SuiBaseHelperImpl,
        address_name: &str,
    ) -> Result<SuiAddress, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_client_address(&mut self.root, address_name)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get a RPC URL for the selected workdir.
    pub fn get_rpc_url(&mut self) -> Result<String, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_rpc_url(&mut self.root)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }

    // Get a Websocket URL for the selected workdir.
    pub fn get_ws_url(&mut self) -> Result<String, SuiBaseError> {
        match &self.workdir {
            Some(wd) => Ok(wd.get_ws_url(&mut self.root)?),
            None => Err(SuiBaseError::WorkdirNotSelected),
        }
    }
}