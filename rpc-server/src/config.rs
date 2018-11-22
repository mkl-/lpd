use tls_api_native_tls::TlsAcceptor;
use tls_api::Error as TlsError;
use std::{net::{SocketAddr, AddrParseError}, io::Error as IoError};

#[derive(Debug)]
pub enum Error {
    Tls(TlsError),
    ReadCertificate(IoError),
    AddressParse(AddrParseError),
}

enum CommandLineKey {
    RpcAddress,
    Pkcs12,
    Pkcs12Password,
}

impl CommandLineKey {
    fn pattern<'a>(&self) -> &'a str {
        use self::CommandLineKey::*;

        match self {
            RpcAddress => "--rpclisten=",
            Pkcs12 => "--pkcs12=",
            Pkcs12Password => "--pkcs12-password=",
        }
    }

    fn predicate(&self, arg: &String) -> bool {
        arg.starts_with(self.pattern())
    }

    fn value(&self, arg: String) -> String {
        arg.trim_start_matches(self.pattern()).to_owned()
    }
}

pub struct Argument {
    pub address: SocketAddr,
    pub tls_acceptor: Option<TlsAcceptor>,
}

impl Argument {
    pub fn from_env() -> Result<Self, Error> {
        use tls_api_native_tls::{
            TlsAcceptorBuilder as TlsAcceptorBuilderImpl,
        };
        use tls_api::TlsAcceptorBuilder;
        use std::{env, fs::File, io::Read, net::{Ipv4Addr, IpAddr}};
        use self::Error::*;
        use self::CommandLineKey::*;

        let default_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9735);
        let address = env::args()
            .find(|arg| RpcAddress.predicate(arg))
            .map(|arg| RpcAddress.value(arg).parse::<SocketAddr>())
            .unwrap_or(Ok(default_address))
            .map_err(AddressParse)?;

        let acceptor = {
            let pkcs12 = env::args().find(|arg| Pkcs12.predicate(arg)).map(|arg| {
                let path = Pkcs12.value(arg);
                let mut file = File::open(path)?;
                let mut vec = Vec::new();
                file.read_to_end(&mut vec)?;
                Ok(vec)
            });
            let pkcs12_password = env::args()
                .find(|arg| Pkcs12Password.predicate(arg))
                .map(|arg| Pkcs12Password.value(arg));
            match (pkcs12, pkcs12_password) {
                (Some(data), Some(password)) => {
                    let data = data.map_err(ReadCertificate)?;
                    let acceptor =
                        TlsAcceptorBuilderImpl::from_pkcs12(data.as_slice(), password.as_str())
                            .map_err(Tls)?
                            .build()
                            .map_err(Tls)?;
                    Some(acceptor)
                }
                _ => None,
            }
        };

        Ok(Argument {
            address: address,
            tls_acceptor: acceptor,
        })
    }
}
