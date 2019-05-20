use oci::{self, Spec};
use lazy_static;
use nix::mount::MsFlags;
use libcontainer::configs::namespaces;
use libcontainer::configs::device::Device;
use std::collections::HashMap;

pub struct CreateOpts {
	cgroup_name: String,
	use_systemd_cgroup: bool,
	no_pivot_root: bool,
	no_new_keyring: bool,
	spec: Option<Spec>,
	rootless_euid: bool,
	rootless_cgroup: bool,
}

const WILDCARD: i32 = -1;

lazy_static! {
	static ref NAEMSPACEMAPPING: HashMap<&'static str, &'static str> = {
		m = HashMap::new();
		m.insert(oci::PIDNAMESPACE, namespaces::NEWPID);
		m.insert(oci::NETWORKNAMESPACE, namespaces::NEWNET);
		m.insert(oci::UTSNAMESPACE, namespace::NEWUTS);
		m.insert(oci::MOUNTNAMESPACE, namespaces::NEWNS);
		m.insert(oci::IPCNAMESPACE, namespaces::NEWIPC);
		m.insert(oci::USERNAMESPACE, namespaces::NEWUSER);
		m.insert(oci::CGROUPNAMESPACE, namespaces::NEWCGROUP);
		m
	};

	static ref MOUNTPROPAGATIONMAPPING: HashMap<&'static str, MsFlags> = {
		m = HashMap::new();
		m.insert("rprivate", MsFlags::MS_PRIVATE | MsFlags::MS_REC);
		m.insert("private", MsFlags::MS_PRIVATE);
		m.insert("rslave", MsFlags::MS_SLAVE | MsFlags::MS_REC);
		m.insert("slave", MsFlags::MS_SLAVE);
		m.insert("rshared", MsFlags::MS_SHARED | MsFlags::MS_REC);
		m.insert("shared", MsFlags::MS_SHARED);
		m.insert("runbindable", MsFlags::MS_UNBINDABLE | MsFlags::MS_REC);
		m.insert("unbindable", MsFlags::MS_UNBINDABLE);
		m
	};

	static ref ALLOWED_DEVICES: Vec<Device> = {
		m = Vec::new();
		m.push(Device {
			r#type: 'c',
			major: WILDCARD,
			minor: WILDCARD,
			permissions: "m",
			allow: true,
		});

		m.push(Device {
			r#type: 'b',
			major: WILDCARD,
			minor: WILDCARD,
			permissions: "m",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: "/dev/null".to_string(),
			major: 1,
			minor: 3,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/random"),
			major: 1,
			minor: 8,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/full"),
			major: 1,
			minor: 7,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/tty"),
			major: 5,
			minor: 0,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/zero"),
			major: 1,
			minor: 5,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/urandom"),
			major: 1,
			minor: 9,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from("/dev/console"),
			major: 5,
			minor: 1,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from(""),
			major: 136,
			minor: WILDCARD,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from(""),
			major: 5,
			minor: 2,
			permissions: "rwm",
			allow: true,
		});

		m.push(Device {
			r#type: 'c',
			path: String::from(""),
			major: 10,
			minor: 200,
			permissions: "rwm",
			allow: true,
		});
		m
	};
}
