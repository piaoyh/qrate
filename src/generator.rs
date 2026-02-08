// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::Write;
use std::path::Path;

use docx_rs::{ Docx, Paragraph, Run, BreakType, PageMargin, AlignmentType,
                Footer, InstrText, InstrPAGE, InstrNUMPAGES, FieldCharType };
use genpdf::{ Document, elements, fonts, style, Element, SimplePageDecorator };
use genpdf::Alignment;

use crate::{ Choices, QBank, Questions, check_path };
use crate::{ Students, Student };
use crate::{ ShuffledQSet, ShuffledQSets };


pub struct Generator
{
    origin: QBank,
    shuffled_qsets: ShuffledQSets,
    current_question_number: u16,
}

impl Generator
{
    // pub fn new_one_set(qbank: &QBank, start: u16, end: u16, selected: usize) -> Option<Self>
    /// Creates a new `Generator` instance for a single shuffled set.
    ///
    /// This function generates a single shuffled question set based on the provided
    /// question bank, starting and ending question numbers.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The starting number of the questions to include (inclusive).
    /// * `end` - The ending number of the questions to include (inclusive).
    /// * `selected` - The number of questions to be randomly selected.
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
    /// let generator = Generator::new_one_set(&qbank, 1, 2, 2);
    /// assert!(generator.is_some());
    /// ```
    pub fn new_one_set(qbank: &QBank, start: u16, end: u16, selected: usize) -> Option<Self>
    {
        let student = Student::new_empty();
        let students = vec![student];
        Self::new(qbank, start, end, selected, &students)
    }

    // pub fn new(qbank: &QBank, start: u16, end: u16, selected: usize, students: &Students) -> Option<Self>
    /// Creates a new `Generator` instance for multiple shuffled sets, one for each student.
    ///
    /// This function generates shuffled question sets for each student based on the
    /// provided question bank, considering a specified range and number of randomly selected questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing the original questions.
    /// * `start` - The 1-based starting index of questions to consider (inclusive).
    /// * `end` - The 1-based ending index of questions to consider (inclusive).
    /// * `selected` - The number of questions to be randomly selected for each student.
    /// * `students` - A slice of `Student` instances for whom shuffled sets will be generated.
    ///
    /// # Output
    /// An `Option<Self>` which is `Some(Generator)` if successful, or `None` if
    /// the generation fails (e.g., invalid question range, insufficient questions, or selected count).
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
    /// // Generate exams with 2 questions selected for each student
    /// let generator = Generator::new(&qbank, 1, 2, 2, &students);
    /// assert!(generator.is_some());
    /// ```
    pub fn new(qbank: &QBank, start: u16, end: u16, selected: usize, students: &Students) -> Option<Self>
    {
        let mut shuffled_qsets = ShuffledQSets::new();
        for i in 0..students.len()
        {
            let mut shuffled_qset = ShuffledQSet::new(qbank, start, end, selected, &students[i])?;
            shuffled_qset.shuffle();
            shuffled_qsets.push(shuffled_qset);
        }
        Some(Self { origin: qbank.clone(), shuffled_qsets, current_question_number: 0 })
    }

    // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    // Retrieves a specific shuffled question set by its index.
    //
    // This function returns a cloned `ShuffledQSet` for the given index,
    // if the index is within the bounds of the generated shuffled sets.
    //
    // # Arguments
    // * `idx` - The zero-based index of the shuffled question set to retrieve.
    //
    // # Output
    // An `Option<ShuffledQSet>` which is `Some(ShuffledQSet)` if the index is valid,
    // or `None` if the index is out of bounds.
    //
    // # Examples
    // ```
    // use qrate::{ QBank, Generator, Student, Students };
    //
    // let mut qbank = QBank::new_empty();
    // qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    // qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    //
    // let student1 = Student::new_from_name("Alice".to_string());
    // let students = Students::new(vec![student1]);
    //
    // let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    // let shuffled_qset = generator.get_shuffled_qset(0);
    // assert!(shuffled_qset.is_some());
    // let no_shuffled_qset = generator.get_shuffled_qset(1);
    // assert!(no_shuffled_qset.is_none());
    // ```
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
    /// It is primarily used for self-testing scenarios, suchs as in the `exam()`
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
        let category = self.origin.get_header().get_category(origin_question.get_category())?.clone();
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

    // pub fn save_shuffled_exams(&self, path: String, extention: &str) -> Result<(), String>
    /// Saves the shuffled exam sets for all students to a single file.
    ///
    /// The output format is determined by the file extension of the provided path.
    /// Supported formats are: .txt, .docx, and .pdf.
    /// This function delegates the actual saving process to format-specific private functions.
    ///
    /// # Arguments
    /// * `path` - The file path where the exams will be saved.
    /// * `extention` - The desired file extension (e.g., "txt", "docx", "pdf").
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs; // For std::fs::remove_file
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// // Generate exams with 1 question selected for each student
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams("exam.txt".to_string(), "txt");
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.txt").unwrap();
    /// ```
    pub fn save_shuffled_exams(&self, path: String, extention: &str) -> Result<(), String>
    {
        let checked = check_path(path, extention);
        let file_path = Path::new(&checked);
        match file_path.extension().and_then(|s| s.to_str())
        {
            Some("txt") => self.save_shuffled_exams_in_txt(file_path),
            Some("docx") => self.save_shuffled_exams_in_docx(file_path),
            Some("pdf") => self.save_shuffled_exams_in_pdf(file_path),
            _ => Err("Unsupported file format. Please use .txt, .docx, or .pdf.".to_string()),
        }
    }

    // fn format_exam_for_student(&self, student: &Student, qbank: &QBank) -> String
    /// Formats the exam content for a single student into a human-readable string.
    ///
    /// This private helper function generates the textual representation of an exam
    /// for a given student and their shuffled question bank. It includes the student's
    /// name, the exam title, and all questions with their shuffled choices.
    ///
    /// # Arguments
    /// * `student` - A reference to the `Student` for whom the exam is being formatted.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// A `String` containing the fully formatted exam content for the student.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator, Student, Header };
    ///
    /// let mut qbank = QBank::new_empty();
    /// let mut header = Header::new_empty();
    /// header.set_title("Test Exam".to_string());
    /// qbank.set_header(header);
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    ///
    /// let student = Student::new_from_name("John Doe".to_string());
    /// let generator = Generator::new_one_set(&qbank, 1, 1).unwrap();
    ///
    /// // Since format_exam_for_student is private, we can't directly call it in an example.
    /// // This example demonstrates how the data would be prepared.
    /// let (retrieved_student, retrieved_qbank) = generator.get_shuffled_qbank(0).unwrap();
    /// let formatted_content = format!(
    ///     "Student: {}\nExam: {}\n\n1. What is 1+1?\n    (A) 1\n    (B) 2\n",
    ///     retrieved_student.get_name(),
    ///     retrieved_qbank.get_header().get_title()
    /// );
    /// // In a real test, you'd assert against the output of the function,
    /// // but for a private helper, we rely on its callers to be tested.
    /// ```
    fn format_exam_for_student(&self, student: &Student, qbank: &QBank) -> String
    {
        let mut content = String::new();
        let header = qbank.get_header();

        // Exam Title
        content.push_str(&format!("{}\n", header.get_title()));

        // Student Information
        content.push_str(&format!("{}: {}        {}: {}\n\n", header.get_name(), student.get_name(), header.get_id(), student.get_id()));

        for (i, question) in qbank.get_questions().iter().enumerate()
        {
            let modum = header.get_category(question.get_category()).unwrap();
            content.push_str(&format!("{}. [{}]   {}\n", i + 1, modum, question.get_question()));
            for (j, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
            {
                let choice_char = (b'A' + j as u8) as char;
                content.push_str(&format!("    ({}) {}\n", choice_char, choice_text));
            }
            content.push_str("\n"); // Blank line after each question
        }
        content
    }

    // pub fn save_shuffled_exams_in_txt(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a text file.
    ///
    /// This function generates a text file containing the shuffled exam sets
    /// for all students, with each student's exam separated by a clear delimiter.
    ///
    /// # Arguments
    /// * `path` - The file path where the text document will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question_with_choices(
    ///     "What is 1+1?".to_string(),
    ///     vec![("1".to_string(), false), ("2".to_string(), true)]
    /// );
    /// qbank.add_question_with_choices(
    ///     "What is 2+2?".to_string(),
    ///     vec![("3".to_string(), false), ("4".to_string(), true)]
    /// );
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_txt(Path::new("exam_shuffled.txt"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam_shuffled.txt").unwrap();
    /// ```
    pub fn save_shuffled_exams_in_txt(&self, path: &Path) -> Result<(), String>
    {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (student, qbank) in &shuffled_qbanks
        {
            let content = self.format_exam_for_student(&student, &qbank);
            writeln!(file, "{}", content).map_err(|e| e.to_string())?;
            // Add a separator for multiple students, if applicable
            if self.shuffled_qsets.len() > 1
                { writeln!(file, "-------------- CUT -------------- 자르기 -------------- резать --------------\n\n").map_err(|e| e.to_string())?; }
        }
        // Add a separator for the answer sheet
        write!(file, "\n\u{000C}\n").map_err(|e| e.to_string())?; // Form feed for page break

        let header = self.origin.get_header(); // Need the original header for titles
        writeln!(file, "Answer Sheet        정답지        Ответы\n").map_err(|e| e.to_string())?;

        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            writeln!(file, "{}: {}        {}: {}",
                header.get_name(), student.get_name(),
                header.get_id(), student.get_id()
            ).map_err(|e| e.to_string())?;

            // Answers
            let mut answer_line = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let correct_choices: Vec<String> = question.get_choices()
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, is_correct))| *is_correct)
                    .map(|(j, _)| ((b'a' + j as u8) as char).to_string())
                    .collect();
                let answer_string = correct_choices.join(", ");

                let entry = format!("{}. {}    ", i + 1, answer_string);

                // Simple line wrapping logic
                if answer_line.len() + entry.len() > 80 && !answer_line.is_empty() {
                    writeln!(file, "{}", answer_line).map_err(|e| e.to_string())?;
                    answer_line.clear();
                }
                answer_line.push_str(&entry);
            }
            if !answer_line.is_empty() {
                writeln!(file, "{}", answer_line).map_err(|e| e.to_string())?;
            }
            writeln!(file, "").map_err(|e| e.to_string())?; // Blank line after each student
        }

        Ok(())
    }

    // pub fn save_shuffled_exams_in_docx(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a DOCX file.
    ///
    /// This function generates a DOCX document containing the shuffled exam sets
    /// for all students, applying specified page margins and a footer with page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the DOCX document will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_docx(Path::new("exam.docx"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.docx").unwrap();
    /// ```
    pub fn save_shuffled_exams_in_docx(&self, path: &Path) -> Result<(), String>
    {
        let footer = Footer::new()
            .add_paragraph(
                Paragraph::new()
                    .add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::PAGE(InstrPAGE::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(20) // 10 pt
                    )
                    .add_run(Run::new().add_text(" / ").size(20)) // 10 pt
                    .add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::NUMPAGES(InstrNUMPAGES::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(20) // 10 pt
                    )
                    .align(AlignmentType::Center)
            );
        let mut docx = Docx::new()
            .page_margin(PageMargin::new().top(567).bottom(567).left(567).right(567)) // 1cm top, bottom, left, right
            .footer(footer);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (idx, (student, qbank)) in shuffled_qbanks.iter().enumerate()
        {
            if idx > 0
                { docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page))); } // Page break for subsequent students
            self.write_exam_content_to_docx(&mut docx, &student, &qbank)?;
        }

        // Add answer sheet
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text("Answer Sheet        정답지        Ответы").size(28)).align(AlignmentType::Center)); // 14pt
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        let header = self.origin.get_header();

        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(),
                header.get_id(), student.get_id()
            );
            let student_info_paragraph = Paragraph::new()
                .add_run(Run::new().add_text(student_info_text).size(24)) // 12pt
                .line_spacing(docx_rs::LineSpacing::new().line(240));   // Single line spacing
            docx = docx.add_paragraph(student_info_paragraph);

            // Answers
            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let correct_choices: Vec<String> = question.get_choices()
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, is_correct))| *is_correct)
                    .map(|(j, _)| ((b'a' + j as u8) as char).to_string())
                    .collect();
                let answer_string = correct_choices.join(", ");
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }

            let answers_paragraph = Paragraph::new()
                .add_run(Run::new().add_text(answers_text).size(24)) // 12pt
                .line_spacing(docx_rs::LineSpacing::new().line(240));   // Single line spacing
            docx = docx.add_paragraph(answers_paragraph);
            docx = docx.add_paragraph(Paragraph::new()); // Blank line
        }

        let file = File::create(path).map_err(|e| e.to_string())?;
        docx.build().pack(file).map_err(|e| e.to_string())?;
        Ok(())
    }

    // fn write_exam_content_to_docx(&self, docx: &mut Docx, student: &Student, qbank: &QBank) -> Result<(), String>
    /// Writes the formatted exam content for a single student to a DOCX document.
    ///
    /// This private helper function takes a mutable DOCX `Docx` object and appends
    /// the exam content for the given student and their shuffled question bank,
    /// applying DOCX-specific formatting such as font sizes.
    ///
    /// # Arguments
    /// * `docx` - A mutable reference to the `docx_rs::Docx` object.
    /// * `student` - A reference to the `Student` for whom the exam content is being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    fn write_exam_content_to_docx(&self, docx: &mut Docx, student: &Student, qbank: &QBank) -> Result<(), String>
    {
        let paragraph = |txt, size| -> Paragraph
        {
            let elem = Run::new().add_text(txt).size(size << 1);  // `size` pt
            Paragraph::new().add_run(elem)
        };
        let header = qbank.get_header();

        // Exam Title
        let ex = paragraph(format!("{}", header.get_title()), 14_usize);

        // Student Information
        let st = paragraph(format!("{}: {}        {}: {}\n\n", header.get_name(), student.get_name(), header.get_id(), student.get_id()), 11_usize);

        // Blank line
        let blank_line = paragraph(format!(""), 11_usize);

        // Clone to prevent move, then reassign
        *docx = docx.clone().add_paragraph(ex).add_paragraph(st).add_paragraph(blank_line.clone());

        for (i, question) in qbank.get_questions().iter().enumerate()
        {
            let modum = header.get_category(question.get_category()).unwrap();
            let para = paragraph(format!("{}. [{}]   {}\n", i + 1, modum, question.get_question()), 11_usize);
            // Clone to prevent move, then reassign
            *docx = docx.clone().add_paragraph(para);
            for (j, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
            {
                let choice_char = (b'A' + j as u8) as char;
                let para = paragraph(format!("    ({}) {}", choice_char, choice_text), 11_usize);
                // Clone to prevent move, then reassign
                *docx = docx.clone().add_paragraph(para);
            }
            // Blank line after each question
            *docx = docx.clone().add_paragraph(blank_line.clone());
        }
        Ok(())
    }

    // pub fn save_shuffled_exams_in_pdf(&self, path: &Path) -> Result<(), String>
    /// Saves the shuffled exam sets to a PDF file.
    ///
    /// This function generates a PDF document containing the shuffled exam sets
    /// for all students, with a footer showing page numbers.
    ///
    /// # Arguments
    /// * `path` - The file path where the PDF document will be saved.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ QBank, Generator, Student, Students, Question };
    /// use std::fs;
    /// use std::path::Path;
    ///
    /// let mut qbank = QBank::new_empty();
    /// qbank.add_question(Question::new(1, 1, 1, "Question 1".to_string(), vec![]));
    /// qbank.add_question(Question::new(2, 2, 1, "Question 2".to_string(), vec![]));
    ///
    /// let student1 = Student::new_from_name("Alice".to_string());
    /// let students = Students::new(vec![student1]);
    ///
    /// let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    ///
    /// let result = generator.save_shuffled_exams_in_pdf(Path::new("exam.pdf"));
    /// assert!(result.is_ok());
    /// std::fs::remove_file("exam.pdf").unwrap();
    /// ```
    pub fn save_shuffled_exams_in_pdf(&self, path: &Path) -> Result<(), String>
    {
        // let font_style = style::
        let font_family = fonts::from_files("./fonts", "font", None).map_err(|e| format!("Failed to load font: {}", e))?;
        let mut doc = Document::new(font_family);
        // Set 1cm margins (10mm) and page numbers for all sides
        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(10); // 10mm = 1cm
        doc.set_page_decorator(decorator);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (idx, (student, qbank)) in shuffled_qbanks.iter().enumerate()
        {
            if idx > 0
                { doc.push(elements::PageBreak::new()); } // Page break for subsequent students
            self.write_exam_content_to_pdf(&mut doc, &student, &qbank)?;
        }

        // Add answer sheet
        doc.push(elements::PageBreak::new());
        let answer_style = style::Style::new().with_font_size(12);
        let answer_title_style = style::Style::new().with_font_size(14);

        let mut title_paragraph = elements::Paragraph::new("Answer Sheet        정답지        Ответы");
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(answer_title_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        let header = self.origin.get_header();

        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(),
                header.get_id(), student.get_id()
            );
            doc.push(elements::Paragraph::new(student_info_text).styled(answer_style));

            // Answers
            let mut answers_text = String::new();
            for (i, question) in qbank.get_questions().iter().enumerate() {
                let correct_choices: Vec<String> = question.get_choices()
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, is_correct))| *is_correct)
                    .map(|(j, _)| ((b'a' + j as u8) as char).to_string())
                    .collect();
                let answer_string = correct_choices.join(", ");
                answers_text.push_str(&format!("{}. {}    ", i + 1, answer_string));
            }
            doc.push(elements::Paragraph::new(answers_text).styled(answer_style));
            doc.push(elements::Paragraph::new("")); // Blank line
        }

        doc.render_to_file(path).map_err(|e| e.to_string())?;
        Ok(())
    }

    // fn write_exam_content_to_pdf(&self, doc: &mut genpdf::Document, student: &Student, qbank: &QBank) -> Result<(), String>
    /// Writes the formatted exam content for a single student to a PDF document.
    ///
    /// This private helper function takes a mutable PDF `genpdf::Document` object
    /// and appends the exam content for the given student and their shuffled
    /// question bank, applying PDF-specific formatting such as font sizes.
    ///
    /// # Arguments
    /// * `doc` - A mutable reference to the `genpdf::Document` object.
    /// * `student` - A reference to the `Student` for whom the exam content is being written.
    /// * `qbank` - A reference to the `QBank` containing the shuffled questions for this student.
    ///
    /// # Output
    /// `Result<(), String>` - Returns `Ok(())` on success, or an `Err` with a
    ///                        `String` describing the error on failure.
    fn write_exam_content_to_pdf(&self, doc: &mut genpdf::Document, student: &Student, qbank: &QBank) -> Result<(), String>
    {
        // Define font sizes
        let title_font_size = 14;
        let normal_font_size = 11;
        let header = qbank.get_header();

        // Exam Title
        doc.push(elements::Paragraph::new(format!("{}", header.get_title())).styled(style::Style::new().with_font_size(title_font_size)));

        // Student Information
        doc.push(elements::Paragraph::new(format!("{}: {}        {}: {}", header.get_name(), student.get_name(), header.get_id(), student.get_id())).styled(style::Style::new().with_font_size(normal_font_size)));
        doc.push(elements::Paragraph::new("")); // Blank line

        for (i, question) in qbank.get_questions().iter().enumerate()
        {
            let modum = header.get_category(question.get_category()).unwrap();
            doc.push(elements::Paragraph::new(format!("{}. [{}]   {}", i + 1, modum, question.get_question())).styled(style::Style::new().with_font_size(normal_font_size)));
            for (j, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
            {
                let choice_char = (b'A' + j as u8) as char;
                doc.push(elements::Paragraph::new(format!("    ({}) {}", choice_char, choice_text)).styled(style::Style::new().with_font_size(normal_font_size)));
            }
            doc.push(elements::Paragraph::new("")); // Blank line after each question
        }
        Ok(())
    }
}