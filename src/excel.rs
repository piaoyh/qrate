use calamine::{ DataType, Reader, open_workbook_auto };
use rust_xlsxwriter::{ Format, FormatBorder, Workbook };

use crate::Header;
use crate::QBank;
use crate::Question;

/// Represents an Excel file for question bank operations.
///
/// This struct provides methods to read from and write to `.xlsx` files,
/// structuring the data into "header" and "bank" sheets.
pub struct Excel
{
    path: String,
}

impl Excel
{
    // pub fn open(path: String) -> Option<Self>
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
    /// let excel_handler = Excel::open("my_quiz.xlsx".to_string());
    /// assert!(excel_handler.is_some());
    /// assert_eq!(excel_handler.unwrap().get_path(), "my_quiz.xlsx");
    /// ```
    pub fn open(path: String) -> Option<Self>
    {
        let p = match path.find('.')
        {
            Some(_) => path,
            None => path + ".xlsx", // Default extension is .xlsx
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
    /// let excel_handler = Excel::open("my_document.xlsx".to_string()).unwrap();
    /// assert_eq!(excel_handler.get_path(), "my_document.xlsx");
    /// ```
    pub fn get_path(&self) -> &String
    {
        &self.path
    }

    // pub fn write_question_bank(&self, qbank: &QBank) -> Result<(), String>
    /// Writes a `QBank` to the Excel file.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to be written.
    ///
    /// # Output
    /// `Ok(())` on success, or an error message string on failure.
    /// 
    /// # Features
    /// The data is written into two sheets:
    /// 1.  `header`: Contains metadata like title, author, etc.
    /// 2.  `bank`: Contains the list of questions.
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ Excel, QBank, Question, Header };
    /// use std::fs;
    ///
    /// // 1. Set up a temporary file path
    /// let temp_dir = std::env::temp_dir();
    /// let file_path = temp_dir.join("test_quiz.xlsx");
    /// let path_str = file_path.to_str().unwrap().to_string();
    ///
    /// // 2. Create a sample QBank
    /// let mut qbank = QBank::new_with_Default();
    /// let question = Question::new(1, 1, 1, "What is Rust?".to_string(), vec!["A language".to_string()]);
    /// qbank.push_question(question);
    ///
    /// // 3. Write the QBank to the file
    /// let excel_handler = Excel::open(path_str.clone()).unwrap();
    /// let write_result = excel_handler.write_question_bank(&qbank);
    /// assert!(write_result.is_ok());
    ///
    /// // 4. (Demonstrated in `read_question_bank` example) Read and verify
    ///
    /// // 5. Clean up the temporary file
    /// fs::remove_file(path_str).unwrap();
    /// ```
    pub fn write_question_bank(&self, qbank: &QBank) -> Result<(), String>
    {
        let mut workbook = Workbook::new();

        // 1. Create "header" sheet
        let header_sheet = workbook.add_worksheet().set_name("header").map_err(|e| e.to_string())?;
        let header_data = qbank.get_header();

        // Style settings
        let border_format = Format::new().set_border(FormatBorder::Thin);
        let bold_border_format = Format::new().set_bold().set_border(FormatBorder::Thin);

        header_sheet.write_string_with_format(0, 0, "Title", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(0, 1, header_data.get_title(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 0, "Name", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 1, header_data.get_name(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 1, header_data.get_id(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 0, "Notice", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 1, header_data.get_notice(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(4, 0, "Categories", &bold_border_format).map_err(|e| e.to_string())?;
        for (i, cat) in header_data.get_categories().iter().enumerate()
            { header_sheet.write_string_with_format(4, i as u16 + 1, cat, &border_format).map_err(|e| e.to_string())?; }

        // 2. Create "bank" sheet
        let bank_sheet = workbook.add_worksheet().set_name("bank").map_err(|e| e.to_string())?;
        let questions = qbank.get_bank();

        // Determine the maximum number of choices in the entire bank
        let max_choices = questions.iter().map(|q| q.get_choices().len()).max().unwrap_or(0);

        // Write fixed header row part
        bank_sheet.write_string_with_format(0, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        bank_sheet.write_string_with_format(0, 1, "Category", &bold_border_format).map_err(|e| e.to_string())?;
        bank_sheet.write_string_with_format(0, 2, "Question", &bold_border_format).map_err(|e| e.to_string())?;

        // Dynamically generate Choice and IsAnswer headers
        let mut current_col = 3;
        for i in 1..=max_choices
        {
            bank_sheet.write_string_with_format(0, current_col, &format!("Choice{}", i), &bold_border_format).map_err(|e| e.to_string())?;
            current_col += 1;
            bank_sheet.write_string_with_format(0, current_col, &format!("IsAnswer{}", i), &bold_border_format).map_err(|e| e.to_string())?;
            current_col += 1;
        }

        // Write question data
        for (row_idx, question) in questions.iter().enumerate()
        {
            let current_row = (row_idx + 1) as u32; // Start after the header
            bank_sheet.write_number_with_format(current_row, 0, question.get_id() as f64, &border_format).map_err(|e| e.to_string())?;
            bank_sheet.write_number_with_format(current_row, 1, question.get_category() as f64, &border_format).map_err(|e| e.to_string())?;
            bank_sheet.write_string_with_format(current_row, 2, question.get_question(), &border_format).map_err(|e| e.to_string())?;

            let mut current_col_for_choices = 3;
            for (choice_text, is_answer) in question.get_choices().iter()
            {
                bank_sheet.write_string_with_format(current_row, current_col_for_choices, choice_text, &border_format).map_err(|e| e.to_string())?;
                current_col_for_choices += 1;
                bank_sheet.write_string_with_format(current_row, current_col_for_choices, &is_answer.to_string().to_uppercase(), &border_format).map_err(|e| e.to_string())?;
                current_col_for_choices += 1;
            }

            // Fill remaining columns if `max_choices` is greater than current question's choices
            for _ in question.get_choices().len()..max_choices
            {
                bank_sheet.write_string_with_format(current_row, current_col_for_choices, "", &border_format).map_err(|e| e.to_string())?;
                current_col_for_choices += 1;
                bank_sheet.write_string_with_format(current_row, current_col_for_choices, "FALSE", &border_format).map_err(|e| e.to_string())?; // Default to FALSE for empty choices
                current_col_for_choices += 1;
            }
        }

        // Save file
        workbook.save(&self.path).map_err(|e| e.to_string())?;
        Ok(())
    }

    // pub fn read_question_bank(&self) -> Result<QBank, String>
    /// Reads a `QBank` from the Excel file.
    ///
    /// # Output
    /// A `Result<QBank, String>` which is `Ok(QBank)` on success,
    /// or an error message string on failure (e.g., file not found, sheet missing, parsing error).
    /// 
    /// # Features
    /// It expects the data to be in two sheets: "header" and "bank".
    ///
    /// # Examples
    /// ```no_run
    /// use qrate::{ Excel, QBank, Question, Header };
    /// use std::fs;
    ///
    /// // 1. Set up a temporary file path and sample QBank
    /// let temp_dir = std::env::temp_dir();
    /// let file_path = temp_dir.join("test_quiz_read.xlsx");
    /// let path_str = file_path.to_str().unwrap().to_string();
    /// let mut qbank_to_write = QBank::new_with_Default();
    /// let question = Question::new(1, 1, 1, "What is Rust?".to_string(), vec!["A language".to_string(), "A fungus".to_string(), "A mineral".to_string(), "An animal".to_string()]);
    /// qbank_to_write.push_question(question);
    ///
    /// // 2. Write the QBank to the file first
    /// let excel_handler_write = Excel::open(path_str.clone()).unwrap();
    /// excel_handler_write.write_question_bank(&qbank_to_write).unwrap();
    ///
    /// // 3. Read the QBank back
    /// let excel_handler_read = Excel::open(path_str.clone()).unwrap();
    /// let read_result = excel_handler_read.read_question_bank();
    /// assert!(read_result.is_ok());
    /// let qbank_read = read_result.unwrap();
    ///
    /// // 4. Verify the data
    /// assert_eq!(qbank_to_write.get_header().get_title(), qbank_read.get_header().get_title());
    /// assert_eq!(qbank_read.get_bank().len(), 1);
    /// assert_eq!(qbank_read.get_question(1).unwrap().get_question(), "What is Rust?");
    ///
    /// // 5. Clean up the temporary file
    /// fs::remove_file(path_str).unwrap();
    /// ```
    pub fn read_question_bank(&self) -> Result<QBank, String>
    {
        let mut excel = open_workbook_auto(&self.path).map_err(|e| format!("Failed to open workbook '{}': {}", &self.path, e))?;
        let mut qbank = QBank::new_empty();

        // 1. Read "header" sheet
        let sheet_names = excel.sheet_names();
        let header_sheet_idx = sheet_names.iter().position(|s| s == "header").ok_or_else(|| "Sheet 'header' not found".to_string())?;
        let range: calamine::Range<calamine::Data> = excel.worksheet_range_at(header_sheet_idx).ok_or_else(|| "Sheet 'header' not found".to_string())?.map_err(|e| format!("Error reading sheet 'header': {}", e))?;
        let mut header_data = Header::new_empty();
        // Assuming fixed key-value pairs in the header sheet
        header_data.set_title(range.get((0, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default());
        header_data.set_name(range.get((1, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default());
        header_data.set_id(range.get((2, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default());
        header_data.set_notice(range.get((3, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default());

        // Categories can span multiple cells
        let mut categories = Vec::new();
        // Assuming categories start from row 4, column 1
        for col_idx in 1..
        {
            // Start from column index 1
            if let Some(cell_value) = range.get((4, col_idx))
            {
                if let Some(cat) = (cell_value as &calamine::Data).as_string()
                    { categories.push(cat); }
            }
            else
            {
                break; // No more categories
            }
        }
        header_data.set_categories(categories);
        qbank.set_header(header_data);

        // 2. Read "bank" sheet
        let sheet_names = excel.sheet_names();
        let bank_sheet_idx = sheet_names.iter().position(|s| s == "bank").ok_or_else(|| "Sheet 'bank' not found".to_string())?;
        let range: calamine::Range<calamine::Data> = excel.worksheet_range_at(bank_sheet_idx).ok_or_else(|| "Sheet 'bank' not found".to_string())?.map_err(|e| format!("Error reading sheet 'bank': {}", e))?;

        let header_row = range.rows().next().ok_or("Bank sheet is empty, no header row found".to_string())?;
        let mut num_choices_in_excel = 0;
        // Start checking from column 3 (after ID, Category, Question)
        for col_idx in (3..header_row.len()).step_by(2)
        {
            if let Some(cell_value) = header_row.get(col_idx)
            {
                if cell_value.as_string().map_or(false, |s| s.starts_with("Choice"))
                    { num_choices_in_excel += 1; }
                else
                    { break; }
            }
            else
            {
                break;
            }
        }

        // Minimum required columns: ID, Category, Question (3 fixed columns)
        let min_expected_cols = 3;
        // if there are choices, minimum columns should include at least one choice and one is_answer
        let min_expected_cols_with_choices = if num_choices_in_excel > 0 { min_expected_cols + 2 } else { min_expected_cols };
        for (row_idx, row_data) in range.rows().enumerate().skip(1)
        {
            // Skip header row
            if row_data.len() < min_expected_cols_with_choices
                { return Err(format!("Row {} has too few columns for a question in 'bank' sheet. Expected at least {} columns, but found {}.", row_idx + 1, min_expected_cols_with_choices, row_data.len())); }

            let id = row_data[0].get_int().ok_or_else(|| format!("Row {} (ID): Invalid ID format", row_idx + 1)).and_then(|i| Ok(i as u16))?;
            let category = row_data[1].get_int().ok_or_else(|| format!("Row {} (Category): Invalid Category format", row_idx + 1)).and_then(|i| Ok(i as u8))?;
            let question_text = row_data[2].as_string().ok_or_else(|| format!("Row {} (Question): Invalid Question text format", row_idx + 1))?.to_string();

            let mut choices: Vec<(String, bool)> = Vec::new();
            let mut current_choice_col = 3; // Choices start after ID, Category, Question

            for _ in 0..num_choices_in_excel
            {
                if current_choice_col + 1 < row_data.len()
                {
                    let choice_text = row_data[current_choice_col].as_string().unwrap_or_default(); // Default to empty string if not found
                    let is_answer_str = row_data[current_choice_col + 1].as_string().unwrap_or_default();
                    let is_answer = is_answer_str.eq_ignore_ascii_case("TRUE");

                    // Only add if choice has content or is marked as answer, or if it's not the last choice and the next choice has content
                    // This prevents adding a bunch of empty choices if max_choices was determined by a question with many choices
                    // but the current question has fewer valid choices.
                    if !choice_text.is_empty() || is_answer
                        { choices.push((choice_text, is_answer)); }
                    current_choice_col += 2;
                }
                else
                {
                    break; // No more choice columns in this row
                }
            }
            let question = Question::new(id, category, question_text, choices);
            qbank.push_question(question);
        }

        Ok(qbank)
    }
}
