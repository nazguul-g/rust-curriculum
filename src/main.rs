#![allow(warnings)]
use crate::concepts::actix_first::actix_fun;
use crate::concepts::dynamic_dispatch::dynamic;
use crate::concurrency::thread_safe_counter::shared_counter_with_atomictype;
use crate::pjs::pj3::calculator;
use crate::pjs::pj21_minigrep::grep;
use crate::pjs::pj21_minigrep_betterversion::minigrep;
use crate::pjs::pj22_logger_utility::logger_util;
use crate::pjs::pj23_data_validation_pool::data_validation_util;
use crate::pjs::pj25_trait_plugin_using_dynamic_dispatch::plugin;
use crate::pjs::pj26_polymorphic_shape_renderer::polymorphic_shape;
use crate::pjs::pj27_multi_threaded_counter::multithreadedcounter;
use crate::pjs::pj28_thread_pool::thread_pool;
use crate::pjs::pj29_simple_parallel_web_crawler::crawler;
use crate::pjs::pj30_producer_consumer_demo::producer_consumer;
use crate::pjs::pj32_dirscanner_and_file_producer::dir;
use crate::pjs::pj33_file_compression::compression_algo;
use crate::pjs::pj34_system_monitor::system_monitor;
use crate::pjs::pj35_multi_threaded_file_copier::file_copier;
use crate::pjs::pj36_csv_processor::csv;
use crate::pjs::pj37_file_encryption_tool::aes256ctr;
use crate::pjs::pj38_process_manager::process_manager;
use crate::pjs::pj39_tcp_server::tcp_server;
use crate::pjs::pj40_tcp_client::tcp_client;
use crate::pjs::pj41_multi_client_chat_server::chat_server;
use crate::pjs::pj42_http_request_parser::http_parser;
use crate::pjs::pj43_rest_api_consumer::random_joke;
use crate::pjs::pj44_port_scanner::port_scanner;
use crate::pjs::pj45_websocketserver::websockets;
use crate::pjs::pj46_cli_shell::cli_shell;
use crate::pjs::pj47_text_editor::text_editor;
use crate::pjs::pj48_scheduler_simulation::simulator;
use crate::pjs::pj49_remote_file_sync_tool::remote_file_sync_tool;
use crate::pjs::pj50_hello_actix::hello_actix;
use crate::pjs::pj51_rest_api_server::rest_api_server;
use crate::pjs::pj52_user_uthentication_api::auth_api;
use crate::pjs::pj53_crud_api::crud_api;
use crate::pjs::pj54_jwt_authentication::jwt;
use pjs::pj2::temp_converter;
use std::collections::HashMap;
use std::io;

mod concepts;
mod concurrency;
mod pjs;

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
    //polymorphic_shape();
    //multithreadedcounter();
    //thread_pool();
    //dynamic();
    //crawler();
    //producer_consumer();
    //shared_counter_with_atomictype();
    //dir();
    // io::)
    //     }
    //compression_algo() {
    //system_monitor();
    //file_copier()?;
    //csv().unwrap();
    //aes256ctr();
    //process_manager();
    //tcp_server()?;
    //tcp_client()?;
    //chat_server()?;
    //http_parser()?;
    //random_joke();
    //port_scanner();
    //websockets();
    //cli_shell();
    //text_editor();
    //simulator();
    //remote_file_sync_tool()?;
    //hello_actix();
    //actix_fun()?;
    //hello_actix()?;
    //rest_api_server()?;

    //auth_api()?;
    //crud_api()?;
    jwt()?;
    Ok(())
}
