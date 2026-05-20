
#![allow(warnings)]

//use pjs::pj2::temp_converter;
//use crate::pjs::pj3::calculator;

use std::collections::HashMap;
use std::io;
use crate::concepts::dynamic_dispatch::dynamic;
use crate::concurrency::thread_safe_counter::{ shared_counter_with_atomictype};
use crate::pjs::pj21_minigrep::grep;
use crate::pjs::pj21_minigrep_betterversion::minigrep;
use crate::pjs::pj22_logger_utility::logger_util;
use crate::pjs::pj23_data_validation_pool::data_validation_util;
use crate::pjs::pj25_trait_plugin_using_dynamic_dispatch::plugin;
use crate::pjs::pj26_polymorphic_shape_renderer::polymorphic_shape;
use crate::pjs::pj27_multi_threaded_counter::{multithreadedcounter};
use crate::pjs::pj28_thread_pool::thread_pool;
use crate::pjs::pj29_simple_parallel_web_crawler::crawler;
use crate::pjs::pj30_producer_consumer_demo::producer_consumer;
use crate::pjs::pj32_dirscanner_and_file_producer::dir;
use crate::pjs::pj33_file_compression::compression_algo;

mod pjs;
mod concepts;
mod concurrency;

fn main() -> std::io::Result<()> {
    //temp_converter()
    //calculator();
    //guess()?;
    //word_counter()?;// cargo run "file path" . to run this
    //bmi_calculator()?;
    //palindrome_checker()?;
    //prime_number()?;
    //todo_app()?;
    //todoapp();
    //timer_tool();
    //play();
    //json_parser();
    //memory_mgmt();
    //string_manipulate();
    //manager();
    //chat_app()?;
    //toml_parser()?;

    //minigrep();
    //logger_util();
    //data_validation_util();
    //plugin();
    // polymorphic_shape();
    //multithreadedcounter();
    // thread_pool();
    //dynamic();
    //crawler();
    // producer_consumer();
    //shared_counter_with_atomictype();
    //dir();
   if let Err(e) = compression_algo() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())

}
