use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Compliance {
    /** Strictly conform to an older more strict version of the spec
     * or reference software. */
    VeryStrict,
    /** Strictly conform to all the things in the spec no matter
     * what consequences. */
    Strict,
    Normal,
    /** Allow unofficial extensions. */
    Unofficial,
    /** Allow nonstandardized experimental things. */
    Experimental,
}

impl From<c_int> for Compliance {
    fn from(value: c_int) -> Self {
        match value {
            FF_COMPLIANCE_VERY_STRICT => Compliance::VeryStrict,
            FF_COMPLIANCE_STRICT => Compliance::Strict,
            FF_COMPLIANCE_NORMAL => Compliance::Normal,
            FF_COMPLIANCE_UNOFFICIAL => Compliance::Unofficial,
            FF_COMPLIANCE_EXPERIMENTAL => Compliance::Experimental,

            _ => Compliance::Normal,
        }
    }
}

impl From<Compliance> for c_int {
    fn from(value: Compliance) -> c_int {
        match value {
            Compliance::VeryStrict => FF_COMPLIANCE_VERY_STRICT,
            Compliance::Strict => FF_COMPLIANCE_STRICT,
            Compliance::Normal => FF_COMPLIANCE_NORMAL,
            Compliance::Unofficial => FF_COMPLIANCE_UNOFFICIAL,
            Compliance::Experimental => FF_COMPLIANCE_EXPERIMENTAL,
        }
    }
}
