#![no_std]
use gstd::{msg, prelude::*};

//Program entry point
#[no_mangle]
extern "C" fn handle(){
    msg::reply(String::from("Hello"), 0).expect("Error in sending a reply message");
}



