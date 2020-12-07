#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod kirby;

#[skyline::main(name = "Kirbycide")]
pub fn main() {
    kirby::install();
}
