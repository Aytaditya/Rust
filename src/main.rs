#[derive(Debug)] // this is a rust macro that allows us to print the struct in a readable format
struct Deck{
    cards:Vec<String> // vectors are like arrays and they can grow and shrink in size 
    // rust also has arrays but they are fixed in size
}

fn main() {

    // declaration of array 
    let arr=[1,2,3,4,5];  // array is fixed and cannot grow or shrink in size


    // Generating all 52 cards in a deck

    // list of suits- hearts , spades
    //let suits=vec!["hearts","spades","clubs"]; // vector of strings
    let suits=["hearts","spades","clubs","diamonds"]; // array of strings

    // list of values - one two three
    //let values=vec!["one","two","three"]; // vector of strings
    let values=["one","two","three"]; // array of strings

    let mut cards=vec![]; // empty vector of cards 
    // by default it is immutable so we need to make it mutable by using mut keyword

    // double nested for loop to generate all 52 cards
    for suit in suits{
        for value in values{
            let card=format!("{} of {}",value,suit); // format! is a macro that allows us to format strings
            cards.push(card); // push the card into the cards vector
        }
    }

    println!("Here are the cards:{:?}",cards);

    let deck=Deck{cards:vec![]}; // creating a new instance of the Deck struct
    //let deck=Deck{cards:vec!["two of hearts".to_string(),"ace of spades".to_string()]};
    // let deck is a variable known as binding in rust 

    println!("Here is deck:{:?}",deck);
}
