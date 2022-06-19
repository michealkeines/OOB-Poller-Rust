# OOB-Poller-Rust

Advanced Polling Server for Out-Of-Band Automated Testing Scripts

# API

`/{sha256sum}` - Stored shasum to internal HashMap - returns Added Message

`/check/{ip}/{sha256sum}` - Checks if shasum is stored under the given IP - return Vulnerable if shasum is found or Not Vulnerable

`/remove/{ip}/{sha256sum}` - Removes the shasum under the given IP - returns Removed Message

# Use Cases

-> Vulnerable Service is injected with a payload to invoke a Out-Of-Band HTTP request with an unique sha256sum and if the request was made we can confirm, by making a call to `/check/{target_ip}/{unique_sha256sum}`

-> Vulnerable Service is injected with a payload to invoke protocol specific initial request, which will be hashed and stored, if the request was made we can confirm, by making a call to `/check/{target_ip}/{protocol_initial_request_sha256sum}`

# Clean-up

-> after every Vulnerable response, we can call `/remove/{ip}/{hash}` to remove that hash from the in-memory HashMap

# Usage

```
we need two open ports the HTTP Listener will be started in 9000 for the TCP Listener we can specify the IP:PORT

cargo run -- 127.0.0.1:9001
```

# Demo

![](https://github.com/michealkeines/OOB-Poller-Rust/blob/main/oob.gif)