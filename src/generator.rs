// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-200> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use crate::{ QBank, Questions, ShuffledQSet, ShuffledQSets, Student, Students };


pub struct Generator
{
    origin: QBank,
    shuffled_sets: ShuffledQSets,
}

impl Generator
{
    // pub fn new_one_set(qbank: &QBank, start: u16, end: u16) -> Option<Self>
    /// Creates a new `Generator` instance for a single shuffled set.
    ///
    /// This function generates a single shuffled question set based on the provided
    /// question bank, starting and ending question numbers.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The starting number of the questions to include (inclusive).
    /// * `end` - The ending number of the questions to include (inclusive).
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or `None` if
    /// the generation fails (e.g., invalid question range).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let generator = Generator::new_one_set(&qbank, 1, 2);
    /// assert!(generator.is_some());
    /// ```
    pub fn new_one_set(qbank: &QBank, start: u16, end: u16) -> Option<Self>
    {
        let student = Student::new_empty();
        let students = vec![student];
        Self::new(qbank, start, end, &students)
    }

    // pub fn new(qbank: &QBank, start: u16, end: u16, students: &Students) -> Option<Self>
    /// Creates a new `Generator` instance for multiple shuffled sets, one for each student.
    ///
    /// This function generates shuffled question sets for each student based on the
    /// provided question bank, starting and ending question numbers.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The starting number of the questions to include (inclusive).
    /// * `end` - The ending number of the questions to include (inclusive).
    /// * `students` - A slice of `Student` instances for whom shuffled sets will be generated.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or `None` if
    /// the generation fails (e.g., invalid question range).
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let student2 = Student::new_from_name("Bob".to_string());
    /// let students = Students::new(vec![student1, student2]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students);
    /// assert!(generator.is_some());
    /// ```
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

    // pub fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    /// Retrieves a specific shuffled question set by its index.
    ///
    /// This function returns a cloned `ShuffledQSet` for the given index,
    /// if the index is within the bounds of the generated shuffled sets.
    ///
    /// # Arguments
    /// * `idx` - The zero-based index of the shuffled question set to retrieve.
    ///
    /// # Output
    /// An `Option<ShuffledQSet>` which is `Some(ShuffledQSet)` if the index is valid,
    /// or `None` if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students).unwrap();
    /// let shuffled_qset = generator.get_shuffled_qset(0);
    /// assert!(shuffled_qset.is_some());
    /// let no_shuffled_qset = generator.get_shuffled_qset(1);
    /// assert!(no_shuffled_qset.is_none());
    /// ```
    pub fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    {
        if idx < self.shuffled_sets.len() { Some(self.shuffled_sets[idx].clone()) } else { None }
    }

    // pub fn get_shuffled_qbank(&self, idx: usize) -> Option<(Student, QBank)>
    /// Retrieves a specific shuffled `QBank` and its associated `Student` by index.
    ///
    /// This function reconstructs a `QBank` with shuffled questions for a given student
    /// based on the original `QBank` and the shuffled question set at the specified index.
    ///
    /// # Arguments
    /// * `idx` - The zero-based index of the shuffled question set.
    ///
    /// # Output
    /// An `Option<(Student, QBank)>` which is `Some((Student, QBank))` if the index is valid,
    /// or `None` if the index is out of bounds or a question cannot be found.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students).unwrap();
    /// let shuffled_qbank_tuple = generator.get_shuffled_qbank(0);
    /// assert!(shuffled_qbank_tuple.is_some());
    /// let (student, shuffled_qbank) = shuffled_qbank_tuple.unwrap();
    /// assert_eq!(student.get_name(), "Alice");
    /// assert_eq!(shuffled_qbank.get_questions().len(), 2);
    ///
    /// let no_shuffled_qbank = generator.get_shuffled_qbank(1);
    /// assert!(no_shuffled_qbank.is_none());
    /// ```
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
            Some((self.shuffled_sets[idx].get_student().clone(), qbank))
        }
        else
        {
            None
        }
    }

    // pub fn get_shuffled_qbanks(&self) -> Vec::<(Student, QBank)>
    /// Retrieves all generated shuffled `QBank` instances
    /// with their associated `Student`s.
    ///
    /// This function iterates through all generated shuffled question sets and
    /// reconstructs a `QBank` for each, paired with its corresponding `Student`.
    ///
    /// # Output
    /// A `Vec<(Student, QBank)>` containing all shuffled question banks and
    /// their students.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    /// qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let student2 = Student::new_from_name("Bob".to_string());
    /// let students = Students::new(vec![student1, student2]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, &students).unwrap();
    /// let shuffled_qbanks = generator.get_shuffled_qbanks();
    /// assert_eq!(shuffled_qbanks.len(), 2);
    /// assert_eq!(shuffled_qbanks[0].0.get_name(), "Alice");
    /// assert_eq!(shuffled_qbanks[1].0.get_name(), "Bob");
    /// ```
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