fn main() {}

// 2023.02.07 백준 Greedy 4796문 제출 정답
mod q_4796 {
    use std::{
        cmp::min,
        io::{stdin, Read},
    };

    fn q_4796() {
        let mut cnt = 0;
        loop {
            cnt += 1;

            let mut line = String::new();
            stdin().read_line(&mut line).expect("wrong io");

            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            let l = numbers[0 as usize];
            let p = numbers[1 as usize];
            let v = numbers[2 as usize];

            if l == 0 && p == 0 && v == 0 {
                break;
            }

            let res = (v / p) * l + min(v % p, l);

            println!("Case {cnt}: {res}");
        }
    }
}

// 2023.02.08 백준 Greedy 1439문 제출 정답
mod q_1439 {
    use std::io::stdin;

    fn q_1439() {
        let mut s: String = String::new();

        stdin().read_line(&mut s).expect("Errrr");

        let mut cnt = 0;

        let char_vec: Vec<char> = s.chars().collect();

        let mut v_size = char_vec.len() - 1;

        loop {
            if v_size == 0 {
                break;
            }

            v_size -= 1;

            if char_vec[char_vec.len() - 2] == '1' {
                if char_vec[v_size] == '0' {
                    loop {
                        if v_size == 0 {
                            break;
                        }

                        if char_vec[v_size] == '1' {
                            break;
                        }
                        v_size -= 1;
                    }
                    cnt += 1;
                }
            } else if char_vec[char_vec.len() - 2] == '0' && char_vec[v_size] == '1' {
                loop {
                    if v_size == 0 {
                        break;
                    }

                    if char_vec[v_size] == '0' {
                        break;
                    }
                    v_size -= 1;
                }
                cnt += 1;
            }
        }

        println!("{cnt}");
    }
}

// 2023.02.09 백준 Greedy 1789문 제출 정답
mod q_1789 {

    use std::io::stdin;

    fn q_1789() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<i128> = line
            .split_whitespace()
            .map(|num| num.parse::<i128>().unwrap())
            .collect();

        let mut s = numbers[0_usize];

        let mut n: i128 = 0;
        let mut loo: i128 = 1;

        loop {
            if s - loo < 0 {
                break;
            } else {
                s -= loo;

                loo += 1;
                n += 1;
            }
        }
        println!("{n}");
    }
}

// 2023.02.09 백준 Greedy 1026문 제출 정답
mod q_1026 {
    use std::cmp::Reverse;
    use std::io::stdin;

    fn q_1026() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let N = numbers[0_usize];

        let mut a: Vec<usize> = vec![];

        let mut b: Vec<usize> = vec![];

        let mut _result: usize = 0;

        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        a = numbers.clone();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        b = numbers.clone();

        a.sort_by_key(|w| Reverse(*w));
        b.sort();

        for i in 0..N {
            _result += a[i] * b[i];
        }

        println!("{_result}");
    }
}

// 2023.02.10 백준 Greedy 11047문 제출 정답
mod q_11047 {

    use std::io::stdin;

    fn q_11047() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let n = numbers[0_usize];
        let mut k = numbers[1_usize];
        let mut val: Vec<usize> = vec![];

        let mut loo: usize = n - 1;

        let mut cnt: usize = 0;

        for _i in 0..n {
            let mut line = String::new();
            stdin().read_line(&mut line).expect("wrong io");

            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            val.push(numbers[0_usize]);
        }

        loop {
            if k == 0 {
                break;
            } else if k >= val[loo] {
                cnt += k / val[loo];

                k -= val[loo] * (k / val[loo]);
            } else {
                loo -= 1;
            }
        }

        println!("{cnt}");
    }
}

// 2023.02.11 백준 Greedy 11399문 제출 정답
mod q_11399 {
    use std::io::stdin;

    fn q_11399() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let n = numbers[0_usize];

        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mut a_min: Vec<usize> = numbers;

        a_min.sort();

        let mut _result: usize = 0;

        let mut temp2: usize = 0;

        for _i in 0..n {
            let mut temp: usize = a_min[_i];

            if _i == 0 {
                temp2 = temp;
                _result += temp2;
            } else if _i != 0 {
                temp2 += temp;
                _result += temp2;
            }
        }

        println!("{_result}");
    }
}

// 2023.02.15 백준 Greedy 16953문 제출 정답
mod q_16953 {
    use std::io::stdin;

    fn q_16953() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let a = numbers[0_usize];
        let mut b = numbers[1_usize];

        let mut cnt: usize = 1;

        loop {
            if a == b {
                println!("{cnt}");
                break;
            } else if a != b {
                if b % 10 == 1 {
                    b /= 10;
                } else if b % 2 == 0 && b != 0 {
                    b /= 2;
                } else {
                    println!("-1");
                    break;
                }
            }
            cnt += 1;
        }
    }
}

// 2023.02.16 백준 Greedy 15903문 제출 정답
mod q_15903 {
    use std::io::stdin;

    fn q_15903() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let n: usize = numbers[0_usize];
        let m: usize = numbers[1_usize];

        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let mut card: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        for _i in 0..m {
            card.sort();

            let card_sum = card[0] + card[1];

            card[0] = card_sum;

            card[1] = card_sum;
        }

        println!("{}", card.iter().sum::<usize>());
    }
}

// 2023.02.17 백준 backtrack 10974문 제출 정답
// 시간 초과 --> 출력 방식과 parameter 관련 고려해 볼 것.
mod q_10974 {
    use std::io::stdin;
    use std::io::{stdout, Write};

    fn q_10974() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        let n: usize = numbers[0_usize].try_into().unwrap();

        let ans: Vec<usize> = vec![0; 9];

        let visited: Vec<bool> = vec![false; 9];

        permutation(0, ans, n, visited);
    }

    fn permutation(cnt: usize, mut ans: Vec<usize>, n: usize, mut visited: Vec<bool>) {
        if cnt == n {
            let stdout = stdout();
            let mut lock = stdout.lock();

            for i in 0..cnt {
                write!(lock, "{} ", ans[i].clone()).unwrap();
            }
            writeln!(lock, "").unwrap();
            return;
        } else if cnt != n {
            for i in 1..n + 1 {
                if visited[i] == true {
                    continue;
                } else {
                    visited[i] = true;
                    ans[cnt] = i;
                    permutation(cnt + 1, ans.clone(), n, visited.clone());
                    visited[i] = false;
                }
            }
        }
    }
}
