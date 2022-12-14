// more info @ https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_let.html
//
// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    // if Foo::Bar == a {
    // ^-- this causes a compile-time error. Use `if let` instead.
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
