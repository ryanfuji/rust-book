// Let's look at a situation we might want to express in code and see why enums are useful and more
// appropriate to use than struct in this situation. Say we to work with IP addresses. Currently,
// two major standards are used for IP addresses: version 4 and version 6. These are the only
// possibilities for an IP address that our program will come across: we "enumerate" all possible
// variants, which is where enumeration gets its name
//
// Any IP address can be either a version 4 or a version 6 address, but not both at the same time.
// That property of IP addresses makes the enum data structure appropriate, because enum values
// can only be one of its variants. Both version 4 and version 6 addresses are still fundamentally
// IP addresses, so they should be treated as the same type when the code is handling situations
// that apply to any kind of IP address.
//
// We can express this concept in code by defining an `IpAddrKind` enumeration and listing the
// possible variants an IP address can be, `V4` and `V6`. These are the variants of the enum
enum IpAddrKind {
    V4,
    V6,
}
// `IpAddrKind` is now a custom data type that we can use elsewhere in our code

// Enum Values
//
// We can create instances of each of the 2 variants like this
#[allow(unused_variables)]
fn main() {
    // Note that the variants of the enum are namespaced under its identifier, and we use a double
    // colon to separate the two.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // can call the `route()` function like this
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.2"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // We attach data to each variant of the enum directly so there is no need for an extra struct
    let home_with_enum = IpAddrEnum::V4(String::from("127.0.0.2"));
    let loopback_with_enum = IpAddrEnum::V6(String::from("::1"));

    let home_with_enum2 = IpAddrEnum2::V4(127, 0, 0, 2);
    let loopback_with_enum2 = IpAddrEnum2::V6(String::from("::1"));
}

// The reason enums are useful is that now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of
// the same type `IpAddrKind`. We can then, for instance, define a function that takes any
// `IpAddrKind`
#[allow(unused_variables)]
fn route(ip_kind: IpAddrKind) {}

// We don't have a way to store the actual IP address data; we only know what "kind" it is. We could
// store the data like this
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// We can represent the same concept that we did above with the structs in a more concise way just
// using an enum, rather than an enum inside a struct, by putting data directly into each enum
// variant. This new definition of the `IpAddrEnum` says that both `V4` and `V6` variants will have
// associated `String` values.
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// There is another advantage to using an enum instead of a struct: each variant can have
// different types and amounts of associated data. Version 4 type IP addresses will always have
// 4 numeric components that will have values between 0 and 255. If we wanted to store `V4`
// addresses a 4 u8 values but still express `V6` as one `String` value, we wouldn't be able to
// a struct. Enums handle this with ease.
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another way to store the IP address data, this way uses the exact enum and variants that we've
// defined earlier but instead embeds the the address data inside the variants in the form of 2
// different structs, which are differently for each variant
struct Ipv4Addr {
    // data here
}

struct Ipv6Addr {
    // data here
}

#[allow(dead_code)]
enum IpAddrEnum3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
