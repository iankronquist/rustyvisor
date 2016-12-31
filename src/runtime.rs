#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop{}
}
