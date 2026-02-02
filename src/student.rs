use crate::get_bank;

/// Represents a student with a name and an ID.
#[derive(Clone)]
pub struct Student
{
    name: String,
    id: String,
}

impl Student
{
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

    // pub fn init_students() -> Vec<Student>
    /// Initializes a vector of `Student`s with default data.
    ///
    /// # Output
    /// `Vec<Student>` - A vector containing a predefined list of `Student` instances.
    ///
    /// # Examples
    /// ```
    /// use qrate::Student;
    /// let students = Student::init_students();
    /// assert!(!students.is_empty());
    /// assert_eq!(students.len(), 4); // Assuming 4 default students
    /// ```
    pub fn init_students() -> Vec<Student>
    {
        get_bank()
    }
}
