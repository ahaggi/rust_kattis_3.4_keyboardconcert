use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::iter::FromIterator;
use std::time::Instant;

fn gen_input_data() -> (Vec<Vec<u16>>, Vec<u16>) {
    /*
    2 10
    2 1 2
    2 2 3
    1 2 1 2 3 3 2 3 1 3
    */
    fn read_line_u16() -> Vec<u16> {
        let stdin = io::stdin();
        let mut val = String::new();
        stdin.read_line(&mut val).expect("Failed to read line");
        let val: Vec<u16> = val
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        val
    }
    fn read_line_u16_without_fst_char() -> Vec<u16> {
        let stdin = io::stdin();
        let mut val = String::new();
        stdin.read_line(&mut val).expect("Failed to read line");
        let val: Vec<u16> = val
            .trim()
            .split_whitespace()
            .enumerate()
            .map(|(i, num)| {
                if i != 0 {
                    Some(num.parse().unwrap())
                } else {
                    None
                }
            })
            .filter(|o| o.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        val
    }

    let first_line = read_line_u16();
    let nr_of_keyboards: u16 = first_line[0];

    let mut keyboards: Vec<Vec<u16>> = vec![];

    for _ in 1..=nr_of_keyboards {
        let mut v: Vec<u16> = read_line_u16_without_fst_char();
        v.sort();
        keyboards.push(v);
    }
    let tune: Vec<u16> = read_line_u16();

    ///////////////////////////////Dummy data/////////////////////////////////////
    // let mut keyboards: Vec<Vec<u16>> = vec![];

    // let v1 = vec![1, 2];
    // let v2 = vec![2, 3];
    // keyboards.push(v1);
    // keyboards.push(v2);

    // let tune: Vec<u16> = vec![1, 2, 1, 2, 3, 3, 2, 3, 1, 3];
    //-----------------------------------OR--------------------------------------

    // let mut rng = rand::thread_rng();

    // let tune: Vec<u16> = (1..=1000).map(|_| rng.gen_range(1, 1000)).collect();

    // let nr_of_keyboards:usize = 10;
    // let mut keyboards: Vec<Vec<u16>> = vec![ vec![0;1000]; nr_of_keyboards];

    // let tune_hashset:HashSet<u16> = HashSet::from_iter(tune.iter().cloned());
    // for note in tune_hashset {
    //     let base = rng.gen_range(0, nr_of_keyboards);
    //     for ind in base..base+4 {
    //         let k_board = &mut keyboards[ind % nr_of_keyboards];
    //         k_board[note as usize]+=1;
    //     }
    // }

    // for k_board in &mut keyboards {
    //     let nr_of_extra_keys = rng.gen_range(100, 300);

    //     for _ in 0..nr_of_extra_keys {
    //         k_board[rng.gen_range(0, 1000)] +=1;
    //     }

    //     for ind in 0..k_board.len(){
    //         if k_board[ind] == 0 { continue; }
    //         k_board[ind] = ind as u16;
    //     }
    //     k_board.retain(|val| *val != 0 );

    // }

    ///////////////////////////////////////////////////////////////////////////////

    //***************************************************************************//
    // let mut file = File::create("data.txt").expect("Unable to create file");
    // writeln!(file, "keyboards\n {:?} ", keyboards).expect("Unable to writeln!");

    // writeln!(file, "tune\n {:?} ", tune).expect("Unable to writeln!");

    // println!("keyboards {:?} tune {:?}", keyboards, tune);
    //***************************************************************************//

    (keyboards, tune)
}

pub fn main() {
    // input
    let (mut keyboards, tune): (Vec<Vec<u16>>, Vec<u16>) = gen_input_data();
     println!("{}", calc(&mut keyboards, &tune));
    
}

fn calc(keyboards: &mut Vec<Vec<u16>>, tune: &Vec<u16>) -> u16 {
    let now = Instant::now();
    let mut nr_of_switchs: u16 = 0;
    let mut map: HashMap<u16, HashSet<usize>> = HashMap::new();

    let mut using: HashSet<usize> = HashSet::new();

    for note in tune.iter() {
        if map.get(note).is_none() {
            let mut hash_set: HashSet<usize> = HashSet::new();
            for (keyboard_index, keyboard) in keyboards.iter_mut().enumerate() {
                if keyboard.binary_search(note).is_ok() {
                    hash_set.insert(keyboard_index);
                }
            }
            map.insert(*note, hash_set);
        }

        let available_keyboards_ind = map.get(note).unwrap();

        //first note in the tune
        if using.len() == 0 {
            using = available_keyboards_ind.clone();
        } else {
            using = using
                .intersection(&available_keyboards_ind)
                .copied()
                .collect::<HashSet<usize>>();

            if using.len() == 0 {
                nr_of_switchs += 1;
                using = available_keyboards_ind.clone();
            }
        }
    }
    nr_of_switchs
}

fn calc_alt(keyboards: Vec<Vec<u16>>, tune: Vec<u16>) -> u16 {
    // input filtered
    let (keyboards, ignore_notes): (Vec<Vec<u16>>, Vec<u16>) =
        retain_only_distinctive_keys(keyboards);

    let mut set: Vec<usize> = rest_the_set(&keyboards);

    let mut nr_of_used_keyboards: u16 = 0;

    let mut chosen: i16 = -1;
    for elm in tune.iter() {
        if ignore_notes.binary_search(elm).is_err() {
            if chosen != -1 {
                if keyboards[chosen as usize].binary_search(&elm).is_ok() {
                    continue; // read the next elm
                } else {
                    set = rest_the_set(&keyboards);
                    chosen = -1;
                    reduce_set(&keyboards, &mut set, elm);
                    if set.len() == 1 {
                        nr_of_used_keyboards += 1;
                        chosen = set[0] as i16;
                    }
                }
            } else {
                reduce_set(&keyboards, &mut set, elm);
                if set.len() == 1 {
                    nr_of_used_keyboards += 1;
                    chosen = set[0] as i16;
                }
            }
        }
    }
    // in case we can play (the whole OR last bit of the tune) with more than one keyboard
    if set.len() != 1 {
        nr_of_used_keyboards += 1;
    }

    fn rest_the_set(keyboards: &Vec<Vec<u16>>) -> Vec<usize> {
        (0..keyboards.len()).map(|x| x).collect()
    }
    fn reduce_set(keyboards: &Vec<Vec<u16>>, set: &mut Vec<usize>, elm: &u16) {
        assert!(set.len() != 1); // eqv to chosen == -1
        set.retain(|ind| keyboards[*ind].binary_search(elm).is_ok());
    }
    fn retain_only_distinctive_keys(keyboards: Vec<Vec<u16>>) -> (Vec<Vec<u16>>, Vec<u16>) {
        let mut ignore_notes: Vec<u16> = vec![];
        let mut keyboards = keyboards;
        for keyboard in &mut keyboards {
            keyboard.sort();
        }
        let mut keyboards_distinctive: Vec<Vec<u16>> = vec![];
        let mut i = keyboards.len();
        while i > 0 {
            if let Some(keyboard) = keyboards.pop() {
                let mut keyboard = keyboard;
                keyboard.retain(|key| {
                    let mut is_uniq = false;
                    if ignore_notes.binary_search(key).is_err() {
                        for keyboard_uniq in keyboards_distinctive.iter() {
                            is_uniq = keyboard_uniq.binary_search(key).is_err();
                            if is_uniq {
                                break;
                            }
                        }
                        if !is_uniq {
                            let mut j: usize = 0;
                            while j < keyboards.len() && !is_uniq {
                                is_uniq = keyboards[j].binary_search(key).is_err();
                                j += 1;
                            }
                            if !is_uniq {
                                // key  eksisteres på alle av resten keyboards
                                // slik at den blir lagt til ignore_notes; HVIS OG BARE HVIS "key" eksisteres på alle av resten keyboards
                                let insert_at = ignore_notes.binary_search(&key);
                                if let Err(indx) = insert_at {
                                    ignore_notes.insert(indx, *key);
                                }
                            }
                        }
                    }
                    is_uniq // eqv to keyboard.remove_item(key)
                });
                i -= 1;
                keyboards_distinctive.push(keyboard);
            }
        }
        keyboards_distinctive.sort_by(|a, b| b.len().cmp(&a.len()));
        // println!(
        //     "keyboards_distinctive{:#?}\n ignore_notes{:#?}",
        //     keyboards_distinctive, ignore_notes
        // );
        (keyboards_distinctive, ignore_notes)
        // let mut uniq_keyboards: Vec<Vec<u16>> = vec![]; // kan innholder vec med lengde == 0, hvis vi vil ha uniq_keyboards[i] == keyboards[i]
        // for _ in 0..keyboards.len() {
        //     let v: Vec<u16> = vec![];
        //     uniq_keyboards.push(v);
        // }
        // for (i, keyboard) in keyboards.iter().enumerate() {
        //     for elm in keyboard {
        //         let mut ind: usize = 0;
        //         let mut is_uniq = false;
        //         while ind < keyboards.len() && !is_uniq {
        //             if ind == i {
        //                 ind += 1;
        //                 continue;
        //             }
        //             // is_uniq = is_uniq || keyboards[ind].binary_search(elm).is_err()
        //             is_uniq = keyboards[ind].binary_search(elm).is_err();
        //             if is_uniq {
        //                 uniq_keyboards[i].push(*elm);
        //             }
        //             ind += 1;
        //         }
        //         if !is_uniq {
        //             ignore_notes.push(*elm);
        //         }
        //     }
        // }
        // println!("uniq_keyboards{:#?}\n ignore_notes{:#?}",uniq_keyboards, ignore_notes);
        // (uniq_keyboards, ignore_notes)
    }
    println!("nr_of_switch {}", nr_of_used_keyboards - 1);
    nr_of_used_keyboards - 1
}
