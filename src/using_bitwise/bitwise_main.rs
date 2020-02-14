use std::io;

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

fn read_line_as_1024bits_without_fst_byte() -> [u32; 32] {
    let stdin = io::stdin();
    let mut val = String::new();
    stdin.read_line(&mut val).expect("Failed to read line");

    //     for (i, &item) in val.trim().as_bytes().iter().enumerate() {
    //         if item == b' ' {
    //             let val = &val[i+1..];
    //               break;
    //         }
    //    }

    let mut res: [u32; 32] = [0; 32];

    for (i, s) in val.trim().split_whitespace().enumerate() {
        if i == 0 {
            continue;
        }
        let index_to_set: u32 = s.parse().unwrap();
        res = set_bit_in_array_of_1024bit(&mut res, index_to_set);
    }
    res
}

fn set_bit_in_array_of_1024bit(a: &mut [u32; 32], index_to_set: u32) -> [u32; 32] {
    let (pos_in_arr, nr_of_shifts) = calc_index(index_to_set as u32);
    a[pos_in_arr] |= 1 << nr_of_shifts;
    *a
}

fn calc_index(to_set: u32) -> (usize, usize) {
    // if nr_of_bits is 8
    //       n=0                 n=1
    // 0|1|2|3|4|5|6|7   8|9|10|11|12|13|14|15

    // to set bit 14:
    //     * n = 14 / nr_of_bits = 1
    //     * nr_of_shifts = (8-1) - (14 % 8) = 1  or   ((n+1) * nr_of_bits) - (14+1) = 1
    //     * a[n] & (1 << nr_of_shifts)

    // base = total bits in arr / (nr of elm in arr[u32;32]) = 32
    // base =       1024        /         32                 = 32
    let base = 32;

    let to_set = to_set as usize;
    let pos_in_arr: usize = to_set / base;
    // let nr_of_shifts = ((n+1) * base) - (index_to_set  +1);
    let nr_of_shifts = (base - 1) - (to_set % base);
    (pos_in_arr, nr_of_shifts)
}

fn is_indx_set(a: &[u32; 32], to_set: u32) -> bool {
    let (pos_in_arr, nr_of_shifts) = calc_index(to_set as u32);
    a[pos_in_arr] & 1 << nr_of_shifts != 0
}

fn intersection(a: &[u32; 32], b: &[u32; 32]) -> [u32; 32] {
    let mut res: [u32; 32] = [0; 32];

    for i in 0..32 {
        res[i] = a[i] & b[i];
    }

    res
}


fn calc(keyboards: &mut Vec<[u32; 32]>, tune: &Vec<u16>) -> u32 {
    let mut nr_of_switchs: u32 = 0;

    let mut tune_mapping: Vec<[u32; 32]> = vec![[0; 32];1001];
    let mut using: [u32; 32] = [0; 32];

    let mut fst_note:bool = true;

    for note in tune {
        let note = *note as usize;
        if tune_mapping[note] == [0; 32] {
            for (keyboard_index, keyboard) in keyboards.iter().enumerate() {
                if is_indx_set(keyboard, note as u32) {
                    tune_mapping[note] =
                        set_bit_in_array_of_1024bit(&mut tune_mapping[note], keyboard_index as u32);
                }
            }
            // println!(" tone {} init to {:?}", note, tune_mapping[note])
        }

        let available_keyboards_ind = tune_mapping[note];

        //first note in the tune
        if fst_note {
            using = available_keyboards_ind;
            fst_note = false;
        } else {
            using = intersection(&using, &available_keyboards_ind);

            if using == [0; 32] {
                nr_of_switchs += 1;
                using = available_keyboards_ind;
            }
        }
    }
    println!("{}", nr_of_switchs);

    nr_of_switchs
}



pub fn main() {
    let first_line = read_line_u16();
    let nr_of_keyboards: u16 = first_line[0];

    let mut keyboards: Vec<[u32; 32]> = vec![];

    for _ in 1..=nr_of_keyboards {
        let  v: [u32; 32] = read_line_as_1024bits_without_fst_byte();
        keyboards.push(v);
    }
    let tune: Vec<u16> = read_line_u16();

    // input

    calc(&mut keyboards, &tune);
}
