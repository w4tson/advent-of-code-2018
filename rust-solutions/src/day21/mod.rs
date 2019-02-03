use crate::day19::machine::Machine;
use crate::utils::file::read_puzzle_input;
use crate::day19::machine::Instruction;
use std::thread;
use std::time;


#[test]
fn part1() {
    let input = read_puzzle_input("day21");
    //seti 5 0 1

    let instructions : Vec<Instruction> =
        input.lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .expect("should be good");
    println!("starting... ");

    for x in (i64::min_value()..0).step_by(100000) {
        println!("{:6}: ", x);
        let mut m = Machine::new( &instructions, 2, x as i128);
        m.run();
    }
}

#[test]
fn part2() {
    let input = read_puzzle_input("day21");

    let instructions : Vec<Instruction> =
        input.lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .expect("should be good");

    let mut machine = Machine::new( &instructions, 2, 0);
    machine.run();
}


#[test]
fn asdf() {

    let mut a : i128 = 12213578;
    let mut b : i128 = 0;
    let mut c : i128 = 0;
    let mut d : i128 = 0;
    let mut e : i128 = 0;
    let mut f : i128 = 0;

    let mut loop_count = 0;


    //f < 256
    //
    
    loop {
        f = e | 65536;
        e = 1765573;


        loop {

            loop_count += 1;

            b = f & 255;
            e += b;

            e = (e & 16777215) * 65899;
            e = e & 16777215;

//                println!("{} {} {} {} {} {} ",a,b,c,d,e,f);
            thread::sleep(time::Duration::from_millis(3));

            if f < 256 { break; }

            b = f / 256;
            f = b;
        }

        println!("***          {}    {}", e, loop_count );
        loop_count =0;

        if a == e {

        }
    }
    println!("a {}", a);
}

