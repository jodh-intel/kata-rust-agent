//use crate::container::Container;
use crate::namespace::{setup_persistent_ns, Namespace, NSTYPEIPC, NSTYPEUTS};
use crate::network::Network;
use libcontainer::container::LinuxContainer;
use libcontainer::cgroups::Manager as CgroupManager;
use libcontainer::cgroups::fs::Manager as FsManager;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Sandbox {
    pub id: String,
    pub hostname: String,
    pub containers: HashMap<String, LinuxContainer<FsManager>>,
    pub network: Network,
    pub mounts: Vec<String>,
    pub pci_device_map: HashMap<String, String>,
    shared_utsns: Namespace,
    shared_ipcns: Namespace,
    storages: HashMap<String, u32>,
    pub running: bool,
    pub no_pivot_root: bool,
    enable_grpc_trace: bool,
    sandbox_pid_ns: bool,
}

impl Sandbox{
    pub fn new() -> Self {
        Sandbox {
            id: "".to_string(),
            hostname: "".to_string(),
            network: Network::new(),
            containers: HashMap::new(),
            mounts: Vec::new(),
            pci_device_map: HashMap::new(),
            shared_utsns: Namespace {
                path: "".to_string(),
            },
            shared_ipcns: Namespace {
                path: "".to_string(),
            },
            storages: HashMap::new(),
            running: false,
            no_pivot_root: false,
            enable_grpc_trace: false,
            sandbox_pid_ns: false,
        }
    }

    pub fn unset_sandbox_storage(&self, path: &str) -> bool {
        false
    }

    pub fn remove_sandbox_storage(&self, path: &str) -> bool {
        false
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    pub fn setup_shared_namespaces(&mut self) -> Result<bool, String> {
        // Set up shared IPC namespace
        self.shared_ipcns = match setup_persistent_ns(NSTYPEIPC) {
            Ok(ns) => ns,
            Err(err) => return Err("Failed to setup persisten IPC namespace ".to_string() + &err),
        };

        // Set up shared UTS namespace
        self.shared_utsns = match setup_persistent_ns(NSTYPEUTS) {
            Ok(ns) => ns,
            Err(err) => return Err("Failed to setup persisten UTS namespace ".to_string() + &err),
        };

        Ok(true)
    }

    pub fn add_container(&mut self, c: LinuxContainer<FsManager>) {
        self.containers.insert(c.id.clone(), c);
    }

    pub fn get_container(&mut self, id: &str) -> Option<&mut LinuxContainer<FsManager>> {
        self.containers.get_mut(id)
    }

    // set_sandbox_storage sets the sandbox level reference
    // counter for the sandbox storage.
    // This method also returns a boolean to let
    // callers know if the storage already existed or not.
    // It will return true if storage is new.
    //
    // It's assumed that caller is calling this method after
    // acquiring a lock on sandbox.
    pub fn set_sandbox_storage(&mut self, path: &str) -> bool {
        match self.storages.get_mut(path) {
            None => {
                self.storages.insert(path.to_string(), 1);
                true
            }
            Some(count) => {
                *count += 1;
                false
            }
        }
    }
}