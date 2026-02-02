use calamine::{ DataType, Reader, open_workbook_auto };
use rust_xlsxwriter::{ Format, FormatBorder, Workbook };

use crate::Header;
use crate::QBank;
use crate::SQLiteDB;
use crate::Excel;
use crate::{ Choices, Question };

/// A trait defining the database operations for a Question Bank (`QBank`).
///
/// This abstracts the storage mechanism for question banks, allowing for different
/// backend implementations (e.g., SQLite, flat files).
pub trait QBDB
{
    // fn open(path: String) -> Option<SQLiteDB>
    /// Opens a connection to the question bank database.
    /// If the path has no extension, `.qbdb` is appended.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ SQLiteDB, SBDB };
    ///
    /// let db = SQLiteDB::open(":memory:".to_string());
    /// assert!(db.is_some());
    /// ```
    fn open(path: String) -> Option<Self> where Self: Sized;

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates the necessary tables (`tblHeader`, `tblQuestions`) in the database.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in `tblHeader`.
    /// * `choices` - The number of choice columns to create in `tblQuestions`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error string on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// let result = db.make_tables(2, 4); // 2 categories, 4 choices
    /// assert!(result.is_ok());
    /// ```
    fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>;

    // fn read_header(&self) -> Option<Header>
    /// Reads the `Header` data from the database.
    ///
    /// # Output
    /// `Option<Header>` - An `Option<Header>` which is `Some(Header)` on success, or `None` if not found or on error.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    /// use qrate::Header;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let header = db.read_header();
    /// assert!(header.is_some());
    /// assert_eq!(header.unwrap().get_title(), "Examination");
    /// ```
    fn read_header(&self) -> Option<Header>;

    // fn write_header_with_default(&self) -> Result<(), String>
    /// Writes a default `Header` to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// let result = db.write_header_with_default();
    /// assert!(result.is_ok());
    /// ```
    fn write_header_with_default(&mut self) -> Result<(), String>;

    // fn write_header(&self, header: &Header) -> Result<(), String>
    /// Writes a given `Header` to the database.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    /// use qrate::Header;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// let custom_header = Header::new(
    ///     "Custom Title".to_string(),
    ///     "Author".to_string(),
    ///     "ID123".to_string(),
    ///     vec!["Type C".to_string()],
    ///     "Custom Notice".to_string(),
    /// );
    /// let result = db.write_header(&custom_header);
    /// assert!(result.is_ok());
    /// let read_header = db.read_header().unwrap();
    /// assert_eq!(read_header.get_title(), "Custom Title");
    /// ```
    fn write_header(&mut self, header: &Header) -> Result<(), String>;

    // fn read_qbank(&self) -> Option<QBank>
    /// Reads the entire `QBank` (header and all questions) from the database.
    ///
    /// # Output
    /// `Option<QBank>` - An `Option<QBank>` which is `Some(QBank)` on success, or `None` on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    /// use qrate::{QBank, Question, Header};
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let mut qbank_to_write = QBank::new_with_Default();
    /// let question = Question::new(1, 1, 1, "Test Question".to_string(), vec!["Ch1".to_string(), "Ch2".to_string(), "Ch3".to_string(), "Ch4".to_string()]);
    /// qbank_to_write.push_question(question);
    /// db.write_qbank(&qbank_to_write).unwrap();
    ///
    /// let qbank_read = db.read_qbank();
    /// assert!(qbank_read.is_some());
    /// assert_eq!(qbank_read.unwrap().get_bank().len(), 1);
    /// ```
    fn read_qbank(&self) -> Option<QBank>;

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Writes an entire `QBank` (header and all questions) to the database.
    ///
    /// Note: This typically writes the questions. The header should be written separately
    /// if it's not already present.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::QBDB;
    /// use qrate::{QBank, Question, Header};
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_tables(2, 4).unwrap();
    /// db.write_header_with_default().unwrap();
    ///
    /// let mut qbank = QBank::new_with_Default();
    /// let question = Question::new(1, 1, 1, "Test Question".to_string(), vec!["Choice A".to_string(), "Choice B".to_string(), "Choice C".to_string(), "Choice D".to_string()]);
    /// qbank.push_question(question);
    ///
    /// let result = db.write_qbank(&qbank);
    /// assert!(result.is_ok());
    /// ```
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>;
}

impl QBDB for SQLiteDB
{
    // fn open(path: String) -> Option<Self> where Self: Sized
    /// Implements `open` for `SQLiteDB`.
    /// Appends `.qbdb` to the path if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.
    #[inline]
    fn open(path: String) -> Option<Self>
    where Self: Sized
    {
        Self::open(path, ".qbdb")
    }

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates tables for `SQLiteDB`.
    ///
    /// Dynamically constructs `CREATE TABLE` SQL statements for `tblHeader` and `tblQuestions`
    /// based on the number of categories and choices required.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in `tblHeader`.
    /// * `choices` - The number of choice columns to create in `tblQuestions`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    {
        let mut sql = r#"CREATE TABLE tblHeader (
    title	TEXT NOT NULL,
    name	TEXT NOT NULL,
    id  	TEXT NOT NULL,
    notice	TEXT NOT NULL"#.to_string();
        for i in 1..=categories
            { sql += format!(",\n\tcategory{}\tTEXT NOT NULL", i).as_str(); }
        sql += "\n);";
        if let Err(e) = self.conn.execute(sql.as_str(), [])
            {return Err(format!("Failed to create table tblHeader!! {}", e)); }

        sql = r#"CREATE TABLE tblQuestions (
    id	        INTEGER NOT NULL UNIQUE,
    category    INTEGER NOT NULL,
    question	TEXT NOT NULL"#.to_string();
        for i in 1..=choices
        {
            sql += &format!(",\n\tchoice{}_text\tTEXT", i);
            sql += &format!(",\n\tchoice{}_is_answer\tBOOLEAN", i);
        }
        sql += ",\n\tPRIMARY KEY(id)\n);";
        match self.conn.execute(&sql, [])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create table tblQuestions!! {}", e)),
        }
    }

    // fn read_header(&self) -> Option<Header>
    /// Implements `read_header` for `SQLiteDB`.
    ///
    /// Queries the `tblHeader` table and maps the first row to a `Header` struct.
    ///
    /// # Output
    /// `Option<Header>` - An optional `Header` containing the header data from the database.
    fn read_header(&self) -> Option<Header>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM tblHeader;").ok()?;
        let vec_header = stmt.query_map([], |row| {
            let mut categories = Vec::new();
            let mut i = 4;
            while let Ok(c) = row.get(i)
            {
                categories.push(c);
                i += 1;
            }
            Ok(Header::new(row.get(0)?, row.get(1)?, row.get(2)?, categories, row.get(3)?))
        }).ok()?;

        for info in vec_header
        {
            if let Ok(ff) = info
                { return Some(ff); }
        }
        None
    }

    // fn write_header_with_default(&self) -> Result<(), String>
    /// Implements `write_header_with_default` for `SQLiteDB`.
    /// Creates a default `Header` and calls `write_header`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), String>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&self, header: &Header) -> Result<(), String>
    /// Implements `write_header` for `SQLiteDB`.
    ///
    /// Constructs and executes an `INSERT` statement for the `tblHeader` table.
    /// It dynamically binds parameters based on the number of categories in the `Header`.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_header(&mut self, header: &Header) -> Result<(), String>
    {
        let length = header.get_categories().len();
        let mut sql = format!("INSERT INTO tblHeader values (?1, ?2, ?3, ?4");
        for i in 5..(5 + length)
            { sql += format!(", ?{}", i).as_str(); }
        sql += ");";

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        params.push(header.get_title());
        params.push(header.get_name());
        params.push(header.get_id());
        params.push(header.get_notice());
        for category in header.get_categories()
            { params.push(category); }

        self.conn.execute(sql.as_str(), &params[..]).map_err(|e| e.to_string())?;
        Ok(())
    }

    // fn read_qbank(&self) -> Option<QBank>
    /// Implements `read_qbank` for `SQLiteDB`.
    ///
    /// First, it reads the header using `read_header`. Then, it queries the `tblQuestions` table,
    /// maps each row to a `Question` struct, and collects them into a new `QBank`.
    ///
    /// # Output
    /// `Option<QBank>` - An optional `QBank` containing the header and all questions from the database.
    fn read_qbank(&self) -> Option<QBank>
    {
        let header = self.read_header()?;
        let mut stmt = self.conn.prepare("SELECT * FROM tblQuestions;").ok()?;
        let vec_question = stmt.query_map([], |row| {
            let id: u16 = row.get(0)?;
            let category: u8 = row.get(1)?;
            let question: String = row.get(2)?;
            let mut choices = Choices::new();

            // The loop will attempt to read pairs of choice_text and choice_is_answer.
            // It stops when it can't read a pair, which is safer than a fixed limit.
            
            let mut idx = 3;
            loop
            {
                if let (Ok(choice), Ok(is_answer)) = (row.get(idx), row.get(idx + 1))
                    { choices.push((choice, is_answer)); }
                else    // Stop if we can't read a complete choice pair.
                    { break; }
                idx += 2;
            }
            Ok(Question::new(id, category, question, choices))
        }).ok()?;

        let mut question_bank = QBank::new_with_header(header);
        for result_question in vec_question
        {
            if let Ok(question) = result_question
                { question_bank.push_question(question); }
            else
                { return None; }
        }
        Some(question_bank)
    }

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Implements `write_qbank` for `SQLiteDB`.
    ///
    /// Iterates through the questions in the provided `QBank` and inserts each one
    /// into the `tblQuestions` table. It dynamically constructs the `INSERT` statement
    /// and binds parameters based on the number of choices in the questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing questions to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    {
        if qbank.get_bank().is_empty()   // Nothing to write
            { return Err("Empty QBank".to_string()); }

        // 1. Determine the maximum number of choices in the entire bank to create a uniform SQL statement.
        let max_choices = qbank.get_bank().iter().map(|q| q.get_choices().len()).max().unwrap_or(0);

        if max_choices == 0
        {
            // still write question if there are no choices
        }

        // 2. Build the SQL statement dynamically.
        let mut sql = "INSERT INTO tblQuestions (id, category, question".to_string();
        let mut values = "?, ?, ?".to_string();
        for i in 1..=max_choices
        {
            sql += &format!(", choice{}_text, choice{}_is_answer", i, i);
            values += ", ?, ?";
        }
        sql += &format!(") VALUES ({});", values);

        // 3. Iterate through questions and execute the INSERT statement.
        for elem in qbank.get_bank()
        {
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            params.push(Box::new(elem.get_id()));
            params.push(Box::new(elem.get_category()));
            params.push(Box::new(elem.get_question().clone()));

            let choices = elem.get_choices();
            for i in 0..max_choices
            {
                if i < choices.len()
                {
                    params.push(Box::new(choices[i].0.clone())); // choice_text
                    params.push(Box::new(choices[i].1)); // choice_is_answer
                }
                else    // Pad with NULLs if the question has fewer choices than the max.
                {
                    params.push(Box::new(rusqlite::types::Value::Null));
                    params.push(Box::new(rusqlite::types::Value::Null));
                }
            }

            let params_for_exec: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
            self.conn.execute(&sql, &params_for_exec[..]).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}



impl QBDB for Excel
{
    // fn open(path: String) -> Option<Self> where Self: Sized
    /// Implements `open` for `Excel`.
    /// Appends `.qb.xlsx` to the path if no extension is present and opens an excel file.
    ///
    /// # Arguments
    /// * `path` - The file path for the excel file.
    ///
    /// # Output
    /// `Option<SQLiteDB>` - An optional `Excel` instance if opening is successful.
    #[inline]
    fn open(path: String) -> Option<Self>
    where Self: Sized
    {
        Self::open(path, ".qb.xlsx")
    }

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Creates sheets for `Excel`.
    ///
    /// Dynamically creates sheetts for `Header` and `Questions`
    /// based on the number of categories and choices required.
    ///
    /// # Arguments
    /// * `categories` - The number of category columns to create in `Header` sheet.
    /// * `choices` - The number of choice columns to create in `Questions` sheet.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn make_tables(&self, _categories: u8, choices: u8) -> Result<(), String>
    {
        let mut workbook = Workbook::new();

        // 1. Create "header" sheet
        let header_sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;

        // Style settings
        // let border_format = Format::new().set_border(FormatBorder::Thin);
        let bold_border_format = Format::new().set_bold().set_border(FormatBorder::Thin);

        header_sheet.write_string_with_format(0, 0, "Title", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 0, "Name", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 0, "Notice", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(4, 0, "Categories", &bold_border_format).map_err(|e| e.to_string())?;

        // 2. Create "bank" sheet
        let questions_sheet = workbook.add_worksheet().set_name("Questions").map_err(|e| e.to_string())?;

        // Write fixed header row part
        questions_sheet.write_string_with_format(0, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 1, "Category", &bold_border_format).map_err(|e| e.to_string())?;
        questions_sheet.write_string_with_format(0, 2, "Question", &bold_border_format).map_err(|e| e.to_string())?;

        // Dynamically generate Choice and IsAnswer headers
        let mut current_col = 3;
        for i in 1..=choices
        {
            let txt = format!("Choice{}", i);
            questions_sheet.write_string_with_format(0, current_col, txt, &bold_border_format).map_err(|e| e.to_string())?;
            current_col += 1;
            let txt = format!("IsAnswer{}", i);
            questions_sheet.write_string_with_format(0, current_col, txt, &bold_border_format).map_err(|e| e.to_string())?;
            current_col += 1;
        }

        // Save file
        workbook.save(&self.path).map_err(|e| e.to_string())?;
        Ok(())
    }

    // fn read_header(&self) -> Option<Header>
    /// Implements `read_header` for `Excel`.
    ///
    /// Queries the `tblHeader` table and maps the first row to a `Header` struct.
    ///
    /// # Output
    /// `Option<Header>` - An optional `Header` containing the header data from the database.
    fn read_header(&self) -> Option<Header>
    {
        let mut excel = open_workbook_auto(&self.path).ok()?;

        // Find "Header" sheet
        let sheet_names = excel.sheet_names();
        let header_sheet_idx = sheet_names.iter().position(|s| s == "Header")?;
        let range: calamine::Range<calamine::Data> = excel.worksheet_range_at(header_sheet_idx)?.ok()?;

        // Assuming fixed key-value pairs in the header sheet
        let title = range.get((0, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default();
        let name = range.get((1, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default();
        let id = range.get((2, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default();
        let notice = range.get((3, 1)).and_then(|c: &calamine::Data| c.as_string()).unwrap_or_default();
        
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
        Some(Header::new(title, name, id, categories, notice))
    }

    // fn write_header_with_default(&self) -> Result<(), String>
    /// Implements `write_header_with_default` for `Excel`.
    /// Creates a default `Header` and calls `write_header`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    #[inline]
    fn write_header_with_default(&mut self) -> Result<(), String>
    {
        self.write_header(&Header::new_with_default())
    }

    // fn write_header(&self, header: &Header) -> Result<(), String>
    /// Implements `write_header` for `Excel`.
    ///
    /// Constructs and executes an `INSERT` statement for the `tblHeader` table.
    /// It dynamically binds parameters based on the number of categories in the `Header`.
    ///
    /// # Arguments
    /// * `header` - A reference to the `Header` to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_header(&mut self, header: &Header) -> Result<(), String>
    {
        let excel = open_workbook_auto(&self.path).map_err(|e| format!("Failed to open workbook '{}': {}", &self.path, e))?;
        let mut qbank;
        if let Some(qb) = self.read_qbank()
            { qbank = qb; }
        else
            { return Err("Failed to read question bank".to_string()); }
        drop(excel);
        qbank.set_header(header.clone());
        self.write_qbank(&qbank)
    }

    // fn read_qbank(&self) -> Option<QBank>
    /// Implements `read_qbank` for `Excel`.
    ///
    /// First, it reads the header using `read_header`. Then, it queries the `tblQuestions` table,
    /// maps each row to a `Question` struct, and collects them into a new `QBank`.
    ///
    /// # Output
    /// `Option<QBank>` - An optional `QBank` containing the header and all questions from the database.
    fn read_qbank(&self) -> Option<QBank>
    {
        let mut qbank = QBank::new_empty();
        qbank.set_header(self.read_header()?);
        
        let mut excel = open_workbook_auto(&self.path).ok()?;

        // 1. Read "Questions" sheet
        let sheet_names = excel.sheet_names();
        let questions_sheet_idx = sheet_names.iter().position(|s| s == "Questions")?;
        let range: calamine::Range<calamine::Data> = excel.worksheet_range_at(questions_sheet_idx)?.ok()?;

        let header_row = range.rows().next()?;
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
        for (_, row_data) in range.rows().enumerate().skip(1)
        {
            // Skip header row
            if row_data.len() < min_expected_cols_with_choices
                { return None; }

            let id = row_data[0].get_int()? as u16;
            let category = row_data[1].get_int()? as u8;
            let question_text = row_data[2].as_string()?;
            let mut choices = Choices::new();
            let mut current_choice_col = 3; // Choices start after ID, Category, Question

            for _ in 0..num_choices_in_excel
            {
                if current_choice_col + 1 < row_data.len()
                {
                    let choice_text = row_data[current_choice_col].as_string().unwrap_or_default(); // Default to empty string if not found
                    let is_answer = row_data[current_choice_col + 1].as_string().unwrap_or_default().eq_ignore_ascii_case("TRUE");

                    // Add only when choice has content or is marked as answer.
                    // This prevents adding a bunch of empty choices
                    // if max_choices was determined by a question with many choices
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
            qbank.push_question(Question::new(id, category, question_text, choices));
        }
        Some(qbank)
    }

    // fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    /// Implements `write_qbank` for `Excel`.
    ///
    /// Iterates through the questions in the provided `QBank` and inserts each one
    /// into the `tblQuestions` table. It dynamically constructs the `INSERT` statement
    /// and binds parameters based on the number of choices in the questions.
    ///
    /// # Arguments
    /// * `qbank` - A reference to the `QBank` containing questions to be written to the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_qbank(&mut self, qbank: &QBank) -> Result<(), String>
    {    
        let excel: Self;
        if let Some(xls) = QBDB::open(self.path.clone())
            { excel = xls; }
        else
            { return Err(format!("Failed to open the file {}", self.path)); }

        self.path = excel.path;
        let mut workbook = Workbook::new();

        // 1. Create "header" sheet
        let header_sheet = workbook.add_worksheet().set_name("Header").map_err(|e| e.to_string())?;

        // Style settings
        let border_format = Format::new().set_border(FormatBorder::Thin);
        let bold_border_format = Format::new().set_bold().set_border(FormatBorder::Thin);

        let header = qbank.get_header();
        header_sheet.write_string_with_format(0, 0, "Title", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(0, 1, header.get_title(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 0, "Name", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(1, 1, header.get_name(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(2, 1, header.get_id(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 0, "Notice", &bold_border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(3, 1, header.get_notice(), &border_format).map_err(|e| e.to_string())?;
        header_sheet.write_string_with_format(4, 0, "Categories", &bold_border_format).map_err(|e| e.to_string())?;
        for (i, cat) in header.get_categories().iter().enumerate()
            { header_sheet.write_string_with_format(4, i as u16 + 1, cat, &border_format).map_err(|e| e.to_string())?; }

        // 2. Create "bank" sheet
        let bank_sheet = workbook.add_worksheet().set_name("Questions").map_err(|e| e.to_string())?;

        // Write fixed header row part
        bank_sheet.write_string_with_format(0, 0, "ID", &bold_border_format).map_err(|e| e.to_string())?;
        bank_sheet.write_string_with_format(0, 1, "Category", &bold_border_format).map_err(|e| e.to_string())?;
        bank_sheet.write_string_with_format(0, 2, "Question", &bold_border_format).map_err(|e| e.to_string())?;

        // Determine the maximum number of choices in the entire bank
        let max_choices = qbank.get_bank().iter().map(|q| q.get_choices().len()).max().unwrap_or(0);
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
        for (row_idx, question) in qbank.get_bank().iter().enumerate()
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
}
