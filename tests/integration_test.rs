use anyhow::Result;

#[tokio::test]
async fn test_check_vulnerabilty() -> Result<()> {
    let rsa_pub = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAABBAQDHHTnQl3oedd1FfTHJro2vt7wXPZwpB831gBnOvg21+ldhiG/p1K1hk9mOYmgg1Y0//bEMFGv7LTZestJHzdhN/pdgU/hMqExTpomgbTkyx7QNz3oCMJtEuLrbxUAYZdOk7zU3Xp/sKIakdlBLuF+m+8d0D+gzfzjctffaQJ5JK3yz4CyHXI/NUc8A4YG3P525ergpt81HqCL/Fhb55UyhE+/bmnMTk+MIAK0HiNn+praMN/SvgpQApw7SVrMMUBr/N7SoxLx6AInUNt1UXQf77/C81ewGUoHSwR8wbi7KYrsdE27nirav4YZyf4X6jWBM1XYXEwNONp0o1TtNMiwh secret@secret".to_string();
    match cve::is_vulnerable("181.150.41.101", "admin", rsa_pub).await? {
        true => println!("Service is VULNERABLE"),
        false => println!("Service is not vulnerable"),
    };
    Ok(())
}
