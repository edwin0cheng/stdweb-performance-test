#[macro_use]
extern crate stdweb;

fn test_js_macro() {
    let inc : i32 = 1;

    js!{
        @(no_return)
        window.pcounter += @{inc};
    };    
}

fn test_setup() {
    js!{
        @(no_return)
        window.pcounter = 0;
        window.ptime = performance.now();
    };  
}

fn test_time(s : &str) {
    js!{
        console.log(@{s} + " pcounter = " + window.pcounter);        
        console.log(@{s} + " ptime = " + (performance.now() - window.ptime));
    };
}


fn test_js_raw_macro() {
    let inc : i32 = 1;

    __js_raw_asm!(
        "window.pcounter += $0", inc
    );    
}

fn main() {
    stdweb::initialize();

    // Testing js! macro
    test_setup();

    for _ in 0..500000 {
        test_js_macro();
    }

    test_time("js! macro");

    // Testing js_raw macro
    test_setup();

    for _ in 0..500000 {
        test_js_raw_macro();
    }

    test_time("__js_raw_asm! macro");
}
