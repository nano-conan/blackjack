use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// Why am i deriving here
enum Suit{
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    NoCard
}



fn main() {
    println!("Blackjack!");

    //  What will I need for data,
    //
    //  The numbers of cards in hand    this could be u32
    //  the value of cards in hand      this could be u32
    //
    //  the suit of cards in hand       this could be enum or an interpreted u32
    //  the running total               this could be u32
    //
    //  the number of cards in dealer hand
    //  the value of cards in dealer
    //  the suit of cards in dealer
    //
    //
    //  TODO: add multiple decks
    //  TODO: check for blackjack
    //

    let number_of_decks = 1;
    //this could be in a config file
    const MAX_CARDS:usize = 5;

    let mut player_cards_in_hand: u32 = 0;
    let mut player_hand_value = 0;
    let mut player_suits = [Suit::NoCard; MAX_CARDS];
    
    for _i in 0..5 {
        let card = draw_card(number_of_decks, &mut player_cards_in_hand, &mut player_hand_value, &mut player_suits);
    }
    //put this stuff in the function
    //
    //Should i? ya i should but use another function to show what im doing




    println!("Hello here are your arrays: player_cards_hand: {}", player_cards_in_hand);
    println!("Hello here are your arrays: player_suits: {:?}", player_suits);
    println!("Hello here are your arrays: player_hand_value: {}", player_hand_value);

}

fn draw_card(number_of_decks: u32, player_cards_in_hand: &mut u32, player_hand_value: &mut u32, player_suits: &mut [Suit]) -> (Suit, u32){

    let min_range = 0;
    let max_range = 52 * number_of_decks;
    println!("min_range: {}", min_range);
    println!("max_range: {}", max_range);
    let card = rand::thread_rng().gen_range(min_range, max_range); //Will this ever go to 52?? if it does, it'll cause an error in card_value
   
    println!("card: {}", card);

    let card: (Suit, u32)  = card_value(card);
    
    player_suits[*player_cards_in_hand as usize] = card.0;
    *player_cards_in_hand += 1;
    *player_hand_value += card.1;


    return card;

}

fn card_value(card_in: u32) -> (Suit, u32){
    let card_in = card_in % 52; //Cut off the "deck number"
    let suit = card_in / 13;
    let value = card_in%13;
    
    let suit: Suit = match suit {
        0 => Suit::Clubs, 
        1 => Suit::Diamonds,
        2 => Suit::Hearts,
        3 => Suit::Spades,
        _ => Suit::NoCard,
    };

    if suit == Suit::NoCard
    {
        println!("Error: Card number is larger than 51 or lower than 0");
    }

    return (suit, value);
}
