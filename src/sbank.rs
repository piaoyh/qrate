use crate::Student;

/// A type alias for a vector of `Student`s, representing a bank of students.
pub type SBank = Vec<Student>;

// pub fn get_bank() -> SBank
/// Returns a default, hardcoded list of students.
///
/// This function is used to get a predefined `SBank` for testing or demonstration purposes.
///
/// # Output
///
/// `SBank` - A vector containing a predefined list of `Student` instances.
///
/// # Examples
///
/// ```
/// use qrate::get_bank;
/// let sbank = get_bank();
/// assert!(!sbank.is_empty());
/// assert_eq!(sbank.len(), 4);
/// assert_eq!(sbank[0].get_name(), "Бакытбеков Акылбек Бакытбекович");
/// ```
pub fn get_bank() -> SBank
{
    let mut students = SBank::new();
    students.push(Student::new(
        "Бакытбеков Акылбек Бакытбекович".to_string(),
        "1219018".to_string(),
    ));
    students.push(Student::new(
        "Тюменев Даниил Равильевич".to_string(),
        "1219009".to_string(),
    ));
    students.push(Student::new(
        "Нурлан кызы Диана".to_string(),
        "".to_string(),
    ));
    students.push(Student::new(
        "Акылбек кызы Турсунай".to_string(),
        "2221002".to_string(),
    ));

    /*
        let students = vec![
                Student { name: "Бакытбеков Акылбек Бакытбекович", id: "1219018" },
                Student { name: "Тюменев Даниил 라비레비치", id: "1219009" },
                Student { name: "Нурлан 기지 디아나", id:"" },
                Student { name: "Акылбек 기지 투르수나이", id: "2221002" },
            ];
    */
    return students;
}
