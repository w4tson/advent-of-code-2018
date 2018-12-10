#[derive(Debug, Clone)]
pub struct Worker {
    pub current: Option<char>,
    pub time_to_complete: usize,
    pub step: u8
}

impl Worker {
    
    pub fn new(step_size: usize) -> Worker {
        Worker { current: None, time_to_complete: 0, step: step_size as u8 }
    }

    pub fn is_free(&self) -> bool {
        self.current.is_none() || self.time_to_complete == 0
    }

    pub fn accept(&mut self, ch : char) {
        let wait = (ch as u8 - 'A' as u8 + self.step + 1) as usize;
        self.time_to_complete = wait;
        self.current = Some(ch);
    }

    //Ugly use of Option
    pub fn do_work(&mut self) -> Option<char> {
        if self.current.is_some() {
            self.time_to_complete -= 1;
            match self.time_to_complete {
                0 => {
                    let ch = self.current
                        .unwrap_or_else(|| panic!("Worker without current char"));
                    self.current = None;
                    Some(ch)
                }
                _ => None
            }
        } else { None }
    }
}