// use std::time::Instant;
mod using_bitwise;
mod using_hash_map;

fn main() {


    

    

    using_hash_map::map_main::main();

    using_bitwise::bitwise_main::main();

    // let mut avg = 0;
    // for _ in 0..100 {
    //     let now = Instant::now();

        
    //     let elapsed = now.elapsed().as_millis();
    //     println!("The time elapsed is: {} ms.", elapsed);

    //     avg += elapsed;

    // }

    // println!("avg {}" , avg/100);

}

