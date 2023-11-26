mod proxy {
    use core::fmt;
    use std::{fmt::Formatter, net::Ipv4Addr, str::FromStr};

    use anyhow::Context;

    #[derive(Debug, Clone)]
    pub struct Proxy {
        pub proxy_type: ProxyType,
        pub ip: Ipv4Addr,
        pub port: u16,
    }

    #[derive(Debug, Clone)]
    pub enum ProxyType {
        Socks5,
        Socks4,
        Https,
        Http,
        None,
    }

    impl FromStr for Proxy {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split(':');
            let schema = match split.nth(0) {
                Some("http") => ProxyType::Http,
                Some("https") => ProxyType::Https,
                Some("socks4") => ProxyType::Socks4,
                Some("socks5") => ProxyType::Socks5,
                _ => ProxyType::None,
            };
            let ip: Ipv4Addr = split.next().context("invalid format")?.parse()?;
            let port: u16 = split.next().context("invalid format")?.parse()?;
            Ok(Proxy {
                proxy_type: schema,
                ip,
                port,
            })
        }
    }

    impl fmt::Display for ProxyType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Socks5 => write!(f, "socks5"),
                Self::Socks4 => write!(f, "socks4"),
                Self::Http => write!(f, "http"),
                _ => write!(f, "https"),
            }
        }
    }

    impl fmt::Display for Proxy {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}://{}:{}", self.proxy_type, self.ip, self.port)
        }
    }
}
