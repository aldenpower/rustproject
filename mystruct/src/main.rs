fn main() {
    let aldenpower = build_user(
        String::from("anderson_afsl@hotmail.com"),
        false
    );

    println!("{}", aldenpower.email);

    let black = Color(0, 0, 0);

    let Color(x, y, z) = black;

    println!("{} {}", x, black.1);

    let subject = AlwaysEqual;

    // let mut entry = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // println!("{}", entry.email);
    // entry.email = String::from("rustacean@learn");
    // println!("{}", entry.email);
    // let k = String::from("jdaisjd");
    // println!("{}", build_user(k, false).email);

    // let mut entry2 = User {
    //     email: entry.email,
    //     username: entry.username,
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let mut entry2 = User {
    //     email: String::from("aaaaaa"),
    //     ..entry2
    // };

    // println!("{}", entry2.email)


}
// Tuple struct
struct Color(i32, i32, i32);

// Unit-like structs (trait implement)
struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user (email: String, active : bool) -> User {
    User {
        email,
        active,
        username : String::from("aldenpower"),
        sign_in_count : 1,
    }
}

