// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::Student;

/// A type alias for a vector of `Student`s, representing a bank of students.
pub type SBank = Vec<Student>;

pub trait SBankHelper
{
    // fn get_student(&self, number: usize) -> Option<&Student>
    /// Gets a reference to a `Student` by its 1-based index.
    ///
    /// # Arguments
    /// * `number` - The 1-based index of the question to retrieve.
    ///
    /// # Output
    /// `Option<&Question>` - An optional reference to the `Student` at the specified index.
    ///
    /// # Examples
    /// ```
    /// ```
    fn get_student(&self, number: usize) -> Option<&Student>;

    // fn push_student(&mut self, student: Student)
    /// Adds a `Student` to the bank.
    ///
    /// # Arguments
    /// * `student` - The `Student` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// ```
    fn push_student(&mut self, student: Student);
}

impl SBankHelper for SBank
{
    fn get_student(&self, number: usize) -> Option<&Student>
    {
        if (number <= self.len()) && number > 0
            { Some(&self[number - 1]) }
        else
            { None }
    }

    #[inline]
    fn push_student(&mut self, student: Student)
    {
        self.push(student);
    }
}

