use std::ops::{Generator, GeneratorState};



pub fn gebn() {


    let mut g =  || {

        yield 1;
        yield 2;


        return "foo"
    };

    match unsafe { g.resume() } {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }

    if let GeneratorState::Yielded(y) = unsafe { g.resume() } {
        println!("{:#?}", y);
    };

    let x = unsafe { g.resume() };
    
}