use deno_core::{error::AnyError, FsModuleLoader};
use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel, deno_web::BlobUrlStore,
    permissions::Permissions,
};
use std::{io::Read, path::Path, rc::Rc, sync::Arc};

use deno_runtime::worker::{MainWorker, WorkerOptions};

use gag::{BufferRedirect};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let broadcast_channel = InMemoryBroadcastChannel::default();

    let options = WorkerOptions {
        apply_source_maps: false,
        args: vec![],
        debug_flag: false,
        unstable: false,
        ca_data: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        runtime_version: "x".to_string(),
        ts_version: "x".to_string(),
        no_color: false,
        get_error_class_fn: Some(&get_error_class_name),
        location: None,
        blob_url_store: BlobUrlStore::default(),
        attach_inspector: false,
        origin_storage_dir: None,
        broadcast_channel,
    };
    let js_path = Path::new("main.js");
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy())?;

    let permissions = Permissions::allow_all();

    // Initialize a runtime instance
    let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);

    let mut buf = BufferRedirect::stdout().unwrap();
        
    worker.bootstrap(&options);
    worker.execute_module(&main_module).await?;
    
    worker.run_event_loop(false).await?;

    
    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();
    drop(buf);

    println!("Yolo holo molo {}", output);
    
    Ok(())
}
