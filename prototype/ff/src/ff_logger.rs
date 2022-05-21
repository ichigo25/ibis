//=============================================================================
// LogLevel
//=============================================================================
#[derive(Debug)]
pub enum LogLevel
{
    Debug,
    Info,
    Notice,
    Warning,
    Error,
}


//=============================================================================
// Logger
//=============================================================================
#[derive(Debug)]
pub struct Logger
{
    log_level: LogLevel,
    file_name: String,
}

impl Logger
{
    //=========================================================================
    // コンストラクタ
    //=========================================================================
    pub fn init(log_level: LogLevel, file_name: String) -> Self
    {
        Self
        {
            log_level,
            file_name,
        }
    }
}


//=============================================================================
// Logger
//=============================================================================
#[macro_export]
macro_rules! debug
{
   ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        println!("{}", res);
    }}
}


//=============================================================================
// Logger
//=============================================================================
#[macro_export]
macro_rules! info
{
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        println!("{}", res);
    }}
}

