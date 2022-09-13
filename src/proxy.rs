use std::net::Ipv4Addr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProxyType {
    HTTP = 1,
    HTTPS = 2
}

impl From<String> for ProxyType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "http,https" => ProxyType::HTTPS,
            "https"=> Self::HTTPS,
            _ => Self::HTTP   
        }
    }
}

impl From<ProxyType> for String {
    fn from(p: ProxyType) -> Self {
        match p {
            ProxyType::HTTPS => "https".to_string(),
            _ => "http".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Proxy {
    pub protocol: ProxyType,
    pub ipv4addr: Ipv4Addr,
    pub port: u16,
}

impl Proxy {

    pub fn new(protocol: String, address: String, port: u16) -> Self {
        Self { 
            protocol: protocol.into(), 
            ipv4addr: address.parse::<Ipv4Addr>().unwrap(), 
            port: port.into() 
        }
    }

    pub fn into_string(self) -> String {
        self.into()
    }
    
}

impl Into<String> for Proxy {
    fn into(self) -> String {
        format!("{}://{}:{}", String::from(self.protocol), self.ipv4addr, self.port)
    }
}




#[cfg(test)]
mod tests {
    use crate::{ProxyType, Proxy};

    #[test]
    fn test_proxy_type() {
        let http: String = ProxyType::HTTP.into();
        assert_eq!("http".to_string(), http);

        let https: ProxyType = "https".to_string().into();
        assert_eq!(ProxyType::HTTPS, https);
    }

    #[test]
    fn test_proxy() {
        let proxy: String = Proxy::new("https".to_string(), "127.0.0.1".to_string(), 8080).into();
        assert_eq!(proxy, "https://127.0.0.1:8080");
    }
}