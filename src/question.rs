// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


pub type ChoiceAnswer = (String, bool);
pub type Choices = Vec<ChoiceAnswer>;

/// Represents a single question with its properties.
#[derive(Clone)]
pub struct Question
{
    id: u16,        // 1-based unique identifier. Should be in order as class progress
    group: u16,     // The questions that belong to the same group will not appear in an exam set.
    category: u8,   // 1-based category: 1 for single choice, 2 for multiple choice, 3 for short answer.
    question: String,   // The text of the question
    choices: Choices,   // For category 3, choice[0] or get_choice(1) is the answer.
}

impl Question
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `Question`.
    ///
    /// # Output
    /// `Self` - A new, empty `Question` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_id(), 0);
    /// assert_eq!(question.get_question(), "");
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        Self
        {
            id: 0,
            group: 0,
            category: 1,
            question: String::new(),
            choices: Choices::new(),
        }
    }

    // pub fn new(id: u16, category: u8, question: String, choices: Choices) -> Self
    /// Creates a new `Question` instance with the given properties.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the question.
    /// * `group` - The questions that belong to the same group will not appear in an exam set.
    /// * `category` - The category of the question (e.g., 1 for single choice, 2 for multiple choice).
    /// * `question` - The text of the question.
    /// * `choices` - A vector of `ChoiceAnswer` tuples for the question.
    ///
    /// # Output
    /// `Self` - A new `Question` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::{Question, Choices};
    /// let question = Question::new(1, 1, "What is Rust?".to_string(), vec![("A language".to_string(), true)]);
    /// assert_eq!(question.get_id(), 1);
    /// assert_eq!(question.get_question(), "What is Rust?");
    /// ```
    #[inline]
    pub fn new(id: u16, group: u16, category: u8, question: String, choices: Choices) -> Self
    {
        Self { id, group, category, question, choices }
    }

    // pub fn get_id(&self) -> u16
    /// Gets the ID of the question.
    ///
    /// # Output
    /// `u16` - The ID of the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_id(), 0);
    /// ```
    #[inline]
    pub fn get_id(&self) -> u16
    {
        self.id
    }

    // pub fn set_id(&mut self, id: u16)
    /// Sets the ID of the question.
    ///
    /// # Arguments
    /// * `id` - The new ID for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_id(5);
    /// assert_eq!(question.get_id(), 5);
    /// ```
    #[inline]
    pub fn set_id(&mut self, id: u16)
    {
        self.id = id;
    }

    // pub fn get_group(&self) -> u16
    /// Gets the group of the question.
    ///
    /// # Output
    /// `u16` - The group of the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_group(), 0);
    /// ```
    #[inline]
    pub fn get_group(&self) -> u16
    {
        self.group
    }

    // pub fn set_group(&mut self, group: u16)
    /// Sets the group of the question.
    ///
    /// # Arguments
    /// * `group` - The new group for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_group(2); // Multi-choice
    /// assert_eq!(question.get_group(), 2);
    /// ```
    #[inline]
    pub fn set_group(&mut self, group: u16)
    {
        self.group = group;
    }

    // pub fn get_category(&self) -> u8
    /// Gets the category of the question.
    ///
    /// # Output
    /// `u8` - The category of the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new_empty();
    /// assert_eq!(question.get_category(), 1);
    /// ```
    #[inline]
    pub fn get_category(&self) -> u8
    {
        self.category
    }

    // pub fn set_category(&mut self, category: u8)
    /// Sets the category of the question.
    ///
    /// # Arguments
    /// * `category` - The new category for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_category(2); // Multi-choice
    /// assert_eq!(question.get_category(), 2);
    /// ```
    #[inline]
    pub fn set_category(&mut self, category: u8)
    {
        self.category = category;
    }

    // pub fn get_question(&self) -> &String
    /// Gets a reference to the question text.
    ///
    /// # Output
    /// `&String` - A reference to the question text.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, 1, "Hello".to_string(), vec![]);
    /// assert_eq!(question.get_question(), "Hello");
    /// ```
    #[inline]
    pub fn get_question(&self) -> &String
    {
        &self.question
    }

    // pub fn set_question(&mut self, question: String)
    /// Sets the question text.
    ///
    /// # Arguments
    /// * `question` - The new text for the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_question("New Question Text".to_string());
    /// assert_eq!(question.get_question(), "New Question Text");
    /// ```
    #[inline]
    pub fn set_question(&mut self, question: String)
    {
        self.question = question;
    }

    // pub fn get_choice(&self, choice: usize) -> Option<&ChoiceAnswer>
    /// Gets a reference to a choice by its 1-based index.
    ///
    /// # Arguments
    /// * `choice` - The 1-based index of the choice to retrieve.
    ///
    /// # Output
    /// `Option<&ChoiceAnswer>` - An optional reference to the `ChoiceAnswer` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, "Q".to_string(), vec![("Opt A".to_string(), false), ("Opt B".to_string(), true)]);
    /// assert_eq!(question.get_choice(1).unwrap().0, "Opt A");
    /// assert!(question.get_choice(3).is_none());
    /// ```
    pub fn get_choice(&self, choice: usize) -> Option<&ChoiceAnswer>
    {
        if (choice <= self.choices.len()) && (choice > 0)
            { Some(&self.choices[choice - 1]) }
        else
            { None }
    }

    // pub fn push_choice(&mut self, choice: ChoiceAnswer)
    /// Adds a new choice to the question.
    ///
    /// # Arguments
    /// * `choice` - The `ChoiceAnswer` to add to the question.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.push_choice(("Option A".to_string(), false));
    /// assert_eq!(question.get_choices().len(), 1);
    /// ```
    #[inline]
    pub fn push_choice(&mut self, choice: ChoiceAnswer)
    {
        self.choices.push(choice);
    }

    // pub fn get_choices(&self) -> &Choices
    /// Gets a reference to the vector of choices.
    ///
    /// # Output
    /// `&Choices` - A reference to the vector of `ChoiceAnswer`s.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let question = Question::new(1, 1, "Q".to_string(), vec![("A".to_string(), false), ("B".to_string(), false)]);
    /// assert_eq!(question.get_choices().len(), 2);
    /// ```
    #[inline]
    pub fn get_choices(&self) -> &Choices
    {
        &self.choices
    }

    // pub fn set_choices(&mut self, choices: Choices)
    /// Sets the entire vector of choices.
    ///
    /// # Arguments
    /// * `choices` - The new vector of `ChoiceAnswer`s to set.
    ///
    /// # Examples
    /// ```
    /// use qrate::Question;
    /// let mut question = Question::new_empty();
    /// question.set_choices(vec![("New A".to_string(), false), ("New B".to_string(), false)]);
    /// assert_eq!(question.get_choices().len(), 2);
    /// ```
    #[inline]
    pub fn set_choices(&mut self, choices: Choices)
    {
        self.choices = choices;
    }
}
