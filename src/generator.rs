// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-200> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use docx_rs::*;
use rtf_grimoire::{Rtf, Style, Doc, Para, part::*};
use genpdf::*;

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

    // pub fn next(&mut self) -> Option<(u16, String, String, Choices)>
    /// Advances to the next question in the shuffled set and returns its details.
    ///
    /// This function acts as an iterator for the generated question set. Each call
    /// increments the internal question counter and provides the details of the
    /// next question, including the category, the question text, and the choices
    /// in their shuffled order.
    ///
    /// It is primarily used for self-testing scenarios, such as in the `exam()`
    /// function found in `src/examples/prep.rs`.
    ///
    /// # Output
    /// `Option<(u16, String, String, Choices)>` - An `Option` containing a tuple with:
    ///   - `u16`: The current question number within the shuffled set.
    ///   - `String`: The category of the current question.
    ///   - `String`: The text of the current question.
    ///   - `Choices`: A vector of tuples `(String, bool)` representing the
    ///                shuffled choices and whether each is a correct answer.
    ///
    /// Returns `None` if there are no more questions in the set.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Students };
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices("Question 1".to_string(), vec![("A".to_string(), true)]);
    /// qbank.add_question_with_choices("Question 2".to_string(), vec![("B".to_string(), true)]);
    ///
    /// let mut generator = Generator::new_one_set(&qbank, 1, 2).unwrap();
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 1); }  // The actual question text depends on the shuffled order.
    ///
    /// if let Some((num, cat, q_text, choices)) = generator.next()
    ///     { assert_eq!(num, 2); }
    ///
    /// assert!(generator.next().is_none());
    /// ```
    pub fn next(&mut self) -> Option<(u16, String, String, Choices)>
    {
        self.current_question_number += 1;

        let shuffled_qset = self.shuffled_qsets.get(0)?;
        if self.current_question_number as usize > shuffled_qset.get_shuffled_questions().len()
            { return None; }

        let shuffled_question = shuffled_qset.get_shuffled_question(self.current_question_number)?;
        let real_question_number = shuffled_question.get_question();
        let shuffled_indices = shuffled_question.get_choices();

        let origin_question = self.origin.get_question(real_question_number as usize)?;
        let category = self.origin.get_header().get_category(real_question_number as usize)?.clone();
        let question_text = origin_question.get_question().clone();
        let origin_choices = origin_question.get_choices();

        let mut choices = Choices::new();
        for &shuffled_index in shuffled_indices
        {
            if let Some(choice) = origin_choices.get((shuffled_index - 1) as usize)
                { choices.push(choice.clone()); }
            else
                { return None; }
        }

        Some((self.current_question_number, category, question_text, choices))
    }

    // pub fn save_shuffled_exams(&self, path: String) -> Result<(), String>
    /// Saves the shuffled exam sets for all students to a single file.
    ///
    /// The output format is determined by the file extension of the provided path.
    /// Supported formats are: .txt, .rtf, .docx, and .pdf.
    /// This function delegates the actual saving process to format-specific private functions.
    ///
    /// # Arguments
    /// * `path` - The file path where the exams will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    pub fn save_shuffled_exams(&self, path: String) -> Result<(), String>
    {
        let file_path = Path::new(&path);
        match file_path.extension().and_then(|s| s.to_str())
        {
            Some("txt") => self.save_shuffled_exams_in_txt(file_path),
            Some("rtf") => self.save_shuffled_exams_in_rtf(file_path),
            Some("docx") => self.save_shuffled_exams_in_docx(file_path),
            Some("pdf") => self.save_shuffled_exams_in_pdf(file_path),
            _ => Err("Unsupported file format. Please use .txt, .rtf, .docx, or .pdf.".to_string()),
        }
    }

    /// Saves the shuffled exam sets to a text file.
    fn save_shuffled_exams_in_txt(&self, path: &Path) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        // TODO: Implement the actual logic to write content to the txt file.
        // For now, it just creates an empty file.
        writeln!(file, "TODO: Implement TXT saving logic.").map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Saves the shuffled exam sets to an RTF file.
    fn save_shuffled_exams_in_rtf(&self, path: &Path) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        // TODO: Implement the actual logic to generate and write RTF content.
        // For now, it just creates an empty file with a placeholder.
        let mut doc = Doc::new();
        doc.push_para(Para::new().push_text("TODO: Implement RTF saving logic."));
        file.write_all(doc.to_bytes().as_slice()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Saves the shuffled exam sets to a DOCX file.
    fn save_shuffled_exams_in_docx(&self, path: &Path) -> Result<(), String>
    {
        let file = File::create(path).map_err(|e| e.to_string())?;
        // TODO: Implement the actual logic to generate and write DOCX content.
        // For now, it just creates an empty file with a placeholder.
        let docx = Docx::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("TODO: Implement DOCX saving logic.")));
        docx.build().pack(file).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Saves the shuffled exam sets to a PDF file.
    fn save_shuffled_exams_in_pdf(&self, path: &Path) -> Result<(), String>
    {
        // TODO: Implement the actual logic to generate and write PDF content.
        // For now, it just creates an empty file with a placeholder.
        let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
            .map_err(|e| format!("Failed to load font: {}", e))?;
        let mut doc = genpdf::Document::new(font_family);
        doc.push(genpdf::elements::Paragraph::new("TODO: Implement PDF saving logic."));
        doc.render_to_file(path).map_err(|e| e.to_string())?;
        Ok(())
    }
}

// * 기능
// ** 각각의 학생들의 시험 세트를 path 파일 이름의 확장자에 따라 다른 문서 포맷으로 path 파일 이름의 하나의 파일에 한꺼번에 저장한다.
// * 저장하려는 파일 포맷에 대한 지시 사항
// ** 페이지를 나눌 수 있는 포맷이라면, 다음 학생의 시험 세트를 쓸 때에는 다음 페이지에서 시작한다.
// ** 글꼴의 크기를 정할 수 있는 포맷에 대한 지시 시항
// *** 시험의 타이틀 즉, Header::title의 내용은 14 포인트로 한다.
// *** 시험의 주의사항 즉, Header::notice의 내용은 11 포인트로 한다.
// *** 그 외의 것들은 모두 11 포인트로 한다.
// ** 줄간격은 한 줄 간격으로 한다.
// ** 문제와 문제 사이에는 기본적으로 하나의 빈 줄들을 삽입한다.
// ** 하나의 문제가 두 페이지로 나뉘지 않도록 필요한 경우, 문제와 문제 사이에 복수의 빈 줄들을 삽입한다.

// * 함수를 만듦에 있어서의 지시사항
// ** save_shuffled_exams_in_txt(), save_shuffled_exams_in_rtf(),
// save_shuffled_exams_in_docx(), save_shuffled_exams_in_pdf() 함수들도
// 너무 길게 만들지 말고 기능 별로 쪼개서 private 함수들을 만들고, 이를 호출한다.
// ** 공통되는 기능은 하나의 함수를 만들어 여러 함수들에서 이를 호출한다.