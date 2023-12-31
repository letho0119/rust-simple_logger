use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::warn!("This is an example message.");

    log::logger().flush();
}
