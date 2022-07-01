fn larg(vec: &[i32]) -> i32 {
    let mut largest = vec[0];

    for &item in vec {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}


fn main () {
    let number_list = vec![12, 50, 25, 10, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("the largest number is {}", largest);

}