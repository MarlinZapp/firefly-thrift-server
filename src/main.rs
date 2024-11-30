mod firefly;
mod firefly_service;

use std::env;

use firefly_service::FireflyServiceHandler;
use thrift::protocol::{TBinaryInputProtocolFactory, TBinaryOutputProtocolFactory};
use thrift::transport::{TBufferedReadTransportFactory, TBufferedWriteTransportFactory};

struct EnvironmentInformation {
    num_rows: usize,
    num_cols: usize,
}

fn main() {
    let environment_information = get_environment_information().unwrap();
    let num_clients = environment_information.num_cols * environment_information.num_rows + 1;

    // Create factories for transport and protocol
    let read_transport_factory = TBufferedReadTransportFactory::new();
    let write_transport_factory = TBufferedWriteTransportFactory::new();
    let input_protocol_factory = TBinaryInputProtocolFactory::new();
    let output_protocol_factory = TBinaryOutputProtocolFactory::new();

    let handler = FireflyServiceHandler::new();
    let processor = firefly::FireflyServiceSyncProcessor::new(handler);
    let mut server = thrift::server::TServer::new(
        read_transport_factory,
        input_protocol_factory,
        write_transport_factory,
        output_protocol_factory,
        processor,
        num_clients,
    );

    let address = "127.0.0.1:9090";
    println!("Starting firefly server on {}", address);
    server.listen(address).unwrap();
}

fn get_environment_information() -> Result<EnvironmentInformation, Box<dyn std::error::Error>> {
    let num_rows;
    let num_cols;
    match env::var("NUM_ROWS") {
        Ok(value) => {
            num_rows = value.parse::<usize>()?;
        }
        Err(e) => panic!("Couldn't read NUM_ROWS: {}", e),
    }
    match env::var("NUM_COLS") {
        Ok(value) => {
            num_cols = value.parse::<usize>()?;
        }
        Err(e) => panic!("Couldn't read NUM_COLS: {}", e),
    }
    Ok(EnvironmentInformation { num_rows, num_cols })
}
