// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use crate::Excel;
use crate::SBank;
use crate::SQLiteDB;
use crate::Student;
use calamine::{Reader, DataType}; // Add DataType here

/// A trait defining the database operations for a Student Bank (`SBank`).
///
/// This abstracts the storage mechanism for student data.
pub trait SBDB
{
    fn open(path: String) -> Option<Self> where Self: Sized;
    fn make_table(&self) -> Result<(), String>;
    fn read_sbank(&self) -> Option<SBank>;
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), String>;
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
        let sql = r#"CREATE TABLE IF NOT EXISTS tblStudents (
    name        TEXT NOT NULL,
    id          TEXT NOT NULL
);"#;
        self.conn.execute(sql, []).map(|_| ()).map_err(|e| format!("Failed to create table tblStudents: {}", e))
    }

    // fn read_sbank(&self) -> Option<SBank>
    /// Implements `read_sbank` for `SQLiteDB`.
    /// Queries the `tblStudents` table and maps each row to a `Student` struct.
    ///
    /// # Output
    /// `Option<SBank>` - An optional `SBank` containing all students from the database.
    fn read_sbank(&self) -> Option<SBank>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM tblStudents;").ok()?;
        let student_iter = stmt.query_map([], |row| {
            Ok(Student::new(row.get(0)?, row.get(1)?))
        }).ok()?;

        student_iter.collect::<Result<SBank, _>>().ok()
    }

    // fn write_sbank(&self, sbank: &SBank) -> Result<(), String>
    /// Implements `write_sbank` for `SQLiteDB`.
    /// Iterates through the `SBank` and inserts each `Student` into the `tblStudents` table.
    ///
    /// # Arguments
    /// * `sbank` - A reference to the `SBank` to be written to the database.
    /// # Output
    /// `Result<(), String>` - `Ok(())` on success, or an error message string on failure.
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), String>
    {
        if sbank.is_empty()
        {
            return Ok(()); // Nothing to write, which is a success.
        }

        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx.prepare("INSERT INTO tblStudents (name, id) VALUES (?1, ?2);").map_err(|e| e.to_string())?;
            for student in sbank
                { stmt.execute((student.get_name(), student.get_id())).map_err(|e| format!("Failed to insert student {}: {}", student.get_id(), e))?; }
        }
        tx.commit().map_err(|e| e.to_string())
    }
}


impl SBDB for Excel
{
    #[inline]
    fn open(path: String) -> Option<Self>
    where Self: Sized
    {
        Self::open(path, ".sb.xlsx")
    }

    /// Creates a new Excel file with a "Students" sheet and headers.
    fn make_table(&self) -> Result<(), String>
    {
        let mut workbook = rust_xlsxwriter::Workbook::new();
        let sheet = workbook.add_worksheet().set_name("Students").map_err(|e| e.to_string())?;
        let format = rust_xlsxwriter::Format::new().set_bold();

        sheet.write_string_with_format(0, 0, "Name", &format).map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 1, "ID", &format).map_err(|e| e.to_string())?;

        workbook.save(&self.path).map_err(|e| e.to_string())
    }

    /// Reads students from the "Students" sheet in an Excel file.
        fn read_sbank(&self) -> Option<SBank>
        {
            let mut excel = calamine::open_workbook_auto(&self.path).ok()?;
            let range = excel.worksheet_range("Students").ok()?;
            
            let mut sbank = SBank::new();
            for row in range.rows().skip(1) { // Skip header row
                sbank.push(Student::new(
                    row.get(0).and_then(|d| d.as_string())?,
                    row.get(1).and_then(|d| d.as_string())? // Assuming ID is always string or convertible
                ));
            }
            Some(sbank)
        }
    
    /// Writes a collection of students to a "Students" sheet in an Excel file.
    fn write_sbank(&mut self, sbank: &SBank) -> Result<(), String>
    {
        let mut workbook = rust_xlsxwriter::Workbook::new();
        let sheet = workbook.add_worksheet().set_name("Students").map_err(|e| e.to_string())?;
        let header_format = rust_xlsxwriter::Format::new().set_bold();
        
        // Write header
        sheet.write_string_with_format(0, 0, "Name", &header_format).map_err(|e| e.to_string())?;
        sheet.write_string_with_format(0, 1, "ID", &header_format).map_err(|e| e.to_string())?;

        // Write student data
        for (row_idx, student) in sbank.iter().enumerate()
        {
            let row = (row_idx + 1) as u32;
            sheet.write_string(row, 0, student.get_name()).map_err(|e| e.to_string())?;
            sheet.write_string(row, 1, student.get_id()).map_err(|e| e.to_string())?;
        }

        workbook.save(&self.path).map_err(|e| e.to_string())
    }
}