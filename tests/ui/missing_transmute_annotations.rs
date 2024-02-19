//@aux-build:macro_rules.rs

#![warn(clippy::missing_transmute_annotations)]

#[macro_use]
extern crate macro_rules;

macro_rules! local_bad_transmute {
    ($e:expr) => {
        std::mem::transmute($e)
    };
}

fn bar(x: i32) -> i32 {
    x
}

unsafe fn foo1() -> i32 {
    std::mem::transmute([1u16, 2u16])
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo2() -> i32 {
    std::mem::transmute::<_, _>([1u16, 2u16])
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo3() -> i32 {
    std::mem::transmute::<_, i32>([1u16, 2u16])
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo4() -> i32 {
    std::mem::transmute::<[u16; 2], _>([1u16, 2u16])
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo5() -> i32 {
    let x: i32 = bar(std::mem::transmute::<[u16; 2], _>([1u16, 2u16]));
    //~^ ERROR: transmute used without annotations
    bar(std::mem::transmute::<[u16; 2], _>([1u16, 2u16]))
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo6() -> i32 {
    local_bad_transmute!([1u16, 2u16])
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo7() -> i32 {
    // Should not warn.
    bad_transmute!([1u16, 2u16])
}

#[repr(i32)]
enum Foo {
    A = 0,
}

unsafe fn foo8() -> Foo {
    std::mem::transmute(0i32)
    //~^ ERROR: transmute used without annotations
}

unsafe fn foo9() -> i32 {
    std::mem::transmute(Foo::A)
    //~^ ERROR: transmute used without annotations
}

fn main() {
    unsafe {
        // Should not warn.
        std::mem::transmute::<[u16; 2], i32>([1u16, 2u16]);
        let x = std::mem::transmute::<[u16; 2], i32>([1u16, 2u16]);
        let x: i32 = std::mem::transmute::<[u16; 2], _>([1u16, 2u16]);
        let x: i32 = std::mem::transmute::<_, i32>([1u16, 2u16]);
        let x: i32 = std::mem::transmute([1u16, 2u16]);
    }
}
