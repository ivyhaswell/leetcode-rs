// 状态表格
//               ' '	   +/-      number     other
// (0)start	    start     signed   in_number    end
// (1)signed     end	   end     in_number    end
// (2)in_number  end	   end     in_number    end
// (3)end        end	   end       end        end
//
struct Automoton {
    sign: i64,
    num: i64,
    state: usize,
    table: Vec<Vec<usize>>,
}

impl Automoton {
    pub fn new() -> Self {
        Self {
            sign: 1,
            num: 0,
            state: 0,
            table: vec![
                vec![0, 1, 2, 3],
                vec![3, 3, 2, 3],
                vec![3, 3, 2, 3],
                vec![3, 3, 3, 3],
            ],
        }
    }

    pub fn update(self: &mut Self, c: &u8) {
        let char_type_index = match c {
            b' ' => 0,
            b'+' | b'-' => 1,
            b'0'..=b'9' => 2,
            _ => 3,
        };
        self.state = self.table[self.state][char_type_index];

        match self.state {
            1 => {
                if c == &b'-' {
                    self.sign = -1
                }
            }
            2 => {
                self.num = self.num * 10 + (c - b'0') as i64;
                if self.num > i32::MAX as i64 + 1 {
                    self.num = i32::MAX as i64 + 1
                }
            }
            _ => {}
        }
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut automoton = Automoton::new();
        for i in s.as_bytes() {
            automoton.update(i);
            if automoton.state == 3 {
                break;
            }
        }

        let ans = automoton.sign * automoton.num;
        if ans > i32::MAX as i64 {
            i32::MAX
        } else if ans < i32::MIN as i64 {
            i32::MIN
        } else {
            ans as i32
        }
    }
}

pub struct Solution {}
