mod builtin_words;
use builtin_words::ACCEPTABLE;
use builtin_words::FINAL;

use std::{cmp::min};

use text_io::read;

mod calculate_info_entropy;
use calculate_info_entropy::cie;

mod check_word;
use check_word::check_word;

use indicatif::ProgressBar;


fn main() {
    // 循环进行
    
    let mut acceptable : Vec<String> = Vec::new();
    for word in ACCEPTABLE {
        acceptable.push(word.to_string());
    }

    println!("What do you want to do? S(olver) B(est) A(verage)");
    let intention : char = read!(); 


    if intention == 'S' {
        loop {
            // 开始一局游戏
            println!("Start? [Y/N]");
            let is_continue : char = read!();
            if is_continue == 'N' {
                println!("Over!");
                break;
            }
            let mut word_state : Vec<(String, String)> = Vec::new();
            
            for i in 0..5 {
                let mut possible_words : Vec<String> = Vec::new();
                println!("Please input the word and its state.");
                let word : String = read!();
                let state : String = read!();
                word_state.push((word, state));
    
    
                for word in ACCEPTABLE {
                    let mut is_qulified = true;
                    for i in 0..word_state.len() {
                        let mut guess_chars : Vec<char> = Vec::new();
                        for i in word.trim().chars() {
                            guess_chars.push(i);
                        } 
    
                        let mut current_chars : Vec<char> = Vec::new();
                        let mut current_color : Vec<char> = Vec::new();
    
                        for i in word_state[i].0.chars() {
                            current_chars.push(i);
                        }
    
                        for i in word_state[i].1.chars() {
                            current_color.push(i);
                        }
    
                        for j in 0..5 {
                            if current_color[j] == 'G' {
                                // 判断绿色是否合格
                                if guess_chars[j] != current_chars[j] {
                                    is_qulified = false;
                                    break;
                                } else {
                                    current_chars[j] = '0';
                                    guess_chars[j] = '0';
                                    
                                    continue;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("green");
                        }
                        for j in 0..5 {
                            if current_color[j] == 'Y' {
                                // 判断黄色是否合格
                                let mut is_yellow_qualified = false;
    
    
                                // 待测词与该黄色字母对应位置不是该黄色字母
                                if current_chars[j] == guess_chars[j] {
                                    is_qulified = false;
                                    break;
                                }
    
                                // 去除绿色后该黄色字母仍包含在待检测词中
                                for k in 0..5 {
                                    if current_chars[j] == guess_chars[k] {
                                        guess_chars[k] = '0';
                                        current_chars[j] = '0';
                                        is_yellow_qualified = true;
                                        break;
                                    }
                                }
                                
                                if !is_yellow_qualified {
                                    is_qulified = false;
                                    break;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("yellow");
                        }
                        for j in 0..5 {
                            //let mut is_red_qualified = true;
                            if current_color[j] == 'R' {
                                // 判断红色是否合格
                                if guess_chars.contains(&current_chars[j]) {
                                    is_qulified = false;
                                    break;
                                }
                            }
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("red");
                        }
                    }
                    if is_qulified {
                        possible_words.push(word.to_string().clone());
                    }
                }
    
                if possible_words.len() == 1 {
                    println!("Answer : {}", possible_words[0]);
                    break;
                }
                // println!("{:?}", possible_words);
                let word_entropy = cie(&acceptable, &possible_words);
                
                for i in 0..min(5, word_entropy.len()) {
                    println!("{}  {}", word_entropy[i].0, word_entropy[i].1);
                }
                if i == 4 {
                    println!("Failed!");
                    println!("The possible word : {:?}", possible_words);
                }
            }
        }
    } else if intention == 'B' {
        // 计算最优开局词 请注释上面的loop循环
        /*
        tares  6.1940525443754435
        lares  6.149918742453124
        rales  6.114343099454227
        rates  6.096242642514605
        teras  6.076619177276175
        */
        

        let outcome = cie(&acceptable, &acceptable);

        for i in 0..5 {
            println!("{}  {}", outcome[i].0, outcome[i].1);
        }
        
    } else if intention == 'A' {
        // 计算平均猜测次数 请注释上面的loop循环
        /* 
        开局词固定为本算法计算出的初始信息熵最大的词 tares
        计算出的平均猜测次数为4.149892008639309
        */
        

        let pb = ProgressBar::new(FINAL.len() as u64);
        let mut total_times = 0;

        for i in FINAL{
            let answer_word = i.clone().to_string();
            let mut word_state : Vec<(String, String)> = Vec::new();
            total_times += 1;
            word_state.push((String::from("tares"), check_word(&answer_word, &String::from("tares")).0));
            if &answer_word[..] == "tares" {
                continue;
            }
            for _j in 0..5 {
                total_times += 1;
                let mut possible_words : Vec<String> = Vec::new();
                for word in &acceptable {
                    let mut is_qulified = true;
                    for k in 0..word_state.len() {
                        let mut guess_chars : Vec<char> = Vec::new();
                        for l in word.trim().chars() {
                            guess_chars.push(l);
                        } 

                        let mut current_chars : Vec<char> = Vec::new();
                        let mut current_color : Vec<char> = Vec::new();

                        for l in word_state[k].0.chars() {
                            current_chars.push(l);
                        }

                        for l in word_state[k].1.chars() {
                            current_color.push(l);
                        }

                        for l in 0..5 {
                            if current_color[l] == 'G' {
                                // 判断绿色是否合格
                                if guess_chars[l] != current_chars[l] {
                                    is_qulified = false;
                                    break;
                                } else {
                                    current_chars[l] = '0';
                                    guess_chars[l] = '0';
                                    
                                    continue;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("green");
                        }
                        for l in 0..5 {
                            if current_color[l] == 'Y' {
                                // 判断黄色是否合格
                                let mut is_yellow_qualified = false;


                                // 待测词与该黄色字母对应位置不是该黄色字母
                                if current_chars[l] == guess_chars[l] {
                                    is_qulified = false;
                                    break;
                                }

                                // 去除绿色后该黄色字母仍包含在待检测词中
                                for m in 0..5 {
                                    if current_chars[l] == guess_chars[m] {
                                        guess_chars[m] = '0';
                                        current_chars[l] = '0';
                                        is_yellow_qualified = true;
                                        break;
                                    }
                                }
                                
                                if !is_yellow_qualified {
                                    is_qulified = false;
                                    break;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("yellow");
                        }
                        for l in 0..5 {
                            //let mut is_red_qualified = true;
                            if current_color[l] == 'R' {
                                // 判断红色是否合格
                                if guess_chars.contains(&current_chars[l]) {
                                    is_qulified = false;
                                    break;
                                }
                            }
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("red");
                        }
                    }
                    if is_qulified {
                        possible_words.push(word.to_string().clone());
                    }
                }
                // 胜利!
                if possible_words.len() == 1 {
                    break;
                }
                
                let guess_word = cie(&acceptable, &possible_words)[0].0.clone();
                let state = check_word(&answer_word, &guess_word).0;
                word_state.push((guess_word, state));
            }
            pb.inc(1);
            //println!("{}", total_times);
        } 
        
        let avearge_attempts = (total_times as f64) / (FINAL.len() as f64 );
        println!("{}", avearge_attempts);
    }
}