use crate::header::Header;
use crate::question::Question;

/// Represents a Question Bank, containing a header and a vector of questions.
#[derive(Clone)]
pub struct QBank
{
    header: Header,
    bank: Vec<Question>,
}

impl QBank
{
    // pub fn new_empty() -> Self
    /// Creates a new, empty `QBank` with an empty header.
    ///
    /// # Output
    /// `Self` - A new, empty `QBank` instance.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_empty();
    /// assert!(qbank.get_bank().is_empty());
    /// ```
    #[inline]
    pub fn new_empty() -> Self
    {
        QBank
        {
            header: Header::new_empty(),
            bank: Vec::new(),
        }
    }

    // pub fn new_with_default() -> Self
    /// Creates a new `QBank` with a default header.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with a default header.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn new_with_default() -> Self
    {
        QBank
        {
            header: Header::new_with_default(),
            bank: Vec::new(),
        }
    }

    // pub fn new_with_header(header: Header) -> Self
    /// Creates a new `QBank` with a provided `Header`.
    ///
    /// # Arguments
    /// * `header` - The `Header` to be used for the new `QBank`.
    ///
    /// # Output
    /// `Self` - A new `QBank` instance with the specified header.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let custom_header = Header::new_empty();
    /// let qbank = QBank::new_with_header(custom_header);
    /// assert!(qbank.get_bank().is_empty());
    /// ```
    #[inline]
    pub fn new_with_header(header: Header) -> Self
    {
        QBank
        {
            header,
            bank: Vec::new(),
        }
    }

    // pub fn get_header(&self) -> &Header
    /// Gets a reference to the `Header`.
    ///
    /// # Output
    /// `&Header` - A reference to the `Header` of the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new_with_default();
    /// assert_eq!(qbank.get_header().get_title(), "Examination");
    /// ```
    #[inline]
    pub fn get_header(&self) -> &Header
    {
        &self.header
    }

    // pub fn set_header(&mut self, header: Header)
    /// Sets the `Header`.
    ///
    /// # Arguments
    /// * `header` - The new `Header` to set for the question bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Header };
    /// let mut qbank = QBank::new_empty();
    /// let mut new_header = Header::new_empty();
    /// new_header.set_title("My Custom Exam".to_string());
    /// qbank.set_header(new_header);
    /// assert_eq!(qbank.get_header().get_title(), "My Custom Exam");
    /// ```
    #[inline]
    pub fn set_header(&mut self, header: Header)
    {
        self.header = header;
    }

    // pub fn get_bank(&self) -> &Vec<Question>
    /// Gets a reference to the vector of `Question`s.
    ///
    /// # Output
    /// `&Vec<Question>` - A reference to the vector of `Question`s in the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new_empty());
    /// assert_eq!(qbank.get_bank().len(), 1);
    /// ```
    #[inline]
    pub fn get_bank(&self) -> &Vec<Question>
    {
        &self.bank
    }

    // pub fn set_bank(&mut self, bank: Vec<Question>)
    /// Sets the vector of `Question`s.
    ///
    /// # Arguments
    /// * `bank` - The new vector of `Question`s to set for the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// qbank.set_bank(vec![Question::new_empty(), Question::new_empty()]);
    /// assert_eq!(qbank.get_bank().len(), 2);
    /// ```
    #[inline]
    pub fn set_bank(&mut self, bank: Vec<Question>)
    {
        self.bank = bank;
    }

    // pub fn get_question(&self, q_number: usize) -> Option<&Question>
    /// Gets a reference to a `Question` by its 1-based index.
    ///
    /// # Arguments
    /// * `q_number` - The 1-based index of the question to retrieve.
    ///
    /// # Output
    /// `Option<&Question>` - An optional reference to the `Question` at the specified index.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// 
    /// let mut qbank = QBank::new_empty();
    /// qbank.push_question(Question::new(1, 1, "Test Q".to_string(), vec![]));
    /// assert_eq!(qbank.get_question(1).unwrap().get_id(), 1);
    /// assert!(qbank.get_question(2).is_none());
    /// ```
    pub fn get_question(&self, q_number: usize) -> Option<&Question>
    {
        if (q_number <= self.bank.len()) && q_number > 0
            { Some(&self.bank[q_number - 1]) }
        else
            { None }
    }

    // pub fn push_question(&mut self, question: Question)
    /// Adds a `Question` to the bank.
    ///
    /// # Arguments
    /// * `question` - The `Question` to add to the bank.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new_empty();
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_bank().len(), 1);
    /// ```
    #[inline]
    pub fn push_question(&mut self, question: Question)
    {
        self.bank.push(question);
    }

    // pub fn get_choice(&self, q_number: usize, ch_number: usize) -> Option<&String>
    /// Gets a reference to a choice `String` by question number and choice number (both 1-based).
    ///
    /// # Arguments
    /// * `q_number` - The 1-based index of the question.
    /// * `ch_number` - The 1-based index of the choice within the question.
    ///
    /// # Output
    /// `Option<&String>` - An optional reference to the choice string.
    ///
    /// # Examples
    /// ```
    /// use qrate::{ QBank, Question };
    /// let mut qbank = QBank::new_empty();
    /// let question = Question::new(1, 1, "Q1".to_string(), vec![("Choice A".to_string(), false), ("Choice B".to_string(), false)]);
    /// qbank.push_question(question);
    /// assert_eq!(qbank.get_choice(1, 1).unwrap(), "Choice A");
    /// assert!(qbank.get_choice(1, 3).is_none());
    /// assert!(qbank.get_choice(2, 1).is_none());
    /// ```
    pub fn get_choice(&self, q_number: usize, ch_number: usize) -> Option<&String>
    {
        if (q_number <= self.bank.len()) && q_number > 0
            { self.bank[q_number - 1].get_choice(ch_number).map(|(text, _is_answer)| text) }
        else
            { None }
    }
}
