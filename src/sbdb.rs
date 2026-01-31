use crate::SBank;
use crate::SQLiteDB;
use crate::Student;

/// A trait defining the database operations for a Student Bank (`SBank`).
///
/// This abstracts the storage mechanism for student data.
pub trait SBDB
{
    // fn open(path: String) -> Option<SQLiteDB>
    /// Opens a connection to the student database.
    /// If the path has no extension, `.sbdb` is appended.
    /// # Arguments
    /// * `path` - The file path for the database.
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::SBDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string());
    /// assert!(db.is_some());
    /// ```
    fn open(path: String) -> Option<SQLiteDB>;

    // fn make_table(&self) -> Result<(), String>
    /// Creates the `tblStudents` table in the database.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::SBDB;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// let result = db.make_table();
    /// assert!(result.is_ok());
    /// ```
    fn make_table(&self) -> Result<(), String>;

    // fn read_sbank(&self) -> Option<SBank>
    /// Reads the entire `SBank` from the database.
    ///
    /// # Output
    /// `Option<SBank>` - An optional `SBank` containing all students from the database.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::SBDB;
    /// use qrate::Student;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_table().unwrap();
    /// db.write_sbank(&vec![Student::new("Test Student".to_string(), "123".to_string())]).unwrap();
    ///
    /// let sbank = db.read_sbank();
    /// assert!(sbank.is_some());
    /// assert_eq!(sbank.unwrap().len(), 1);
    /// ```
    fn read_sbank(&self) -> Option<SBank>;

    // fn write_sbank(&self, sbank: &SBank) -> Result<(), String>
    /// Writes an `SBank` to the database.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank` to be written to the database.
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    ///
    /// # Examples
    /// ```
    /// use qrate::SQLiteDB;
    /// use qrate::SBDB;
    /// use qrate::Student;
    ///
    /// let db = SQLiteDB::open(":memory:".to_string()).unwrap();
    /// db.make_table().unwrap();
    /// let sbank = vec![
    ///     Student::new("Student One".to_string(), "001".to_string()),
    ///     Student::new("Student Two".to_string(), "002".to_string()),
    /// ];
    /// let result = db.write_sbank(&sbank);
    /// assert!(result.is_ok());
    /// ```
    fn write_sbank(&self, sbank: &SBank) -> Result<(), String>;
}

impl SBDB for SQLiteDB
{
    // fn open(path: String) -> Option<SQLiteDB>
    /// Implements `open` for `SQLiteDB`.
    /// Appends `.sbdb` to the path if no extension is present and opens a connection.
    ///
    /// # Arguments
    /// * `path` - The file path for the database.
    /// # Output
    /// `Option<SQLiteDB>` - An optional `SQLiteDB` instance if the connection is successful.
    fn open(path: String) -> Option<SQLiteDB>
    {
        Self::open(path, ".sbdb")
    }

    // fn make_table(&self) -> Result<(), String>
    /// Implements `make_table` for `SQLiteDB`.
    /// Executes a `CREATE TABLE` SQL statement for `tblStudents`.
    ///
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn make_table(&self) -> Result<(), String>
    {
        let sql = r#"CREATE TABLE tblStudents (
    name        TEXT NOT NULL,
    id          TEXT NOT NULL
);"#;
        match self.conn.execute(sql, [])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to create table tblStudents!! {}", e.to_string())),
        }
    }

    // fn read_sbank(&self) -> Option<SBank>
    /// Implements `read_sbank` for `SQLiteDB`.
    /// Queries the `tblStudents` table and maps each row to a `Student` struct.
    ///
    /// # Output
    /// `Option<SBank>` - An optional `SBank` containing all students from the database.
    fn read_sbank(&self) -> Option<SBank>
    {
        let mut stmt;
        if let Ok(st) = self.conn.prepare("SELECT * FROM tblStudents;")
            { stmt = st; }
        else
            { return None; }

        let vec_students;
        if let Ok(vq) = stmt.query_map([], |row| {
            let name = row.get(0)?;
            let id = row.get(1)?;
            Ok(Student::new(name, id))
        })
            { vec_students = vq; }
        else
            { return None; }

        let mut sbank = SBank::new();
        for info in vec_students
        {
            match info
            {
                Ok(s) => sbank.push(s),
                Err(_) => {}
            }
        }
        Some(sbank)
    }

    // fn write_sbank(&self, sbank: &SBank) -> Result<(), String>
    /// Implements `write_sbank` for `SQLiteDB`.
    /// Iterates through the `SBank` and inserts each `Student` into the `tblStudents` table.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank` to be written to the database.
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_sbank(&self, sbank: &SBank) -> Result<(), String>
    {
        if sbank.len() == 0
            { return Err("No Students!".to_string()); }

        let sql = "INSERT INTO tblStudents (name, id) values (?1, ?2);";
        for elem in sbank
        {
            let name = elem.get_name();
            let id = elem.get_id();
            let params = (name, id);
            let res = self.conn.execute(sql, params);
            if let Err(_) = res
                { return Err("Cannot INSERT INTO tblStudents".to_string()); }
        }
        Ok(())
    }
}