// more info @ https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap.html

// An enum called Option<T> in the std library is used when absence is a possibility.
// It manifests itself as one of two "options":
// 
// - Some(T): An element of type T was found
// - None: No element was found
//
// These cases can either be explicitly handled via match or implicitly with unwrap.
// Implicit handling will either return the inner element or panic.

// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "poison" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let coke = Some("coke");
    let nothing = None;

    drink(coffee);
    drink(coke);
    drink(nothing);

    //let poison = Some("poison");
    //drink(poison);
}
