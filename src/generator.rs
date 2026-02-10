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
use genpdf::{ Document, elements, fonts, style, Element, SimplePageDecorator, Alignment };

use crate::{ Choices, QBank, Questions, check_path };
use crate::{ Students, Student };
use crate::{ ShuffledQSet, ShuffledQSets };


pub struct Generator
{
    origin: QBank,
    shuffled_qsets: ShuffledQSets,
    current_question_number: u16,
    title_font_size: f32,
    default_font_size: f32,
    footer_font_size: f32,
    answer_sheet_font_size: f32,
    margin_left_in_mm: f32,
    margin_right_in_mm: f32,
    margin_top_in_mm: f32,
    margin_buttom_in_mm: f32,
    line_spacing: f32,
    answer_sheet_title: String,
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
        Some(
            Self
            {
                origin: qbank.clone(),
                shuffled_qsets,
                current_question_number: 0,
                title_font_size: 14.0,
                default_font_size: 11.0,
                footer_font_size: 9.0,
                answer_sheet_font_size: 12.0,
                margin_left_in_mm: 10.0,
                margin_right_in_mm: 10.0,
                margin_top_in_mm: 10.0,
                margin_buttom_in_mm: 10.0,
                line_spacing: 1.0,
                answer_sheet_title: "Answer Sheet        정답지        Ответы".to_string()
             }
        )
    }

    // pub fn get_title_font_size(&self) -> f32
    /// Retrieves the current title font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_title_font_size();
    /// assert_eq!(font_size, 14.0);
    /// ```
    #[inline]
    pub fn get_title_font_size(&self) -> f32
    {
        self.title_font_size
    }
    
    // pub fn set_title_font_size(&mut self, title_font_size: f32)
    /// Sets the title font siz in points.
    ///
    /// # Arguments
    /// * `title_font_size` - The new font size to be used for titles.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_title_font_size(16.0);
    /// assert_eq!(generator.get_title_font_size(), 16.0);
    /// ```
    #[inline]
    pub fn set_title_font_size(&mut self, title_font_size: f32)
    {
        self.title_font_size = title_font_size;
    }
    
    // pub fn get_default_font_size(&self) -> f32
    /// Retrieves the current default font size in points.
    ///
    /// # Output
    /// `f32` - The current default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_default_font_size();
    /// assert_eq!(font_size, 11.0);
    /// ```
    #[inline]
    pub fn get_default_font_size(&self) -> f32
    {
        self.default_font_size
    }
    
    // pub fn set_default_font_size(&mut self, default_font_size: f32)
    /// Sets the default font size in points.
    ///
    /// # Arguments
    /// * `default_font_size` - The new default font size.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_default_font_size(12.0);
    /// assert_eq!(generator.get_default_font_size(), 12.0);
    /// ```
    #[inline]
    pub fn set_default_font_size(&mut self, default_font_size: f32)
    {
        self.default_font_size = default_font_size
    }
    
    // pub fn get_footer_font_size(&self) -> f32
    /// Retrieves the current footer font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_footer_font_size();
    /// assert_eq!(font_size, 9.0);
    /// ```
    #[inline]
    pub fn get_footer_font_size(&self) -> f32
    {
        self.footer_font_size
    }
    
    // pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    /// Sets the footer font size in points.
    ///
    /// # Arguments
    /// * `footer_font_size` - The new font size to be used for footers.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_footer_font_size(10.0);
    /// assert_eq!(generator.get_footer_font_size(), 10.0);
    /// ```
    #[inline]
    pub fn set_footer_font_size(&mut self, footer_font_size: f32)
    {
        self.footer_font_size = footer_font_size
    }
    
    // pub fn get_answer_sheet_font_size(&self) -> f32
    /// Retrieves the current answer sheet font size in points.
    ///
    /// # Output
    /// `f32` - The current font size used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let font_size = generator.get_answer_sheet_font_size();
    /// assert_eq!(font_size, 12.0);
    /// ```
    #[inline]
    pub fn get_answer_sheet_font_size(&self) -> f32
    {
        self.answer_sheet_font_size
    }
    
    // pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    /// Sets the answer sheet font size in points.
    ///
    /// # Arguments
    /// * `answer_sheet_font_size` - The new font size to be used for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_font_size(13.0);
    /// assert_eq!(generator.get_answer_sheet_font_size(), 13.0);
    /// ```
    #[inline]
    pub fn set_answer_sheet_font_size(&mut self, answer_sheet_font_size: f32)
    {
        self.answer_sheet_font_size = answer_sheet_font_size;
    }
    
    // pub fn get_margin_left_in_mm(&self) -> f32
    /// Retrieves the current left margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_left_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_left_in_mm(&self) -> f32
    {
        self.margin_left_in_mm
    }
    
    // pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    /// Sets the left margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_left_in_mm` - The new left margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_left_in_mm(15.0);
    /// assert_eq!(generator.get_margin_left_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_left_in_mm(&mut self, margin_left_in_mm: f32)
    {
        self.margin_left_in_mm = margin_left_in_mm;
    }
    
    // pub fn get_margin_right_in_mm(&self) -> f32
    /// Retrieves the current right margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_right_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_right_in_mm(&self) -> f32
    {
        self.margin_right_in_mm
    }
    
    // pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    /// Sets the right margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_right_in_mm` - The new right margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_right_in_mm(15.0);
    /// assert_eq!(generator.get_margin_right_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_right_in_mm(&mut self, margin_right_in_mm: f32)
    {
        self.margin_right_in_mm = margin_right_in_mm;
    }
    
    // pub fn get_margin_top_in_mm(&self) -> f32
    /// Retrieves the current top margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_top_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_top_in_mm(&self) -> f32
    {
        self.margin_top_in_mm
    }
    
    // pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    /// Sets the top margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_top_in_mm` - The new top margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_top_in_mm(15.0);
    /// assert_eq!(generator.get_margin_top_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_top_in_mm(&mut self, margin_top_in_mm: f32)
    {
        self.margin_top_in_mm = margin_top_in_mm;
    }
    
    // pub fn get_margin_buttom_in_mm(&self) -> f32
    /// Retrieves the current bottom margin in millimeters.
    ///
    /// # Output
    /// `f32` - The current bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let margin = generator.get_margin_buttom_in_mm();
    /// assert_eq!(margin, 10.0);
    /// ```
    #[inline]
    pub fn get_margin_buttom_in_mm(&self) -> f32
    {
        self.margin_buttom_in_mm
    }
    
    // pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    /// Sets the bottom margin in millimeters.
    ///
    /// # Arguments
    /// * `margin_buttom_in_mm` - The new bottom margin in millimeters.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_margin_buttom_in_mm(15.0);
    /// assert_eq!(generator.get_margin_buttom_in_mm(), 15.0);
    /// ```
    #[inline]
    pub fn set_margin_buttom_in_mm(&mut self, margin_buttom_in_mm: f32)
    {
        self.margin_buttom_in_mm = margin_buttom_in_mm;
    }
    
    // pub fn get_line_spacing(&self) -> f32
    /// Retrieves the current line spacing in lines.
    ///
    /// # Output
    /// `f32` - The current line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let spacing = generator.get_line_spacing();
    /// assert_eq!(spacing, 1.0);
    /// ```
    #[inline]
    pub fn get_line_spacing(&self) -> f32
    {
        self.line_spacing
    }
    
    // pub fn set_line_spacing(&mut self, line_spacing: f32)
    /// Sets the line spacing in lines.
    ///
    /// # Arguments
    /// * `line_spacing` - The new line spacing value.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_line_spacing(1.5);
    /// assert_eq!(generator.get_line_spacing(), 1.5);
    /// ```
    #[inline]
    pub fn set_line_spacing(&mut self, line_spacing: f32)
    {
        self.line_spacing = line_spacing;
    }
    
    // pub fn get_answer_sheet_title(&self) -> String
    /// Retrieves the current answer sheet title.
    ///
    /// # Output
    /// `String` - The current title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// let title = generator.get_answer_sheet_title();
    /// assert_eq!(title, "Answer Sheet        정답지        Ответы".to_string());
    /// ```
    #[inline]
    pub fn get_answer_sheet_title(&self) -> String
    {
        self.answer_sheet_title.clone()
    }
    
    // pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    /// Sets the answer sheet title.
    ///
    /// # Arguments
    /// * `answer_sheet_title` - The new title for the answer sheet.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Generator };
    ///
    /// let qbank = QBank::new_empty();
    /// let mut generator = Generator::new_one_set(&qbank, 1, 1, 1).unwrap();
    /// generator.set_answer_sheet_title("New Answer Sheet Title".to_string());
    /// assert_eq!(generator.get_answer_sheet_title(), "New Answer Sheet Title".to_string());
    /// ```
    #[inline]
    pub fn set_answer_sheet_title(&mut self, answer_sheet_title: String)
    {
        self.answer_sheet_title = answer_sheet_title;
    }

    // // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    // // Retrieves a specific shuffled question set by its index.
    // //
    // // This function returns a cloned `ShuffledQSet` for the given index,
    // // if the index is within the bounds of the generated shuffled sets.
    // //
    // // # Arguments
    // // * `idx` - The zero-based index of the shuffled question set to retrieve.
    // //
    // // # Output
    // // An `Option<ShuffledQSet>` which is `Some(ShuffledQSet)` if the index is valid,
    // // or `None` if the index is out of bounds.
    // //
    // // # Examples
    // // ```
    // // use qrate::{ QBank, Generator, Student, Students };
    // //
    // // let mut qbank = QBank::new_empty();
    // // qbank.add_question("Question 1".to_string(), "Answer 1".to_string());
    // // qbank.add_question("Question 2".to_string(), "Answer 2".to_string());
    // //
    // // let student1 = Student::new_from_name("Alice".to_string());
    // // let students = Students::new(vec![student1]);
    // //
    // // let generator = Generator::new(&qbank, 1, 2, 1, &students).unwrap();
    // // let shuffled_qset = generator.get_shuffled_qset(0);
    // // assert!(shuffled_qset.is_some());
    // // let no_shuffled_qset = generator.get_shuffled_qset(1);
    // // assert!(no_shuffled_qset.is_none());
    // // ```
    // #[inline]
    // pub(crate) fn get_shuffled_qset(&self, idx: usize) -> Option<ShuffledQSet>
    // {
    //     if idx < self.shuffled_qsets.len() { Some(self.shuffled_qsets[idx].clone()) } else { None }
    // }

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
                { writeln!(file, "-------X------- CUT -------X------- 자르기 -------X------- резать -------X-------\n\n").map_err(|e| e.to_string())?; }
        }
        // Add a separator for the answer sheet
        write!(file, "\n\u{000C}\n").map_err(|e| e.to_string())?; // Form feed for page break

        let header = self.origin.get_header(); // Need the original header for titles
        writeln!(file, "{}{}", self.answer_sheet_title, "\n").map_err(|e| e.to_string())?;
        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            writeln!(file, "{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
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
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        let linespacing_to_twips = |linespacing: f32| -> i32 { (linespacing * 240.0) as i32 };
        let footer_font_size = pt_to_usize(self.footer_font_size);
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
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(
                        Run::new()
                            .add_text(" / ")
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::NUMPAGES(InstrNUMPAGES::default()))
                            .add_field_char(FieldCharType::Separate, false)
                            .add_text("1") // Placeholder text
                            .add_field_char(FieldCharType::End, false)
                            .size(footer_font_size)   // 9 pt for default
                    )
                    .align(AlignmentType::Center)
            );
        let mm_to_twips = |mm: f32| -> i32  { (mm * 56.6929).round() as i32 };
        let left = mm_to_twips(self.margin_left_in_mm);
        let right = mm_to_twips(self.margin_right_in_mm);
        let top = mm_to_twips(self.margin_top_in_mm);
        let buttom = mm_to_twips(self.margin_buttom_in_mm);
        let mut docx = Docx::new()
                        .page_margin(
                            PageMargin::new()
                                .left(left)
                                .right(right)
                                .top(top)
                                .bottom(buttom)
            ) // 1 cm for default left, right, top, bottom
            .footer(footer);
        let shuffled_qbanks = self.get_shuffled_qbanks();

        for (idx, (student, qbank)) in shuffled_qbanks.iter().enumerate()
        {
            if idx > 0
                { docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page))); } // Page break for subsequent students
            self.write_exam_content_to_docx(&mut docx, &student, &qbank)?;
        }

        // Add answer sheet
        let title_font_size = pt_to_usize(self.title_font_size);
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)));
        docx = docx.add_paragraph(Paragraph::new()
                                    .add_run(
                                        Run::new()
                                            .add_text(self.answer_sheet_title.as_str())
                                            .size(title_font_size)
                                        )
                                        .align(AlignmentType::Center)); // 14 pt for default font size
        docx = docx.add_paragraph(Paragraph::new()); // Blank line

        let header = self.origin.get_header();
        let line_spacing = linespacing_to_twips(self.line_spacing);
        for (student, qbank) in &shuffled_qbanks
        {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
            );
            let student_info_paragraph = Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(student_info_text)
                        .size(footer_font_size)) // 9 pt for default
                .line_spacing(docx_rs::LineSpacing::new().line(line_spacing));   // Single line spacing
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

            let answer_sheet_font_size = pt_to_usize(self.answer_sheet_font_size);
            let answers_paragraph = Paragraph::new()
                                        .add_run(Run::new()
                                                    .add_text(answers_text)
                                                    .size(answer_sheet_font_size)
                                                ) // 12 pt for default answer sheet font size
                                        .line_spacing(docx_rs::LineSpacing::new().line(line_spacing));   // Single line spacing
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
        let pt_to_usize = |pt: f32| -> usize { (pt as usize) << 1 };
        let default_font_size = pt_to_usize(self.default_font_size);
        let title_font_size = pt_to_usize(self.title_font_size);
        let paragraph = |txt, size| -> Paragraph
        {
            let elem = Run::new().add_text(txt).size(size);  // `size` pt
            Paragraph::new().add_run(elem)
        };
        let header = qbank.get_header();

        // Exam Title
        let ex = paragraph(format!("{}", header.get_title()), title_font_size);

        // Student Information
        let st = paragraph(format!("{}: {}        {}: {}\n\n", header.get_name(), student.get_name(), header.get_id(), student.get_id()), default_font_size);

        // Blank line
        let blank_line = paragraph(format!(""), default_font_size);

        // Clone to prevent move, then reassign
        *docx = docx.clone().add_paragraph(ex).add_paragraph(st).add_paragraph(blank_line.clone());

        for (i, question) in qbank.get_questions().iter().enumerate()
        {
            let modum = header.get_category(question.get_category()).unwrap();
            let para = paragraph(format!("{}. [{}]   {}\n", i + 1, modum, question.get_question()), default_font_size);
            // Clone to prevent move, then reassign
            *docx = docx.clone().add_paragraph(para);
            for (j, (choice_text, _is_correct)) in question.get_choices().iter().enumerate()
            {
                let choice_char = (b'A' + j as u8) as char;
                let para = paragraph(format!("    ({}) {}", choice_char, choice_text), default_font_size);
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
    /// `String` describing the error on failure.
    ///
    /// # Caution
    ///
    /// This method searches for four specific font files within a `./fonts` 
    /// subdirectory relative to the current working directory.
    ///
    /// The following files must be present for the function to operate correctly:
    /// * `font-Regular.ttf`
    /// * `font-Italic.ttf`
    /// * `font-Bold.ttf`
    /// * `font-BoldItalic.ttf`
    ///
    /// If the directory or any of these files are missing, the function will fail. 
    /// Ensure that the `fonts` directory is created and all four files are 
    /// correctly named before calling this method.
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
        let font_family = fonts::from_files("./fonts", "font", None).map_err(|e| format!("Failed to load font: {}", e))?;
        let mut doc = Document::new(font_family);
        // Set 1cm margins (10mm) and page numbers for all sides
        let mut decorator = SimplePageDecorator::new();
        let margin = (self.margin_left_in_mm + self.margin_right_in_mm + self.margin_top_in_mm + self.margin_buttom_in_mm) / 4.0;
        decorator.set_margins(margin); // 10mm = 1cm
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
        let answer_style = style::Style::new().with_font_size(self.answer_sheet_font_size as u8);
        let answer_title_style = style::Style::new().with_font_size(self.title_font_size as u8);

        let mut title_paragraph = elements::Paragraph::new(self.answer_sheet_title.clone());
        title_paragraph.set_alignment(Alignment::Center);
        doc.push(title_paragraph.styled(answer_title_style));
        doc.push(elements::Paragraph::new("")); // Blank line

        let header = self.origin.get_header();

        for (student, qbank) in &shuffled_qbanks {
            // Student Info
            let student_info_text = format!("{}: {}        {}: {}",
                header.get_name(), student.get_name(), header.get_id(), student.get_id()
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
        let title_font_size = self.title_font_size as u8;       // 14 pt for default
        let normal_font_size = self.default_font_size as u8;    // 11 pt for default
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