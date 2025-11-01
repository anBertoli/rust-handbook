// # Closures

// Rust’s closures are anonymous functions you can save in a variable
// or pass as arguments to other functions. You can create the closure
// in one place and then call the closure elsewhere to evaluate it in a
// different context. Unlike functions, closures can capture values
// from the scope in which they’re defined.
//
// There are more differences between functions and closures. Closures
// don’t usually require you to annotate the types of the parameters or
// the return value like fn functions do. In addition, closures can capture
// values and references from the environment (scope).

fn ex_closures() {
    let n = 3;

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x| x + n;

    // v4 signature inferred from context
    add_one_v4(1_u32);

    let val = 3;
    let add_from_env = |x: i32| x + val;
    println!("{}", add_from_env(2));
}

// ## Closure Traits
// Each closure is of an anonymous type the compiler creates. You can't refer
// to the closure concrete type explicitly. The way a closure captures
// and handles values from the environment affects which traits the closure
// automatically implements. The compiler will decide which of these to use
// based on what the body of the function does with the captured values.
// Closures will automatically implement one, two, or all three of the following
// traits, in an incremental fashion, depending on how the closure’s body
// handles the env values.
//
// - `FnOnce`: consumes env values so can be called only once. A closure that
// consumes captured values (or drops them) will only implement FnOnce and none
// of the other closure traits. All closures implement at least this trait,
// because all closures can be called at least once.
//
// - `FnMut`: borrows mutably env values, applies to closures that mutate the
// captured values. These closures can be called more than once. It's a subtype
// of FnOnce, so it is accepted when a FnOnce is present in a trait bound.
//
// - `Fn`: borrow immutably env values, applies to closures that don’t consume
// captured values and that don’t mutate captured values. These closures can
// be called more than once without mutating their environment, which is
// important in cases such as calling a closure multiple times concurrently.
// It's a subtype of FnMut, so it is accepted when a FnOnce or a FnMut is
// present in a trait bound.
//
// Every `Fn` meets the requirements for `FnMut`, and every `FnMut` meets
// the requirements for `FnOnce`. They’re not three separate categories.
// Instead, `Fn` is a subtype of `FnMut`, which is a subtype of `FnOnce`;
// this makes `Fn` the most specific type of closure.

fn ex_multiple_calls() {

    // ❌ The closure is a FnOnce because it consumes
    // the string, so it can be called only one time.
    let non_copy_val = String::from("ehy");
    let my_fn_once = || {
        let str_bytes = non_copy_val.into_bytes();
        println!("str bytes: {:?}", str_bytes);
    };

    my_fn_once();
    // my_fn_once(); // doesn't compile

    // ✅ The closure is a FnMut because it modifies the string.
    // It can be called multiple times because it doesn't consume
    // any value. It is a subtype of FnOnce, so it is accepted
    // by FnOnce trait bound.
    //
    // Note that the closure must be declared as mutable to be
    // to mutate the captured values (you can think of it like
    // a struct capturing vals/references).
    let mut non_copy_val = String::from("ehy");
    let mut my_fn_mut = || {
        non_copy_val.push_str(" guys");
        String::new()
    };

    my_fn_mut();
    my_fn_mut();
    my_fn_mut();

    // ✅ The closure is a Fn because it doesn't modify the
    // string, so it can be called multiple times. It is a
    // subtype of FnOnce so it is accepted by unwrap_or_else.
    let my_fn = || {
        println!("{:?}", non_copy_val);
        String::new()
    };

    my_fn();
    my_fn();
    my_fn();
}

// ### `FnOnce`
// Using `FnOnce` in a trait bound expresses the constraint that the
// generic function/struct/item is only going to call the closure at
// most one time. Every closure trait is an `FnOnce` so all can be used
// in place of a `FnOnce`. A `FnOnce` closure is consumed after it is called.

enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyOption::Some(x) => x,
            MyOption::None => f(),
        }
    }
}

fn ex_trait_bounds_fn_once() {

    // ✅ Trivially, FnOnce is accepted by unwrap_or_else
    // which expects an FnOnce closure.
    let non_copy_val = String::from("ehy");
    let my_fn_once = || {
        let str_bytes = non_copy_val.into_bytes();
        println!("str bytes: {:?}", str_bytes);
    };

    MyOption::None.unwrap_or_else(my_fn_once);

    // ✅ The closure is a FnMut because it modifies the string,
    // It is a subtype of FnOnce so it is accepted by unwrap_or_else.
    let mut non_copy_val = String::from("ehy");
    let my_fn_mut = || {
        non_copy_val.push_str(" guys");
        println!("{}", non_copy_val);
    };

    MyOption::None.unwrap_or_else(my_fn_mut);

    // ✅ The closure is a Fn because it doesn't modify the
    // string, just uses a ref to it. It is a subtype of FnOnce
    // so it is accepted by unwrap_or_else.
    let my_fn = || {
        println!("{:?}", non_copy_val);
    };

    MyOption::None.unwrap_or_else(my_fn);
}

// ### `FnMut`
// FnMut is a subtype of FnOnce, so FnOnce closures don't satisfy
// FnMut trait bounds, while Fn closures do. A FnMut must be mut
// to be called. In the example the `map` function requires a
// closure that can be called multiple times (once for every item
// of the vector), and it's ok even if it mutates the captured env.

fn map<V, U, F>(list: Vec<V>, mut map_fn: F) -> Vec<U>
where
    F: FnMut(V) -> U,
{
    let mut out = Vec::with_capacity(list.len());
    for l in list {
        out.push(map_fn(l))
    }
    out
}

fn ex_trait_bounds_fn_mut() {

    // ❌ The closure is FnOnce because it consumes a variable from
    // its environment, so it doesn't meet the `map` requirements.
    let non_copy_val = String::from("ehy");
    let my_fn_once = || {
        let str_bytes = non_copy_val.into_bytes();
        println!("str bytes: {:?}", str_bytes);
    };

    // map(vec![1, 2, 3, 4], my_fn_once); doesn't compile

    // ✅ The closure is FnMut because it mutates the environment
    // but doesn’t consume anything, so it meets the `map` bound
    // requirements.
    let mut sum = 0;
    let my_fn_mut = |n| {
        sum += n;
        n + 1
    };

    map(vec![1, 2, 3, 4], my_fn_mut);

    // ✅The closure is a Fn because it doesn't modify the environment.
    // It is a subtype of the required FnMut trait, so it is accepted
    // by the `map` function.
    let my_fn = |n| format!("num: {:?}", n);

    map(vec![1, 2, 3, 4], my_fn);
}

// ### `Fn`
// `Fn` is a subtype of FnOnce and FnMut so FnOnce and FnMut closures
// doesn't satisfy Fn. It borrows env values immutably and can be
// called multiple times.

fn requires_fn<F: Fn(i32)>(my_fn: F) {
    my_fn(1);
    my_fn(2);
    my_fn(3);
}

fn example() {

    // ❌ The closure is FnOnce because it moves out a variable from
    // its environment, so it doesn't meet the `requires_fn` requirements.
    let mut strings: Vec<String> = vec![];
    let s = "str".to_string();
    let my_fn_once = |n: i32| {
        strings.push(s);
        n + 1;
    };

    // let res = requires_fn(my_fn_once); doesn't compile

    // ❌ The closure is FnMut because it mutates the environment but
    // doesn’t mutate or consumes anything from its environment. It
    // doesn't meet the stricter Fn requirements.
    let mut sum = 0;
    let my_fn_mut = |n: i32| {
        sum += n;
        n + 1
    };

    // let res = requires_fn(my_fn_mut); doesn't compile

    // ✅ The closure is a Fn because it doesn't modify or consumes
    // the environment. It meets the Fn function requirements.
    let my_fn = |n| {
        println!("num: {:?}", n);
    };

    let res = requires_fn(my_fn);
    println!("result: {:?}", res);
}

// ### `Move` keyword
//
// If you want to force the closure to take ownership of the values it
// uses in the environment, even though the body of the closure doesn’t
// strictly need ownership, you can use the `move` keyword before the
// parameters. Note that `move` and the trait of the closure are orthogonal
// features: whatever the trait of the closure is, `move` is an option.
// In other words: `move' determines how values are captured, the closure
// trait determines how values are used.

fn ex_move() {
    let list = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Before: {:?}", list);

    // It could be only borrowed, but `move` forces
    // the list to be moved inside the closure.
    let cl = move || {
        println!("from thread: {:?}", list);
        0
    };

    // ❌ doesn't compile, `list` was moved into the closure
    // println!("After: {:?}", list);
}

fn ex_fn_move() {
    // This closure uses the `move` keyword,
    // but it is a Fn closure nonetheless.
    let list = vec![1, 2, 3];
    let my_fn = move |n| println!("From thread: {:?}", list);
    requires_fn(my_fn);
}