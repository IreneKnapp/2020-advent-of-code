use advent_lib::prelude::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone,Debug)]
struct Food {
  ingredients: Vec<String>,
  allergens: Vec<String>,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut foods: Vec<Food> = Vec::new();
  let mut ingredients: BTreeSet<String> = BTreeSet::new();
  let mut allergens: BTreeSet<String> = BTreeSet::new();
  let mut solved_ingredients: BTreeSet<String> = BTreeSet::new();
  let mut solved_allergens: BTreeSet<String> = BTreeSet::new();

  for line in &input {
    let paren_point = line.find('(').unwrap();
    let (ingredients_input, _) = line.split_at(paren_point - 1);
    let (_, rest) = line.split_at(paren_point);

    let mut food_ingredients = Vec::new();
    for word in ingredients_input.split(' ') {
      food_ingredients.push(word.to_string());
      ingredients.insert(word.to_string());
    }

    let mut food_allergens = Vec::new();
    let mut allergens_iterator = rest.split(' ');
    let _ = allergens_iterator.next();
    for word in allergens_iterator {
      let (stripped_word, _) = word.split_at(word.len() - 1);
      food_allergens.push(stripped_word.to_string());
      allergens.insert(stripped_word.to_string());
    }

    foods.push(Food {
      ingredients: food_ingredients,
      allergens: food_allergens,
    });
  }

  let mut ingredients_could_have_allergens: BTreeMap<String,BTreeSet<String>> =
      BTreeMap::new();
  for food in &foods {
    for ingredient in &food.ingredients {
      let mut allergen_set = BTreeSet::new();
      for allergen in &food.allergens {
        allergen_set.insert(allergen.clone());
      }

      match ingredients_could_have_allergens.get(ingredient) {
        Some(previous_set) => {
          for allergen in previous_set {
            allergen_set.insert(allergen.to_string());
          }
        },
        None => { }
      }

      ingredients_could_have_allergens.insert(ingredient.to_string(), allergen_set);
    }
  }

  let mut allergens_could_be_ingredients: BTreeMap<String,BTreeSet<String>> =
      BTreeMap::new();
  for food in &foods {
    for allergen in &food.allergens {
      let mut ingredient_set = BTreeSet::new();
      for ingredient in &food.ingredients {
        ingredient_set.insert(ingredient.clone());
      }

      match allergens_could_be_ingredients.get(allergen) {
        Some(previous_set) => {
          let mut intersected_set = BTreeSet::new();
          for ingredient in ingredient_set.intersection(previous_set) {
            intersected_set.insert(ingredient.to_string());
          }
          ingredient_set = intersected_set;
        },
        None => { }
      }

      allergens_could_be_ingredients.insert(allergen.to_string(), ingredient_set);
    }
  }

  loop {
    let mut removed_any = false;

    loop {
      let mut mapping_to_remove: Option<(String,String)> = None;
      for (allergen, possible_ingredients) in &allergens_could_be_ingredients {
        if solved_allergens.contains(allergen) {
          continue;
        }

        if possible_ingredients.len() == 1 {
          let allergen = allergen.to_string();
          let ingredient = possible_ingredients.iter().next().unwrap().to_string();

          solved_ingredients.insert(ingredient.clone());
          solved_allergens.insert(allergen.clone());
          mapping_to_remove = Some((allergen, ingredient));
          removed_any = true;
          break;
        }
      }

      match mapping_to_remove {
        Some((allergen_to_remove, ingredient_to_remove)) => {
          for allergen in &allergens {
            if *allergen == allergen_to_remove {
              continue;
            }
            let mut possible_ingredients =
                allergens_could_be_ingredients.get(allergen).unwrap().clone();
            possible_ingredients.remove(&ingredient_to_remove);
            allergens_could_be_ingredients.insert(
                allergen.to_string(), possible_ingredients);
          }

          for ingredient in &ingredients {
            if *ingredient == ingredient_to_remove {
              continue;
            }
            let mut possible_allergens =
                ingredients_could_have_allergens.get(ingredient).unwrap().clone();
            possible_allergens.remove(&allergen_to_remove);
            ingredients_could_have_allergens.insert(
                ingredient.to_string(), possible_allergens);
          }
        },
        None => {
          break;
        },
      }
    }

    loop {
      let mut mapping_to_remove: Option<(String,String)> = None;
      for (ingredient, possible_allergens) in &ingredients_could_have_allergens {
        if solved_ingredients.contains(ingredient) {
          continue;
        }

        if possible_allergens.len() == 1 {
          let ingredient = ingredient.to_string();
          let allergen = possible_allergens.iter().next().unwrap().to_string();

          solved_ingredients.insert(ingredient.clone());
          solved_allergens.insert(allergen.clone());
          mapping_to_remove = Some((ingredient, allergen));
          removed_any = true;
          break;
        }
      }

      match mapping_to_remove {
        Some((ingredient_to_remove, allergen_to_remove)) => {
          for allergen in &allergens {
            if *allergen == allergen_to_remove {
              continue;
            }
            let mut possible_ingredients =
                allergens_could_be_ingredients.get(allergen).unwrap().clone();
            possible_ingredients.remove(&ingredient_to_remove);
            allergens_could_be_ingredients.insert(
                allergen.to_string(), possible_ingredients);
          }

          for ingredient in &ingredients {
            if *ingredient == ingredient_to_remove {
              continue;
            }
            let mut possible_allergens =
                ingredients_could_have_allergens.get(ingredient).unwrap().clone();
            possible_allergens.remove(&allergen_to_remove);
            ingredients_could_have_allergens.insert(
                ingredient.to_string(), possible_allergens);
          }
        },
        None => {
          break;
        },
      }
    }

    if !removed_any {
      break;
    }
  }

  let mut ingredients_without_allergens = BTreeSet::new();
  for ingredient in &ingredients {
    let possible_allergens = ingredients_could_have_allergens.get(ingredient).unwrap();
    if possible_allergens.len() == 0 {
      ingredients_without_allergens.insert(ingredient.to_string());
    }
  }

  let mut appearances = 0;
  for food in &foods {
    for ingredient in &food.ingredients {
      if ingredients_without_allergens.contains(ingredient) {
        appearances += 1;
      }
    }
  }
  println!("{}", appearances);

  let mut dangerous_ingredients: Vec<String> = Vec::new();
  let mut sorted_allergens = Vec::new();
  for allergen in &allergens {
    sorted_allergens.push(allergen);
  }
  sorted_allergens.sort();
  for allergen in &sorted_allergens {
    let ingredients = allergens_could_be_ingredients.get(&allergen.to_string()).unwrap();
    if ingredients.len() == 1 {
      dangerous_ingredients.push(ingredients.iter().next().unwrap().to_string());
    }
  }
  let list = dangerous_ingredients.join(",");
  println!("{}", list);

  Ok(())
}

