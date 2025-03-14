
pub const INPUTFILENAME: &str = "input.png";

pub const OUTPUT_SVG_FILE: &str = "output.svg";

pub const OUTPUTFILENAME: &str = "output.png";

pub const WAIT_FOR_MESSAGE_UPDATE_TIMEOUT_S: u64 = 5;

use std::fmt;

#[derive(PartialEq, Eq)]
pub enum FfiResult {
    SuccessCode = 0,
    AssumptionWrong,
    TemplateFileNotFound,
    SvgSpaceError,
    ReadFileError,
    ArrayDiffSizeError,
    NullArgumentError,
    OverflowError,
    BadArgumentError,
    NotPngError,
    LowBoundariesCreated,
    UnknownError
}

impl From<i32> for FfiResult {
    fn from(num: i32) -> Self {
        match num {
            0 => FfiResult::SuccessCode,
            1 => FfiResult::AssumptionWrong,
            2 => FfiResult::TemplateFileNotFound,
            3 => FfiResult::SvgSpaceError,
            4 => FfiResult::ReadFileError,
            5 => FfiResult::ArrayDiffSizeError,
            6 => FfiResult::NullArgumentError,
            7 => FfiResult::OverflowError,
            8 => FfiResult::BadArgumentError,
            9 => FfiResult::NotPngError,
            10 => FfiResult::LowBoundariesCreated,
            _ => FfiResult::UnknownError
        }
    }
}

impl Into<&'static str> for FfiResult {
    fn into(self) -> &'static str {
        ffiresult_to_string(&self)
    }
}

impl fmt::Display for FfiResult
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", ffiresult_to_string(&self))
    }
}

fn ffiresult_to_string(input: &FfiResult) -> &'static str {
    match input {
        FfiResult::SuccessCode => "SuccessCode",
        FfiResult::AssumptionWrong => "AssumptionWrong",
        FfiResult::TemplateFileNotFound => "TemplateFileNotFound",
        FfiResult::SvgSpaceError => "SvgSpaceError",
        FfiResult::ReadFileError => "ReadFileError",
        FfiResult::ArrayDiffSizeError => "ArrayDiffSizeError",
        FfiResult::NullArgumentError => "NullArgumentError",
        FfiResult::OverflowError => "OverflowError",
        FfiResult::BadArgumentError => "BadArgumentError",
        FfiResult::NotPngError => "NotPngError",
        FfiResult::LowBoundariesCreated => "LowBoundariesCreated",
        FfiResult::UnknownError => "UnknownError"
    }
}
