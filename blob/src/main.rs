use glob::glob;

// ? matches any one character
// * matches any number of characters
// {!glob} Matches anything that does not match glob
// {a,b,c} matches any one of a, b or c
// [abc] matches any character in the set a, b or c
// [^abc] matches any character not in the set a, b or c
// [a-z] matches any character in the range a to z, inclusive. A leading or trailing dash will be interpreted literally

fn main () {

    for entry in glob("/home/aldenpower/[b]*/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}