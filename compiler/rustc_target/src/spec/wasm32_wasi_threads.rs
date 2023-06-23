use super::crt_objects::{self, LinkSelfContainedDefault};
use super::{wasm_base, Cc, LinkerFlavor, Target};

pub fn target() -> Target {
    let mut options = wasm_base::options();

    options.os = "wasi".into();

    options.add_pre_link_args(
        LinkerFlavor::WasmLld(Cc::No),
        &["--import-memory", "--export-memory", "--shared-memory"],
    );
    options.add_pre_link_args(
        LinkerFlavor::WasmLld(Cc::Yes),
        &[
            "--target=wasm32-wasi-threads",
            "-Wl,--import-memory",
            "-Wl,--export-memory,",
            "-Wl,--shared-memory",
        ],
    );

    options.pre_link_objects_self_contained = crt_objects::pre_wasi_self_contained();
    options.post_link_objects_self_contained = crt_objects::post_wasi_self_contained();

    // FIXME: Figure out cases in which WASM needs to link with a native toolchain.
    options.link_self_contained = LinkSelfContainedDefault::True;

    // Right now this is a bit of a workaround but we're currently saying that
    // the target by default has a static crt which we're taking as a signal
    // for "use the bundled crt". If that's turned off then the system's crt
    // will be used, but this means that default usage of this target doesn't
    // need an external compiler but it's still interoperable with an external
    // compiler if configured correctly.
    options.crt_static_default = true;
    options.crt_static_respected = true;

    // Allow `+crt-static` to create a "cdylib" output which is just a wasm file
    // without a main function.
    options.crt_static_allows_dylibs = true;

    // WASI's `sys::args::init` function ignores its arguments; instead,
    // `args::args()` makes the WASI API calls itself.
    options.main_needs_argc_argv = false;

    // And, WASI mangles the name of "main" to distinguish between different
    // signatures.
    options.entry_name = "__main_void".into();

    options.singlethread = false;
    options.features = "+atomics,+bulk-memory,+mutable-globals".into();

    Target {
        llvm_target: "wasm32-wasi".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20".into(),
        arch: "wasm32".into(),
        options,
    }
}
