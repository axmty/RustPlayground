enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // Awkward solution with match.
    match number {
        Some(i) => println!("Matched {:?}!", i),
        _ => {}, // Required, although it is 'useless'.
    }

    // Equivalent, but shorter and cleaner with if let.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // With an else branch.
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    // With an else-if branch.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }


    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
