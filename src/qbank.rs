// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::Header;
use crate::Question;

/// Represents a Question Bank, containing a header and a vector of questions.
#[derive(Clone)]
pub struct QBank
{
    header: Header,
    questions: Vec<Question>,
}

impl QBank
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `QBank` with an empty header.
    ///
    /// # Output
    /// `Self` - A new, empty `QBank` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_empty();
    /// assert!(qbank.get_questions().is_empty());
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        QBank
        {
            header: Header::new_empty(),
            questions: Vec::new(),
        }
    }

    // pub fn new_with_default() -> Self
    /// Creates a new `QBank` with a default header.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with a default header.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn new_with_default() -> Self
    {
        QBank
        {
            header: Header::new_with_default(),
            questions: Vec::new(),
        }
    }

    // pub fn new_with_header(header: Header) -> Self
    /// Creates a new `QBank` with a provided `Header`.
    ///
    /// # Arguments
    /// * `header` - The `Header` to be used for the new `QBank`.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with the specified header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let custom_header = Header::new_empty();
    /// let qbank = QBank::new_with_header(custom_header);
    /// assert!(qbank.get_questions().is_empty());
    /// ```
    #[inline]
    pub fn new_with_header(header: Header) -> Self
    {
        QBank
        {
            header,
            questions: Vec::new(),
        }
    }

    // pub fn get_header(&self) -> &Header
    /// Gets a reference to the `Header`.
    ///
    /// # Output
    /// `&Header` - A reference to the `Header` of the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn get_header(&self) -> &Header
    {
        &self.header
    }

    // pub fn set_header(&mut self, header: Header)
    /// Sets the `Header`.
    ///
    /// # Arguments
    /// * `header` - The new `Header` to set for the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let mut qbank = QBank::new_empty();
    /// let mut new_header = Header::new_empty();
    /// new_header.set_title("My Custom Exam".to_string());
    /// qbank.set_header(new_header);
    /// assert_eq!(qbank.get_header().get_title(), "My Custom Exam");
    /// ```
    #[inline]
    pub fn set_header(&mut self, header: Header)
    {
        self.header = header;
    }

    // pub fn get_questions(&self) -> &Vec<Question>
    /// Gets a reference to the vector of `Question`s.
    ///
    /// # Output
    /// `&Vec<Question>` - A reference to the vector of `Question`s in the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new_empty());
    /// assert_eq!(qbank.get_questions().len(), 1);
    /// ```
    #[inline]
    pub fn get_questions(&self) -> &Vec<Question>
    {
        &self.questions
    }

    // pub fn set_questions(&mut self, questions: Vec<Question>)
    /// Sets the vector of `Question`s.
    ///
    /// # Arguments
    /// * `questions` - The new vector of `Question`s to set for the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.set_questions(vec![Question::new_empty(), Question::new_empty()]);
    /// assert_eq!(qbank.get_questions().len(), 2);
    /// ```
    #[inline]
    pub fn set_questions(&mut self, questions: Vec<Question>)
    {
        self.questions = questions;
    }

    // pub fn get_question(&self, q_number: usize) -> Option<&Question>
    /// Gets a reference to a `Question` by its 1-based index.
    ///
    /// # Arguments
    /// * `q_number` - The 1-based index of the question to retrieve.
    ///
    /// # Output
    /// `Option<&Question>` - An optional reference to the `Question` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, 1, "Test Q".to_string(), vec![]));
    /// assert_eq!(qbank.get_question(1).unwrap().get_id(), 1);
    /// assert!(qbank.get_question(2).is_none());
    /// ```
    pub fn get_question(&self, q_number: usize) -> Option<&Question>
    {
        if (q_number <= self.questions.len()) && q_number > 0
            { Some(&self.questions[q_number - 1]) }
        else
            { None }
    }

    // pub fn push_question(&mut self, question: Question)
    /// Adds a `Question` to the bank.
    ///
    /// # Arguments
    /// * `question` - The `Question` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new_empty();
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_questions().len(), 1);
    /// ```
    #[inline]
    pub fn push_question(&mut self, question: Question)
    {
        self.questions.push(question);
    }

    // pub fn get_choice(&self, q_number: usize, ch_number: usize) -> Option<&String>
    /// Gets a reference to a choice `String` by question number and choice number (both 1-based).
    ///
    /// # Arguments
    /// * `q_number` - The 1-based index of the question.
    /// * `ch_number` - The 1-based index of the choice within the question.
    ///
    /// # Output
    /// `Option<&String>` - An optional reference to the choice string.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new(1, 1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]);
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_choice(1, 1).unwrap(), "Choice A");
    /// assert!(qbank.get_choice(1, 3).is_none());
    /// assert!(qbank.get_choice(2, 1).is_none());
    /// ```
    pub fn get_choice(&self, q_number: usize, ch_number: usize) -> Option<&String>
    {
        if (q_number <= self.questions.len()) && q_number > 0
            { self.questions[q_number - 1].get_choice(ch_number).map(|(text, _is_answer)| text) }
        else
            { None }
    }

    
    #[inline]
    pub fn get_max_choices(&self) -> usize
    {
        self.get_questions().iter().map(|q| q.get_choices().len()).max().unwrap_or(0)
    }
}
