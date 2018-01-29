# stdweb-performance-test
A simple performance test for stdweb


## Prerequisite
cargo-web

## Build : 
cargo web start --target-webasm --release

## Result :
Open develop console, you should see something like :
```
js! macro pcounter = 500000
js! macro ptime = 36

__js_raw_asm! macro pcounter = 500000
__js_raw_asm! macro ptime = 13.32000000000005
'''
