#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrencyFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrencyFormatter {}
impl ::core::clone::Clone for CurrencyFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl ::core::marker::Copy for CurrencyFormatterMode {}
impl ::core::clone::Clone for CurrencyFormatterMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DecimalFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DecimalFormatter {}
impl ::core::clone::Clone for DecimalFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyFormatter {}
impl ::core::clone::Clone for ICurrencyFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyFormatter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyFormatter2 {}
impl ::core::clone::Clone for ICurrencyFormatter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrencyFormatterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrencyFormatterFactory {}
impl ::core::clone::Clone for ICurrencyFormatterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDecimalFormatterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDecimalFormatterFactory {}
impl ::core::clone::Clone for IDecimalFormatterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIncrementNumberRounder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIncrementNumberRounder {}
impl ::core::clone::Clone for IIncrementNumberRounder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberFormatter {}
impl ::core::clone::Clone for INumberFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberFormatter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberFormatter2 {}
impl ::core::clone::Clone for INumberFormatter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberFormatterOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberFormatterOptions {}
impl ::core::clone::Clone for INumberFormatterOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberParser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberParser {}
impl ::core::clone::Clone for INumberParser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberRounder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberRounder {}
impl ::core::clone::Clone for INumberRounder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumberRounderOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumberRounderOption {}
impl ::core::clone::Clone for INumberRounderOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumeralSystemTranslator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumeralSystemTranslator {}
impl ::core::clone::Clone for INumeralSystemTranslator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INumeralSystemTranslatorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INumeralSystemTranslatorFactory {}
impl ::core::clone::Clone for INumeralSystemTranslatorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPercentFormatterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPercentFormatterFactory {}
impl ::core::clone::Clone for IPercentFormatterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPermilleFormatterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPermilleFormatterFactory {}
impl ::core::clone::Clone for IPermilleFormatterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignedZeroOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignedZeroOption {}
impl ::core::clone::Clone for ISignedZeroOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignificantDigitsNumberRounder {}
impl ::core::clone::Clone for ISignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISignificantDigitsOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISignificantDigitsOption {}
impl ::core::clone::Clone for ISignificantDigitsOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IncrementNumberRounder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IncrementNumberRounder {}
impl ::core::clone::Clone for IncrementNumberRounder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NumeralSystemTranslator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NumeralSystemTranslator {}
impl ::core::clone::Clone for NumeralSystemTranslator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PercentFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PercentFormatter {}
impl ::core::clone::Clone for PercentFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PermilleFormatter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PermilleFormatter {}
impl ::core::clone::Clone for PermilleFormatter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: Self = Self(0i32);
    pub const RoundDown: Self = Self(1i32);
    pub const RoundUp: Self = Self(2i32);
    pub const RoundTowardsZero: Self = Self(3i32);
    pub const RoundAwayFromZero: Self = Self(4i32);
    pub const RoundHalfDown: Self = Self(5i32);
    pub const RoundHalfUp: Self = Self(6i32);
    pub const RoundHalfTowardsZero: Self = Self(7i32);
    pub const RoundHalfAwayFromZero: Self = Self(8i32);
    pub const RoundHalfToEven: Self = Self(9i32);
    pub const RoundHalfToOdd: Self = Self(10i32);
}
impl ::core::marker::Copy for RoundingAlgorithm {}
impl ::core::clone::Clone for RoundingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SignificantDigitsNumberRounder {}
impl ::core::clone::Clone for SignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        *self
    }
}
