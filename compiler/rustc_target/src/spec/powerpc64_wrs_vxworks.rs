use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::vxworks_base::opts();
    base.cpu = "ppc64".to_string();
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push("-m64".to_string());
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "powerpc64-unknown-linux-gnu".to_string(),
        target_endian: "big".to_string(),
        pointer_width: 64,
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-i64:64-n32:64".to_string(),
        arch: "powerpc64".to_string(),
        target_os: "vxworks".to_string(),
        target_env: "gnu".to_string(),
        target_vendor: "wrs".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions { ..base },
    }
}
