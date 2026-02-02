
// use std::io;
// use std::fs::File;
// use std::io::Write;
// use cryptocol::random::Slapdash as PRNG;

// use crate::{QBank, Student};

const NUMBER_CHOICES: usize = 4;
// const NUMBER_QUESTIONS: usize = 48; // 51;
pub const NUMBER_SELECTED_QUESTIONS: usize = 25;
// const SAVE_PAPER_SPACE: &str = "";  // "\t"


#[derive(Debug)]
pub struct ShuffledQuestion
{
    question: u16,    // 1-based
    choice: [u8; NUMBER_CHOICES],
}

impl ShuffledQuestion
{
    pub fn question(&self) -> u16       { self.question }
    pub fn choice(&self, num: u8) -> u8 { self.choice[(num - 1) as usize] }
}
/*
pub struct Exam
{
    qbs: Vec::<Question>,
    selected_qb: Vec::<ShuffledQuestion>,
}

impl Exam
{
    pub fn instantiate() -> Exam
    {
        let mut e = Exam
        {
            qbs: Vec::<OldQuestion>::new(),
            selected_qb: Vec::<ShuffledQuestion>::new(),    
        };
        e.init_bank();
        e.select_questions();
        return e;
    }

    pub fn exam(&mut self)
    {
        let mut number: u8 = 1;
        let mut score: i8 = 0;
        for selected in &self.selected_qb
        {
            let qb = &self.qbs[(selected.question()-1) as usize];
            println!("{}. {}", number, qb.question());
            for ch in 1..=4
            {
                println!("\t{}) {}", ch, qb.choice(selected.choice(ch)));
            }
            number += 1;
            println!("");
            let (mut answer1, mut answer2) = qb.answer();
            if (answer1 != 0) && (answer2 != 0) && (answer1 > answer2)
            {
                (answer1, answer2) = (answer2, answer1);
            }
            let answer1 = answer1;
            let answer2 = answer2;
            let mut txt = String::new();
            io::stdin().read_line(&mut txt).expect("Not proper input");
            txt = txt.trim().to_string();
            if txt.len() < 1
            {
                txt = " ".to_string();
            }
            let txt1 = &txt[..1];
            let mut ans1 = match txt1.parse::<u8>()
            {
                Err(_) => { println!("{} cannot be answer.", txt); 0u8},
                Ok(digit) => digit as u8,
            };
            ans1 =  if ans1 > 4         { println!("{} cannot be answer.", ans1); 0 }
                    else if ans1 == 0   { 0 }
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

    fn initialize_selected_questions(&self) -> [u16; NUMBER_QUESTIONS]
    {
        let mut sq = [0u16; NUMBER_QUESTIONS];
        for i in 0..NUMBER_QUESTIONS
        {
            sq[i] = (i+1) as u16;
        }

        let mut prng = PRNG::new();
        for _ in 0..NUMBER_QUESTIONS*3
        {
            let i = prng.random_under_uint_(NUMBER_SELECTED_QUESTIONS);
            let mut j = prng.random_under_uint_(NUMBER_QUESTIONS);
            while i == j
            {
                j = prng.random_under_uint_(NUMBER_QUESTIONS);
            }
            let tmp = sq[i];
            sq[i] = sq[j];
            sq[j] = tmp;
        }

        return sq;
    }

    fn shuffle_choices(&self) -> [u8; NUMBER_CHOICES]
    {
        let mut c: [u8; NUMBER_CHOICES] = [1, 2, 3, 4];
        let mut prng = PRNG::new();
        for _ in 0..(NUMBER_CHOICES * 3)
        {
            let i = prng.random_under_uint_(NUMBER_CHOICES);
            let mut j = prng.random_under_uint_(NUMBER_CHOICES);
            while i == j
            {
                j = prng.random_under_uint_(NUMBER_CHOICES);
            }
            let tmp = c[i];
            c[i] = c[j];
            c[j] = tmp;
        }

        return c;
    }

    fn select_questions(&mut self)
    {
        self.selected_qb.clear();
        let selected_questions = self.initialize_selected_questions();
    //    println!("{:?}", selected_questions);

        for &question in &selected_questions[0..NUMBER_SELECTED_QUESTIONS]
        {
            let q = ShuffledQuestion {
                question,
                choice: self.shuffle_choices(),
            };
            self.selected_qb.push(q);
        }
    //    println!("{:?}", selected_qb);
    }

    fn write_selected_questions(&self) -> String
    {
        let mut paper = String::new();
        let mut number = 1;
        for selected in &self.selected_qb
        {
            let qb = &self.qbs[(selected.question()-1) as usize];
            paper.push_str(&format!("{}. {}\n", number, qb.question()));
            for ch in 1..=4
            {
                paper.push_str(&format!("{}{}) {}\n", SAVE_PAPER_SPACE, ch, qb.choice(selected.choice(ch))));
            }
            number += 1;
            paper.push_str(&format!("\n"));
        }
        return paper;
    }
    
    pub fn print_selected_questions(&self) -> &Exam
    {
        println!("\n==== {} Randomly Chosen Questions ===\n", NUMBER_SELECTED_QUESTIONS);
        println!("{}", self.write_selected_questions());
        return self;
    }

    fn write_answers_of_selected(&self) -> String
    {
        let mut paper = String::new();
        let mut number = 1;
        for selected in &self.selected_qb
        {
            let qb = &self.qbs[(selected.question()-1) as usize];
            let (mut answer1, mut answer2) = qb.answer();
            for ch in 0..4
            {
                if selected.choice[ch] == answer1
                {
                    answer1 = (ch+1) as u8;
                    break;
                }
            }
            if qb.has_multianswers()
            {
                for ch in 0..4
                {
                    if selected.choice[ch] == answer2
                    {
                        answer2 = (ch+1) as u8;
                        break;
                    }
                }
                paper.push_str(&format!("{}. ({}, {})\t", number, answer1, answer2));
            }
            else
            {
                paper.push_str(&format!("{}. {}\t", number, answer1));
            }
            number += 1;
        }
        return paper;
    }

    pub fn print_answers_of_selected(&self)
    {
        println!("\n==== The Answers of The Above Questions ===\n");
        println!("{}", self.write_answers_of_selected());
        println!("\n");
    }

    // fn write_exams(&mut self, students: &Vec::<Student>) -> (String, String)
    // {
    //     let mut paper = String::new();
    //     let mut card = String::new();
    //     for st in students
    //     {
    //         self.select_questions();
    //         paper.push_str(is_question_bank::ISQuestionBank::get_header(&st).as_str());
    //         paper.push_str(&self.write_selected_questions());
    //         paper.push('\n');
    //         card.push_str(&format!("Name: {}\tID: {}\n", st.get_name(), st.get_id()));
    //         card.push_str(&self.write_answers_of_selected());
    //         card.push('\n');
    //     }
    //     return (paper, card);
    // }

    pub fn print_exams(&mut self, students: &Vec::<Student>)
    {
        let (paper, card) = self.write_exams(&students);
        println!("{}", paper);
        println!("{}", card);
    }

    pub fn save_exams(&mut self, students: &Vec::<Student>, path: &str)
    {
        let (paper, card) = self.write_exams(&students);
        let mut f = File::create(&path).unwrap();
        let _ = f.write(paper.as_bytes());
        let _ = f.write(card.as_bytes());
    }

    // fn init_bank(&mut self)
    // {
    //     self.qbs = is_question_bank::ISQuestionBank::new().questions;
    // }  
}
*/