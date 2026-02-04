// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use crate::{ QBank, Questions, ShuffledQSet, ShuffledQSets, Student, Students, qbank, question, student };


pub struct Generator
{
    origin: QBank,
    shuffled_sets: ShuffledQSets,
}

impl Generator
{
    pub fn new_one_set(qbank: &QBank, start: u16, end: u16) -> Option<Self>
    {
        let student = Student::new_empty();
        let students = vec![student];
        Self::new(qbank, start, end, &students)
    }

    pub fn new(qbank: &QBank, start: u16, end: u16, students: &Students) -> Option<Self>
    {
        let mut shuffled_sets = ShuffledQSets::new();
        for i in 0..students.len()
        {
            let mut shuffled_set = ShuffledQSet::new(qbank, &students[i], start, end)?;
            shuffled_set.shuffle();
            shuffled_sets.push(shuffled_set);
        }
        Some(Self { origin: qbank.clone(), shuffled_sets })
    }

    pub fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    {
        if idx < self.shuffled_sets.len() { Some(self.shuffled_sets[idx].clone()) } else { None }
    }

    pub fn get_shuffled_qbank(&self, idx: usize) -> Option<(Student, QBank)>
    {
        if idx < self.shuffled_sets.len()
        {
            let header = self.origin.get_header().clone();
            let mut qbank = QBank::new_with_header(header);
            let mut questions = Questions::new();
            for i in 0..self.shuffled_sets[idx].get_shuffled_questions().len()
            {
                let qn = self.shuffled_sets[idx].get_shuffled_questions()[i].get_question();
                let question = self.origin.get_question(qn as usize)?;
                questions.push(question.clone());
            }
            qbank.set_questions(questions);
            Some((self.shuffled_sets[idx].get_student(), qbank))
        }
        else
        {
            None
        }
    }

    pub fn get_shuffled_qbanks(&self) -> Vec::<(Student, QBank)>
    {
        let mut shuffled_qbanks = Vec::new();
        for i in 0..self.shuffled_sets.len()
        {
            let shuffled_qbank = self.get_shuffled_qbank(i).unwrap();
            shuffled_qbanks.push(shuffled_qbank);
        }
        shuffled_qbanks
    }
}