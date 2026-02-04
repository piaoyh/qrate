use std::io;

use qrate::{ SQLiteDB, Excel, QBDB };
use qrate::{ Header, Question, QBank, Choices };
use qrate::Generator;

fn main()
{
    let mut db = SQLiteDB::open("./Information_Security".to_string()).unwrap();
    let qbank;
    if let Some(qb) = db.read_qbank()
    {
        let last = qb.get_questions().len() as u16;
        let mut generator = Generator::new_one_set(&qb, 1, last).unwrap();
        qbank = generator.get_shuffled_qbank(0).unwrap();
    }
    else
    {
        println!("error = {}", e);
        return;
    }

    exam(&(qbank.1));
}

pub fn exam(qbank: &QBank)
{
    let categories = qbank.get_header().get_categories();
    let mut number: u8 = 1;
    let mut score: i8 = 0;
    let note = qbank.get_header().get_notice();
    println!("{}", note);
    for question in qbank.get_questions()
    {
        let id = question.get_id();
        let cat = categories[question.get_category() as usize - 1];
        print!("{}. [{}]   ", id, cat);
        println!("{}", question.get_question());
        let mut answers = 0;
        let last = question.get_choices().len();
        for choice_number in 1..=last
        {
            let cho = question.get_choice(choice_number).unwrap();
            println!("\t{}) {}", choice_number, cho.0);
            if cho.1
                { answers += 1; }
        }
        println!("");

        if answers == 1
            { println!("Enter {} answer bellow.", answers); }
        else
            { println!("Enter {} answers bellow", answers); }

        let mut entered = Vec::new();
        for i in 1..answers
        {
            match i % 10
            {
                1 => { println!("Enter {}-st answer.", i); },
                2 => { println!("Enter {}-nd answer.", i); },
                3 => { println!("Enter {}-rd answer.", i); },
                _ => { println!("Enter {}-th answer.", i); }
            }
            let mut text = String::new();
            while let Err(e) = io::stdin().read_line(&mut text)
                { println!("Not proper input! Enter proper answer!"); }
            text = text.trim().to_string();
            let ans = text.parse::<u8>().unwrap();
            entered.push((ans, false));
        }

        let mut found = false;
        let mut repetition = true;
        for i in 1..=question.get_choices().len()
        {
            for j in 1..=entered.len()
            {
                if question.get_choice(i).1 && ans == i as u8
                {
                    found = true;
                    break;
                        }
                        else if !question.get_choice(i).1 && ans == i as u8
                        {
                            found = false;
                            break;
                        }
                    }
                }

            }
                2 => {
                    let mut found = false;
                    let mut going = true;
                    while going
                    {
                        for i in 1..=question.get_choices().len()
                        {
                            let txt = text.split_whitespace().into_iter();
                            let ans = txt.next().or(Some("0")).unwrap();
                            let ans = ans.parse::<u8>().unwrap();
                            if question.get_choice(i).1 && ans == i as u8
                            {
                                found = true;
                                break;
                            }
                            else if !question.get_choice(i).1 && ans == i as u8
                            {
                                found = false;
                                break;
                            }
                        }
                    }
                }
            }
            repetition = false;
        }
        
        let mut ans1 = match txt1.parse::<u8>()
        {
            Err(_) => { println!("{} is not the answer.", txt); 0u8},
            Ok(digit) => digit as u8,
        };
        ans1 =  if ans1 > last as u8    { println!("{} cannot be answer.", ans1); 0 }
                else if ans1 == 0       { 0 }
                else                { selected.choice[(ans1-1) as usize] };
        if qb.has_multianswers()
        {
            if txt.len() < 3
            {
                println!("You should had chosen two answers.");
                if ans1 == answer1 || ans1 == answer2
                {
                    score += 0;
                }
                else
                {
                    score += -3;
                };
                /////
                let mut a = 1u8;
                let mut b = 1u8;
                let mut c = a;
                for i in selected.choice
                {
                    if i == answer1 || i == answer2
                    {
                        if a == b
                        {
                            c = a;
                            b += 1;
                            continue;
                        }
                        println!("The answers are {} and {}.", c, b);
                        break;
                    }
                    a += 1;
                    b += 1;
                }
                /////
                println!("Your score is {} points at the moment!", score);
                println!("\n-------------------------------------\n");
                /////
                continue;
            }
            let txt2 = &txt[2..].trim();
            let mut ans2 = match txt2.parse::<u8>()
            {
                Err(_) => { println!("{} cannot be answer.", txt); 0},
                Ok(digit) => digit,
            };
            ans2 =  if ans2 > 4         { println!("{} cannot be answer.", ans2); 0 }
                    else if ans2 == 0   { 0 }
                    else                { selected.choice[(ans2-1) as usize] };
            if ans1 > ans2
            {
                (ans1, ans2) = (ans2, ans1);
            }
            let ans1 = ans1;
            let ans2 = ans2;
            if ans1 == ans2
            {
                println!("You should had chosen two answers.");
                if ans1 == answer1 || ans1 == answer2
                {
                    score += 0;
                }
                else
                {
                    score += -3;
                };
            }
            else if ans1 == answer1 && ans2 == answer2
            {
                println!("Both answers are Correct!");
                score += 3;
            }
            else if ans1 != answer1 && ans2 != answer2
            {
                println!("Both answers are Incorrect!");
                score -= 3;
            }
            else
            {
                println!("One answer is Correct but the other answer is Incorrect!");
                score += 0;
            }
            /////
            let mut a = 1u8;
            let mut b = 1u8;
            let mut c = a;
            for i in selected.choice
            {
                if i == answer1 || i == answer2
                {
                    if a == b
                    {
                            c = a;
                            b += 1;
                            continue;
                        }
                        println!("The answers are {} and {}.", c, b);
                        break;
                    }
                    a += 1;
                    b += 1;
                }
                /////
            }
            else
            {
                let ans1 = ans1;
                if ans1 == answer1
                {
                    println!("Correct!");
                    score += 3;
                }
                else
                {
                    println!("Incorrect!");
                    score -= 1;
                }
                let mut a = 1u8;
                for i in selected.choice
                {
                    if i == answer1
                    {
                        println!("The answer is {}.", a);
                        break;
                    }
                    a += 1;
                }
            }
            /////
            println!("Your score is {} points at the moment!", score);
            println!("\n-------------------------------------\n");
            /////
        }
        println!("You've got {} points!", score);
    }
