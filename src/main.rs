#![no_std]
#![no_main]

// Create a module named "sample_module" with version 1.0
psp::module!("deno_psp", 1, 0);

fn psp_main() {
    psp::enable_home_button();
    psp::dprintln!("Hello PSP from rust!");
}