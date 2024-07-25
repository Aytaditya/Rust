struct Deck(){
    cards:Vec<String> // vectors are like arrays and they can grow and shrink in size 
    // rust also has arrays but they are fixed in size
}

fn main() {

    let deck=Deck{cards:vec![]};
    // let deck is a variable known as binding in rust 


    println!("Hello, world!");
    println!("Here is deck:{}",deck);s
}
