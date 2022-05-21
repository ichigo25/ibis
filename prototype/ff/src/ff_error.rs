use std::error;
use std::fmt;


//=============================================================================
// ErrorType
//=============================================================================
#[derive(Debug)]
pub enum ErrorType
{
    Simple( &'static str ),
    Custom( (&'static str, Box<dyn error::Error + Send + Sync>) ),
}


//=============================================================================
// Error
//=============================================================================
pub struct Error
{
    error: ErrorType,
}

impl Error
{
    //=========================================================================
    // カスタムエラー作成
    //=========================================================================
    pub fn new<E>(kind: &'static str, error: E) -> Self
        where
            E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self
        {
            error: ErrorType::Custom((kind, error.into())),
        }
    }
}

//=============================================================================
// Display実装
//=============================================================================
impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &self.error
        {
            ErrorType::Simple(s) => f.write_str(s),
            ErrorType::Custom(c) => f.write_str(c.0),
        }
    }
}


//=============================================================================
// Debug実装
//=============================================================================
impl fmt::Debug for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        <Self as fmt::Display>::fmt(self, f)
    }
}

