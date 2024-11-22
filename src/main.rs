mod firefly;
mod firefly_service;

use firefly_service::FireflyServiceHandler;
use thrift::protocol::{TBinaryInputProtocolFactory, TBinaryOutputProtocolFactory};
use thrift::transport::{TBufferedReadTransportFactory, TBufferedWriteTransportFactory};

fn main() {
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
        4,
    );

    let address = "127.0.0.1:9090";
    println!("Starting firefly server on {}", address);
    server.listen(address).unwrap();
}
