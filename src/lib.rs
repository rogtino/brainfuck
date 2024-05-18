#![allow(dead_code)]
use std::{
    io::Write,
    io::{self, Read},
};

struct Intpre<T> {
    cursor: usize,
    data: Vec<u8>,
    data_ptr: usize,
    code: Vec<u8>,
    buffer: T,
}
impl<T> Intpre<T>
where
    T: Default + Write,
{
    fn new(s: &str, buffer: T) -> Self {
        Intpre {
            code: s
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|i| i as u8)
                .collect(),
            buffer,
            cursor: 0,
            data: vec![],
            data_ptr: 0,
        }
    }
    fn process(mut self) -> T {
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
                    // UB if overflowing
                    self.data_ptr -= 1;
                }
                x if x as char == '+' => {
                    if self.data.get(self.data_ptr).is_none() {
                        self.data.push(0);
                    }
                    self.data[self.data_ptr] = self.data[self.data_ptr].saturating_add(1);
                }
                x if x as char == '-' => {
                    if self.data.get(self.data_ptr).is_none() {
                        self.data.push(0);
                    }
                    self.data[self.data_ptr] = self.data[self.data_ptr].saturating_sub(1);
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
                x if x as char == '.' => {
                    write!(self.buffer, "{}", self.data[self.data_ptr] as char)
                        .expect("write error");
                }

                x if x as char == ',' => {
                    let mut c: [u8; 1] = [0];
                    io::stdin().read_exact(&mut c).expect("read error");
                    self.data[self.data_ptr] = c[0];
                }
                _ => unimplemented!(),
            }
            self.cursor += 1;
        }
        self.buffer
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_hello_world() {
        const S:&str = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let v: Vec<u8> = Vec::new();
        let ipr = Intpre::new(S, v);
        let res = ipr.process();
        assert_eq!(std::str::from_utf8(&res[..]).unwrap(), "Hello World!\n");
    }
}
