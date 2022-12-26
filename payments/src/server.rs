use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentResponse, BtcPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}


fn main() {}
