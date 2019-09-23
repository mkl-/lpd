use super::lightning_address::LightningAddress;
use super::network_graph;

use std::io::Error as IoError;
use std::sync::Arc;

use dependencies::grpc::Error as GrpcError;
use dependencies::grpc::{Client, ClientStub};
use dependencies::httpbis::Error as HttpbisError;
use dependencies::futures::future::Future;
use structopt::StructOpt;

use interface::{
    routing_grpc::{RoutingServiceClient, RoutingService},
    routing::{ConnectPeerRequest, LightningAddress as LightningAddressRPC, ChannelGraphRequest},
    common::Void,
};
use build_info::get_build_info;

#[derive(Debug)]
pub enum Error {
    Grpc(GrpcError),
    Httpbis(HttpbisError),
    IoError{
        inner: IoError,
        description: String,
    },
}

impl Error {
    pub fn new_io_error(err: IoError, description: &str) -> Self {
        Error::IoError {
            inner: err,
            description: description.to_owned(),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt()]
pub enum Command{
    /// Get general info
    #[structopt(name="get-info")]
    GetInfo,

    /// Connect to specified peer
    #[structopt(name="connect-peer")]
    ConnectPeer{
        #[structopt()]
        address: LightningAddress,
    },

    /// Report graph info
    #[structopt(name="describe-graph")]
    DescribeGraph,

    /// Report graph info in dot format
    #[structopt(name="describe-graph-dot")]
    DescribeGraphDot,

    /// Print version
    #[structopt(name="get-version")]
    GetVersion,

    /// List peers
    #[structopt(name="list-peers")]
    ListPeers,
}

impl Command {
    pub fn execute(&self, client: Arc<Client>) -> Result<(), Error> {
        use self::Command::*;

        let routing_service = RoutingServiceClient::with_client(client);
        match self {
            GetInfo => {
                let response = routing_service
                    .get_info(Default::default(), Void::new())
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            ConnectPeer{address} => {
                let mut request = ConnectPeerRequest::new();

                let mut lightning_address_rpc = LightningAddressRPC::new();
                lightning_address_rpc.set_pubkey(address.pub_key.clone());
                lightning_address_rpc.set_host(format!("{}", address.host));

                request.set_address(lightning_address_rpc);
                let response = routing_service
                    .connect_peer(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            DescribeGraph => {
                let mut request = ChannelGraphRequest::new();
                request.set_include_unannounced(false);
                let response = routing_service
                    .describe_graph(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            DescribeGraphDot => {
                let mut request = ChannelGraphRequest::new();
                request.set_include_unannounced(false);
                let response = routing_service
                    .describe_graph(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                let dot = network_graph::dot_format(response);
                println!("{:?}", dot);
                Ok(())
            },
            GetVersion => {
                println!("Client's version:");
                println!("{:#?}", get_build_info!());

                println!("Server's version:");
                let response = routing_service
                    .get_info(Default::default(), Void::new())
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{}", response.version);
                Ok(())
            },
            ListPeers => {
                let request = Void::new();
                let response = routing_service
                    .list_peers(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
        }
    }
}