use std::{env, net::{TcpListener, TcpStream}};
use tcp_scan::KtStd;
use rayon::prelude::*;

// tcp-scan [local / ip range(port)]

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if let "local" = &*args[1] {
        (0..=65535).into_par_iter().filter_map(|i| format!("127.0.0.1:{}", i).then(|s|
            if let Err(_) = TcpListener::bind(s) { Some(i) } else { None }
        )).collect::<Vec<_>>().then(|vec| println!("打开的端口号: {:?}", vec));
    } else {
        let ip = &args[1];
        let pre_range = args[2].parse::<i32>().unwrap();
        let suf_range = args[3].parse::<i32>().unwrap();

        let result = (pre_range..=suf_range).into_par_iter().map(|i|
            match TcpStream::connect(format!("{}:{}", ip, i)) {
                Ok(_) => (Some(i), None),
                Err(_) => (None, Some(i))
            }
        ).collect::<Vec<(_, _)>>();

        let open = result.par_iter().filter_map(|i| i.0).collect::<Vec<_>>();
        let close = result.into_par_iter().filter_map(|i| i.1).collect::<Vec<_>>();

        println!("打开的端口号: {:?}", open);
        println!("关闭的端口号: {:?}", close);
    }
}
