use futures::future::*;
use std::any::Any;

#[no_mangle]
pub extern "C" fn r#await(f: Box<dyn Future<Output = Box<dyn Any + 'static + Sync + Send>>>) -> ! {
    unimplemented!()
}
