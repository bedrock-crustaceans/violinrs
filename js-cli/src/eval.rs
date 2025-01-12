use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use deno_runtime::deno_core::{FsModuleLoader, ModuleSpecifier, RuntimeOptions};
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};


#[deno_core::op2(fast)]
pub fn new_pack(#[string] name: &str) {
    println!("New Pack called {name}");
}

deno_core::extension!(
    violin_runtime,
    ops = [new_pack],
    esm_entry_point = "ext:violin_runtime/bootstrap.js",
    esm = [dir "./", "bootstrap.js"]
);

pub async fn eval(path: PathBuf) -> Result<(), ()> {
    let main_module = ModuleSpecifier::from_file_path("./main.js")?;

    let fs = Arc::new(RealFs);
    let permission_desc_parser =
        Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(FsModuleLoader),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            fetch_dns_resolver: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
        },
        WorkerOptions {
            extensions: vec![violin_runtime::init_ops_and_esm()],
            ..Default::default()
        },
    );

    worker.execute_main_module(&main_module).await.unwrap();
    worker.run_event_loop(false).await.unwrap();

    Ok(())
}