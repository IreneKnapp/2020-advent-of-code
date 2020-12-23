use advent_lib::prelude::*;

use std::convert::TryFrom;
use std::rc::Rc;
use std::cell::RefCell;


mod cup_structure {
  use std::rc::Rc;
  use std::cell::RefCell;

  pub struct Cup {
    label: usize,
    numeric_prev: Option<Rc<RefCell<Cup>>>,
    prev: Option<Rc<RefCell<Cup>>>,
    next: Option<Rc<RefCell<Cup>>>,
  }

  impl Cup {
    pub fn new(count: usize) -> Vec<Rc<RefCell<Cup>>> {
      let mut all_cups: Vec<Rc<RefCell<Cup>>> = Vec::new();
      let mut prev: Option<Rc<RefCell<Cup>>> = None;

      for i in 1 .. count + 1 {
        let cup_rc = Rc::new(RefCell::new(Cup {
          label: i,
          numeric_prev: None,
          prev: None,
          next: None,
        }));

        match prev {
          Some(prev_rc) => {
            let mut cup = cup_rc.borrow_mut();
            cup.numeric_prev = Some(Rc::clone(&prev_rc));
            cup.prev = Some(Rc::clone(&prev_rc));

            let mut prev_cup = prev_rc.borrow_mut();
            prev_cup.next = Some(Rc::clone(&cup_rc));
          },
          None => { },
        }

        prev = Some(Rc::clone(&cup_rc));
        all_cups.push(cup_rc);
      }

      {
        let first_rc = &all_cups[0];
        let last_rc = &all_cups[all_cups.len()-1];
        let mut first_cup = first_rc.borrow_mut();
        let mut last_cup = last_rc.borrow_mut();
        first_cup.numeric_prev = Some(Rc::clone(last_rc));
        first_cup.prev = Some(Rc::clone(last_rc));
        last_cup.next = Some(Rc::clone(first_rc));
      }

      all_cups
    }

    #[allow(dead_code)]
    pub fn label(&self) -> usize {
      self.label
    }

    #[allow(dead_code)]
    pub fn numeric_prev(&self) -> Rc<RefCell<Cup>> {
      Rc::clone(self.numeric_prev.as_ref().unwrap())
    }

    #[allow(dead_code)]
    pub fn next(&self) -> Rc<RefCell<Cup>> {
      Rc::clone(self.next.as_ref().unwrap())
    }

    #[allow(dead_code)]
    pub fn prev(&self) -> Rc<RefCell<Cup>> {
      Rc::clone(self.prev.as_ref().unwrap())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
      let mut result = 1;

      let mut next_rc = Rc::clone(self.next.as_ref().unwrap());
      loop {
        if next_rc.borrow().label == self.label {
          break;
        }
        let next_rc_tmp = Rc::clone(&next_rc.borrow().next.as_ref().unwrap());
        next_rc = next_rc_tmp;
        result += 1;
      }

      result
    }

    pub fn find(&self, to_find: usize) -> Option<Rc<RefCell<Cup>>> {
      let self_rc;
      {
        let next_cup = self.next.as_ref().unwrap().borrow();
        self_rc = Rc::clone(&next_cup.prev.as_ref().unwrap());
      }

      if self.label == to_find {
        Some(self_rc)
      } else {
        let mut point_rc = Rc::clone(self.next.as_ref().unwrap());
        loop {
          let point_label;
          {
            let point_cup = point_rc.borrow();
            point_label = point_cup.label;
          }
          if point_label == self.label {
            return None;
          }
          if point_label == to_find {
            return Some(point_rc);
          }
          {
            let next_point_rc = Rc::clone(point_rc.borrow().next.as_ref().unwrap());
            point_rc = next_point_rc;
          }
        }
      }
    }

    pub fn split_at(self_rc: Rc<RefCell<Cup>>, count: usize)
      -> (Rc<RefCell<Cup>>, Rc<RefCell<Cup>>)
    {
      let mut tail_rc = Rc::clone(self_rc.borrow().next.as_ref().unwrap());
      for _ in 1 .. count {
        let new_tail_rc = Rc::clone(tail_rc.borrow().next.as_ref().unwrap());
        tail_rc = new_tail_rc;
      }

      let head_last_rc = Rc::clone(tail_rc.borrow().prev.as_ref().unwrap());
      let tail_last_rc = Rc::clone(self_rc.borrow().prev.as_ref().unwrap());

      if Rc::ptr_eq(&tail_rc, &tail_last_rc) {
        let mut tail_cup = tail_rc.borrow_mut();
        tail_cup.prev = Some(Rc::clone(&tail_rc));
        tail_cup.next = Some(Rc::clone(&tail_rc));
      } else {
        let mut tail_cup = tail_rc.borrow_mut();
        let mut tail_last_cup = tail_last_rc.borrow_mut();
        tail_cup.prev = Some(Rc::clone(&tail_last_rc));
        tail_last_cup.next = Some(Rc::clone(&tail_rc));
      }

      if Rc::ptr_eq(&self_rc, &head_last_rc) {
        let mut self_cup = self_rc.borrow_mut();
        self_cup.next = Some(Rc::clone(&self_rc));
        self_cup.prev = Some(Rc::clone(&head_last_rc));
      } else {
        let mut self_cup = self_rc.borrow_mut();
        let mut head_last_cup = head_last_rc.borrow_mut();
        head_last_cup.next = Some(Rc::clone(&self_rc));
        self_cup.prev = Some(Rc::clone(&head_last_rc));
      }

      (Rc::clone(&self_rc), Rc::clone(&tail_rc))
    }

    pub fn splice_after(self_rc: Rc<RefCell<Cup>>,
                        to_insert_rc: Rc<RefCell<Cup>>)
    {
      let mut self_cup = self_rc.borrow_mut();

      let old_next_rc = Rc::clone(self_cup.next.as_ref().unwrap());

      let to_insert_last_rc =
          Rc::clone(to_insert_rc.borrow().prev.as_ref().unwrap());
      {
        let mut to_insert_last_cup = to_insert_last_rc.borrow_mut();
        to_insert_last_cup.next = Some(Rc::clone(&old_next_rc));
      }

      if Rc::ptr_eq(&old_next_rc, &self_rc) {
        self_cup.prev = Some(Rc::clone(&to_insert_last_rc));
      } else {
        let mut old_next_cup = old_next_rc.borrow_mut();
        old_next_cup.prev = Some(Rc::clone(&to_insert_last_rc));
      }

      let mut to_insert_cup = to_insert_rc.borrow_mut();
      to_insert_cup.prev = Some(Rc::clone(&self_rc));
      self_cup.next = Some(Rc::clone(&to_insert_rc));
    }

    pub fn splice_before(self_rc: Rc<RefCell<Cup>>, to_insert_rc: Rc<RefCell<Cup>>) {
      let prev_cup_rc = Rc::clone(self_rc.borrow().prev.as_ref().unwrap());
      Cup::splice_after(prev_cup_rc, to_insert_rc);
    }
  }
}  // mod cup_structure

use cup_structure::Cup;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut initial_cups: Vec<usize> = Vec::new();
  for c in input[0].chars() {
    let label = usize::try_from(c.to_digit(10).unwrap()).unwrap();
    initial_cups.push(label);
  }

  {
    let max_cup = initial_cups.len();
    let cups_by_zero_based_label = Cup::new(max_cup);

    let mut label_iter = initial_cups.iter();
    let first_label: usize = *label_iter.next().unwrap();
    let first_cup_rc = &cups_by_zero_based_label[first_label-1];
    let (mut cups_rc, _) = Cup::split_at(Rc::clone(first_cup_rc), 1);
    for label in label_iter {
      let next_cup;
      {
        let next_cup_rc = &cups_by_zero_based_label[*label-1];
        let (next_cup_tmp, _) = Cup::split_at(Rc::clone(next_cup_rc), 1);
        next_cup = next_cup_tmp;
      }
      Cup::splice_before(Rc::clone(&cups_rc), next_cup);
    }

    for _ in 0 .. 100 {
      cups_rc = iterate_game(Rc::clone(&cups_rc));
    }

    let output1 = game_output1(Rc::clone(&cups_rc));
    println!("{}", output1);
  }

  {
    let max_cup = 1000000;
    let cups_by_zero_based_label = Cup::new(max_cup);
    let max_defined_cup = initial_cups.len();

    let mut label_iter = initial_cups.iter();
    let first_label: usize = *label_iter.next().unwrap();
    let first_cup_rc = &cups_by_zero_based_label[first_label-1];
    let (mut cups_rc, _) = Cup::split_at(Rc::clone(first_cup_rc), 1);
    for label in label_iter {
      let next_cup_rc = &cups_by_zero_based_label[*label-1];
      let (next_cup, _) = Cup::split_at(Rc::clone(next_cup_rc), 1);
      Cup::splice_before(Rc::clone(&cups_rc), next_cup);
    }
    Cup::splice_before(Rc::clone(&cups_rc),
        Rc::clone(&cups_by_zero_based_label[max_defined_cup]));

    for _ in 0 .. 10000000 {
      cups_rc = iterate_game(Rc::clone(&cups_rc));
    }

    let output = game_output2(Rc::clone(&cups_rc));
    println!("{}", output);
  }

  Ok(())
}


fn game_output1(cups: Rc<RefCell<Cup>>) -> String {
  let mut output = String::new();

  let mut cup = cups.borrow().find(1).unwrap();
  {
    let next_cup = cup.borrow().next();
    cup = next_cup;
  }
  loop {
    if cup.borrow().label() == 1 {
      break;
    }
    output.push_str(&format!("{}", cup.borrow().label()));
    let next_cup = cup.borrow().next();
    cup = next_cup;
  }

  output
}


fn game_output2(cups: Rc<RefCell<Cup>>) -> String {
  let mut result = 1;

  let mut cup = cups.borrow().find(1).unwrap();
  {
    let next_cup = cup.borrow().next();
    cup = next_cup;
  }
  result *= cup.borrow().label();
  {
    let next_cup = cup.borrow().next();
    cup = next_cup;
  }
  result *= cup.borrow().label();

  format!("{}", result)
}


fn iterate_game(cups: Rc<RefCell<Cup>>) -> Rc<RefCell<Cup>> {
  let current_cup = Rc::clone(&cups);
  let cups = cups.borrow().next();

  let (transferred_cups, _) = Cup::split_at(cups, 3);

  let mut destination_cup = current_cup.borrow().numeric_prev();

  loop {
    let destination_cup_label = destination_cup.borrow().label();
    if transferred_cups.borrow().find(destination_cup_label).is_none() {
      break;
    }
    let next_destination_cup =
        Rc::clone(&destination_cup.borrow().numeric_prev());
    destination_cup = next_destination_cup;
  }

  Cup::splice_after(destination_cup, transferred_cups);

  let next_current_cup = current_cup.borrow().next();
  next_current_cup
}
