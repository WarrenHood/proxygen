// These "tests" are literally just here for me to experiment with proc macros at the moment
// in the future there might end up being some actual tests here though

use proxygen_macros::{proxy, pre_hook, post_hook};


#[post_hook]
#[no_mangle]
pub fn SomeTestFunction(steam_apps: usize, app_id: u32) -> bool {
    println!("This is a test!");
}