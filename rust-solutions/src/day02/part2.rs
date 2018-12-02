

pub fn solve_part2(input : &str) -> String {
    let lines : Vec<&str> = input.lines().collect();
    
    let _dupes : Vec<&&str> = lines.iter()
        .filter(|&line| get_dist(line, &lines) == 1)
        .collect();
    
    assert_eq!(_dupes.len(), 2);
    
    letters_minus_diff(*_dupes[0], *_dupes[1])
}

fn letters_minus_diff(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(first, second)| first == second )
        .map(|(c, _)| c)
        .collect()
}

fn get_dist(line: &str, lines: &Vec<&str>) -> i32 {
    let min = lines.iter()
        .map(|s| edit_dist(s, line))
        .filter(|distance| *distance !=0 ) //itself will be in there
        .min()
        .unwrap();
    
    min as i32
}

fn edit_dist(s1: &str, s2: &str) -> i32 {
    s1.chars()
        .zip(s2.chars())
        .filter(|(first, second)| first != second )
        .count() as i32
}