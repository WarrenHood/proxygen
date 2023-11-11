// These "tests" are literally just here for me to experiment with proc macros at the moment
// in the future there might end up being some actual tests here though

use proxygen_macros::{proxy, pre_hook, post_hook};


#[pre_hook(sig = "unknown")]
#[no_mangle]
pub fn SomeTestFunction() {
    println!("This is a test!");
}