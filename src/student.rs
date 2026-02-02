// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


/// Represents a student with a name and an ID.
#[derive(Clone)]
pub struct Student
{
    name: String,
    id: String,
}

impl Student
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `Student`.
    ///
    /// # Output
    /// `Self` - A new, empty `Student` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::Student;
    /// let student = Student::new_empty();
    /// assert_eq!(student.get_id(), "");
    /// assert_eq!(student.get_name(), "");
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        Self
        {
            name: String::new(),
            id: String::new(),
        }
    }

    // pub fn new(name: String, id: String) -> Self
    /// Creates a new `Student`.
    ///
    /// # Arguments
    /// * `name` - The name of the student.
    /// * `id` - The ID of the student.
    ///
    /// # Output
    /// `Self` - A new `Student` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::Student;
    /// let student = Student::new("John Doe".to_string(), "S123".to_string());
    /// assert_eq!(student.get_name(), "John Doe");
    /// assert_eq!(student.get_id(), "S123");
    /// ```
    pub fn new(name: String, id: String) -> Self
    {
        Student { name, id }
    }

    // pub fn get_name(&self) -> &String
    /// Gets the student's name.
    ///
    /// # Output
    /// `&String` - A reference to the student's name.
    ///
    /// # Examples
    /// ```
    /// use qrate::Student;
    /// let student = Student::new("Jane Doe".to_string(), "S456".to_string());
    /// assert_eq!(student.get_name(), "Jane Doe");
    /// ```
    pub fn get_name(&self) -> &String
    {
        &self.name
    }

    // pub fn set_name(&mut self, name: String)
    /// Sets the student's name.
    /// 
    /// # Arguments
    /// `name` is student's name and of the type &String.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }

    // pub fn get_id(&self) -> &String
    /// Gets the student's ID.
    ///
    /// # Output
    /// `&String` - A reference to the student's ID.
    ///
    /// # Examples
    /// ```
    /// use qrate::Student;
    /// let student = Student::new("Jane Doe".to_string(), "S456".to_string());
    /// assert_eq!(student.get_id(), "S456");
    /// ```
    pub fn get_id(&self) -> &String
    {
        &self.id
    }

    // pub fn set_id(&mut self, id: String)
    /// Sets the student's ID.
    ///
    /// # Output
    /// `&String` - A reference to the student's ID.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn set_id(&mut self, id: String)
    {
        self.id = id;
    }
}
