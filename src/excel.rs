
/// Represents an Excel file for question bank operations.
///
/// This struct provides methods to read from and write to `.xlsx` files,
/// structuring the data into "header" and "bank" sheets.
pub struct Excel
{
    /// The path to the Excel file.
    pub(crate) path: String,
}

impl Excel
{
    // pub(crate) fn open(path: String, extention: &str) -> Option<Self>
    /// Creates a new `Excel` instance with a given path.
    ///
    /// # Arguments
    /// * `path` - The file path for the Excel workbook.
    ///
    /// # Output
    /// An `Option<Self>` containing the `Excel` instance.
    /// 
    /// # Features
    /// If the path does not have an extension, `.xlsx` is appended.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let excel_handler = Excel::open("my_quiz.xlsx".to_string(), ".qb.xlsx");
    /// assert!(excel_handler.is_some());
    /// assert_eq!(excel_handler.unwrap().get_path(), "my_quiz.xlsx");
    /// ```
    pub fn open(path: String, extention: &str) -> Option<Self>
    {
        let p = match path.find('.')
        {
            Some(_) => path,
            None => path + extention,
        };
        Some(Self { path: p })
    }

    // pub fn get_path(&self) -> &String
    /// Gets the path of the Excel file.
    ///
    /// # Output
    /// `&String` - A reference to the path of the Excel file.
    ///
    /// # Examples
    /// ```
    /// use qrate::Excel;
    ///
    /// let db = Excel { path: "my.qb.xlsx".to_string() };
    /// assert_eq!(db.get_path(), "my.qb.xlsx");
    /// ```
    pub fn get_path(&self) -> &String
    {
        &self.path
    }
}
