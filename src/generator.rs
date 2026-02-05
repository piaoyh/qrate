// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-200> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use crate::{ Choices, QBank, Questions, ShuffledQSet, ShuffledQSets, Student, Students };


pub struct Generator
{
    origin: QBank,
    shuffled_qsets: ShuffledQSets,
    current_question_number: u16,
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
        let mut shuffled_qsets = ShuffledQSets::new();
        for i in 0..students.len()
        {
            let mut shuffled_qset = ShuffledQSet::new(qbank, &students[i], start, end)?;
            shuffled_qset.shuffle();
            shuffled_qsets.push(shuffled_qset);
        }
        Some(Self { origin: qbank.clone(), shuffled_qsets, current_question_number: 0 })
    }

    // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
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
    #[inline]
    pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    {
        if idx < self.shuffled_qsets.len() { Some(self.shuffled_qsets[idx].clone()) } else { None }
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
        if idx < self.shuffled_qsets.len()
        {
            let header = self.origin.get_header().clone();
            let mut qbank = QBank::new_with_header(header);
            let mut questions = Questions::new();
            for i in 0..self.shuffled_qsets[idx].get_shuffled_questions().len()
            {
                let qn = self.shuffled_qsets[idx].get_shuffled_questions()[i].get_question();
                let question = self.origin.get_question(qn as usize)?;
                questions.push(question.clone());
            }
            qbank.set_questions(questions);
            Some((self.shuffled_qsets[idx].get_student().clone(), qbank))
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
        for i in 0..self.shuffled_qsets.len()
        {
            let shuffled_qbank = self.get_shuffled_qbank(i).unwrap();
            shuffled_qbanks.push(shuffled_qbank);
        }
        shuffled_qbanks
    }

    // pub fn get_notice(&self) -> String
    /// Retrieves the notice string from the original question bank's header.
    ///
    /// This function accesses the header of the `QBank` used to create the
    /// `Generator` instance and returns its notice string.
    ///
    /// # Output
    /// A `String` containing the notice from the question bank's header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Header };
    ///
    /// let mut qbank = QBank::new_empty();
    /// let mut header = Header::new_empty();
    /// header.set_notice("Important Notice!".to_string());
    /// qbank.set_header(header);
    ///
    /// let generator = Generator::new_one_set(&qbank, 1, 1).unwrap();
    /// let notice = generator.get_notice();
    /// assert_eq!(notice, "Important Notice!");
    /// ```
    #[inline]
    pub fn get_notice(&self) -> String
    {
        self.origin.get_header().get_notice().clone()
    }

    pub fn next(&mut self) -> Option<(u16, String, String, Choices)>
    {
        self.current_question_number += 1;
        let real_question_number = self.shuffled_qsets[0].get_shuffled_question(self.current_question_number)?.get_question() as usize;
        let category = self.origin.get_header().get_category( real_question_number)?.clone();
        let question = self.origin.get_question(real_question_number)?.get_question().clone();
        let choices: Choices = self.origin.get_ch(real_question_number);
        // 제미나이야! 이 함수를 만들어 줘. 이 함수는 /src/examples/prep.rs에서
        // 학생이 셀프 테스트를 할 때에 쓰이는 exam() 함수에서 쓰일 함수야.
        // 반환값은 튜플을 Option으로 감싼 형태인데,
        // 그 튜플은 다음과 같이 구성되어 있어.
        // 현재의 문제 번호(self.current_question_number),
        // 현재의 문제의 카테고리 스트링(변수는 category),
        // 현재의 문제의 문제 스트링(변수는 question),
        // 현재의 문제의 선택지와 정답 여부의 튜플의 벡터(변수는 choices)로
        // 튜플이 구성되어 있어.
        // 그리고,이 함수를 호출할 때마다 다음 문제로 넘어가서 해당 반환값을 반환해.
        // 이 함수를 완성한 다음에는 독스트링도 만들어 줘.
        Some((self.current_question_number, category, question, choices))
    }
}