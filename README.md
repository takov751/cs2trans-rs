# cs2trans-rs

Very basic counter-strike2 chat translator using google translate. I am just learning rust, so the code is horrible.
The binary size is ~70 Mb due to local language detection without it roughly ~7Mb

This uses csgo and cs2 telnet console feature. So to make this work you have to run cs2 with the additional command:
```
 %command% -netconport 1337
```
where the number 1337 is the port number you need to connect to.

```
Usage: cs2trans-rs [OPTIONS]

Options:
  -h, --host <HOST>  [default: 127.0.0.1]
  -p, --port <PORT>  [default: 1337]
  -h, --help         Print help
```
