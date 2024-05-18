const S:&str = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
struct Intpre {
    cursor: usize,
    data: Vec<u8>,
    data_ptr: usize,
    code: Vec<u8>,
}
impl Intpre {
    fn new(s_var: &'static str) -> Self {
        Intpre {
            cursor: 0,
            data: vec![],
            code: s_var
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|i| i as u8)
                .collect(),
            data_ptr: 0,
        }
    }
    fn process(&mut self) {
        let len = self.code.len();
        while self.cursor != len {
            match self.code[self.cursor] {
                x if x as char == '>' => {
                    self.data_ptr += 1;
                    if self.data.get(self.data_ptr).is_none() {
                        self.data.push(0);
                    }
                }
                x if x as char == '<' => {
                    self.data_ptr -= 1;
                    if self.data.get(self.data_ptr).is_none() {
                        self.data.push(0);
                    }
                }
                x if x as char == '+' => {
                    if self.data.get(self.data_ptr).is_some() {
                        self.data[self.data_ptr] += 1;
                    } else {
                        self.data.push(0);
                        self.data[self.data_ptr] += 1;
                    }
                }
                x if x as char == '-' => {
                    if self.data.get(self.data_ptr).is_some() {
                        self.data[self.data_ptr] -= 1
                    } else {
                        self.data.push(0);
                        self.data[self.data_ptr] -= 1
                    }
                }
                x if x as char == '[' => {
                    if self.data[self.data_ptr] == 0 {
                        for (ind, d) in self.code.iter().enumerate() {
                            if ind > self.cursor && *d as char == ']' {
                                self.cursor = ind;
                                break;
                            }
                        }
                    }
                }
                x if x as char == ']' => {
                    if self.data[self.data_ptr] != 0 {
                        for (ind, d) in self.code.iter().enumerate() {
                            if ind < self.cursor && *d as char == '[' {
                                self.cursor = ind;
                                break;
                            }
                        }
                    }
                }
                x if x as char == '.' => print!("{}", self.data[self.data_ptr] as char),
                _ => unimplemented!(),
            }
            self.cursor += 1;
        }
    }
}
fn main() {
    let mut ipr = Intpre::new(S);
    ipr.process();
}
