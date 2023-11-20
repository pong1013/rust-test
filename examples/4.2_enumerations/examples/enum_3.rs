#![allow(unused)]
fn main() {
    #[derive(Debug)]
	struct Ipv4Addr { 
        address: (u8, u8, u8, u8), 
    }
	#[derive(Debug)]
	struct Ipv6Addr {
        address: String,
    }

	#[derive(Debug)]
	enum IpAddr {
	    V4(Ipv4Addr),
	    V6(Ipv6Addr),
	}

    let home = IpAddr::V4(
        Ipv4Addr{ 
            address: (127,0,0,1)
        }
    );
    let loopback = IpAddr::V6(
        Ipv6Addr{ 
            address: (String::from("::1"))
        }
    );

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}