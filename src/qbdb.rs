use crate::Header;
use crate::QBank;
use crate::SQLiteDB;
use crate::{Choices, Question};

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
    fn open(path: String) -> Option<SQLiteDB>;

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
    fn write_header_with_default(&self) -> Result<(), String>;

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
    fn write_header(&self, header: &Header) -> Result<(), String>;

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

    // fn write_qbank(&self, qbank: &QBank) -> Result<(), String>
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
    fn write_qbank(&self, qbank: &QBank) -> Result<(), String>;
}

impl QBDB for SQLiteDB
{
    // fn open(path: String) -> Option<SQLiteDB>
    /// Implements `open` for `SQLiteDB`.
    /// Appends `.qbdb` to the path if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    ///
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.
    fn open(path: String) -> Option<SQLiteDB>
    {
        Self::open(path, ".qbdb")
    }

    // fn make_tables(&self, categories: u8, choices: u8) -> Result<(), String>
    /// Implements `make_tables` for `SQLiteDB`.
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
        let mut stmt;
        if let Ok(st) = self.conn.prepare("SELECT * FROM tblHeader;")
            { stmt = st; }
        else
            { return None; }

        let vec_header;
        if let Ok(vh) = stmt.query_map([], |row| {
            let mut category = Vec::<String>::new();
            let mut i = 4;
            while let Ok(c) = row.get(i)
            {
                category.push(c);
                i += 1;
            }
            Ok(Header::new(row.get(0)?, row.get(1)?, row.get(2)?, category, row.get(3)?))
        })
            { vec_header = vh; }
        else
            { return None; }

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
    fn write_header_with_default(&self) -> Result<(), String>
    {
        let header = Header::new_with_default();
        self.write_header(&header)
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
    fn write_header(&self, header: &Header) -> Result<(), String>
    {
        let length = header.get_categories().len();
        let mut sql = format!("INSERT INTO tblHeader values (?1, ?2, ?3, ?4");
        for i in (5 + 0)..(5 + length)
            { sql += format!(", ?{}", i).as_str(); }
        sql += ");";

        let title = header.get_title();
        let name = header.get_name();
        let id = header.get_id();
        let notice = header.get_notice();
        let category = header.get_categories();
        let res = match length {
            1 => {
                let params = (title, name, id, notice, &category[0]);
                self.conn.execute(sql.as_str(), params)
            }
            2 => {
                let params = (title, name, id, notice, &category[0], &category[1]);
                self.conn.execute(sql.as_str(), params)
            }
            3 => {
                let params = (title, name, id, notice, &category[0], &category[1], &category[2]);
                self.conn.execute(sql.as_str(), params)
            }
            4 => {
                let params = (name, id, notice, &category[0], &category[1], &category[2], &category[3]);
                self.conn.execute(sql.as_str(), params)
            }
            _ => self.conn.execute(sql.as_str(), ()),
        };
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to insert into tblHeader".to_string()),
        }
    }

    // fn read_qbank(&self) -> Option<QBank>
    /// Implements `read_qbank` for `SQLiteDB`.
    ///
    /// First, it reads the header using `read_header`. Then, it queries the `tblQuestions` table,
    /// maps each row to a `Question` struct, and collects them into a new `QBank`.
    ///
    /// # Output
    /// `Option<QBank>` - An optional `QBank` containing the header and all questions from the database.
    fn read_qbank(&self) -> Option<QBank> {
        let mut stmt = self.conn.prepare("SELECT * FROM tblQuestions;").ok()?;
        let vec_question_result = stmt.query_map([], |row| {
            let id: u16 = row.get(0)?;
            let category: u8 = row.get(1)?;
            let question: String = row.get(2)?;
            let mut choices = Choices::new();

            // Assuming a maximum of, for example, 10 choices.
            // The loop will attempt to read pairs of choice_text and choice_is_answer.
            // It stops when it can't read a pair, which is safer than a fixed limit.
            for i in 0..10
            {
                let text_idx = 3 + i * 2;
                let is_answer_idx = 3 + i * 2 + 1;
                let choice_text: Result<String, _> = row.get(text_idx);
                let is_answer: Result<bool, _> = row.get(is_answer_idx);

                if let (Ok(text), Ok(answer_flag)) = (choice_text, is_answer)
                    { choices.push((text, answer_flag)); }
                else    // Stop if we can't read a complete choice pair.
                    { break; }
            }
            Ok(Question::new(id, category, question, choices))
        });

        if let Ok(vec_question) = vec_question_result
        {
            let mut question_bank = QBank::new_with_header(self.read_header()?);
            for info in vec_question
            {
                if let Ok(q) = info
                    { question_bank.push_question(q); }
            }
            Some(question_bank)
        }
        else
        {
            None
        }
    }

    // fn write_qbank(&self, qbank: &QBank) -> Result<(), String>
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
    fn write_qbank(&self, qbank: &QBank) -> Result<(), String>
    {
        if qbank.get_bank().is_empty()   // Nothing to write
            { return Ok(()); }

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
