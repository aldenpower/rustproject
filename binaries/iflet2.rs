fn main(){    
    let key = "HOME";

    let home = if let Ok(val) = env::var(key)
    {  
        val
    } else {
        "none to".to_string()
    };

    // let home = match env::var(key) {
    //     Ok(val) => val,
    //     Err(_e) => "none".to_string(),
    // };

    println!("{:?}", home);
}