
// use std::io;
// use std::fs::File;
// use std::io::Write;
use cryptocol::random::Slapdash as PRNG;

use crate::{ QBank, Question, Questions, Student };


// const NUMBER_QUESTIONS: usize = 48; // 51;
pub const NUMBER_SELECTED_QUESTIONS: usize = 25;
// const SAVE_PAPER_SPACE: &str = "";  // "\t"

pub type ShuffledQuestions = Vec::<ShuffledQuestion>;
pub type ShuffledQSets = Vec::<ShuffledQSet>;


#[derive(Debug, Clone)]
pub struct ShuffledQuestion
{
    question: u16,      // 1-based
    choices: Vec<u8>,   // 1-based
}

impl ShuffledQuestion
{
    pub fn new(question: u16, number_of_choices: u8) -> Self
    {
        let mut choices = Vec::new();
        for i in 1..=number_of_choices
            { choices.push(i); }
        ShuffledQuestion { question, choices }
    }

    pub fn get_question(&self) -> u16
    {
        self.question
    }

    pub fn set_question(&mut self, question: u16)
    {
        self.question = question;
    }

    pub fn get_choice(&self, idx: usize) -> u8
    {
        if idx <= self.choices.len() as usize
            { self.choices[idx as usize] }
        else
            { 0 }
    }

    pub fn set_choice(&mut self, idx: usize, choice: u8)
    {
        self.choices[idx] = choice;
    }

    pub fn get_choices(&mut self) -> &Vec<u8>
    {
        &self.choices
    }

    pub fn set_choices(&mut self, choices: Vec<u8>)
    {
        self.choices = choices;
    }

    pub fn how_many_choices(&self) -> usize
    {
        self.choices.len()
    }

    pub fn shuffle(&mut self)
    {
        let mut prng = PRNG::new();
        let max = self.how_many_choices();
        for _ in 0..3
        {
            for i in 0..max
            {
                let j = prng.random_under_uint_(max);
                (self.choices[i], self.choices[j]) = (self.choices[j], self.choices[i]);
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct ShuffledQSet
{
    student: Student,
    questions: ShuffledQuestions,
}

impl ShuffledQSet
{
    pub fn new(qbank: &QBank, student: &Student, start: u16, end: u16) -> Option<Self>
    {
        let last = qbank.get_questions().len() as u16;
        if (start > end) || (start > last) || (end > last)
            { return None }

        let mut questions = ShuffledQuestions::new();
        for question in (start - 1)..(end - 1)
        {
            let number_of_choices = qbank.get_question(question as usize).unwrap().get_choices().len() as u8;
            let mut shuffled_question = ShuffledQuestion::new(question, number_of_choices);
            shuffled_question.shuffle();
            questions.push(shuffled_question);
        }

        Some(Self{ student: student.clone(), questions })
    }

    pub fn shuffle(&mut self)
    {
        let mut prng = PRNG::new();
        let max = self.questions.len();
        for _ in 0..3
        {
            for i in 0..max
            {
                let j = prng.random_under_uint_(max);
                (self.questions[i], self.questions[j]) = (self.questions[j].clone(), self.questions[i].clone());
            }
        }
    }

    pub fn get_student(&self) -> Student
    {
        self.student.clone()
    }

    pub fn set_student(&mut self, student: &Student)
    {
        self.student = student.clone();
    }

    pub fn get_shuffled_questions(&self) -> ShuffledQuestions
    {
        self.questions.clone()
    }

    pub fn set_shuffled_questions(&mut self, questions: ShuffledQuestions)
    {
        self.questions = questions;
    }
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

    // fn write_exams(&mut self, students: &Students) -> (String, String)
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

    pub fn print_exams(&mut self, students: &Students)
    {
        let (paper, card) = self.write_exams(&students);
        println!("{}", paper);
        println!("{}", card);
    }

    pub fn save_exams(&mut self, students: &Students, path: &str)
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