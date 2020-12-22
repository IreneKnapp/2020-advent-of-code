use advent_lib::prelude::*;

//use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::TryFrom;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;
  let input = advent_lib::group_lines_by_blanks(input);

  let mut decks: Vec<Vec<i64>> = Vec::new();

  for player_input in &input {
    let mut deck = Vec::new();

    let mut card_iterator = player_input.iter();
    let _ = card_iterator.next();
    for line in card_iterator {
      let card_id = line.parse::<i64>().unwrap();
      deck.push(card_id);
    }

    decks.push(deck);
  }

  let (_winner, score) = run_game(&decks);

  println!("{}", score);

  let (_winner2, score2) = run_game2(&decks);

  println!("{}", score2);

  Ok(())
}


fn run_game(input_decks: &Vec<Vec<i64>>) -> (usize, i64) {
  let mut decks: Vec<Vec<i64>> = Vec::new();
  for input_deck in input_decks {
    let mut deck = Vec::new();
    for card in input_deck {
      deck.push(*card);
    }
    decks.push(deck);
  }

  loop {
    let mut is_game_over = false;
    for deck in &decks {
      if deck.len() == 0 {
        is_game_over = true;
        break;
      }
    }
    if is_game_over {
      break;
    }

    let round_winner;
    if decks[0][0] > decks[1][0] {
      round_winner = 0;
    } else {
      round_winner = 1;
    }
    let round_loser = 1 - round_winner;

    let mut transferred_cards = Vec::new();
    transferred_cards.push(decks[round_winner][0]);
    transferred_cards.push(decks[round_loser][0]);

    for i in 0 .. decks.len() {
      let (_, rest_of_deck) = decks[i].split_at(1);
      let new_deck;
      if i == round_winner {
        new_deck = [rest_of_deck, transferred_cards.as_slice()].concat();
      } else {
        new_deck = rest_of_deck.to_vec();
      }
      decks[i] = new_deck;
    }
  }

  let mut winner = 0;
  let mut score = 0;
  for i in 0 .. decks.len() {
    let deck = &decks[i];
    if deck.len() > 0 {
      winner = i;

      for j in 0 .. deck.len() {
        let j_from_bottom = i64::try_from(deck.len() - j).unwrap();
        score += deck[j] * j_from_bottom;
      }
    }
  }

  (winner, score)
}


fn run_game2(input_decks: &Vec<Vec<i64>>) -> (usize, i64) {
  let mut decks = copy_decks(input_decks);

  let mut previous_configurations: BTreeSet<Vec<Vec<i64>>> = BTreeSet::new();

  loop {
    //println!("{:?}", decks);
    let mut is_game_over = false;
    for deck in &decks {
      if deck.len() == 0 {
        is_game_over = true;
        break;
      }
    }
    if is_game_over {
      break;
    }

    if previous_configurations.contains(&decks) {
      //println!("loop detected");
      return (0, score_deck(&decks[0]));
    }

    previous_configurations.insert(copy_decks(&decks));

    let round_winner;

    if decks[0].len() >= usize::try_from(decks[0][0]).unwrap() + 1
      && decks[1].len() >= usize::try_from(decks[1][0]).unwrap() + 1
    {
      let mut recursive_decks: Vec<Vec<i64>> = Vec::new();
      for i in 0 .. decks.len() {
        let mut recursive_deck = Vec::new();
        let n_to_take = usize::try_from(decks[i][0]).unwrap();
        for j in 1 .. n_to_take + 1 {
          recursive_deck.push(decks[i][j]);
        }
        recursive_decks.push(recursive_deck);
      }

      let (recursive_winner, _) = run_game2(&recursive_decks);
      round_winner = recursive_winner;
      //println!("recursing {:?}", decks);
    } else if decks[0][0] > decks[1][0] {
      round_winner = 0;
    } else {
      round_winner = 1;
    }

    let round_loser = 1 - round_winner;

    let mut transferred_cards = Vec::new();
    transferred_cards.push(decks[round_winner][0]);
    transferred_cards.push(decks[round_loser][0]);

    for i in 0 .. decks.len() {
      let (_, rest_of_deck) = decks[i].split_at(1);
      let new_deck;
      if i == round_winner {
        new_deck = [rest_of_deck, transferred_cards.as_slice()].concat();
      } else {
        new_deck = rest_of_deck.to_vec();
      }
      decks[i] = new_deck;
    }
  }

  let mut winner = 0;
  let mut score = 0;
  for i in 0 .. decks.len() {
    if decks[i].len() > 0 {
      winner = i;
      score = score_deck(&decks[i]);
    }
  }

  (winner, score)
}


fn copy_decks(input_decks: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
  let mut decks: Vec<Vec<i64>> = Vec::new();

  for input_deck in input_decks {
    let mut deck = Vec::new();
    for card in input_deck {
      deck.push(*card);
    }
    decks.push(deck);
  }

  decks
}


fn score_deck(deck: &Vec<i64>) -> i64 {
  let mut score = 0;

  for i in 0 .. deck.len() {
    let i_from_bottom = i64::try_from(deck.len() - i).unwrap();
    score += deck[i] * i_from_bottom;
  }

  score
}

