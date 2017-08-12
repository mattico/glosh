//! # AWS Lambda function to syncronize S3 path with FTP.

#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;
extern crate ftp;
extern crate aws_sdk_rust;
extern crate regex;
extern crate serde_json;
extern crate percent_encoding;
extern crate rb;

pub mod config;
pub mod range;
pub mod downloader;
pub mod buffered_reader;
pub mod uploader;
use config::Config;

use std::sync::Arc;
use std::any::Any;

use serde_json::Value as V;
use crowbar::{Value, LambdaContext, LambdaResult};
use percent_encoding::percent_decode;
use rb::RB;
fn print_error(e: Box<Any + Send + 'static>) {
        println!("Thread is kill");
        if let Some(string) = e.downcast_ref::<String>() {
        println!("{}", string);
    } else {
        println!("Not a string...");
    }
}
fn handler(event: Value, _context: LambdaContext) -> LambdaResult {
    let v: V = serde_json::from_str(&event.to_string()).unwrap();
    let bucket = v["Records"][0]["s3"]["bucket"]["name"].as_str().unwrap().to_owned();
    let config = Arc::new(Config::from_env());
    println!("Initialized");
    if bucket != config.s3_bucket {
        println!("Wrong bucket even");
        panic!("Wrong bucket even");
    }
    let src_key_encoded = v["Records"][0]["s3"]["object"]["key"].as_str().unwrap();
    let src_key = percent_decode(&src_key_encoded.as_bytes()).decode_utf8().unwrap().to_string();
    if !config.regex.is_match(&src_key) {
        println!("File doesn't match regex");
        panic!("File doesn't match regex");
    }
    
    let ring_buffer = rb::SpscRb::new(100 * 1024 * 1024);
    let (producer, consumer) = (ring_buffer.producer(), ring_buffer.consumer());
    let d = downloader::spawn_downloader(config.clone(), producer, src_key.clone());
    let u = uploader::spawn_uploader(config, consumer, src_key.clone());
    match d.join() {
        Ok(_)   => println!("Downloader finished"),
        Err(e)  => print_error(e)
    };
    match u.join() {
        Ok(_)   => println!("Uploader finished"),
        Err(e)  => print_error(e)
    }
    Ok(Value::Null)
}
/// Handler that gets exported to python
lambda!(handler);