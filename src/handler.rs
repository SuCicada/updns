use std::io::{Error, ErrorKind};
use std::net::{IpAddr};
use log::{error, info};
use tokio::time::timeout;
use tokio::{io::Result,
            net::UdpSocket,
};
use updns::{BytePacketBuffer, DnsPacket, DnsRecord, QueryType};
use crate::{HOSTS, PROXY, TIMEOUT};

async fn get_answer(domain: &str, query: QueryType) -> Option<DnsRecord> {
    info!("get_answer: {} {:?}",domain,query);
    info!("get hosts: {:?}",HOSTS.read().await);
    if let Some(ip) = HOSTS.read().await.get(domain) {
        match query {
            QueryType::A => {
                if let IpAddr::V4(addr) = ip {
                    return Some(DnsRecord::A {
                        domain: domain.to_string(),
                        addr: *addr,
                        ttl: 3600,
                    });
                }
            }
            QueryType::AAAA => {
                if let IpAddr::V6(addr) = ip {
                    return Some(DnsRecord::AAAA {
                        domain: domain.to_string(),
                        addr: *addr,
                        ttl: 3600,
                    });
                }
            }
            _ => {}
        }
    }
    None
}

pub async fn handle(mut req: BytePacketBuffer, len: usize) -> Result<Vec<u8>> {
    let mut request = DnsPacket::from_buffer(&mut req)?;

    let query = match request.questions.get(0) {
        Some(q) => q,
        None => return proxy(&req.buf[..len]).await,
    };

    info!("{} {:?}", query.name, query.qtype);

    // Whether to proxy
    let answer = match get_answer(&query.name, query.qtype).await {
        Some(record) => record,
        // None => return proxy(&req.buf[..len]).await,
        None => return Err(Error::new(
            ErrorKind::Other,
            "Proxy server failed to proxy request",
        ))?,
    };

    info!("answer: {:?}", answer);

    request.header.recursion_desired = true;
    request.header.recursion_available = true;
    request.header.response = true;
    request.answers.push(answer);
    let mut res_buffer = BytePacketBuffer::new();
    request.write(&mut res_buffer)?;

    let data = res_buffer.get_range(0, res_buffer.pos())?;
    Ok(data.to_vec())
}


pub async fn proxy(buf: &[u8]) -> Result<Vec<u8>> {
    info!("proxy: {:?}", buf);
    let proxy = PROXY.read().await;
    let duration = *TIMEOUT.read().await;

    for addr in proxy.iter() {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).await?;

        let data: Result<Vec<u8>> = timeout(duration, async {
            socket.send_to(buf, addr).await?;
            let mut res = [0; 512];
            let len = socket.recv(&mut res).await?;
            Ok(res[..len].to_vec())
        }).await?;

        info!("proxy: {:?}", data);
        match data {
            Ok(data) => {
                return Ok(data);
            }
            Err(err) => {
                error!("Agent request to {} {:?}", addr, err);
            }
        }
    }

    Err(Error::new(
        ErrorKind::Other,
        "Proxy server failed to proxy request",
    ))
}
