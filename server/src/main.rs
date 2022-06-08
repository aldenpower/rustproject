fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow: &str = &string;
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    // let server = Server::new(string);
    // server.run();
}


struct Server {
    addr: String,
}

/*
There are two types of funcionality that we can associate
with struct:

METHODS
always take a special first parameter
called self, self represents the instance of the struct

ASSOCIATED FUNCTIONS
associated with the struct type but they dont need a instance
of the struct, similar to static functions in other languages
new is a associated function
*/

impl Server {
    /* 
    In every struct the special type "Self" is an alias for
    the name of the struct, so you can use Self instead of 
    Server
    */
    fn new(addr: String) -> Self {
        Self {
            /*
            If the name in the field is the same in the struct
            we can ommit the declaration addr: addr
            */
            addr
        }
    }
    /* 
    A method needs self. Passing only self the run method will
    get the ownership of the Server instance, passing &self the
    run method will borrow the instance, passig &mut self the
    run method willg get a mutable reference
    */
    fn run (&self) {

    }
}