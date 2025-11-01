//t # Traits

//t A trait defines the functionality a particular type has and can share with
//t other types. We can use traits to define shared behavior in an abstract way.
//t We can use trait bounds to specify that a generic type can be any type that
//t has certain behavior. A type’s behavior consists of the methods we can call
//t on that type. Note that a trait can have associated functions (aka static
//t methods) as well.

pub trait Summary {
    fn summarize(&self) -> String;
    fn tag() -> u8;
}

//t Once a trait is defined, you can start implementing it for how many types
//t you need. If a type implements a trait, it can call the methods defined in
//t the trait like regular methods. The only difference is that the user must
//t bring the trait into scope as well as the types. Other crates that depend
//t on this crate can also bring the `Summary` trait into scope to implement it
//t on their own types.
//t
//t One restriction to note is that we can implement a trait on a type only
//t if either the trait or the type, or both, are local to our crate. In other
//t words, we can’t implement external traits on external types. This is known
//t as the **orphan rule**: it ensures that other people’s code can’t break your
//t code and vice versa by creating multiple ambiguous implementations.

pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let num_words = self.content.split(' ').count();
        format!("{}, by {} ({})", self.title, self.author, num_words)
    }

    fn tag() -> u8 {
        34
    }
}

fn ex_use_trait() {
    // Call the associated function.
    println!("tag: {}", NewsArticle::tag());

    let article = NewsArticle {
        title: "some title".to_string(),
        author: "Mark".to_string(),
        content: "text".to_string(),
    };

    // Call trait method.
    println!("summary: {}", article.summarize());
}

//t ## Default implementations
//t
//t Sometimes it’s useful to have default behavior for some methods in a
//t trait instead of requiring implementations for all methods on every
//t type. Then, as we implement the trait on a particular type, we can
//t keep or override each method’s default behavior.
//t
//t Default implementations can call other methods in the same trait, even
//t if those other methods don’t have a default implementation. In this way,
//t a trait can provide a lot of useful functionality and only require
//t implementors to specify a small part of it.

pub trait Notification {
    // Must be implemented manually.
    fn author(&self) -> String;

    // Default: can be reused or overridden.
    fn text(&self) -> String {
        format!("New notification from {}...)", self.author())
    }
}

//t To implement the trait for a type, we just need to implement the
//t methods that don't have a default implementation. If we don't override
//t the methods that have a default implementation, those implementations
//t will be used.

pub struct WhatsappMessage {
    pub sender: String,
    pub content: String,
}

impl Notification for WhatsappMessage {
    // The text() method uses the default implementation.
    // The author() method is implemented manually.
    fn author(&self) -> String {
        self.sender.clone()
    }
}

pub struct SmsMessage {
    pub sender: String,
    pub content: String,
}

impl Notification for SmsMessage {
    // The author() method is implemented manually.
    fn author(&self) -> String {
        self.sender.clone()
    }

    // The text() method overrides the default implementation.
    fn text(&self) -> String {
        format!("New SMS from {}: {}", self.sender.clone(), self.content)
    }
}

fn ex_use_trait_def() {
    // Both methods implemented manually.
    let message = SmsMessage {
        sender: "Mark".to_string(),
        content: "Hello!".to_string(),
    };

    println!("{}", message.author());
    println!("{}", message.text());

    // ✅ We didn't implement the text() method, but we are still
    // able to call it because there's a default implementation.
    let message = WhatsappMessage {
        sender: "Mark".to_string(),
        content: "Hello!".to_string(),
    };

    println!("{}", message.author());
    println!("{}", message.text());
}

//t ## Traits as bounds
//t
//t Trait can be used as bounds in generic functions and wrapper types to put a
//t constraint on which type can be uses as a type parameter (`T` in the example
//t below). In the example, you can instantiate a queue holding, each time, a
//t different type `T` as long as `T` implements the `Notification` trait.

struct NotificationQueue<T: Notification> {
    queue: Vec<T>,
    rpm: f32,
}

impl<T: Notification> NotificationQueue<T> {
    pub fn push(&mut self, n: T) {
        self.queue.push(n);
    }

    pub fn send(&mut self) -> bool {
        match self.queue.pop() {
            None => false,
            Some(item) => {
                // ... send notification
                true
            }
        }
    }
}

//t For functions, several syntaxes are available to bound the type parameter;
//t all are equivalent when the generic type is used as a function argument.
//t
//t - The simplest syntax is to use the `<T: Notification>` syntax, which means
//t that the type parameter `T` must implement the `Notification` trait.
//t
//t - Another option is to use the `where` clause to specify in a cleaner way
//t the trait bounds on the generic types. This is usually preferred when the
//t trait bounds are complex.
//t
//t - Finally, another syntax is to use the `impl` keyword in the parameter
//t list to specify that the argument must implement that trait. The result is
//t identical to the other 2 syntaxes, but note that it's less flexible.

pub fn ex_use_notif<T: Notification>(item: T) {
    println!("{}", item.text());
}

pub fn ex_use_notif_2<T>(item: T)
where
    T: Notification,
{
    println!("{}", item.text());
}

pub fn ex_use_notif_3(item: impl Notification) {
    println!("{}", item.text());
}

//t A type parameter can be subject to multiple trait bounds. In the example
//t below `T` must be a type that implements both the `Notification` and `Debug`
//t traits. We skipped the third option here (`impl` keyword), but it would
//t have used the + as in the other options.

use std::fmt::{Debug, Display};

// ⚠️ Signature starts to be a bit cluttered.
pub fn ex_use_notif_4<T: Notification + Debug, U: Display>(disclaimer: U, item: T) {
    println!("disclaimer: {}", disclaimer);
    println!("{}", item.text());
    println!("(full text: {:?})", item);
}

// ✅ Cleaner signature.
pub fn ex_use_notif_5<T, U>(disclaimer: U, item: T)
where
    T: Notification + Debug,
    U: Display,
{
    println!("disclaimer: {}", disclaimer);
    println!("{}", item.text());
    println!("(full text: {:?})", item);
}

//t We can also use the `impl Trait` syntax in the return position to return a
//t value of some type that implements a trait. By using `impl Notification`
//t for the return type, we specify that the function returns some type that
//t implements that trait without naming the concrete type. The syntax allows
//t you to concisely specify that a function returns some type that implements
//t the given trait without needing to write out a very long type.
//t
//t However, you can only use `impl Trait` if you’re returning a single type
//t from the function. This is because the compiler must compile down the
//t function return type to a single concrete type.

fn ex_ret_notif() -> impl Notification {
    WhatsappMessage {
        sender: "Mark".to_string(),
        content: "Hello!".to_string(),
    }
}

fn ex_ret_notif_2(b: bool) -> impl Notification {
    // ❌ This doesn't compile because the return type is ambiguous.
    // if b {
    //     WhatsappMessage {
    //         sender: "Mark".to_string(),
    //         content: "Hello!".to_string(),
    //     }
    // } else {
    //     SmsMessage{
    //         sender: "Simon".to_string(),
    //         content: "Hey!".to_string(),
    //     }
    // }

    // ✅ This compiles because the return type is well-defined.
    if b {
        return WhatsappMessage {
            sender: "Luke".to_string(),
            content: "Hello!".to_string(),
        };
    }

    WhatsappMessage {
        sender: "Mark".to_string(),
        content: "Hello!".to_string(),
    }
}

//t ### Conditional type implementations
//t
//t By using a trait bound within an `impl` block that uses generic type
//t parameters, we can implement methods conditionally for types the
//t satisfy the trait bound.

use std::ops::Add;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // This block defines methods that
    // are available on all Pair types.
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Add + Copy,
{
    // This block defines methods that are available
    // on Pair<T> types whose T support the addition
    // operation and can be copied cheaply.
    fn sum(&self) -> T::Output {
        self.x + self.y
    }
}

fn ex_cond_impl() {
    // ✅ new() and sum() are both available for Pair<i32>
    // since i32 implements both Add and Copy.
    let pair: Pair<i32> = Pair::new(1, 2);
    println!("x = {}, y = {}", pair.x, pair.y);
    println!("pair.sum() = {}", pair.sum());

    // ❌ new() is available on all Pairs, but sum()
    // is not available for Pair<&str> since &str
    // doesn't implement Add.
    let pair_str: Pair<&str> = Pair::new("hello", "world");
    println!("x = {}, y = {}", pair_str.x, pair_str.y);
    // println!("pair.sum() = {}", pair_str.sum()); doesn't compile
}

//t We can also conditionally implement a trait for any type that implements
//t another trait. Implementations of a trait on any type that satisfies the
//t trait bounds are called **blanket implementations** and are used extensively
//t in the Rust standard library. For example, the standard library implements
//t the `ToString` trait on any type that implements the `Display` trait, roughly
//t equivalent to `impl<T: Display> ToString for T { ... }`.
//t
//t Because the standard library has this blanket implementation, we can call
//t the `to_string` method defined by the `ToString` trait "for free" on any
//t type that implements the `Display` trait.

use std::fmt::{Formatter, Result};

impl Display for WhatsappMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Notification from {}, {}", self.author(), self.text())
    }
}

fn ex_blanket_impl_to_string() {
    let message = WhatsappMessage {
        sender: "Mark".to_string(),
        content: "Hello!".to_string(),
    };

    // ✅ We didn't implement to_string() for this type,
    // but the blanket implementation got us covered.
    println!("{}", message.to_string());
}

//t Note that the compiler detects and rejects conflicting implementations.
//t In this case, the compiler detects that the `ToString` trait is already
//t implemented for `WhatsappMessage` and rejects the manual implementation.
//t
//t ```rust
//t // ❌ Conflicting implementation.
//t impl ToString for WhatsappMessage {
//t     fn to_string(&self) -> String {
//t         self.text()
//t     }
//t }
//t
//t // error[E0119]: conflicting implementations of trait `ToString` for type `WhatsappMessage`
//t // --> src/chapters/traits.rs:339:1
//t // |
//t // 339 | impl ToString for WhatsappMessage {
//t //     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//t //     |
//t //     = note: conflicting implementation in crate `alloc`:
//t //     - impl<T> ToString for T
//t //       where T: std::fmt::Display, T: ?Sized;
//t ```

//t In an equivalent way, we can implement, for example, the `Notification` trait
//t on every tuple of two items when both implement the `ToString` trait.

impl<T: ToString, U: ToString> Notification for (T, U) {
    fn author(&self) -> String {
        self.0.to_string()
    }
    fn text(&self) -> String {
        self.1.to_string()
    }
}

fn ex_blanket_impl_notification() {
    let tuple = ("Mark", "Hello!");

    // ✅ We didn't implement `Notification` for (&str, &str),
    // but the blanket implementation above automatically
    // implemented it for us.
    println!("{}", tuple.text());
}
