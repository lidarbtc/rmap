<div align=center>
 
# rmap
 <p>
 <img src="https://img.shields.io/github/stars/lidarbtc/rmap?color=%23DF0067&style=for-the-badge"/> &nbsp;
 <img src="https://img.shields.io/github/forks/lidarbtc/rmap?color=%239999FF&style=for-the-badge"/> &nbsp;
 <img src="https://img.shields.io/github/license/lidarbtc/ramp?color=%23E8E8E8&style=for-the-badge"/> &nbsp;
 
Fast http/https single host scanner for find original IP behind CDN

## Language</br>

<img src="https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324"/></br>

</div>

## Features

- http/https support
- Detect with word or favicon
- Async
- Fast

## Install on Unix like

```sh
git clone https://github.com/lidarbtc/rmap.git

cd rmap

cargo build --release

mv target/release/rmap .
```

## Example

```sh
Use command line : ./rmap <option>
      └──────────> ./rmap --help
```

### Before scan with rmap

Before starting the scan, use `masscan` or `zmap` to obtain a list of IP addresses with open ports 80 and 443. Use this list for scanning with `rmap`. This approach allows `rmap` to efficiently scan these IP addresses to find the original IP hidden behind a CDN.

## Help

```sh
./rmap --help
Fast http/https single host scanner for find original IP behind CDN

Usage: rmap [OPTIONS]

Options:
  -i, --input <IP_FILE_PATH>    Path to the IP file
  -f, --favicon <FAVICON_PATH>  Path to the favicon file; saves IP if the website contains a matching favicon
  -t, --trigger <TRIGGER_WORD>  Trigger word to identify matching websites; saves IP if the website contains this word
  -d, --delay <DELAY>           Delay time (in milliseconds) between task creations
  -o, --output <OUTPUT_PATH>    Path to the output file where results will be saved
  -w, --host <HOST>             Host (domain) to search for
  -s, --https                   Set to true to use HTTPS, false for HTTP
  -h, --help                    Print help
  -V, --version                 Print version
```
