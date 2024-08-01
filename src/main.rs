#[derive(Debug)] // this is a rust macro that allows us to print the struct in a readable format
struct Deck{
    cards:Vec<String> // vectors are like arrays and they can grow and shrink in size 
    // rust also has arrays but they are fixed in size
}

fn main() {

    // Generating all 52 cards in a deck
    // list of suits- hearts , spades
    // list of values - one two three

    // double nested for loop to generate all 52 cards

    
    let deck=Deck{cards:vec![]}; // creating a new instance of the Deck struct
    //let deck=Deck{cards:vec!["two of hearts".to_string(),"ace of spades".to_string()]};
    // let deck is a variable known as binding in rust 

    println!("Here is deck:{:?}",deck);
}
