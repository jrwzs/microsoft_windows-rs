#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_System_WinRT_Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Win32_System_WinRT_Graphics_Direct2D")]
pub mod Direct2D;
#[cfg(feature = "Win32_System_WinRT_Graphics_Imaging")]
pub mod Imaging;
