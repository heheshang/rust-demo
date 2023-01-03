#[derive(Debug)]
enum PokerSuit {
    Spade,
    Heart,
    Diamond,
    Club,
}

fn main() {
    let heart = PokerSuit::Heart;
    let spade = PokerSuit::Spade;
    let diamond = PokerSuit::Diamond;
    let club = PokerSuit::Club;
    print_suit(heart);
    print_suit(spade);
    print_suit(diamond);
    print_suit(club);

    let c1 = PokerCard {
        suit: PokerSuit::Spade,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Heart,
        value: 2,
    };
    println!("{:?}", c1);
    println!("{:?}", c2);
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn print_suit(card: PokerSuit) {
    // match card {
    //     PokerSuit::Spade => println!("Spade"),
    //     PokerSuit::Heart => println!("Heart"),
    //     PokerSuit::Diamond => println!("Diamond"),
    //     PokerSuit::Club => println!("Club"),
    // }
    println!("{:?}", card)
}
