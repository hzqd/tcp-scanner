use std::{env, net::{TcpListener, ToSocketAddrs}};
use tcp_scan::KtStd;
use rayon::prelude::*;
use futures::future::join_all;
use tokio::{net::TcpSocket, time::{Duration, timeout}, spawn};

// tcp-scan [local / ip range(port)]

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<_>>();

    if let "local" = &*args[1] {
        (0..=65535).into_par_iter().filter_map(|i| format!("127.0.0.1:{}", i).then(|s|
            if let Err(_) = TcpListener::bind(s) { Some(i) } else { None }
        )).collect::<Vec<_>>().then(|vec| println!("打开的端口号: {:?}", vec));
    } else {
        let pre_range = args[2].parse::<i32>().unwrap();
        let suf_range = args[3].parse::<i32>().unwrap();
        let mut tasks = vec![];

        (pre_range..=suf_range).for_each(|i| tasks.push(spawn(async move {
            if let Ok(res) = timeout(Duration::from_secs(3), test_port(i))
                .await { res } else { None }
        })));
    
        let res = join_all(tasks).await;
        let open = res
            .into_par_iter()
            .filter_map(|r|
                if let Ok(Some(_)) = r { Some(r) } else { None }
            )
            .map(|r| r.unwrap().unwrap())
            .collect::<Vec<_>>();

        println!("打开的端口号: {:?}", open);
    }
}

async fn test_port(port: i32) -> Option<u16> {
    let addr = env::args().collect::<Vec<_>>()[1].clone().then(|ip|
        format!("{}:{}", ip, port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    );
    if let Ok(_) = TcpSocket::new_v4().unwrap().connect(addr)
        .await { Some(addr.port()) } else { None }
}