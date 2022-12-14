use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // 这种大括号的姿势需要实现fmt::Display接口
    println!("student {}", Student(3));
    // 这种 :? 的姿势需要实现Debug接口
    println!("student {:?}", Student(3));
    println!("parse_bool_expr {}", parse_bool_expr(String::from("f")));
    println!("interpret {}", interpret(String::from("G()(al)")));
    println!("result is {:?}", ambiguous_coordinates(String::from("(123)")));
}

#[derive(Debug)]
struct Student(i32);

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}

pub fn parse_bool_expr(expression: String) -> bool {
    if expression.eq("t") {
        return true;
    }
    if expression.eq("f") {
        return false;
    }
    let mut s = vec![];
    for c in expression.as_bytes() {
        if *c != b',' && *c != b')' {
            s.push(*c);
        } else if *c == b')' {
            let (mut true_count, mut false_count) = (0, 0);
            while !s.is_empty() {
                let temp = s.pop().unwrap();
                if temp == b'(' {
                    break;
                } else if temp == b't' {
                    true_count += 1;
                } else if temp == b'f' {
                    false_count += 1;
                }
            }
            let operator = s.pop().unwrap();
            if operator == b'!' {
                if false_count == 1 {
                    s.push(b't');
                } else {
                    s.push(b'f');
                }
            } else if operator == b'&' {
                if false_count == 0 {
                    s.push(b't');
                } else {
                    s.push(b'f');
                }
            } else if operator == b'|' {
                if true_count > 0 {
                    s.push(b't');
                } else {
                    s.push(b'f');
                }
            }
        }
    }
    let result = s.pop().unwrap();
    if result == b't' { true } else { false }
}

pub fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}

pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    let mut ret = vec![];

    for i in 2..s.len() - 1 {
        for x in possibilites(s.get(1..i).unwrap()) {
            for y in possibilites(s.get(i..s.len() - 1).unwrap()) {
                ret.push(format!("({}, {})", x, y));
            }
        }
    }

    ret
}

pub fn possibilites(s: &str) -> Vec<String> {
    let mut ret = vec![];

    if s == "0" || !s.starts_with('0') {
        ret.push(s.to_string());
    }
    for i in 1..s.len() {
        let x = format!("{}.{}", s.get(..i).unwrap(), s.get(i..).unwrap());
        if validity(&x) {
            ret.push(x);
        }
    }

    ret
}

pub fn validity(s: &str) -> bool {
    let vec = s.split('.').collect::<Vec<_>>();

    (vec[0] == "0" || !vec[0].starts_with('0')) && !vec[1].ends_with('0')
}

pub fn halves_are_alike(s: String) -> bool {
    let (s_arr, ch_arr) = (s.chars().collect::<Vec<_>>(), vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
    s_arr[..s.len() / 2].iter().filter(|ch| ch_arr.contains(ch)).count() == s_arr[s.len() / 2..].iter().filter(|ch| ch_arr.contains(ch)).count()
}

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut mmp = HashMap::new();
    let order_chars = order.chars().collect::<Vec<char>>();
    for c in &order_chars {
        mmp.entry(*c).or_insert(Vec::new());
    }
    for c in s.chars().collect::<Vec<char>>() {
        mmp.entry(c).or_insert(Vec::new()).push(c.to_string());
    }
    let mut ret = String::new();
    for c in order_chars {
        let tmp = mmp.get(&c);
        if tmp != None {
            ret.push_str(&tmp.unwrap().join(""));
            mmp.remove(&c);
        }
    }
    for (_, v) in mmp {
        ret.push_str(&v.join(""));
    }
    ret
}

pub fn split_array_same_average(nums: Vec<i32>) -> bool {
    let (sum, n) = (nums.iter().sum::<i32>(), nums.len());
    let mut dp = vec![0; sum as usize + 1];
    dp[0] = 1;
    for num in nums {
        for s in (num..=sum).rev() {
            if dp[(s - num) as usize] > 0 { dp[s as usize] |= (dp[(s - num) as usize] << 1); }
        }
    }
    for i in 1..n {
        if sum * i as i32 % n as i32 != 0 { continue; }
        let s = sum * i as i32 / n as i32;
        if dp[s as usize] > 0 && (dp[s as usize] & (1 << i as i32)) > 0 { return true; }
    }
    false
}

pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    let mut maximum = 0;
    for i in 2..nums.len() {
        maximum = maximum.max(nums[i - 2]);
        if maximum > nums[i] { return false; }
    }
    true
}

pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let words = words
        .into_iter()
        .map(|v| v.into_bytes())
        .collect::<Vec<_>>();
    let mut heads = vec![vec![]; 26];
    for word in &words {
        heads[(word[0] - b'a') as usize].push(&word[1..]);
    }
    let mut res = 0;
    for ch in s.bytes() {
        let tails = std::mem::take(&mut heads[(ch - b'a') as usize]);
        for tail in tails {
            if tail.is_empty() {
                res += 1;
            } else {
                heads[(tail[0] - b'a') as usize].push(&tail[1..]);
            }
        }
    }
    return res;
}

pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let n = nums.len();
    let mut ans = 0;
    let mut k = 1;

    for i in 0..n {
        ans = (ans + nums[i] as i64 * k - nums[n - i - 1] as i64 * k) % 1_000_000_007;
        k = k * 2 % 1_000_000_007;
    }

    ans as i32
}

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut counts = vec![0; 46];
    let mut max = 0;
    for mut i in low_limit..=high_limit {
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }
        counts[sum as usize] += 1;
        max = max.max(counts[sum as usize]);
    }
    return max;
}