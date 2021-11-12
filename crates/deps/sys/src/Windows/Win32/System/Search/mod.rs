#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_System_Search_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    pub fn ODBCGetTryWaitValue() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ODBCSetTryWaitValue(dwvalue: u32) -> super::super::Foundation::BOOL;
    pub fn SQLAllocConnect(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut *mut ::core::ffi::c_void) -> i16;
    pub fn SQLAllocEnv(environmenthandle: *mut *mut ::core::ffi::c_void) -> i16;
    pub fn SQLAllocHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void, outputhandle: *mut *mut ::core::ffi::c_void) -> i16;
    pub fn SQLAllocHandleStd(fhandletype: i16, hinput: *mut ::core::ffi::c_void, phoutput: *mut *mut ::core::ffi::c_void) -> i16;
    pub fn SQLAllocStmt(connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_ind: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_ind: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *mut ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u64, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i64, pcbvalue: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLBindParameter(hstmt: *mut ::core::ffi::c_void, ipar: u16, fparamtype: i16, fctype: i16, fsqltype: i16, cbcoldef: u32, ibscale: i16, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    pub fn SQLBrowseConnect(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    pub fn SQLBrowseConnectA(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16) -> i16;
    pub fn SQLBrowseConnectW(hdbc: *mut ::core::ffi::c_void, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16) -> i16;
    pub fn SQLBulkOperations(statementhandle: *mut ::core::ffi::c_void, operation: i16) -> i16;
    pub fn SQLCancel(statementhandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLCancelHandle(handletype: i16, inputhandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLCloseCursor(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLCloseEnumServers(henumhandle: super::super::Foundation::HANDLE) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttribute(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, fieldidentifier: u16, characterattribute: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16, numericattribute: *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeA(hstmt: *mut ::core::ffi::c_void, icol: i16, ifield: i16, pcharattr: *mut ::core::ffi::c_void, cbcharattrmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributeW(hstmt: *mut ::core::ffi::c_void, icol: u16, ifield: u16, pcharattr: *mut ::core::ffi::c_void, cbdescmax: i16, pcbcharattr: *mut i16, pnumattr: *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributes(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesA(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLColAttributesW(hstmt: *mut ::core::ffi::c_void, icol: u16, fdesctype: u16, rgbdesc: *mut ::core::ffi::c_void, cbdescmax: i16, pcbdesc: *mut i16, pfdesc: *mut i32) -> i16;
    pub fn SQLColumnPrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    pub fn SQLColumnPrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    pub fn SQLColumnPrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    pub fn SQLColumns(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, columnname: *const u8, namelength4: i16) -> i16;
    pub fn SQLColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    pub fn SQLColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    pub fn SQLCompleteAsync(handletype: i16, handle: *mut ::core::ffi::c_void, asyncretcodeptr: *mut i16) -> i16;
    pub fn SQLConnect(connectionhandle: *mut ::core::ffi::c_void, servername: *const u8, namelength1: i16, username: *const u8, namelength2: i16, authentication: *const u8, namelength3: i16) -> i16;
    pub fn SQLConnectA(hdbc: *mut ::core::ffi::c_void, szdsn: *const u8, cbdsn: i16, szuid: *const u8, cbuid: i16, szauthstr: *const u8, cbauthstr: i16) -> i16;
    pub fn SQLConnectW(hdbc: *mut ::core::ffi::c_void, szdsn: *const u16, cchdsn: i16, szuid: *const u16, cchuid: i16, szauthstr: *const u16, cchauthstr: i16) -> i16;
    pub fn SQLCopyDesc(sourcedeschandle: *mut ::core::ffi::c_void, targetdeschandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLDataSources(environmenthandle: *mut ::core::ffi::c_void, direction: u16, servername: *mut u8, bufferlength1: i16, namelength1ptr: *mut i16, description: *mut u8, bufferlength2: i16, namelength2ptr: *mut i16) -> i16;
    pub fn SQLDataSourcesA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u8, cbdsnmax: i16, pcbdsn: *mut i16, szdescription: *mut u8, cbdescriptionmax: i16, pcbdescription: *mut i16) -> i16;
    pub fn SQLDataSourcesW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdsn: *mut u16, cchdsnmax: i16, pcchdsn: *mut i16, wszdescription: *mut u16, cchdescriptionmax: i16, pcchdescription: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u64, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeCol(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, columnname: *mut u8, bufferlength: i16, namelength: *mut i16, datatype: *mut i16, columnsize: *mut u32, decimaldigits: *mut i16, nullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColA(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u8, cbcolnamemax: i16, pcbcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeColW(hstmt: *mut ::core::ffi::c_void, icol: u16, szcolname: *mut u16, cchcolnamemax: i16, pcchcolname: *mut i16, pfsqltype: *mut i16, pcbcoldef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u64, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLDescribeParam(hstmt: *mut ::core::ffi::c_void, ipar: u16, pfsqltype: *mut i16, pcbparamdef: *mut u32, pibscale: *mut i16, pfnullable: *mut i16) -> i16;
    pub fn SQLDisconnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLDriverConnect(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cchconnstrin: i16, szconnstrout: *mut u8, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    pub fn SQLDriverConnectA(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u8, cbconnstrin: i16, szconnstrout: *mut u8, cbconnstroutmax: i16, pcbconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    pub fn SQLDriverConnectW(hdbc: *mut ::core::ffi::c_void, hwnd: isize, szconnstrin: *const u16, cchconnstrin: i16, szconnstrout: *mut u16, cchconnstroutmax: i16, pcchconnstrout: *mut i16, fdrivercompletion: u16) -> i16;
    pub fn SQLDrivers(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u8, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    pub fn SQLDriversA(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u8, cbdriverdescmax: i16, pcbdriverdesc: *mut i16, szdriverattributes: *mut u8, cbdrvrattrmax: i16, pcbdrvrattr: *mut i16) -> i16;
    pub fn SQLDriversW(henv: *mut ::core::ffi::c_void, fdirection: u16, szdriverdesc: *mut u16, cchdriverdescmax: i16, pcchdriverdesc: *mut i16, szdriverattributes: *mut u16, cchdrvrattrmax: i16, pcchdrvrattr: *mut i16) -> i16;
    pub fn SQLEndTran(handletype: i16, handle: *mut ::core::ffi::c_void, completiontype: i16) -> i16;
    pub fn SQLError(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, statementhandle: *mut ::core::ffi::c_void, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    pub fn SQLErrorA(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    pub fn SQLErrorW(henv: *mut ::core::ffi::c_void, hdbc: *mut ::core::ffi::c_void, hstmt: *mut ::core::ffi::c_void, wszsqlstate: *mut u16, pfnativeerror: *mut i32, wszerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    pub fn SQLExecDirect(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    pub fn SQLExecDirectA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    pub fn SQLExecDirectW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, textlength: i32) -> i16;
    pub fn SQLExecute(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i64, pcrow: *mut u64, rgfrowstatus: *mut u16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLExtendedFetch(hstmt: *mut ::core::ffi::c_void, ffetchtype: u16, irow: i32, pcrow: *mut u32, rgfrowstatus: *mut u16) -> i16;
    pub fn SQLFetch(statementhandle: *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLFetchScroll(statementhandle: *mut ::core::ffi::c_void, fetchorientation: i16, fetchoffset: i32) -> i16;
    pub fn SQLForeignKeys(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cchpkcatalogname: i16, szpkschemaname: *const u8, cchpkschemaname: i16, szpktablename: *const u8, cchpktablename: i16, szfkcatalogname: *const u8, cchfkcatalogname: i16, szfkschemaname: *const u8, cchfkschemaname: i16, szfktablename: *const u8, cchfktablename: i16) -> i16;
    pub fn SQLForeignKeysA(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u8, cbpkcatalogname: i16, szpkschemaname: *const u8, cbpkschemaname: i16, szpktablename: *const u8, cbpktablename: i16, szfkcatalogname: *const u8, cbfkcatalogname: i16, szfkschemaname: *const u8, cbfkschemaname: i16, szfktablename: *const u8, cbfktablename: i16) -> i16;
    pub fn SQLForeignKeysW(hstmt: *mut ::core::ffi::c_void, szpkcatalogname: *const u16, cchpkcatalogname: i16, szpkschemaname: *const u16, cchpkschemaname: i16, szpktablename: *const u16, cchpktablename: i16, szfkcatalogname: *const u16, cchfkcatalogname: i16, szfkschemaname: *const u16, cchfkschemaname: i16, szfktablename: *const u16, cchfktablename: i16) -> i16;
    pub fn SQLFreeConnect(connectionhandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLFreeEnv(environmenthandle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLFreeHandle(handletype: i16, handle: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLFreeStmt(statementhandle: *mut ::core::ffi::c_void, option: u16) -> i16;
    pub fn SQLGetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlengthptr: *mut i32) -> i16;
    pub fn SQLGetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    pub fn SQLGetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    pub fn SQLGetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLGetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLGetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, pvparam: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLGetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *mut u8, bufferlength: i16, namelengthptr: *mut i16) -> i16;
    pub fn SQLGetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u8, cbcursormax: i16, pcbcursor: *mut i16) -> i16;
    pub fn SQLGetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *mut u16, cchcursormax: i16, pcchcursor: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i64, strlen_or_indptr: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetData(statementhandle: *mut ::core::ffi::c_void, columnnumber: u16, targettype: i16, targetvalue: *mut ::core::ffi::c_void, bufferlength: i32, strlen_or_indptr: *mut i32) -> i16;
    pub fn SQLGetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    pub fn SQLGetDescFieldA(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    pub fn SQLGetDescFieldW(hdesc: *mut ::core::ffi::c_void, irecord: i16, ifield: i16, rgbvalue: *mut ::core::ffi::c_void, cbbufferlength: i32, stringlength: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i64, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, name: *mut u8, bufferlength: i16, stringlengthptr: *mut i16, typeptr: *mut i16, subtypeptr: *mut i16, lengthptr: *mut i32, precisionptr: *mut i16, scaleptr: *mut i16, nullableptr: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecA(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u8, cbnamemax: i16, pcbname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i64, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLGetDescRecW(hdesc: *mut ::core::ffi::c_void, irecord: i16, szname: *mut u16, cchnamemax: i16, pcchname: *mut i16, pftype: *mut i16, pfsubtype: *mut i16, plength: *mut i32, pprecision: *mut i16, pscale: *mut i16, pnullable: *mut i16) -> i16;
    pub fn SQLGetDiagField(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, diagidentifier: i16, diaginfo: *mut ::core::ffi::c_void, bufferlength: i16, stringlength: *mut i16) -> i16;
    pub fn SQLGetDiagFieldA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbdiaginfomax: i16, pcbdiaginfo: *mut i16) -> i16;
    pub fn SQLGetDiagFieldW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, fdiagfield: i16, rgbdiaginfo: *mut ::core::ffi::c_void, cbbufferlength: i16, pcbstringlength: *mut i16) -> i16;
    pub fn SQLGetDiagRec(handletype: i16, handle: *mut ::core::ffi::c_void, recnumber: i16, sqlstate: *mut u8, nativeerror: *mut i32, messagetext: *mut u8, bufferlength: i16, textlength: *mut i16) -> i16;
    pub fn SQLGetDiagRecA(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u8, pfnativeerror: *mut i32, szerrormsg: *mut u8, cberrormsgmax: i16, pcberrormsg: *mut i16) -> i16;
    pub fn SQLGetDiagRecW(fhandletype: i16, handle: *mut ::core::ffi::c_void, irecord: i16, szsqlstate: *mut u16, pfnativeerror: *mut i32, szerrormsg: *mut u16, ccherrormsgmax: i16, pccherrormsg: *mut i16) -> i16;
    pub fn SQLGetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    pub fn SQLGetFunctions(connectionhandle: *mut ::core::ffi::c_void, functionid: u16, supported: *mut u16) -> i16;
    pub fn SQLGetInfo(connectionhandle: *mut ::core::ffi::c_void, infotype: u16, infovalue: *mut ::core::ffi::c_void, bufferlength: i16, stringlengthptr: *mut i16) -> i16;
    pub fn SQLGetInfoA(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    pub fn SQLGetInfoW(hdbc: *mut ::core::ffi::c_void, finfotype: u16, rgbinfovalue: *mut ::core::ffi::c_void, cbinfovaluemax: i16, pcbinfovalue: *mut i16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLGetNextEnumeration(henumhandle: super::super::Foundation::HANDLE, prgenumdata: *mut u8, pienumlength: *mut i32) -> i16;
    pub fn SQLGetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *mut ::core::ffi::c_void, bufferlength: i32, stringlength: *mut i32) -> i16;
    pub fn SQLGetStmtAttrA(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    pub fn SQLGetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32, pcbvalue: *mut i32) -> i16;
    pub fn SQLGetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLGetTypeInfo(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    pub fn SQLGetTypeInfoA(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    pub fn SQLGetTypeInfoW(statementhandle: *mut ::core::ffi::c_void, datatype: i16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLInitEnumServers(pwchservername: super::super::Foundation::PWSTR, pwchinstancename: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR, param2: i16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SQLLinkedCatalogsW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR, param2: i16) -> i16;
    pub fn SQLLinkedServers(param0: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLMoreResults(hstmt: *mut ::core::ffi::c_void) -> i16;
    pub fn SQLNativeSql(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cchsqlstrin: i32, szsqlstr: *mut u8, cchsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    pub fn SQLNativeSqlA(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u8, cbsqlstrin: i32, szsqlstr: *mut u8, cbsqlstrmax: i32, pcbsqlstr: *mut i32) -> i16;
    pub fn SQLNativeSqlW(hdbc: *mut ::core::ffi::c_void, szsqlstrin: *const u16, cchsqlstrin: i32, szsqlstr: *mut u16, cchsqlstrmax: i32, pcchsqlstr: *mut i32) -> i16;
    pub fn SQLNumParams(hstmt: *mut ::core::ffi::c_void, pcpar: *mut i16) -> i16;
    pub fn SQLNumResultCols(statementhandle: *mut ::core::ffi::c_void, columncount: *mut i16) -> i16;
    pub fn SQLParamData(statementhandle: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u64, pirow: *mut u64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLParamOptions(hstmt: *mut ::core::ffi::c_void, crow: u32, pirow: *mut u32) -> i16;
    pub fn SQLPrepare(statementhandle: *mut ::core::ffi::c_void, statementtext: *const u8, textlength: i32) -> i16;
    pub fn SQLPrepareA(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u8, cbsqlstr: i32) -> i16;
    pub fn SQLPrepareW(hstmt: *mut ::core::ffi::c_void, szsqlstr: *const u16, cchsqlstr: i32) -> i16;
    pub fn SQLPrimaryKeys(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    pub fn SQLPrimaryKeysA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    pub fn SQLPrimaryKeysW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    pub fn SQLProcedureColumns(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16, szcolumnname: *const u8, cchcolumnname: i16) -> i16;
    pub fn SQLProcedureColumnsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16, szcolumnname: *const u8, cbcolumnname: i16) -> i16;
    pub fn SQLProcedureColumnsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16, szcolumnname: *const u16, cchcolumnname: i16) -> i16;
    pub fn SQLProcedures(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, szprocname: *const u8, cchprocname: i16) -> i16;
    pub fn SQLProceduresA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, szprocname: *const u8, cbprocname: i16) -> i16;
    pub fn SQLProceduresW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, szprocname: *const u16, cchprocname: i16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLPutData(statementhandle: *mut ::core::ffi::c_void, data: *const ::core::ffi::c_void, strlen_or_ind: i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLRowCount(statementhandle: *const ::core::ffi::c_void, rowcount: *mut i32) -> i16;
    pub fn SQLSetConnectAttr(connectionhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    pub fn SQLSetConnectAttrA(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    pub fn SQLSetConnectAttrW(hdbc: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *const ::core::ffi::c_void, cbvalue: i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOption(connectionhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionA(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetConnectOptionW(hdbc: *mut ::core::ffi::c_void, foption: u16, vparam: u32) -> i16;
    pub fn SQLSetCursorName(statementhandle: *mut ::core::ffi::c_void, cursorname: *const u8, namelength: i16) -> i16;
    pub fn SQLSetCursorNameA(hstmt: *mut ::core::ffi::c_void, szcursor: *const u8, cbcursor: i16) -> i16;
    pub fn SQLSetCursorNameW(hstmt: *mut ::core::ffi::c_void, szcursor: *const u16, cchcursor: i16) -> i16;
    pub fn SQLSetDescField(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *const ::core::ffi::c_void, bufferlength: i32) -> i16;
    pub fn SQLSetDescFieldW(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, fieldidentifier: i16, value: *mut ::core::ffi::c_void, bufferlength: i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i64, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i64, indicator: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetDescRec(descriptorhandle: *mut ::core::ffi::c_void, recnumber: i16, r#type: i16, subtype: i16, length: i32, precision: i16, scale: i16, data: *mut ::core::ffi::c_void, stringlength: *mut i32, indicator: *mut i32) -> i16;
    pub fn SQLSetEnvAttr(environmenthandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u64, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetParam(statementhandle: *mut ::core::ffi::c_void, parameternumber: u16, valuetype: i16, parametertype: i16, lengthprecision: u32, parameterscale: i16, parametervalue: *const ::core::ffi::c_void, strlen_or_ind: *mut i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u64, foption: u16, flock: u16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetPos(hstmt: *mut ::core::ffi::c_void, irow: u16, foption: u16, flock: u16) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i64, crowrowset: u16) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetScrollOptions(hstmt: *mut ::core::ffi::c_void, fconcurrency: u16, crowkeyset: i32, crowrowset: u16) -> i16;
    pub fn SQLSetStmtAttr(statementhandle: *mut ::core::ffi::c_void, attribute: i32, value: *const ::core::ffi::c_void, stringlength: i32) -> i16;
    pub fn SQLSetStmtAttrW(hstmt: *mut ::core::ffi::c_void, fattribute: i32, rgbvalue: *mut ::core::ffi::c_void, cbvaluemax: i32) -> i16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u64) -> i16;
    #[cfg(any(target_arch = "x86",))]
    pub fn SQLSetStmtOption(statementhandle: *mut ::core::ffi::c_void, option: u16, value: u32) -> i16;
    pub fn SQLSpecialColumns(statementhandle: *mut ::core::ffi::c_void, identifiertype: u16, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, scope: u16, nullable: u16) -> i16;
    pub fn SQLSpecialColumnsA(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, fscope: u16, fnullable: u16) -> i16;
    pub fn SQLSpecialColumnsW(hstmt: *mut ::core::ffi::c_void, fcoltype: u16, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, fscope: u16, fnullable: u16) -> i16;
    pub fn SQLStatistics(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, unique: u16, reserved: u16) -> i16;
    pub fn SQLStatisticsA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, funique: u16, faccuracy: u16) -> i16;
    pub fn SQLStatisticsW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, funique: u16, faccuracy: u16) -> i16;
    pub fn SQLTablePrivileges(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cchcatalogname: i16, szschemaname: *const u8, cchschemaname: i16, sztablename: *const u8, cchtablename: i16) -> i16;
    pub fn SQLTablePrivilegesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16) -> i16;
    pub fn SQLTablePrivilegesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16) -> i16;
    pub fn SQLTables(statementhandle: *mut ::core::ffi::c_void, catalogname: *const u8, namelength1: i16, schemaname: *const u8, namelength2: i16, tablename: *const u8, namelength3: i16, tabletype: *const u8, namelength4: i16) -> i16;
    pub fn SQLTablesA(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u8, cbcatalogname: i16, szschemaname: *const u8, cbschemaname: i16, sztablename: *const u8, cbtablename: i16, sztabletype: *const u8, cbtabletype: i16) -> i16;
    pub fn SQLTablesW(hstmt: *mut ::core::ffi::c_void, szcatalogname: *const u16, cchcatalogname: i16, szschemaname: *const u16, cchschemaname: i16, sztablename: *const u16, cchtablename: i16, sztabletype: *const u16, cchtabletype: i16) -> i16;
    pub fn SQLTransact(environmenthandle: *mut ::core::ffi::c_void, connectionhandle: *mut ::core::ffi::c_void, completiontype: u16) -> i16;
    pub fn bcp_batch(param0: *mut ::core::ffi::c_void) -> i32;
    pub fn bcp_bind(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32, param3: i32, param4: *mut u8, param5: i32, param6: i32, param7: i32) -> i16;
    pub fn bcp_colfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: u8, param3: i32, param4: i32, param5: *mut u8, param6: i32, param7: i32) -> i16;
    pub fn bcp_collen(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32) -> i16;
    pub fn bcp_colptr(param0: *mut ::core::ffi::c_void, param1: *mut u8, param2: i32) -> i16;
    pub fn bcp_columns(param0: *mut ::core::ffi::c_void, param1: i32) -> i16;
    pub fn bcp_control(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut ::core::ffi::c_void) -> i16;
    pub fn bcp_done(param0: *mut ::core::ffi::c_void) -> i32;
    pub fn bcp_exec(param0: *mut ::core::ffi::c_void, param1: *mut i32) -> i16;
    pub fn bcp_getcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32, param5: *mut i32) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: super::super::Foundation::PSTR, param4: i32) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_initW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: super::super::Foundation::PWSTR, param4: i32) -> i16;
    pub fn bcp_moretext(param0: *mut ::core::ffi::c_void, param1: i32, param2: *mut u8) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_readfmtW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR) -> i16;
    pub fn bcp_sendrow(param0: *mut ::core::ffi::c_void) -> i16;
    pub fn bcp_setcolfmt(param0: *mut ::core::ffi::c_void, param1: i32, param2: i32, param3: *mut ::core::ffi::c_void, param4: i32) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtA(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PSTR) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bcp_writefmtW(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::PWSTR) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeA(param0: i32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn dbprtypeW(param0: i32) -> super::super::Foundation::PWSTR;
}
#[repr(transparent)]
pub struct ACCESS_MASKENUM(pub i32);
pub const PERM_EXCLUSIVE: ACCESS_MASKENUM = ACCESS_MASKENUM(512i32);
pub const PERM_READDESIGN: ACCESS_MASKENUM = ACCESS_MASKENUM(1024i32);
pub const PERM_WRITEDESIGN: ACCESS_MASKENUM = ACCESS_MASKENUM(2048i32);
pub const PERM_WITHGRANT: ACCESS_MASKENUM = ACCESS_MASKENUM(4096i32);
pub const PERM_REFERENCE: ACCESS_MASKENUM = ACCESS_MASKENUM(8192i32);
pub const PERM_CREATE: ACCESS_MASKENUM = ACCESS_MASKENUM(16384i32);
pub const PERM_INSERT: ACCESS_MASKENUM = ACCESS_MASKENUM(32768i32);
pub const PERM_DELETE: ACCESS_MASKENUM = ACCESS_MASKENUM(65536i32);
pub const PERM_READCONTROL: ACCESS_MASKENUM = ACCESS_MASKENUM(131072i32);
pub const PERM_WRITEPERMISSIONS: ACCESS_MASKENUM = ACCESS_MASKENUM(262144i32);
pub const PERM_WRITEOWNER: ACCESS_MASKENUM = ACCESS_MASKENUM(524288i32);
pub const PERM_MAXIMUM_ALLOWED: ACCESS_MASKENUM = ACCESS_MASKENUM(33554432i32);
pub const PERM_ALL: ACCESS_MASKENUM = ACCESS_MASKENUM(268435456i32);
pub const PERM_EXECUTE: ACCESS_MASKENUM = ACCESS_MASKENUM(536870912i32);
pub const PERM_READ: ACCESS_MASKENUM = ACCESS_MASKENUM(-2147483648i32);
pub const PERM_UPDATE: ACCESS_MASKENUM = ACCESS_MASKENUM(1073741824i32);
pub const PERM_DROP: ACCESS_MASKENUM = ACCESS_MASKENUM(256i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUTHENTICATION_INFO(i32);
#[repr(transparent)]
pub struct AUTH_TYPE(pub i32);
pub const eAUTH_TYPE_ANONYMOUS: AUTH_TYPE = AUTH_TYPE(0i32);
pub const eAUTH_TYPE_NTLM: AUTH_TYPE = AUTH_TYPE(1i32);
pub const eAUTH_TYPE_BASIC: AUTH_TYPE = AUTH_TYPE(2i32);
pub const BCP6xFILEFMT: u32 = 9u32;
pub const BCPABORT: u32 = 6u32;
pub const BCPBATCH: u32 = 4u32;
pub const BCPFILECP: u32 = 12u32;
pub const BCPFILECP_ACP: u32 = 0u32;
pub const BCPFILECP_OEMCP: u32 = 1u32;
pub const BCPFILECP_RAW: i32 = -1i32;
pub const BCPFILEFMT: u32 = 15u32;
pub const BCPFIRST: u32 = 2u32;
pub const BCPHINTS: u32 = 11u32;
pub const BCPHINTSA: u32 = 10u32;
pub const BCPHINTSW: u32 = 11u32;
pub const BCPKEEPIDENTITY: u32 = 8u32;
pub const BCPKEEPNULLS: u32 = 5u32;
pub const BCPLAST: u32 = 3u32;
pub const BCPMAXERRS: u32 = 1u32;
pub const BCPODBC: u32 = 7u32;
pub const BCPTEXTFILE: u32 = 14u32;
pub const BCPUNICODEFILE: u32 = 13u32;
pub const BCP_FMT_COLLATION: u32 = 6u32;
pub const BCP_FMT_COLLATION_ID: u32 = 7u32;
pub const BCP_FMT_DATA_LEN: u32 = 3u32;
pub const BCP_FMT_INDICATOR_LEN: u32 = 2u32;
pub const BCP_FMT_SERVER_COL: u32 = 5u32;
pub const BCP_FMT_TERMINATOR: u32 = 4u32;
pub const BCP_FMT_TYPE: u32 = 1u32;
pub const BMK_DURABILITY_INTRANSACTION: i32 = 1i32;
pub const BMK_DURABILITY_REORGANIZATION: i32 = 3i32;
pub const BMK_DURABILITY_ROWSET: i32 = 0i32;
pub const BMK_DURABILITY_XTRANSACTION: i32 = 2i32;
#[repr(C)]
pub struct BUCKETCATEGORIZE(i32);
pub const BUCKET_EXPONENTIAL: u32 = 1u32;
pub const BUCKET_LINEAR: u32 = 0u32;
#[repr(transparent)]
pub struct CASE_REQUIREMENT(pub i32);
pub const CASE_REQUIREMENT_ANY: CASE_REQUIREMENT = CASE_REQUIREMENT(0i32);
pub const CASE_REQUIREMENT_UPPER_IF_AQS: CASE_REQUIREMENT = CASE_REQUIREMENT(1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct CATEGORIZATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct CATEGORIZATIONSET(i32);
pub const CATEGORIZE_BUCKETS: u32 = 2u32;
pub const CATEGORIZE_CLUSTER: u32 = 1u32;
pub const CATEGORIZE_RANGE: u32 = 3u32;
pub const CATEGORIZE_UNIQUE: u32 = 0u32;
pub const CATEGORY_COLLATOR: i32 = 2i32;
pub const CATEGORY_GATHERER: i32 = 3i32;
pub const CATEGORY_INDEXER: i32 = 4i32;
pub const CATEGORY_SEARCH: i32 = 1i32;
pub const CDBBMKDISPIDS: u32 = 8u32;
pub const CDBCOLDISPIDS: u32 = 28u32;
pub const CDBSELFDISPIDS: u32 = 8u32;
pub const CERT_E_NOT_FOUND_OR_NO_PERMISSSION: i32 = -2147211263i32;
#[repr(transparent)]
pub struct CHANNEL_AGENT_FLAGS(pub i32);
pub const CHANNEL_AGENT_DYNAMIC_SCHEDULE: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(1i32);
pub const CHANNEL_AGENT_PRECACHE_SOME: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(2i32);
pub const CHANNEL_AGENT_PRECACHE_ALL: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(4i32);
pub const CHANNEL_AGENT_PRECACHE_SCRNSAVER: CHANNEL_AGENT_FLAGS = CHANNEL_AGENT_FLAGS(8i32);
pub const CI_E_CORRUPT_FWIDX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473491i32 as _);
pub const CI_E_DIACRITIC_SETTINGS_DIFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473490i32 as _);
pub const CI_E_INCONSISTENT_TRANSACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473486i32 as _);
pub const CI_E_INVALID_CATALOG_LIST_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215313i32 as _);
pub const CI_E_MULTIPLE_PROTECTED_USERS_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473483i32 as _);
pub const CI_E_NO_AUXMETADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215318i32 as _);
pub const CI_E_NO_CATALOG_MANAGER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473487i32 as _);
pub const CI_E_NO_PROTECTED_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473484i32 as _);
pub const CI_E_PROTECTED_CATALOG_NON_INTERACTIVE_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473481i32 as _);
pub const CI_E_PROTECTED_CATALOG_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473485i32 as _);
pub const CI_E_PROTECTED_CATALOG_SID_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073473482i32 as _);
pub const CI_S_CATALOG_RESET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268336i32 as _);
pub const CI_S_CLIENT_REQUESTED_ABORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268331i32 as _);
pub const CI_S_NEW_AUXMETADATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268329i32 as _);
pub const CI_S_RETRY_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268332i32 as _);
pub const CLSID_DataShapeProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 877240776, data2: 50540, data3: 4560, data4: [173, 114, 0, 192, 79, 194, 152, 99] };
pub const CLSID_MSDASQL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3367314123, data2: 23795, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
pub const CLSID_MSDASQL_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3367314125, data2: 23795, data3: 4558, data4: [173, 229, 0, 170, 0, 68, 119, 61] };
pub const CLSID_MSPersist: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2080891088, data2: 17432, data3: 4562, data4: [146, 18, 0, 192, 79, 187, 191, 179] };
pub const CLSID_SQLOLEDB: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 209711468, data2: 14563, data3: 4560, data4: [151, 171, 0, 192, 79, 194, 173, 152] };
pub const CLSID_SQLOLEDB_ENUMERATOR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3751947150, data2: 59021, data3: 4560, data4: [151, 228, 0, 192, 79, 194, 173, 152] };
pub const CLSID_SQLOLEDB_ERROR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3230870626, data2: 14565, data3: 4560, data4: [151, 171, 0, 192, 79, 194, 173, 152] };
#[repr(transparent)]
pub struct CLUSION_REASON(pub i32);
pub const CLUSIONREASON_UNKNOWNSCOPE: CLUSION_REASON = CLUSION_REASON(0i32);
pub const CLUSIONREASON_DEFAULT: CLUSION_REASON = CLUSION_REASON(1i32);
pub const CLUSIONREASON_USER: CLUSION_REASON = CLUSION_REASON(2i32);
pub const CLUSIONREASON_GROUPPOLICY: CLUSION_REASON = CLUSION_REASON(3i32);
pub const CMDLINE_E_ALREADY_INIT: i32 = -2147216123i32;
pub const CMDLINE_E_NOT_INIT: i32 = -2147216124i32;
pub const CMDLINE_E_NUM_PARAMS: i32 = -2147216122i32;
pub const CMDLINE_E_PARAM_SIZE: i32 = -2147216125i32;
pub const CMDLINE_E_PAREN: i32 = -2147216126i32;
pub const CMDLINE_E_UNEXPECTED: i32 = -2147216127i32;
pub const CM_E_CONNECTIONTIMEOUT: i32 = -2147219963i32;
pub const CM_E_DATASOURCENOTAVAILABLE: i32 = -2147219964i32;
pub const CM_E_INSUFFICIENTBUFFER: i32 = -2147219957i32;
pub const CM_E_INVALIDDATASOURCE: i32 = -2147219959i32;
pub const CM_E_NOQUERYCONNECTIONS: i32 = -2147219965i32;
pub const CM_E_REGISTRY: i32 = -2147219960i32;
pub const CM_E_SERVERNOTFOUND: i32 = -2147219962i32;
pub const CM_E_TIMEOUT: i32 = -2147219958i32;
pub const CM_E_TOOMANYDATASERVERS: i32 = -2147219967i32;
pub const CM_E_TOOMANYDATASOURCES: i32 = -2147219966i32;
pub const CM_S_NODATASERVERS: i32 = 263687i32;
pub const COLL_E_BADRESULT: i32 = -2147220218i32;
pub const COLL_E_BADSEQUENCE: i32 = -2147220223i32;
pub const COLL_E_BUFFERTOOSMALL: i32 = -2147220220i32;
pub const COLL_E_DUPLICATEDBID: i32 = -2147220216i32;
pub const COLL_E_INCOMPATIBLECOLUMNS: i32 = -2147220221i32;
pub const COLL_E_MAXCONNEXCEEDED: i32 = -2147220213i32;
pub const COLL_E_NODEFAULTCATALOG: i32 = -2147220214i32;
pub const COLL_E_NOMOREDATA: i32 = -2147220222i32;
pub const COLL_E_NOSORTCOLUMN: i32 = -2147220217i32;
pub const COLL_E_TOOMANYMERGECOLUMNS: i32 = -2147220215i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct COLUMNSET(i32);
#[repr(transparent)]
pub struct CONDITION_CREATION_OPTIONS(pub u32);
pub const CONDITION_CREATION_DEFAULT: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(0u32);
pub const CONDITION_CREATION_NONE: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(0u32);
pub const CONDITION_CREATION_SIMPLIFY: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(1u32);
pub const CONDITION_CREATION_VECTOR_AND: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(2u32);
pub const CONDITION_CREATION_VECTOR_OR: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(4u32);
pub const CONDITION_CREATION_VECTOR_LEAF: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(8u32);
pub const CONDITION_CREATION_USE_CONTENT_LOCALE: CONDITION_CREATION_OPTIONS = CONDITION_CREATION_OPTIONS(16u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct CONTENTRESTRICTION(i32);
pub const CONTENT_SOURCE_E_CONTENT_CLASS_READ: i32 = -2147208188i32;
pub const CONTENT_SOURCE_E_CONTENT_SOURCE_COLUMN_TYPE: i32 = -2147208185i32;
pub const CONTENT_SOURCE_E_NULL_CONTENT_CLASS_BSTR: i32 = -2147208186i32;
pub const CONTENT_SOURCE_E_NULL_URI: i32 = -2147208183i32;
pub const CONTENT_SOURCE_E_OUT_OF_RANGE: i32 = -2147208184i32;
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_BAD_VECTOR_SIZE: i32 = -2147208189i32;
pub const CONTENT_SOURCE_E_PROPERTY_MAPPING_READ: i32 = -2147208191i32;
pub const CONTENT_SOURCE_E_UNEXPECTED_EXCEPTION: i32 = -2147208187i32;
pub const CONTENT_SOURCE_E_UNEXPECTED_NULL_POINTER: i32 = -2147208190i32;
pub const CQUERYDISPIDS: u32 = 11u32;
pub const CQUERYMETADISPIDS: u32 = 10u32;
pub const CQUERYPROPERTY: u32 = 64u32;
#[repr(transparent)]
pub struct CREATESUBSCRIPTIONFLAGS(pub i32);
pub const CREATESUBS_ADDTOFAVORITES: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(1i32);
pub const CREATESUBS_FROMFAVORITES: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(2i32);
pub const CREATESUBS_NOUI: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(4i32);
pub const CREATESUBS_NOSAVE: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(8i32);
pub const CREATESUBS_SOFTWAREUPDATE: CREATESUBSCRIPTIONFLAGS = CREATESUBSCRIPTIONFLAGS(16i32);
pub const CRESTRICTIONS_DBSCHEMA_ASSERTIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CATALOGS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_CHARACTER_SETS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_CHECK_CONSTRAINTS_BY_TABLE: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_COLLATIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMNS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_DOMAIN_USAGE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_COLUMN_PRIVILEGES: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_COLUMN_USAGE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_CONSTRAINT_TABLE_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_FOREIGN_KEYS: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_INDEXES: u32 = 5u32;
pub const CRESTRICTIONS_DBSCHEMA_KEY_COLUMN_USAGE: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_LINKEDSERVERS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_OBJECTS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_OBJECT_ACTIONS: u32 = 1u32;
pub const CRESTRICTIONS_DBSCHEMA_PRIMARY_KEYS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURES: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_COLUMNS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROCEDURE_PARAMETERS: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_PROVIDER_TYPES: u32 = 2u32;
pub const CRESTRICTIONS_DBSCHEMA_REFERENTIAL_CONSTRAINTS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_SCHEMATA: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_SQL_LANGUAGES: u32 = 0u32;
pub const CRESTRICTIONS_DBSCHEMA_STATISTICS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLES: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLES_INFO: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_CONSTRAINTS: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_PRIVILEGES: u32 = 5u32;
pub const CRESTRICTIONS_DBSCHEMA_TABLE_STATISTICS: u32 = 7u32;
pub const CRESTRICTIONS_DBSCHEMA_TRANSLATIONS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_TRUSTEE: u32 = 4u32;
pub const CRESTRICTIONS_DBSCHEMA_USAGE_PRIVILEGES: u32 = 6u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEWS: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_COLUMN_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_DBSCHEMA_VIEW_TABLE_USAGE: u32 = 3u32;
pub const CRESTRICTIONS_MDSCHEMA_ACTIONS: u32 = 8u32;
pub const CRESTRICTIONS_MDSCHEMA_COMMANDS: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_CUBES: u32 = 3u32;
pub const CRESTRICTIONS_MDSCHEMA_DIMENSIONS: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_FUNCTIONS: u32 = 4u32;
pub const CRESTRICTIONS_MDSCHEMA_HIERARCHIES: u32 = 6u32;
pub const CRESTRICTIONS_MDSCHEMA_LEVELS: u32 = 7u32;
pub const CRESTRICTIONS_MDSCHEMA_MEASURES: u32 = 5u32;
pub const CRESTRICTIONS_MDSCHEMA_MEMBERS: u32 = 12u32;
pub const CRESTRICTIONS_MDSCHEMA_PROPERTIES: u32 = 9u32;
pub const CRESTRICTIONS_MDSCHEMA_SETS: u32 = 5u32;
pub const CSTORAGEPROPERTY: u32 = 23u32;
#[repr(C)]
pub struct CSearchLanguageSupport(i32);
#[repr(C)]
pub struct CSearchManager(i32);
#[repr(C)]
pub struct CSearchRoot(i32);
#[repr(C)]
pub struct CSearchScopeRule(i32);
#[repr(transparent)]
pub struct CatalogPausedReason(pub i32);
pub const CATALOG_PAUSED_REASON_NONE: CatalogPausedReason = CatalogPausedReason(0i32);
pub const CATALOG_PAUSED_REASON_HIGH_IO: CatalogPausedReason = CatalogPausedReason(1i32);
pub const CATALOG_PAUSED_REASON_HIGH_CPU: CatalogPausedReason = CatalogPausedReason(2i32);
pub const CATALOG_PAUSED_REASON_HIGH_NTF_RATE: CatalogPausedReason = CatalogPausedReason(3i32);
pub const CATALOG_PAUSED_REASON_LOW_BATTERY: CatalogPausedReason = CatalogPausedReason(4i32);
pub const CATALOG_PAUSED_REASON_LOW_MEMORY: CatalogPausedReason = CatalogPausedReason(5i32);
pub const CATALOG_PAUSED_REASON_LOW_DISK: CatalogPausedReason = CatalogPausedReason(6i32);
pub const CATALOG_PAUSED_REASON_DELAYED_RECOVERY: CatalogPausedReason = CatalogPausedReason(7i32);
pub const CATALOG_PAUSED_REASON_USER_ACTIVE: CatalogPausedReason = CatalogPausedReason(8i32);
pub const CATALOG_PAUSED_REASON_EXTERNAL: CatalogPausedReason = CatalogPausedReason(9i32);
pub const CATALOG_PAUSED_REASON_UPGRADING: CatalogPausedReason = CatalogPausedReason(10i32);
#[repr(transparent)]
pub struct CatalogStatus(pub i32);
pub const CATALOG_STATUS_IDLE: CatalogStatus = CatalogStatus(0i32);
pub const CATALOG_STATUS_PAUSED: CatalogStatus = CatalogStatus(1i32);
pub const CATALOG_STATUS_RECOVERING: CatalogStatus = CatalogStatus(2i32);
pub const CATALOG_STATUS_FULL_CRAWL: CatalogStatus = CatalogStatus(3i32);
pub const CATALOG_STATUS_INCREMENTAL_CRAWL: CatalogStatus = CatalogStatus(4i32);
pub const CATALOG_STATUS_PROCESSING_NOTIFICATIONS: CatalogStatus = CatalogStatus(5i32);
pub const CATALOG_STATUS_SHUTTING_DOWN: CatalogStatus = CatalogStatus(6i32);
#[repr(C)]
pub struct CompoundCondition(i32);
#[repr(C)]
pub struct ConditionFactory(i32);
#[repr(C)]
pub struct DATE_STRUCT(i32);
#[repr(transparent)]
pub struct DBACCESSORFLAGSENUM(pub i32);
pub const DBACCESSOR_INVALID: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(0i32);
pub const DBACCESSOR_PASSBYREF: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(1i32);
pub const DBACCESSOR_ROWDATA: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(2i32);
pub const DBACCESSOR_PARAMETERDATA: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(4i32);
pub const DBACCESSOR_OPTIMIZED: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(8i32);
pub const DBACCESSOR_INHERITED: DBACCESSORFLAGSENUM = DBACCESSORFLAGSENUM(16i32);
#[repr(transparent)]
pub struct DBASYNCHOPENUM(pub i32);
pub const DBASYNCHOP_OPEN: DBASYNCHOPENUM = DBASYNCHOPENUM(0i32);
#[repr(transparent)]
pub struct DBASYNCHPHASEENUM(pub i32);
pub const DBASYNCHPHASE_INITIALIZATION: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(0i32);
pub const DBASYNCHPHASE_POPULATION: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(1i32);
pub const DBASYNCHPHASE_COMPLETE: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(2i32);
pub const DBASYNCHPHASE_CANCELED: DBASYNCHPHASEENUM = DBASYNCHPHASEENUM(3i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBBINDEXT(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBBINDEXT(i32);
#[repr(transparent)]
pub struct DBBINDFLAGENUM(pub i32);
pub const DBBINDFLAG_HTML: DBBINDFLAGENUM = DBBINDFLAGENUM(1i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DBBINDING(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DBBINDING(i32);
#[repr(transparent)]
pub struct DBBINDSTATUSENUM(pub i32);
pub const DBBINDSTATUS_OK: DBBINDSTATUSENUM = DBBINDSTATUSENUM(0i32);
pub const DBBINDSTATUS_BADORDINAL: DBBINDSTATUSENUM = DBBINDSTATUSENUM(1i32);
pub const DBBINDSTATUS_UNSUPPORTEDCONVERSION: DBBINDSTATUSENUM = DBBINDSTATUSENUM(2i32);
pub const DBBINDSTATUS_BADBINDINFO: DBBINDSTATUSENUM = DBBINDSTATUSENUM(3i32);
pub const DBBINDSTATUS_BADSTORAGEFLAGS: DBBINDSTATUSENUM = DBBINDSTATUSENUM(4i32);
pub const DBBINDSTATUS_NOINTERFACE: DBBINDSTATUSENUM = DBBINDSTATUSENUM(5i32);
pub const DBBINDSTATUS_MULTIPLESTORAGE: DBBINDSTATUSENUM = DBBINDSTATUSENUM(6i32);
#[repr(transparent)]
pub struct DBBINDURLFLAGENUM(pub i32);
pub const DBBINDURLFLAG_READ: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(1i32);
pub const DBBINDURLFLAG_WRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(2i32);
pub const DBBINDURLFLAG_READWRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(3i32);
pub const DBBINDURLFLAG_SHARE_DENY_READ: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4i32);
pub const DBBINDURLFLAG_SHARE_DENY_WRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8i32);
pub const DBBINDURLFLAG_SHARE_EXCLUSIVE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(12i32);
pub const DBBINDURLFLAG_SHARE_DENY_NONE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16i32);
pub const DBBINDURLFLAG_ASYNCHRONOUS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4096i32);
pub const DBBINDURLFLAG_COLLECTION: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8192i32);
pub const DBBINDURLFLAG_DELAYFETCHSTREAM: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16384i32);
pub const DBBINDURLFLAG_DELAYFETCHCOLUMNS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(32768i32);
pub const DBBINDURLFLAG_RECURSIVE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(4194304i32);
pub const DBBINDURLFLAG_OUTPUT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(8388608i32);
pub const DBBINDURLFLAG_WAITFORINIT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(16777216i32);
pub const DBBINDURLFLAG_OPENIFEXISTS: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(33554432i32);
pub const DBBINDURLFLAG_OVERWRITE: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(67108864i32);
pub const DBBINDURLFLAG_ISSTRUCTUREDDOCUMENT: DBBINDURLFLAGENUM = DBBINDURLFLAGENUM(134217728i32);
#[repr(transparent)]
pub struct DBBINDURLSTATUSENUM(pub i32);
pub const DBBINDURLSTATUS_S_OK: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(0i32);
pub const DBBINDURLSTATUS_S_DENYNOTSUPPORTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(1i32);
pub const DBBINDURLSTATUS_S_DENYTYPENOTSUPPORTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(4i32);
pub const DBBINDURLSTATUS_S_REDIRECTED: DBBINDURLSTATUSENUM = DBBINDURLSTATUSENUM(8i32);
#[repr(transparent)]
pub struct DBBOOKMARK(pub i32);
pub const DBBMK_INVALID: DBBOOKMARK = DBBOOKMARK(0i32);
pub const DBBMK_FIRST: DBBOOKMARK = DBBOOKMARK(1i32);
pub const DBBMK_LAST: DBBOOKMARK = DBBOOKMARK(2i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct DBCOLUMNACCESS(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct DBCOLUMNACCESS(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBCOLUMNDESC(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBCOLUMNDESC(i32);
#[repr(transparent)]
pub struct DBCOLUMNDESCFLAGSENUM(pub i32);
pub const DBCOLUMNDESCFLAGS_TYPENAME: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(1i32);
pub const DBCOLUMNDESCFLAGS_ITYPEINFO: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(2i32);
pub const DBCOLUMNDESCFLAGS_PROPERTIES: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(4i32);
pub const DBCOLUMNDESCFLAGS_CLSID: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(8i32);
pub const DBCOLUMNDESCFLAGS_COLSIZE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(16i32);
pub const DBCOLUMNDESCFLAGS_DBCID: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(32i32);
pub const DBCOLUMNDESCFLAGS_WTYPE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(64i32);
pub const DBCOLUMNDESCFLAGS_PRECISION: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(128i32);
pub const DBCOLUMNDESCFLAGS_SCALE: DBCOLUMNDESCFLAGSENUM = DBCOLUMNDESCFLAGSENUM(256i32);
#[repr(transparent)]
pub struct DBCOLUMNFLAGS15ENUM(pub i32);
pub const DBCOLUMNFLAGS_ISCHAPTER: DBCOLUMNFLAGS15ENUM = DBCOLUMNFLAGS15ENUM(8192i32);
#[repr(transparent)]
pub struct DBCOLUMNFLAGSENUM(pub i32);
pub const DBCOLUMNFLAGS_ISBOOKMARK: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(1i32);
pub const DBCOLUMNFLAGS_MAYDEFER: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(2i32);
pub const DBCOLUMNFLAGS_WRITE: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(4i32);
pub const DBCOLUMNFLAGS_WRITEUNKNOWN: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(8i32);
pub const DBCOLUMNFLAGS_ISFIXEDLENGTH: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(16i32);
pub const DBCOLUMNFLAGS_ISNULLABLE: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(32i32);
pub const DBCOLUMNFLAGS_MAYBENULL: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(64i32);
pub const DBCOLUMNFLAGS_ISLONG: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(128i32);
pub const DBCOLUMNFLAGS_ISROWID: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(256i32);
pub const DBCOLUMNFLAGS_ISROWVER: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(512i32);
pub const DBCOLUMNFLAGS_CACHEDEFERRED: DBCOLUMNFLAGSENUM = DBCOLUMNFLAGSENUM(4096i32);
#[repr(transparent)]
pub struct DBCOLUMNFLAGSENUM20(pub i32);
pub const DBCOLUMNFLAGS_SCALEISNEGATIVE: DBCOLUMNFLAGSENUM20 = DBCOLUMNFLAGSENUM20(16384i32);
pub const DBCOLUMNFLAGS_RESERVED: DBCOLUMNFLAGSENUM20 = DBCOLUMNFLAGSENUM20(32768i32);
#[repr(transparent)]
pub struct DBCOLUMNFLAGSENUM21(pub i32);
pub const DBCOLUMNFLAGS_ISROWURL: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(65536i32);
pub const DBCOLUMNFLAGS_ISDEFAULTSTREAM: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(131072i32);
pub const DBCOLUMNFLAGS_ISCOLLECTION: DBCOLUMNFLAGSENUM21 = DBCOLUMNFLAGSENUM21(262144i32);
#[repr(transparent)]
pub struct DBCOLUMNFLAGSENUM26(pub i32);
pub const DBCOLUMNFLAGS_ISSTREAM: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(524288i32);
pub const DBCOLUMNFLAGS_ISROWSET: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(1048576i32);
pub const DBCOLUMNFLAGS_ISROW: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(2097152i32);
pub const DBCOLUMNFLAGS_ROWSPECIFICCOLUMN: DBCOLUMNFLAGSENUM26 = DBCOLUMNFLAGSENUM26(4194304i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct DBCOLUMNINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct DBCOLUMNINFO(i32);
#[repr(transparent)]
pub struct DBCOMMANDPERSISTFLAGENUM(pub i32);
pub const DBCOMMANDPERSISTFLAG_NOSAVE: DBCOMMANDPERSISTFLAGENUM = DBCOMMANDPERSISTFLAGENUM(1i32);
#[repr(transparent)]
pub struct DBCOMMANDPERSISTFLAGENUM21(pub i32);
pub const DBCOMMANDPERSISTFLAG_DEFAULT: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(0i32);
pub const DBCOMMANDPERSISTFLAG_PERSISTVIEW: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(2i32);
pub const DBCOMMANDPERSISTFLAG_PERSISTPROCEDURE: DBCOMMANDPERSISTFLAGENUM21 = DBCOMMANDPERSISTFLAGENUM21(4i32);
#[repr(transparent)]
pub struct DBCOMPAREENUM(pub i32);
pub const DBCOMPARE_LT: DBCOMPAREENUM = DBCOMPAREENUM(0i32);
pub const DBCOMPARE_EQ: DBCOMPAREENUM = DBCOMPAREENUM(1i32);
pub const DBCOMPARE_GT: DBCOMPAREENUM = DBCOMPAREENUM(2i32);
pub const DBCOMPARE_NE: DBCOMPAREENUM = DBCOMPAREENUM(3i32);
pub const DBCOMPARE_NOTCOMPARABLE: DBCOMPAREENUM = DBCOMPAREENUM(4i32);
#[repr(transparent)]
pub struct DBCOMPAREOPSENUM(pub i32);
pub const DBCOMPAREOPS_LT: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(0i32);
pub const DBCOMPAREOPS_LE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(1i32);
pub const DBCOMPAREOPS_EQ: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(2i32);
pub const DBCOMPAREOPS_GE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(3i32);
pub const DBCOMPAREOPS_GT: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(4i32);
pub const DBCOMPAREOPS_BEGINSWITH: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(5i32);
pub const DBCOMPAREOPS_CONTAINS: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(6i32);
pub const DBCOMPAREOPS_NE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(7i32);
pub const DBCOMPAREOPS_IGNORE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(8i32);
pub const DBCOMPAREOPS_CASESENSITIVE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(4096i32);
pub const DBCOMPAREOPS_CASEINSENSITIVE: DBCOMPAREOPSENUM = DBCOMPAREOPSENUM(8192i32);
#[repr(transparent)]
pub struct DBCOMPAREOPSENUM20(pub i32);
pub const DBCOMPAREOPS_NOTBEGINSWITH: DBCOMPAREOPSENUM20 = DBCOMPAREOPSENUM20(9i32);
pub const DBCOMPAREOPS_NOTCONTAINS: DBCOMPAREOPSENUM20 = DBCOMPAREOPSENUM20(10i32);
pub const DBCOMPUTEMODE_COMPUTED: u32 = 1u32;
pub const DBCOMPUTEMODE_DYNAMIC: u32 = 2u32;
pub const DBCOMPUTEMODE_NOTCOMPUTED: u32 = 3u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBCONSTRAINTDESC(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBCONSTRAINTDESC(i32);
#[repr(transparent)]
pub struct DBCONSTRAINTTYPEENUM(pub i32);
pub const DBCONSTRAINTTYPE_UNIQUE: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(0i32);
pub const DBCONSTRAINTTYPE_FOREIGNKEY: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(1i32);
pub const DBCONSTRAINTTYPE_PRIMARYKEY: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(2i32);
pub const DBCONSTRAINTTYPE_CHECK: DBCONSTRAINTTYPEENUM = DBCONSTRAINTTYPEENUM(3i32);
#[repr(transparent)]
pub struct DBCONVERTFLAGSENUM(pub i32);
pub const DBCONVERTFLAGS_COLUMN: DBCONVERTFLAGSENUM = DBCONVERTFLAGSENUM(0i32);
pub const DBCONVERTFLAGS_PARAMETER: DBCONVERTFLAGSENUM = DBCONVERTFLAGSENUM(1i32);
#[repr(transparent)]
pub struct DBCONVERTFLAGSENUM20(pub i32);
pub const DBCONVERTFLAGS_ISLONG: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(2i32);
pub const DBCONVERTFLAGS_ISFIXEDLENGTH: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(4i32);
pub const DBCONVERTFLAGS_FROMVARIANT: DBCONVERTFLAGSENUM20 = DBCONVERTFLAGSENUM20(8i32);
#[repr(transparent)]
pub struct DBCOPYFLAGSENUM(pub i32);
pub const DBCOPY_ASYNC: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(256i32);
pub const DBCOPY_REPLACE_EXISTING: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(512i32);
pub const DBCOPY_ALLOW_EMULATION: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(1024i32);
pub const DBCOPY_NON_RECURSIVE: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(2048i32);
pub const DBCOPY_ATOMIC: DBCOPYFLAGSENUM = DBCOPYFLAGSENUM(4096i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBCOST(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBCOST(i32);
#[repr(transparent)]
pub struct DBCOSTUNITENUM(pub i32);
pub const DBUNIT_INVALID: DBCOSTUNITENUM = DBCOSTUNITENUM(0i32);
pub const DBUNIT_WEIGHT: DBCOSTUNITENUM = DBCOSTUNITENUM(1i32);
pub const DBUNIT_PERCENT: DBCOSTUNITENUM = DBCOSTUNITENUM(2i32);
pub const DBUNIT_MAXIMUM: DBCOSTUNITENUM = DBCOSTUNITENUM(4i32);
pub const DBUNIT_MINIMUM: DBCOSTUNITENUM = DBCOSTUNITENUM(8i32);
pub const DBUNIT_MICRO_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(16i32);
pub const DBUNIT_MILLI_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(32i32);
pub const DBUNIT_SECOND: DBCOSTUNITENUM = DBCOSTUNITENUM(64i32);
pub const DBUNIT_MINUTE: DBCOSTUNITENUM = DBCOSTUNITENUM(128i32);
pub const DBUNIT_HOUR: DBCOSTUNITENUM = DBCOSTUNITENUM(256i32);
pub const DBUNIT_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(512i32);
pub const DBUNIT_KILO_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(1024i32);
pub const DBUNIT_MEGA_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(2048i32);
pub const DBUNIT_GIGA_BYTE: DBCOSTUNITENUM = DBCOSTUNITENUM(4096i32);
pub const DBUNIT_NUM_MSGS: DBCOSTUNITENUM = DBCOSTUNITENUM(8192i32);
pub const DBUNIT_NUM_LOCKS: DBCOSTUNITENUM = DBCOSTUNITENUM(16384i32);
pub const DBUNIT_NUM_ROWS: DBCOSTUNITENUM = DBCOSTUNITENUM(32768i32);
pub const DBUNIT_OTHER: DBCOSTUNITENUM = DBCOSTUNITENUM(65536i32);
#[repr(transparent)]
pub struct DBDATACONVERTENUM(pub i32);
pub const DBDATACONVERT_DEFAULT: DBDATACONVERTENUM = DBDATACONVERTENUM(0i32);
pub const DBDATACONVERT_SETDATABEHAVIOR: DBDATACONVERTENUM = DBDATACONVERTENUM(1i32);
pub const DBDATACONVERT_LENGTHFROMNTS: DBDATACONVERTENUM = DBDATACONVERTENUM(2i32);
pub const DBDATACONVERT_DSTISFIXEDLENGTH: DBDATACONVERTENUM = DBDATACONVERTENUM(4i32);
pub const DBDATACONVERT_DECIMALSCALE: DBDATACONVERTENUM = DBDATACONVERTENUM(8i32);
#[repr(C)]
pub struct DBDATE(i32);
#[repr(transparent)]
pub struct DBDEFERRABILITYENUM(pub i32);
pub const DBDEFERRABILITY_DEFERRED: DBDEFERRABILITYENUM = DBDEFERRABILITYENUM(1i32);
pub const DBDEFERRABILITY_DEFERRABLE: DBDEFERRABILITYENUM = DBDEFERRABILITYENUM(2i32);
#[repr(transparent)]
pub struct DBDELETEFLAGSENUM(pub i32);
pub const DBDELETE_ASYNC: DBDELETEFLAGSENUM = DBDELETEFLAGSENUM(256i32);
pub const DBDELETE_ATOMIC: DBDELETEFLAGSENUM = DBDELETEFLAGSENUM(4096i32);
#[repr(transparent)]
pub struct DBEVENTPHASEENUM(pub i32);
pub const DBEVENTPHASE_OKTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(0i32);
pub const DBEVENTPHASE_ABOUTTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(1i32);
pub const DBEVENTPHASE_SYNCHAFTER: DBEVENTPHASEENUM = DBEVENTPHASEENUM(2i32);
pub const DBEVENTPHASE_FAILEDTODO: DBEVENTPHASEENUM = DBEVENTPHASEENUM(3i32);
pub const DBEVENTPHASE_DIDEVENT: DBEVENTPHASEENUM = DBEVENTPHASEENUM(4i32);
#[repr(transparent)]
pub struct DBEXECLIMITSENUM(pub i32);
pub const DBEXECLIMITS_ABORT: DBEXECLIMITSENUM = DBEXECLIMITSENUM(1i32);
pub const DBEXECLIMITS_STOP: DBEXECLIMITSENUM = DBEXECLIMITSENUM(2i32);
pub const DBEXECLIMITS_SUSPEND: DBEXECLIMITSENUM = DBEXECLIMITSENUM(3i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBFAILUREINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBFAILUREINFO(i32);
pub const DBGUID_MSSQLXML: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1565727922, data2: 59117, data3: 4562, data4: [178, 82, 0, 192, 79, 104, 27, 113] };
pub const DBGUID_XPATH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3962192531, data2: 59544, data3: 4562, data4: [177, 183, 0, 192, 79, 104, 12, 86] };
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBIMPLICITSESSION(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBIMPLICITSESSION(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct DBINDEXCOLUMNDESC(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct DBINDEXCOLUMNDESC(i32);
#[repr(transparent)]
pub struct DBINDEX_COL_ORDERENUM(pub i32);
pub const DBINDEX_COL_ORDER_ASC: DBINDEX_COL_ORDERENUM = DBINDEX_COL_ORDERENUM(0i32);
pub const DBINDEX_COL_ORDER_DESC: DBINDEX_COL_ORDERENUM = DBINDEX_COL_ORDERENUM(1i32);
#[repr(transparent)]
pub struct DBLITERALENUM(pub i32);
pub const DBLITERAL_INVALID: DBLITERALENUM = DBLITERALENUM(0i32);
pub const DBLITERAL_BINARY_LITERAL: DBLITERALENUM = DBLITERALENUM(1i32);
pub const DBLITERAL_CATALOG_NAME: DBLITERALENUM = DBLITERALENUM(2i32);
pub const DBLITERAL_CATALOG_SEPARATOR: DBLITERALENUM = DBLITERALENUM(3i32);
pub const DBLITERAL_CHAR_LITERAL: DBLITERALENUM = DBLITERALENUM(4i32);
pub const DBLITERAL_COLUMN_ALIAS: DBLITERALENUM = DBLITERALENUM(5i32);
pub const DBLITERAL_COLUMN_NAME: DBLITERALENUM = DBLITERALENUM(6i32);
pub const DBLITERAL_CORRELATION_NAME: DBLITERALENUM = DBLITERALENUM(7i32);
pub const DBLITERAL_CURSOR_NAME: DBLITERALENUM = DBLITERALENUM(8i32);
pub const DBLITERAL_ESCAPE_PERCENT: DBLITERALENUM = DBLITERALENUM(9i32);
pub const DBLITERAL_ESCAPE_UNDERSCORE: DBLITERALENUM = DBLITERALENUM(10i32);
pub const DBLITERAL_INDEX_NAME: DBLITERALENUM = DBLITERALENUM(11i32);
pub const DBLITERAL_LIKE_PERCENT: DBLITERALENUM = DBLITERALENUM(12i32);
pub const DBLITERAL_LIKE_UNDERSCORE: DBLITERALENUM = DBLITERALENUM(13i32);
pub const DBLITERAL_PROCEDURE_NAME: DBLITERALENUM = DBLITERALENUM(14i32);
pub const DBLITERAL_QUOTE: DBLITERALENUM = DBLITERALENUM(15i32);
pub const DBLITERAL_SCHEMA_NAME: DBLITERALENUM = DBLITERALENUM(16i32);
pub const DBLITERAL_TABLE_NAME: DBLITERALENUM = DBLITERALENUM(17i32);
pub const DBLITERAL_TEXT_COMMAND: DBLITERALENUM = DBLITERALENUM(18i32);
pub const DBLITERAL_USER_NAME: DBLITERALENUM = DBLITERALENUM(19i32);
pub const DBLITERAL_VIEW_NAME: DBLITERALENUM = DBLITERALENUM(20i32);
#[repr(transparent)]
pub struct DBLITERALENUM20(pub i32);
pub const DBLITERAL_CUBE_NAME: DBLITERALENUM20 = DBLITERALENUM20(21i32);
pub const DBLITERAL_DIMENSION_NAME: DBLITERALENUM20 = DBLITERALENUM20(22i32);
pub const DBLITERAL_HIERARCHY_NAME: DBLITERALENUM20 = DBLITERALENUM20(23i32);
pub const DBLITERAL_LEVEL_NAME: DBLITERALENUM20 = DBLITERALENUM20(24i32);
pub const DBLITERAL_MEMBER_NAME: DBLITERALENUM20 = DBLITERALENUM20(25i32);
pub const DBLITERAL_PROPERTY_NAME: DBLITERALENUM20 = DBLITERALENUM20(26i32);
pub const DBLITERAL_SCHEMA_SEPARATOR: DBLITERALENUM20 = DBLITERALENUM20(27i32);
pub const DBLITERAL_QUOTE_SUFFIX: DBLITERALENUM20 = DBLITERALENUM20(28i32);
#[repr(transparent)]
pub struct DBLITERALENUM21(pub i32);
pub const DBLITERAL_ESCAPE_PERCENT_SUFFIX: DBLITERALENUM21 = DBLITERALENUM21(29i32);
pub const DBLITERAL_ESCAPE_UNDERSCORE_SUFFIX: DBLITERALENUM21 = DBLITERALENUM21(30i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBLITERALINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBLITERALINFO(i32);
#[repr(transparent)]
pub struct DBMATCHTYPEENUM(pub i32);
pub const DBMATCHTYPE_FULL: DBMATCHTYPEENUM = DBMATCHTYPEENUM(0i32);
pub const DBMATCHTYPE_NONE: DBMATCHTYPEENUM = DBMATCHTYPEENUM(1i32);
pub const DBMATCHTYPE_PARTIAL: DBMATCHTYPEENUM = DBMATCHTYPEENUM(2i32);
pub const DBMAXCHAR: u32 = 8001u32;
#[repr(transparent)]
pub struct DBMEMOWNERENUM(pub i32);
pub const DBMEMOWNER_CLIENTOWNED: DBMEMOWNERENUM = DBMEMOWNERENUM(0i32);
pub const DBMEMOWNER_PROVIDEROWNED: DBMEMOWNERENUM = DBMEMOWNERENUM(1i32);
#[repr(transparent)]
pub struct DBMOVEFLAGSENUM(pub i32);
pub const DBMOVE_REPLACE_EXISTING: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(1i32);
pub const DBMOVE_ASYNC: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(256i32);
pub const DBMOVE_DONT_UPDATE_LINKS: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(512i32);
pub const DBMOVE_ALLOW_EMULATION: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(1024i32);
pub const DBMOVE_ATOMIC: DBMOVEFLAGSENUM = DBMOVEFLAGSENUM(4096i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBOBJECT(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBOBJECT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBPARAMBINDINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBPARAMBINDINFO(i32);
#[repr(transparent)]
pub struct DBPARAMFLAGSENUM(pub i32);
pub const DBPARAMFLAGS_ISINPUT: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(1i32);
pub const DBPARAMFLAGS_ISOUTPUT: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(2i32);
pub const DBPARAMFLAGS_ISSIGNED: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(16i32);
pub const DBPARAMFLAGS_ISNULLABLE: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(64i32);
pub const DBPARAMFLAGS_ISLONG: DBPARAMFLAGSENUM = DBPARAMFLAGSENUM(128i32);
#[repr(transparent)]
pub struct DBPARAMFLAGSENUM20(pub i32);
pub const DBPARAMFLAGS_SCALEISNEGATIVE: DBPARAMFLAGSENUM20 = DBPARAMFLAGSENUM20(256i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct DBPARAMINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct DBPARAMINFO(i32);
#[repr(transparent)]
pub struct DBPARAMIOENUM(pub i32);
pub const DBPARAMIO_NOTPARAM: DBPARAMIOENUM = DBPARAMIOENUM(0i32);
pub const DBPARAMIO_INPUT: DBPARAMIOENUM = DBPARAMIOENUM(1i32);
pub const DBPARAMIO_OUTPUT: DBPARAMIOENUM = DBPARAMIOENUM(2i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBPARAMS(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBPARAMS(i32);
pub const DBPARAMTYPE_INPUT: u32 = 1u32;
pub const DBPARAMTYPE_INPUTOUTPUT: u32 = 2u32;
pub const DBPARAMTYPE_OUTPUT: u32 = 3u32;
pub const DBPARAMTYPE_RETURNVALUE: u32 = 4u32;
#[repr(transparent)]
pub struct DBPARTENUM(pub i32);
pub const DBPART_INVALID: DBPARTENUM = DBPARTENUM(0i32);
pub const DBPART_VALUE: DBPARTENUM = DBPARTENUM(1i32);
pub const DBPART_LENGTH: DBPARTENUM = DBPARTENUM(2i32);
pub const DBPART_STATUS: DBPARTENUM = DBPARTENUM(4i32);
#[repr(transparent)]
pub struct DBPENDINGSTATUSENUM(pub i32);
pub const DBPENDINGSTATUS_NEW: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(1i32);
pub const DBPENDINGSTATUS_CHANGED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(2i32);
pub const DBPENDINGSTATUS_DELETED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(4i32);
pub const DBPENDINGSTATUS_UNCHANGED: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(8i32);
pub const DBPENDINGSTATUS_INVALIDROW: DBPENDINGSTATUSENUM = DBPENDINGSTATUSENUM(16i32);
#[repr(transparent)]
pub struct DBPOSITIONFLAGSENUM(pub i32);
pub const DBPOSITION_OK: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(0i32);
pub const DBPOSITION_NOROW: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(1i32);
pub const DBPOSITION_BOF: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(2i32);
pub const DBPOSITION_EOF: DBPOSITIONFLAGSENUM = DBPOSITIONFLAGSENUM(3i32);
#[repr(transparent)]
pub struct DBPROMPTOPTIONSENUM(pub i32);
pub const DBPROMPTOPTIONS_NONE: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(0i32);
pub const DBPROMPTOPTIONS_WIZARDSHEET: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(1i32);
pub const DBPROMPTOPTIONS_PROPERTYSHEET: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(2i32);
pub const DBPROMPTOPTIONS_BROWSEONLY: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(8i32);
pub const DBPROMPTOPTIONS_DISABLE_PROVIDER_SELECTION: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(16i32);
pub const DBPROMPTOPTIONS_DISABLESAVEPASSWORD: DBPROMPTOPTIONSENUM = DBPROMPTOPTIONSENUM(32i32);
pub const DBPROMPT_COMPLETE: u32 = 2u32;
pub const DBPROMPT_COMPLETEREQUIRED: u32 = 3u32;
pub const DBPROMPT_NOPROMPT: u32 = 4u32;
pub const DBPROMPT_PROMPT: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROP(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROP(i32);
#[repr(transparent)]
pub struct DBPROPENUM(pub i32);
pub const DBPROP_ABORTPRESERVE: DBPROPENUM = DBPROPENUM(2i32);
pub const DBPROP_ACTIVESESSIONS: DBPROPENUM = DBPROPENUM(3i32);
pub const DBPROP_APPENDONLY: DBPROPENUM = DBPROPENUM(187i32);
pub const DBPROP_ASYNCTXNABORT: DBPROPENUM = DBPROPENUM(168i32);
pub const DBPROP_ASYNCTXNCOMMIT: DBPROPENUM = DBPROPENUM(4i32);
pub const DBPROP_AUTH_CACHE_AUTHINFO: DBPROPENUM = DBPROPENUM(5i32);
pub const DBPROP_AUTH_ENCRYPT_PASSWORD: DBPROPENUM = DBPROPENUM(6i32);
pub const DBPROP_AUTH_INTEGRATED: DBPROPENUM = DBPROPENUM(7i32);
pub const DBPROP_AUTH_MASK_PASSWORD: DBPROPENUM = DBPROPENUM(8i32);
pub const DBPROP_AUTH_PASSWORD: DBPROPENUM = DBPROPENUM(9i32);
pub const DBPROP_AUTH_PERSIST_ENCRYPTED: DBPROPENUM = DBPROPENUM(10i32);
pub const DBPROP_AUTH_PERSIST_SENSITIVE_AUTHINFO: DBPROPENUM = DBPROPENUM(11i32);
pub const DBPROP_AUTH_USERID: DBPROPENUM = DBPROPENUM(12i32);
pub const DBPROP_BLOCKINGSTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(13i32);
pub const DBPROP_BOOKMARKS: DBPROPENUM = DBPROPENUM(14i32);
pub const DBPROP_BOOKMARKSKIPPED: DBPROPENUM = DBPROPENUM(15i32);
pub const DBPROP_BOOKMARKTYPE: DBPROPENUM = DBPROPENUM(16i32);
pub const DBPROP_BYREFACCESSORS: DBPROPENUM = DBPROPENUM(120i32);
pub const DBPROP_CACHEDEFERRED: DBPROPENUM = DBPROPENUM(17i32);
pub const DBPROP_CANFETCHBACKWARDS: DBPROPENUM = DBPROPENUM(18i32);
pub const DBPROP_CANHOLDROWS: DBPROPENUM = DBPROPENUM(19i32);
pub const DBPROP_CANSCROLLBACKWARDS: DBPROPENUM = DBPROPENUM(21i32);
pub const DBPROP_CATALOGLOCATION: DBPROPENUM = DBPROPENUM(22i32);
pub const DBPROP_CATALOGTERM: DBPROPENUM = DBPROPENUM(23i32);
pub const DBPROP_CATALOGUSAGE: DBPROPENUM = DBPROPENUM(24i32);
pub const DBPROP_CHANGEINSERTEDROWS: DBPROPENUM = DBPROPENUM(188i32);
pub const DBPROP_COL_AUTOINCREMENT: DBPROPENUM = DBPROPENUM(26i32);
pub const DBPROP_COL_DEFAULT: DBPROPENUM = DBPROPENUM(27i32);
pub const DBPROP_COL_DESCRIPTION: DBPROPENUM = DBPROPENUM(28i32);
pub const DBPROP_COL_FIXEDLENGTH: DBPROPENUM = DBPROPENUM(167i32);
pub const DBPROP_COL_NULLABLE: DBPROPENUM = DBPROPENUM(29i32);
pub const DBPROP_COL_PRIMARYKEY: DBPROPENUM = DBPROPENUM(30i32);
pub const DBPROP_COL_UNIQUE: DBPROPENUM = DBPROPENUM(31i32);
pub const DBPROP_COLUMNDEFINITION: DBPROPENUM = DBPROPENUM(32i32);
pub const DBPROP_COLUMNRESTRICT: DBPROPENUM = DBPROPENUM(33i32);
pub const DBPROP_COMMANDTIMEOUT: DBPROPENUM = DBPROPENUM(34i32);
pub const DBPROP_COMMITPRESERVE: DBPROPENUM = DBPROPENUM(35i32);
pub const DBPROP_CONCATNULLBEHAVIOR: DBPROPENUM = DBPROPENUM(36i32);
pub const DBPROP_CURRENTCATALOG: DBPROPENUM = DBPROPENUM(37i32);
pub const DBPROP_DATASOURCENAME: DBPROPENUM = DBPROPENUM(38i32);
pub const DBPROP_DATASOURCEREADONLY: DBPROPENUM = DBPROPENUM(39i32);
pub const DBPROP_DBMSNAME: DBPROPENUM = DBPROPENUM(40i32);
pub const DBPROP_DBMSVER: DBPROPENUM = DBPROPENUM(41i32);
pub const DBPROP_DEFERRED: DBPROPENUM = DBPROPENUM(42i32);
pub const DBPROP_DELAYSTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(43i32);
pub const DBPROP_DSOTHREADMODEL: DBPROPENUM = DBPROPENUM(169i32);
pub const DBPROP_GROUPBY: DBPROPENUM = DBPROPENUM(44i32);
pub const DBPROP_HETEROGENEOUSTABLES: DBPROPENUM = DBPROPENUM(45i32);
pub const DBPROP_IAccessor: DBPROPENUM = DBPROPENUM(121i32);
pub const DBPROP_IColumnsInfo: DBPROPENUM = DBPROPENUM(122i32);
pub const DBPROP_IColumnsRowset: DBPROPENUM = DBPROPENUM(123i32);
pub const DBPROP_IConnectionPointContainer: DBPROPENUM = DBPROPENUM(124i32);
pub const DBPROP_IConvertType: DBPROPENUM = DBPROPENUM(194i32);
pub const DBPROP_IRowset: DBPROPENUM = DBPROPENUM(126i32);
pub const DBPROP_IRowsetChange: DBPROPENUM = DBPROPENUM(127i32);
pub const DBPROP_IRowsetIdentity: DBPROPENUM = DBPROPENUM(128i32);
pub const DBPROP_IRowsetIndex: DBPROPENUM = DBPROPENUM(159i32);
pub const DBPROP_IRowsetInfo: DBPROPENUM = DBPROPENUM(129i32);
pub const DBPROP_IRowsetLocate: DBPROPENUM = DBPROPENUM(130i32);
pub const DBPROP_IRowsetResynch: DBPROPENUM = DBPROPENUM(132i32);
pub const DBPROP_IRowsetScroll: DBPROPENUM = DBPROPENUM(133i32);
pub const DBPROP_IRowsetUpdate: DBPROPENUM = DBPROPENUM(134i32);
pub const DBPROP_ISupportErrorInfo: DBPROPENUM = DBPROPENUM(135i32);
pub const DBPROP_ILockBytes: DBPROPENUM = DBPROPENUM(136i32);
pub const DBPROP_ISequentialStream: DBPROPENUM = DBPROPENUM(137i32);
pub const DBPROP_IStorage: DBPROPENUM = DBPROPENUM(138i32);
pub const DBPROP_IStream: DBPROPENUM = DBPROPENUM(139i32);
pub const DBPROP_IDENTIFIERCASE: DBPROPENUM = DBPROPENUM(46i32);
pub const DBPROP_IMMOBILEROWS: DBPROPENUM = DBPROPENUM(47i32);
pub const DBPROP_INDEX_AUTOUPDATE: DBPROPENUM = DBPROPENUM(48i32);
pub const DBPROP_INDEX_CLUSTERED: DBPROPENUM = DBPROPENUM(49i32);
pub const DBPROP_INDEX_FILLFACTOR: DBPROPENUM = DBPROPENUM(50i32);
pub const DBPROP_INDEX_INITIALSIZE: DBPROPENUM = DBPROPENUM(51i32);
pub const DBPROP_INDEX_NULLCOLLATION: DBPROPENUM = DBPROPENUM(52i32);
pub const DBPROP_INDEX_NULLS: DBPROPENUM = DBPROPENUM(53i32);
pub const DBPROP_INDEX_PRIMARYKEY: DBPROPENUM = DBPROPENUM(54i32);
pub const DBPROP_INDEX_SORTBOOKMARKS: DBPROPENUM = DBPROPENUM(55i32);
pub const DBPROP_INDEX_TEMPINDEX: DBPROPENUM = DBPROPENUM(163i32);
pub const DBPROP_INDEX_TYPE: DBPROPENUM = DBPROPENUM(56i32);
pub const DBPROP_INDEX_UNIQUE: DBPROPENUM = DBPROPENUM(57i32);
pub const DBPROP_INIT_DATASOURCE: DBPROPENUM = DBPROPENUM(59i32);
pub const DBPROP_INIT_HWND: DBPROPENUM = DBPROPENUM(60i32);
pub const DBPROP_INIT_IMPERSONATION_LEVEL: DBPROPENUM = DBPROPENUM(61i32);
pub const DBPROP_INIT_LCID: DBPROPENUM = DBPROPENUM(186i32);
pub const DBPROP_INIT_LOCATION: DBPROPENUM = DBPROPENUM(62i32);
pub const DBPROP_INIT_MODE: DBPROPENUM = DBPROPENUM(63i32);
pub const DBPROP_INIT_PROMPT: DBPROPENUM = DBPROPENUM(64i32);
pub const DBPROP_INIT_PROTECTION_LEVEL: DBPROPENUM = DBPROPENUM(65i32);
pub const DBPROP_INIT_PROVIDERSTRING: DBPROPENUM = DBPROPENUM(160i32);
pub const DBPROP_INIT_TIMEOUT: DBPROPENUM = DBPROPENUM(66i32);
pub const DBPROP_LITERALBOOKMARKS: DBPROPENUM = DBPROPENUM(67i32);
pub const DBPROP_LITERALIDENTITY: DBPROPENUM = DBPROPENUM(68i32);
pub const DBPROP_MAXINDEXSIZE: DBPROPENUM = DBPROPENUM(70i32);
pub const DBPROP_MAXOPENROWS: DBPROPENUM = DBPROPENUM(71i32);
pub const DBPROP_MAXPENDINGROWS: DBPROPENUM = DBPROPENUM(72i32);
pub const DBPROP_MAXROWS: DBPROPENUM = DBPROPENUM(73i32);
pub const DBPROP_MAXROWSIZE: DBPROPENUM = DBPROPENUM(74i32);
pub const DBPROP_MAXROWSIZEINCLUDESBLOB: DBPROPENUM = DBPROPENUM(75i32);
pub const DBPROP_MAXTABLESINSELECT: DBPROPENUM = DBPROPENUM(76i32);
pub const DBPROP_MAYWRITECOLUMN: DBPROPENUM = DBPROPENUM(77i32);
pub const DBPROP_MEMORYUSAGE: DBPROPENUM = DBPROPENUM(78i32);
pub const DBPROP_MULTIPLEPARAMSETS: DBPROPENUM = DBPROPENUM(191i32);
pub const DBPROP_MULTIPLERESULTS: DBPROPENUM = DBPROPENUM(196i32);
pub const DBPROP_MULTIPLESTORAGEOBJECTS: DBPROPENUM = DBPROPENUM(80i32);
pub const DBPROP_MULTITABLEUPDATE: DBPROPENUM = DBPROPENUM(81i32);
pub const DBPROP_NOTIFICATIONGRANULARITY: DBPROPENUM = DBPROPENUM(198i32);
pub const DBPROP_NOTIFICATIONPHASES: DBPROPENUM = DBPROPENUM(82i32);
pub const DBPROP_NOTIFYCOLUMNSET: DBPROPENUM = DBPROPENUM(171i32);
pub const DBPROP_NOTIFYROWDELETE: DBPROPENUM = DBPROPENUM(173i32);
pub const DBPROP_NOTIFYROWFIRSTCHANGE: DBPROPENUM = DBPROPENUM(174i32);
pub const DBPROP_NOTIFYROWINSERT: DBPROPENUM = DBPROPENUM(175i32);
pub const DBPROP_NOTIFYROWRESYNCH: DBPROPENUM = DBPROPENUM(177i32);
pub const DBPROP_NOTIFYROWSETCHANGED: DBPROPENUM = DBPROPENUM(211i32);
pub const DBPROP_NOTIFYROWSETRELEASE: DBPROPENUM = DBPROPENUM(178i32);
pub const DBPROP_NOTIFYROWSETFETCHPOSITIONCHANGE: DBPROPENUM = DBPROPENUM(179i32);
pub const DBPROP_NOTIFYROWUNDOCHANGE: DBPROPENUM = DBPROPENUM(180i32);
pub const DBPROP_NOTIFYROWUNDODELETE: DBPROPENUM = DBPROPENUM(181i32);
pub const DBPROP_NOTIFYROWUNDOINSERT: DBPROPENUM = DBPROPENUM(182i32);
pub const DBPROP_NOTIFYROWUPDATE: DBPROPENUM = DBPROPENUM(183i32);
pub const DBPROP_NULLCOLLATION: DBPROPENUM = DBPROPENUM(83i32);
pub const DBPROP_OLEOBJECTS: DBPROPENUM = DBPROPENUM(84i32);
pub const DBPROP_ORDERBYCOLUMNSINSELECT: DBPROPENUM = DBPROPENUM(85i32);
pub const DBPROP_ORDEREDBOOKMARKS: DBPROPENUM = DBPROPENUM(86i32);
pub const DBPROP_OTHERINSERT: DBPROPENUM = DBPROPENUM(87i32);
pub const DBPROP_OTHERUPDATEDELETE: DBPROPENUM = DBPROPENUM(88i32);
pub const DBPROP_OUTPUTPARAMETERAVAILABILITY: DBPROPENUM = DBPROPENUM(184i32);
pub const DBPROP_OWNINSERT: DBPROPENUM = DBPROPENUM(89i32);
pub const DBPROP_OWNUPDATEDELETE: DBPROPENUM = DBPROPENUM(90i32);
pub const DBPROP_PERSISTENTIDTYPE: DBPROPENUM = DBPROPENUM(185i32);
pub const DBPROP_PREPAREABORTBEHAVIOR: DBPROPENUM = DBPROPENUM(91i32);
pub const DBPROP_PREPARECOMMITBEHAVIOR: DBPROPENUM = DBPROPENUM(92i32);
pub const DBPROP_PROCEDURETERM: DBPROPENUM = DBPROPENUM(93i32);
pub const DBPROP_PROVIDERNAME: DBPROPENUM = DBPROPENUM(96i32);
pub const DBPROP_PROVIDEROLEDBVER: DBPROPENUM = DBPROPENUM(97i32);
pub const DBPROP_PROVIDERVER: DBPROPENUM = DBPROPENUM(98i32);
pub const DBPROP_QUICKRESTART: DBPROPENUM = DBPROPENUM(99i32);
pub const DBPROP_QUOTEDIDENTIFIERCASE: DBPROPENUM = DBPROPENUM(100i32);
pub const DBPROP_REENTRANTEVENTS: DBPROPENUM = DBPROPENUM(101i32);
pub const DBPROP_REMOVEDELETED: DBPROPENUM = DBPROPENUM(102i32);
pub const DBPROP_REPORTMULTIPLECHANGES: DBPROPENUM = DBPROPENUM(103i32);
pub const DBPROP_RETURNPENDINGINSERTS: DBPROPENUM = DBPROPENUM(189i32);
pub const DBPROP_ROWRESTRICT: DBPROPENUM = DBPROPENUM(104i32);
pub const DBPROP_ROWSETCONVERSIONSONCOMMAND: DBPROPENUM = DBPROPENUM(192i32);
pub const DBPROP_ROWTHREADMODEL: DBPROPENUM = DBPROPENUM(105i32);
pub const DBPROP_SCHEMATERM: DBPROPENUM = DBPROPENUM(106i32);
pub const DBPROP_SCHEMAUSAGE: DBPROPENUM = DBPROPENUM(107i32);
pub const DBPROP_SERVERCURSOR: DBPROPENUM = DBPROPENUM(108i32);
pub const DBPROP_SESS_AUTOCOMMITISOLEVELS: DBPROPENUM = DBPROPENUM(190i32);
pub const DBPROP_SQLSUPPORT: DBPROPENUM = DBPROPENUM(109i32);
pub const DBPROP_STRONGIDENTITY: DBPROPENUM = DBPROPENUM(119i32);
pub const DBPROP_STRUCTUREDSTORAGE: DBPROPENUM = DBPROPENUM(111i32);
pub const DBPROP_SUBQUERIES: DBPROPENUM = DBPROPENUM(112i32);
pub const DBPROP_SUPPORTEDTXNDDL: DBPROPENUM = DBPROPENUM(161i32);
pub const DBPROP_SUPPORTEDTXNISOLEVELS: DBPROPENUM = DBPROPENUM(113i32);
pub const DBPROP_SUPPORTEDTXNISORETAIN: DBPROPENUM = DBPROPENUM(114i32);
pub const DBPROP_TABLETERM: DBPROPENUM = DBPROPENUM(115i32);
pub const DBPROP_TBL_TEMPTABLE: DBPROPENUM = DBPROPENUM(140i32);
pub const DBPROP_TRANSACTEDOBJECT: DBPROPENUM = DBPROPENUM(116i32);
pub const DBPROP_UPDATABILITY: DBPROPENUM = DBPROPENUM(117i32);
pub const DBPROP_USERNAME: DBPROPENUM = DBPROPENUM(118i32);
#[repr(transparent)]
pub struct DBPROPENUM15(pub i32);
pub const DBPROP_FILTERCOMPAREOPS: DBPROPENUM15 = DBPROPENUM15(209i32);
pub const DBPROP_FINDCOMPAREOPS: DBPROPENUM15 = DBPROPENUM15(210i32);
pub const DBPROP_IChapteredRowset: DBPROPENUM15 = DBPROPENUM15(202i32);
pub const DBPROP_IDBAsynchStatus: DBPROPENUM15 = DBPROPENUM15(203i32);
pub const DBPROP_IRowsetFind: DBPROPENUM15 = DBPROPENUM15(204i32);
pub const DBPROP_IRowsetView: DBPROPENUM15 = DBPROPENUM15(212i32);
pub const DBPROP_IViewChapter: DBPROPENUM15 = DBPROPENUM15(213i32);
pub const DBPROP_IViewFilter: DBPROPENUM15 = DBPROPENUM15(214i32);
pub const DBPROP_IViewRowset: DBPROPENUM15 = DBPROPENUM15(215i32);
pub const DBPROP_IViewSort: DBPROPENUM15 = DBPROPENUM15(216i32);
pub const DBPROP_INIT_ASYNCH: DBPROPENUM15 = DBPROPENUM15(200i32);
pub const DBPROP_MAXOPENCHAPTERS: DBPROPENUM15 = DBPROPENUM15(199i32);
pub const DBPROP_MAXORSINFILTER: DBPROPENUM15 = DBPROPENUM15(205i32);
pub const DBPROP_MAXSORTCOLUMNS: DBPROPENUM15 = DBPROPENUM15(206i32);
pub const DBPROP_ROWSET_ASYNCH: DBPROPENUM15 = DBPROPENUM15(201i32);
pub const DBPROP_SORTONINDEX: DBPROPENUM15 = DBPROPENUM15(207i32);
#[repr(transparent)]
pub struct DBPROPENUM20(pub i32);
pub const DBPROP_IMultipleResults: DBPROPENUM20 = DBPROPENUM20(217i32);
pub const DBPROP_DATASOURCE_TYPE: DBPROPENUM20 = DBPROPENUM20(251i32);
pub const MDPROP_AXES: DBPROPENUM20 = DBPROPENUM20(252i32);
pub const MDPROP_FLATTENING_SUPPORT: DBPROPENUM20 = DBPROPENUM20(253i32);
pub const MDPROP_MDX_JOINCUBES: DBPROPENUM20 = DBPROPENUM20(254i32);
pub const MDPROP_NAMED_LEVELS: DBPROPENUM20 = DBPROPENUM20(255i32);
pub const MDPROP_RANGEROWSET: DBPROPENUM20 = DBPROPENUM20(256i32);
pub const MDPROP_MDX_SLICER: DBPROPENUM20 = DBPROPENUM20(218i32);
pub const MDPROP_MDX_CUBEQUALIFICATION: DBPROPENUM20 = DBPROPENUM20(219i32);
pub const MDPROP_MDX_OUTERREFERENCE: DBPROPENUM20 = DBPROPENUM20(220i32);
pub const MDPROP_MDX_QUERYBYPROPERTY: DBPROPENUM20 = DBPROPENUM20(221i32);
pub const MDPROP_MDX_CASESUPPORT: DBPROPENUM20 = DBPROPENUM20(222i32);
pub const MDPROP_MDX_STRING_COMPOP: DBPROPENUM20 = DBPROPENUM20(224i32);
pub const MDPROP_MDX_DESCFLAGS: DBPROPENUM20 = DBPROPENUM20(225i32);
pub const MDPROP_MDX_SET_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(226i32);
pub const MDPROP_MDX_MEMBER_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(227i32);
pub const MDPROP_MDX_NUMERIC_FUNCTIONS: DBPROPENUM20 = DBPROPENUM20(228i32);
pub const MDPROP_MDX_FORMULAS: DBPROPENUM20 = DBPROPENUM20(229i32);
pub const MDPROP_AGGREGATECELL_UPDATE: DBPROPENUM20 = DBPROPENUM20(230i32);
pub const MDPROP_MDX_AGGREGATECELL_UPDATE: DBPROPENUM20 = DBPROPENUM20(230i32);
pub const MDPROP_MDX_OBJQUALIFICATION: DBPROPENUM20 = DBPROPENUM20(261i32);
pub const MDPROP_MDX_NONMEASURE_EXPRESSIONS: DBPROPENUM20 = DBPROPENUM20(262i32);
pub const DBPROP_ACCESSORDER: DBPROPENUM20 = DBPROPENUM20(231i32);
pub const DBPROP_BOOKMARKINFO: DBPROPENUM20 = DBPROPENUM20(232i32);
pub const DBPROP_INIT_CATALOG: DBPROPENUM20 = DBPROPENUM20(233i32);
pub const DBPROP_ROW_BULKOPS: DBPROPENUM20 = DBPROPENUM20(234i32);
pub const DBPROP_PROVIDERFRIENDLYNAME: DBPROPENUM20 = DBPROPENUM20(235i32);
pub const DBPROP_LOCKMODE: DBPROPENUM20 = DBPROPENUM20(236i32);
pub const DBPROP_MULTIPLECONNECTIONS: DBPROPENUM20 = DBPROPENUM20(237i32);
pub const DBPROP_UNIQUEROWS: DBPROPENUM20 = DBPROPENUM20(238i32);
pub const DBPROP_SERVERDATAONINSERT: DBPROPENUM20 = DBPROPENUM20(239i32);
pub const DBPROP_STORAGEFLAGS: DBPROPENUM20 = DBPROPENUM20(240i32);
pub const DBPROP_CONNECTIONSTATUS: DBPROPENUM20 = DBPROPENUM20(244i32);
pub const DBPROP_ALTERCOLUMN: DBPROPENUM20 = DBPROPENUM20(245i32);
pub const DBPROP_COLUMNLCID: DBPROPENUM20 = DBPROPENUM20(246i32);
pub const DBPROP_RESETDATASOURCE: DBPROPENUM20 = DBPROPENUM20(247i32);
pub const DBPROP_INIT_OLEDBSERVICES: DBPROPENUM20 = DBPROPENUM20(248i32);
pub const DBPROP_IRowsetRefresh: DBPROPENUM20 = DBPROPENUM20(249i32);
pub const DBPROP_SERVERNAME: DBPROPENUM20 = DBPROPENUM20(250i32);
pub const DBPROP_IParentRowset: DBPROPENUM20 = DBPROPENUM20(257i32);
pub const DBPROP_HIDDENCOLUMNS: DBPROPENUM20 = DBPROPENUM20(258i32);
pub const DBPROP_PROVIDERMEMORY: DBPROPENUM20 = DBPROPENUM20(259i32);
pub const DBPROP_CLIENTCURSOR: DBPROPENUM20 = DBPROPENUM20(260i32);
#[repr(transparent)]
pub struct DBPROPENUM21(pub i32);
pub const DBPROP_TRUSTEE_USERNAME: DBPROPENUM21 = DBPROPENUM21(241i32);
pub const DBPROP_TRUSTEE_AUTHENTICATION: DBPROPENUM21 = DBPROPENUM21(242i32);
pub const DBPROP_TRUSTEE_NEWAUTHENTICATION: DBPROPENUM21 = DBPROPENUM21(243i32);
pub const DBPROP_IRow: DBPROPENUM21 = DBPROPENUM21(263i32);
pub const DBPROP_IRowChange: DBPROPENUM21 = DBPROPENUM21(264i32);
pub const DBPROP_IRowSchemaChange: DBPROPENUM21 = DBPROPENUM21(265i32);
pub const DBPROP_IGetRow: DBPROPENUM21 = DBPROPENUM21(266i32);
pub const DBPROP_IScopedOperations: DBPROPENUM21 = DBPROPENUM21(267i32);
pub const DBPROP_IBindResource: DBPROPENUM21 = DBPROPENUM21(268i32);
pub const DBPROP_ICreateRow: DBPROPENUM21 = DBPROPENUM21(269i32);
pub const DBPROP_INIT_BINDFLAGS: DBPROPENUM21 = DBPROPENUM21(270i32);
pub const DBPROP_INIT_LOCKOWNER: DBPROPENUM21 = DBPROPENUM21(271i32);
pub const DBPROP_GENERATEURL: DBPROPENUM21 = DBPROPENUM21(273i32);
pub const DBPROP_IDBBinderProperties: DBPROPENUM21 = DBPROPENUM21(274i32);
pub const DBPROP_IColumnsInfo2: DBPROPENUM21 = DBPROPENUM21(275i32);
pub const DBPROP_IRegisterProvider: DBPROPENUM21 = DBPROPENUM21(276i32);
pub const DBPROP_IGetSession: DBPROPENUM21 = DBPROPENUM21(277i32);
pub const DBPROP_IGetSourceRow: DBPROPENUM21 = DBPROPENUM21(278i32);
pub const DBPROP_IRowsetCurrentIndex: DBPROPENUM21 = DBPROPENUM21(279i32);
pub const DBPROP_OPENROWSETSUPPORT: DBPROPENUM21 = DBPROPENUM21(280i32);
pub const DBPROP_COL_ISLONG: DBPROPENUM21 = DBPROPENUM21(281i32);
#[repr(transparent)]
pub struct DBPROPENUM25(pub i32);
pub const DBPROP_COL_SEED: DBPROPENUM25 = DBPROPENUM25(282i32);
pub const DBPROP_COL_INCREMENT: DBPROPENUM25 = DBPROPENUM25(283i32);
pub const DBPROP_INIT_GENERALTIMEOUT: DBPROPENUM25 = DBPROPENUM25(284i32);
pub const DBPROP_COMSERVICES: DBPROPENUM25 = DBPROPENUM25(285i32);
#[repr(transparent)]
pub struct DBPROPENUM25_DEPRECATED(pub i32);
pub const DBPROP_ICommandCost: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(141i32);
pub const DBPROP_ICommandTree: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(142i32);
pub const DBPROP_ICommandValidate: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(143i32);
pub const DBPROP_IDBSchemaCommand: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(144i32);
pub const DBPROP_IProvideMoniker: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(125i32);
pub const DBPROP_IQuery: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(146i32);
pub const DBPROP_IReadData: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(147i32);
pub const DBPROP_IRowsetAsynch: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(148i32);
pub const DBPROP_IRowsetCopyRows: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(149i32);
pub const DBPROP_IRowsetKeys: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(151i32);
pub const DBPROP_IRowsetNewRowAfter: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(152i32);
pub const DBPROP_IRowsetNextRowset: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(153i32);
pub const DBPROP_IRowsetWatchAll: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(155i32);
pub const DBPROP_IRowsetWatchNotify: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(156i32);
pub const DBPROP_IRowsetWatchRegion: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(157i32);
pub const DBPROP_IRowsetWithParameters: DBPROPENUM25_DEPRECATED = DBPROPENUM25_DEPRECATED(158i32);
#[repr(transparent)]
pub struct DBPROPENUM26(pub i32);
pub const DBPROP_OUTPUTSTREAM: DBPROPENUM26 = DBPROPENUM26(286i32);
pub const DBPROP_OUTPUTENCODING: DBPROPENUM26 = DBPROPENUM26(287i32);
pub const DBPROP_TABLESTATISTICS: DBPROPENUM26 = DBPROPENUM26(288i32);
pub const DBPROP_SKIPROWCOUNTRESULTS: DBPROPENUM26 = DBPROPENUM26(291i32);
pub const DBPROP_IRowsetBookmark: DBPROPENUM26 = DBPROPENUM26(292i32);
pub const MDPROP_VISUALMODE: DBPROPENUM26 = DBPROPENUM26(293i32);
#[repr(transparent)]
pub struct DBPROPFLAGSENUM(pub i32);
pub const DBPROPFLAGS_NOTSUPPORTED: DBPROPFLAGSENUM = DBPROPFLAGSENUM(0i32);
pub const DBPROPFLAGS_COLUMN: DBPROPFLAGSENUM = DBPROPFLAGSENUM(1i32);
pub const DBPROPFLAGS_DATASOURCE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(2i32);
pub const DBPROPFLAGS_DATASOURCECREATE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(4i32);
pub const DBPROPFLAGS_DATASOURCEINFO: DBPROPFLAGSENUM = DBPROPFLAGSENUM(8i32);
pub const DBPROPFLAGS_DBINIT: DBPROPFLAGSENUM = DBPROPFLAGSENUM(16i32);
pub const DBPROPFLAGS_INDEX: DBPROPFLAGSENUM = DBPROPFLAGSENUM(32i32);
pub const DBPROPFLAGS_ROWSET: DBPROPFLAGSENUM = DBPROPFLAGSENUM(64i32);
pub const DBPROPFLAGS_TABLE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(128i32);
pub const DBPROPFLAGS_COLUMNOK: DBPROPFLAGSENUM = DBPROPFLAGSENUM(256i32);
pub const DBPROPFLAGS_READ: DBPROPFLAGSENUM = DBPROPFLAGSENUM(512i32);
pub const DBPROPFLAGS_WRITE: DBPROPFLAGSENUM = DBPROPFLAGSENUM(1024i32);
pub const DBPROPFLAGS_REQUIRED: DBPROPFLAGSENUM = DBPROPFLAGSENUM(2048i32);
pub const DBPROPFLAGS_SESSION: DBPROPFLAGSENUM = DBPROPFLAGSENUM(4096i32);
#[repr(transparent)]
pub struct DBPROPFLAGSENUM21(pub i32);
pub const DBPROPFLAGS_TRUSTEE: DBPROPFLAGSENUM21 = DBPROPFLAGSENUM21(8192i32);
#[repr(transparent)]
pub struct DBPROPFLAGSENUM25(pub i32);
pub const DBPROPFLAGS_VIEW: DBPROPFLAGSENUM25 = DBPROPFLAGSENUM25(16384i32);
#[repr(transparent)]
pub struct DBPROPFLAGSENUM26(pub i32);
pub const DBPROPFLAGS_STREAM: DBPROPFLAGSENUM26 = DBPROPFLAGSENUM26(32768i32);
pub const DBPROPFLAGS_PERSIST: u32 = 8192u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBPROPIDSET(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBPROPIDSET(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPINFO(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPINFOSET(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPINFOSET(i32);
#[repr(transparent)]
pub struct DBPROPOPTIONSENUM(pub i32);
pub const DBPROPOPTIONS_REQUIRED: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(0i32);
pub const DBPROPOPTIONS_SETIFCHEAP: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(1i32);
pub const DBPROPOPTIONS_OPTIONAL: DBPROPOPTIONSENUM = DBPROPOPTIONSENUM(1i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPSET(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DBPROPSET(i32);
pub const DBPROPSET_MSDAORA8_ROWSET: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2131141493,
    data2: 56682,
    data3: 17371,
    data4: [180, 224, 31, 193, 33, 229, 230, 43],
};
pub const DBPROPSET_MSDAORA_ROWSET: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3905703101, data2: 65023, data3: 4560, data4: [184, 101, 0, 160, 201, 8, 28, 29] };
pub const DBPROPSET_MSDSDBINIT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1439404456, data2: 23674, data3: 4561, data4: [173, 173, 0, 192, 79, 194, 152, 99] };
pub const DBPROPSET_MSDSSESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3992024374, data2: 44991, data3: 4561, data4: [136, 71, 0, 0, 248, 121, 249, 140] };
pub const DBPROPSET_PERSIST: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1299724704, data2: 23438, data3: 4561, data4: [166, 179, 0, 160, 201, 19, 140, 102] };
pub const DBPROPSET_PROVIDERCONNATTR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1232888036, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERDATASOURCEINFO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1232888032, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERDBINIT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1232888034, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERROWSET: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1232888033, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_PROVIDERSTMTATTR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1232888035, data2: 28963, data3: 4559, data4: [177, 113, 0, 170, 0, 87, 89, 158] };
pub const DBPROPSET_SQLSERVERCOLUMN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 996408158, data2: 16315, data3: 4563, data4: [159, 41, 0, 192, 79, 142, 233, 220] };
pub const DBPROPSET_SQLSERVERDATASOURCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 686796516, data2: 11564, data3: 4561, data4: [152, 7, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERDATASOURCEINFO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3742419860, data2: 13814, data3: 4562, data4: [156, 84, 0, 192, 79, 121, 113, 211] };
pub const DBPROPSET_SQLSERVERDBINIT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1559546384, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERROWSET: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1559546385, data2: 61217, data3: 4560, data4: [151, 231, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERSESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 686796517, data2: 11564, data3: 4561, data4: [152, 7, 0, 192, 79, 194, 173, 152] };
pub const DBPROPSET_SQLSERVERSTREAM: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2675556467,
    data2: 35437,
    data3: 19402,
    data4: [168, 168, 201, 183, 154, 155, 150, 45],
};
#[repr(transparent)]
pub struct DBPROPSTATUSENUM(pub i32);
pub const DBPROPSTATUS_OK: DBPROPSTATUSENUM = DBPROPSTATUSENUM(0i32);
pub const DBPROPSTATUS_NOTSUPPORTED: DBPROPSTATUSENUM = DBPROPSTATUSENUM(1i32);
pub const DBPROPSTATUS_BADVALUE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(2i32);
pub const DBPROPSTATUS_BADOPTION: DBPROPSTATUSENUM = DBPROPSTATUSENUM(3i32);
pub const DBPROPSTATUS_BADCOLUMN: DBPROPSTATUSENUM = DBPROPSTATUSENUM(4i32);
pub const DBPROPSTATUS_NOTALLSETTABLE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(5i32);
pub const DBPROPSTATUS_NOTSETTABLE: DBPROPSTATUSENUM = DBPROPSTATUSENUM(6i32);
pub const DBPROPSTATUS_NOTSET: DBPROPSTATUSENUM = DBPROPSTATUSENUM(7i32);
pub const DBPROPSTATUS_CONFLICTING: DBPROPSTATUSENUM = DBPROPSTATUSENUM(8i32);
#[repr(transparent)]
pub struct DBPROPSTATUSENUM21(pub i32);
pub const DBPROPSTATUS_NOTAVAILABLE: DBPROPSTATUSENUM21 = DBPROPSTATUSENUM21(9i32);
pub const DBPROPVAL_AO_RANDOM: i32 = 2i32;
pub const DBPROPVAL_AO_SEQUENTIAL: i32 = 0i32;
pub const DBPROPVAL_AO_SEQUENTIALSTORAGEOBJECTS: i32 = 1i32;
pub const DBPROPVAL_ASYNCH_BACKGROUNDPOPULATION: i32 = 8i32;
pub const DBPROPVAL_ASYNCH_INITIALIZE: i32 = 1i32;
pub const DBPROPVAL_ASYNCH_POPULATEONDEMAND: i32 = 32i32;
pub const DBPROPVAL_ASYNCH_PREPOPULATE: i32 = 16i32;
pub const DBPROPVAL_ASYNCH_RANDOMPOPULATION: i32 = 4i32;
pub const DBPROPVAL_ASYNCH_SEQUENTIALPOPULATION: i32 = 2i32;
pub const DBPROPVAL_BD_INTRANSACTION: i32 = 1i32;
pub const DBPROPVAL_BD_REORGANIZATION: i32 = 3i32;
pub const DBPROPVAL_BD_ROWSET: i32 = 0i32;
pub const DBPROPVAL_BD_XTRANSACTION: i32 = 2i32;
pub const DBPROPVAL_BI_CROSSROWSET: i32 = 1i32;
pub const DBPROPVAL_BMK_KEY: i32 = 2i32;
pub const DBPROPVAL_BMK_NUMERIC: i32 = 1i32;
pub const DBPROPVAL_BO_NOINDEXUPDATE: i32 = 1i32;
pub const DBPROPVAL_BO_NOLOG: i32 = 0i32;
pub const DBPROPVAL_BO_REFINTEGRITY: i32 = 2i32;
pub const DBPROPVAL_CB_DELETE: i32 = 1i32;
pub const DBPROPVAL_CB_NON_NULL: i32 = 2i32;
pub const DBPROPVAL_CB_NULL: i32 = 1i32;
pub const DBPROPVAL_CB_PRESERVE: i32 = 2i32;
pub const DBPROPVAL_CD_NOTNULL: i32 = 1i32;
pub const DBPROPVAL_CL_END: i32 = 2i32;
pub const DBPROPVAL_CL_START: i32 = 1i32;
pub const DBPROPVAL_CM_TRANSACTIONS: i32 = 1i32;
pub const DBPROPVAL_CO_BEGINSWITH: i32 = 32i32;
pub const DBPROPVAL_CO_CASEINSENSITIVE: i32 = 8i32;
pub const DBPROPVAL_CO_CASESENSITIVE: i32 = 4i32;
pub const DBPROPVAL_CO_CONTAINS: i32 = 16i32;
pub const DBPROPVAL_CO_EQUALITY: i32 = 1i32;
pub const DBPROPVAL_CO_STRING: i32 = 2i32;
pub const DBPROPVAL_CS_COMMUNICATIONFAILURE: i32 = 2i32;
pub const DBPROPVAL_CS_INITIALIZED: i32 = 1i32;
pub const DBPROPVAL_CS_UNINITIALIZED: i32 = 0i32;
pub const DBPROPVAL_CU_DML_STATEMENTS: i32 = 1i32;
pub const DBPROPVAL_CU_INDEX_DEFINITION: i32 = 4i32;
pub const DBPROPVAL_CU_PRIVILEGE_DEFINITION: i32 = 8i32;
pub const DBPROPVAL_CU_TABLE_DEFINITION: i32 = 2i32;
pub const DBPROPVAL_DF_INITIALLY_DEFERRED: u32 = 1u32;
pub const DBPROPVAL_DF_INITIALLY_IMMEDIATE: u32 = 2u32;
pub const DBPROPVAL_DF_NOT_DEFERRABLE: u32 = 3u32;
pub const DBPROPVAL_DST_DOCSOURCE: i32 = 4i32;
pub const DBPROPVAL_DST_MDP: i32 = 2i32;
pub const DBPROPVAL_DST_TDP: i32 = 1i32;
pub const DBPROPVAL_DST_TDPANDMDP: i32 = 3i32;
pub const DBPROPVAL_FU_CATALOG: i32 = 8i32;
pub const DBPROPVAL_FU_COLUMN: i32 = 2i32;
pub const DBPROPVAL_FU_NOT_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_FU_TABLE: i32 = 4i32;
pub const DBPROPVAL_GB_COLLATE: i32 = 16i32;
pub const DBPROPVAL_GB_CONTAINS_SELECT: i32 = 4i32;
pub const DBPROPVAL_GB_EQUALS_SELECT: i32 = 2i32;
pub const DBPROPVAL_GB_NOT_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_GB_NO_RELATION: i32 = 8i32;
pub const DBPROPVAL_GU_NOTSUPPORTED: i32 = 1i32;
pub const DBPROPVAL_GU_SUFFIX: i32 = 2i32;
pub const DBPROPVAL_HT_DIFFERENT_CATALOGS: i32 = 1i32;
pub const DBPROPVAL_HT_DIFFERENT_PROVIDERS: i32 = 2i32;
pub const DBPROPVAL_IC_LOWER: i32 = 2i32;
pub const DBPROPVAL_IC_MIXED: i32 = 8i32;
pub const DBPROPVAL_IC_SENSITIVE: i32 = 4i32;
pub const DBPROPVAL_IC_UPPER: i32 = 1i32;
pub const DBPROPVAL_IN_ALLOWNULL: i32 = 0i32;
pub const DBPROPVAL_IN_DISALLOWNULL: i32 = 1i32;
pub const DBPROPVAL_IN_IGNOREANYNULL: i32 = 4i32;
pub const DBPROPVAL_IN_IGNORENULL: i32 = 2i32;
pub const DBPROPVAL_IT_BTREE: i32 = 1i32;
pub const DBPROPVAL_IT_CONTENT: i32 = 3i32;
pub const DBPROPVAL_IT_HASH: i32 = 2i32;
pub const DBPROPVAL_IT_OTHER: i32 = 4i32;
pub const DBPROPVAL_LM_INTENT: i32 = 4i32;
pub const DBPROPVAL_LM_NONE: i32 = 1i32;
pub const DBPROPVAL_LM_READ: i32 = 2i32;
pub const DBPROPVAL_LM_RITE: i32 = 8i32;
pub const DBPROPVAL_LM_SINGLEROW: i32 = 2i32;
pub const DBPROPVAL_MR_CONCURRENT: i32 = 2i32;
pub const DBPROPVAL_MR_NOTSUPPORTED: i32 = 0i32;
pub const DBPROPVAL_MR_SUPPORTED: i32 = 1i32;
pub const DBPROPVAL_NC_END: i32 = 1i32;
pub const DBPROPVAL_NC_HIGH: i32 = 2i32;
pub const DBPROPVAL_NC_LOW: i32 = 4i32;
pub const DBPROPVAL_NC_START: i32 = 8i32;
pub const DBPROPVAL_NP_ABOUTTODO: i32 = 2i32;
pub const DBPROPVAL_NP_DIDEVENT: i32 = 16i32;
pub const DBPROPVAL_NP_FAILEDTODO: i32 = 8i32;
pub const DBPROPVAL_NP_OKTODO: i32 = 1i32;
pub const DBPROPVAL_NP_SYNCHAFTER: i32 = 4i32;
pub const DBPROPVAL_NT_MULTIPLEROWS: i32 = 2i32;
pub const DBPROPVAL_NT_SINGLEROW: i32 = 1i32;
pub const DBPROPVAL_OA_ATEXECUTE: i32 = 2i32;
pub const DBPROPVAL_OA_ATROWRELEASE: i32 = 4i32;
pub const DBPROPVAL_OA_NOTSUPPORTED: i32 = 1i32;
pub const DBPROPVAL_OO_BLOB: i32 = 1i32;
pub const DBPROPVAL_OO_DIRECTBIND: i32 = 16i32;
pub const DBPROPVAL_OO_IPERSIST: i32 = 2i32;
pub const DBPROPVAL_OO_ROWOBJECT: i32 = 4i32;
pub const DBPROPVAL_OO_SCOPED: i32 = 8i32;
pub const DBPROPVAL_OO_SINGLETON: i32 = 32i32;
pub const DBPROPVAL_OP_EQUAL: i32 = 1i32;
pub const DBPROPVAL_OP_RELATIVE: i32 = 2i32;
pub const DBPROPVAL_OP_STRING: i32 = 4i32;
pub const DBPROPVAL_ORS_HISTOGRAM: i32 = 8i32;
pub const DBPROPVAL_ORS_INDEX: i32 = 1i32;
pub const DBPROPVAL_ORS_INTEGRATEDINDEX: i32 = 2i32;
pub const DBPROPVAL_ORS_STOREDPROC: i32 = 4i32;
pub const DBPROPVAL_ORS_TABLE: i32 = 0i32;
pub const DBPROPVAL_OS_AGR_AFTERSESSION: i32 = 8i32;
pub const DBPROPVAL_OS_CLIENTCURSOR: i32 = 4i32;
pub const DBPROPVAL_OS_DISABLEALL: i32 = 0i32;
pub const DBPROPVAL_OS_ENABLEALL: i32 = -1i32;
pub const DBPROPVAL_OS_RESOURCEPOOLING: i32 = 1i32;
pub const DBPROPVAL_OS_TXNENLISTMENT: i32 = 2i32;
pub const DBPROPVAL_PERSIST_ADTG: u32 = 0u32;
pub const DBPROPVAL_PERSIST_XML: u32 = 1u32;
pub const DBPROPVAL_PT_GUID: i32 = 8i32;
pub const DBPROPVAL_PT_GUID_NAME: i32 = 1i32;
pub const DBPROPVAL_PT_GUID_PROPID: i32 = 2i32;
pub const DBPROPVAL_PT_NAME: i32 = 4i32;
pub const DBPROPVAL_PT_PGUID_NAME: i32 = 32i32;
pub const DBPROPVAL_PT_PGUID_PROPID: i32 = 64i32;
pub const DBPROPVAL_PT_PROPID: i32 = 16i32;
pub const DBPROPVAL_RD_RESETALL: i32 = -1i32;
pub const DBPROPVAL_RT_APTMTTHREAD: i32 = 2i32;
pub const DBPROPVAL_RT_FREETHREAD: i32 = 1i32;
pub const DBPROPVAL_RT_SINGLETHREAD: i32 = 4i32;
pub const DBPROPVAL_SQL_ANSI89_IEF: i32 = 8i32;
pub const DBPROPVAL_SQL_ANSI92_ENTRY: i32 = 16i32;
pub const DBPROPVAL_SQL_ANSI92_FULL: i32 = 128i32;
pub const DBPROPVAL_SQL_ANSI92_INTERMEDIATE: i32 = 64i32;
pub const DBPROPVAL_SQL_ESCAPECLAUSES: i32 = 256i32;
pub const DBPROPVAL_SQL_FIPS_TRANSITIONAL: i32 = 32i32;
pub const DBPROPVAL_SQL_NONE: i32 = 0i32;
pub const DBPROPVAL_SQL_ODBC_CORE: i32 = 2i32;
pub const DBPROPVAL_SQL_ODBC_EXTENDED: i32 = 4i32;
pub const DBPROPVAL_SQL_ODBC_MINIMUM: i32 = 1i32;
pub const DBPROPVAL_SQL_SUBMINIMUM: i32 = 512i32;
pub const DBPROPVAL_SQ_COMPARISON: i32 = 2i32;
pub const DBPROPVAL_SQ_CORRELATEDSUBQUERIES: i32 = 1i32;
pub const DBPROPVAL_SQ_EXISTS: i32 = 4i32;
pub const DBPROPVAL_SQ_IN: i32 = 8i32;
pub const DBPROPVAL_SQ_QUANTIFIED: i32 = 16i32;
pub const DBPROPVAL_SQ_TABLE: i32 = 32i32;
pub const DBPROPVAL_SS_ILOCKBYTES: i32 = 8i32;
pub const DBPROPVAL_SS_ISEQUENTIALSTREAM: i32 = 1i32;
pub const DBPROPVAL_SS_ISTORAGE: i32 = 4i32;
pub const DBPROPVAL_SS_ISTREAM: i32 = 2i32;
pub const DBPROPVAL_STGM_CONVERT: u32 = 262144u32;
pub const DBPROPVAL_STGM_DELETEONRELEASE: u32 = 2097152u32;
pub const DBPROPVAL_STGM_DIRECT: u32 = 65536u32;
pub const DBPROPVAL_STGM_FAILIFTHERE: u32 = 524288u32;
pub const DBPROPVAL_STGM_PRIORITY: u32 = 1048576u32;
pub const DBPROPVAL_STGM_TRANSACTED: u32 = 131072u32;
pub const DBPROPVAL_SU_DML_STATEMENTS: i32 = 1i32;
pub const DBPROPVAL_SU_INDEX_DEFINITION: i32 = 4i32;
pub const DBPROPVAL_SU_PRIVILEGE_DEFINITION: i32 = 8i32;
pub const DBPROPVAL_SU_TABLE_DEFINITION: i32 = 2i32;
pub const DBPROPVAL_TC_ALL: i32 = 8i32;
pub const DBPROPVAL_TC_DDL_COMMIT: i32 = 2i32;
pub const DBPROPVAL_TC_DDL_IGNORE: i32 = 4i32;
pub const DBPROPVAL_TC_DDL_LOCK: i32 = 16i32;
pub const DBPROPVAL_TC_DML: i32 = 1i32;
pub const DBPROPVAL_TC_NONE: i32 = 0i32;
pub const DBPROPVAL_TI_BROWSE: i32 = 256i32;
pub const DBPROPVAL_TI_CHAOS: i32 = 16i32;
pub const DBPROPVAL_TI_CURSORSTABILITY: i32 = 4096i32;
pub const DBPROPVAL_TI_ISOLATED: i32 = 1048576i32;
pub const DBPROPVAL_TI_READCOMMITTED: i32 = 4096i32;
pub const DBPROPVAL_TI_READUNCOMMITTED: i32 = 256i32;
pub const DBPROPVAL_TI_REPEATABLEREAD: i32 = 65536i32;
pub const DBPROPVAL_TI_SERIALIZABLE: i32 = 1048576i32;
pub const DBPROPVAL_TR_ABORT: i32 = 16i32;
pub const DBPROPVAL_TR_ABORT_DC: i32 = 8i32;
pub const DBPROPVAL_TR_ABORT_NO: i32 = 32i32;
pub const DBPROPVAL_TR_BOTH: i32 = 128i32;
pub const DBPROPVAL_TR_COMMIT: i32 = 2i32;
pub const DBPROPVAL_TR_COMMIT_DC: i32 = 1i32;
pub const DBPROPVAL_TR_COMMIT_NO: i32 = 4i32;
pub const DBPROPVAL_TR_DONTCARE: i32 = 64i32;
pub const DBPROPVAL_TR_NONE: i32 = 256i32;
pub const DBPROPVAL_TR_OPTIMISTIC: i32 = 512i32;
pub const DBPROPVAL_TS_CARDINALITY: i32 = 1i32;
pub const DBPROPVAL_TS_HISTOGRAM: i32 = 2i32;
pub const DBPROPVAL_UP_CHANGE: i32 = 1i32;
pub const DBPROPVAL_UP_DELETE: i32 = 2i32;
pub const DBPROPVAL_UP_INSERT: i32 = 4i32;
pub const DBPROP_HCHAPTER: u32 = 4u32;
pub const DBPROP_INTERLEAVEDROWS: u32 = 8u32;
pub const DBPROP_MAINTAINPROPS: u32 = 5u32;
pub const DBPROP_MSDAORA8_DETERMINEKEYCOLUMNS: u32 = 2u32;
pub const DBPROP_MSDAORA_DETERMINEKEYCOLUMNS: u32 = 1u32;
pub const DBPROP_PersistFormat: u32 = 2u32;
pub const DBPROP_PersistSchema: u32 = 3u32;
pub const DBPROP_Unicode: u32 = 6u32;
#[repr(transparent)]
pub struct DBRANGEENUM(pub i32);
pub const DBRANGE_INCLUSIVESTART: DBRANGEENUM = DBRANGEENUM(0i32);
pub const DBRANGE_INCLUSIVEEND: DBRANGEENUM = DBRANGEENUM(0i32);
pub const DBRANGE_EXCLUSIVESTART: DBRANGEENUM = DBRANGEENUM(1i32);
pub const DBRANGE_EXCLUSIVEEND: DBRANGEENUM = DBRANGEENUM(2i32);
pub const DBRANGE_EXCLUDENULLS: DBRANGEENUM = DBRANGEENUM(4i32);
pub const DBRANGE_PREFIX: DBRANGEENUM = DBRANGEENUM(8i32);
pub const DBRANGE_MATCH: DBRANGEENUM = DBRANGEENUM(16i32);
#[repr(transparent)]
pub struct DBRANGEENUM20(pub i32);
pub const DBRANGE_MATCH_N_SHIFT: DBRANGEENUM20 = DBRANGEENUM20(24i32);
pub const DBRANGE_MATCH_N_MASK: DBRANGEENUM20 = DBRANGEENUM20(255i32);
#[repr(transparent)]
pub struct DBREASONENUM(pub i32);
pub const DBREASON_ROWSET_FETCHPOSITIONCHANGE: DBREASONENUM = DBREASONENUM(0i32);
pub const DBREASON_ROWSET_RELEASE: DBREASONENUM = DBREASONENUM(1i32);
pub const DBREASON_COLUMN_SET: DBREASONENUM = DBREASONENUM(2i32);
pub const DBREASON_COLUMN_RECALCULATED: DBREASONENUM = DBREASONENUM(3i32);
pub const DBREASON_ROW_ACTIVATE: DBREASONENUM = DBREASONENUM(4i32);
pub const DBREASON_ROW_RELEASE: DBREASONENUM = DBREASONENUM(5i32);
pub const DBREASON_ROW_DELETE: DBREASONENUM = DBREASONENUM(6i32);
pub const DBREASON_ROW_FIRSTCHANGE: DBREASONENUM = DBREASONENUM(7i32);
pub const DBREASON_ROW_INSERT: DBREASONENUM = DBREASONENUM(8i32);
pub const DBREASON_ROW_RESYNCH: DBREASONENUM = DBREASONENUM(9i32);
pub const DBREASON_ROW_UNDOCHANGE: DBREASONENUM = DBREASONENUM(10i32);
pub const DBREASON_ROW_UNDOINSERT: DBREASONENUM = DBREASONENUM(11i32);
pub const DBREASON_ROW_UNDODELETE: DBREASONENUM = DBREASONENUM(12i32);
pub const DBREASON_ROW_UPDATE: DBREASONENUM = DBREASONENUM(13i32);
pub const DBREASON_ROWSET_CHANGED: DBREASONENUM = DBREASONENUM(14i32);
#[repr(transparent)]
pub struct DBREASONENUM15(pub i32);
pub const DBREASON_ROWPOSITION_CHANGED: DBREASONENUM15 = DBREASONENUM15(15i32);
pub const DBREASON_ROWPOSITION_CHAPTERCHANGED: DBREASONENUM15 = DBREASONENUM15(16i32);
pub const DBREASON_ROWPOSITION_CLEARED: DBREASONENUM15 = DBREASONENUM15(17i32);
pub const DBREASON_ROW_ASYNCHINSERT: DBREASONENUM15 = DBREASONENUM15(18i32);
#[repr(transparent)]
pub struct DBREASONENUM25(pub i32);
pub const DBREASON_ROWSET_ROWSADDED: DBREASONENUM25 = DBREASONENUM25(19i32);
pub const DBREASON_ROWSET_POPULATIONCOMPLETE: DBREASONENUM25 = DBREASONENUM25(20i32);
pub const DBREASON_ROWSET_POPULATIONSTOPPED: DBREASONENUM25 = DBREASONENUM25(21i32);
#[repr(transparent)]
pub struct DBRESOURCEKINDENUM(pub i32);
pub const DBRESOURCE_INVALID: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(0i32);
pub const DBRESOURCE_TOTAL: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(1i32);
pub const DBRESOURCE_CPU: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(2i32);
pub const DBRESOURCE_MEMORY: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(3i32);
pub const DBRESOURCE_DISK: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(4i32);
pub const DBRESOURCE_NETWORK: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(5i32);
pub const DBRESOURCE_RESPONSE: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(6i32);
pub const DBRESOURCE_ROWS: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(7i32);
pub const DBRESOURCE_OTHER: DBRESOURCEKINDENUM = DBRESOURCEKINDENUM(8i32);
#[repr(transparent)]
pub struct DBRESULTFLAGENUM(pub i32);
pub const DBRESULTFLAG_DEFAULT: DBRESULTFLAGENUM = DBRESULTFLAGENUM(0i32);
pub const DBRESULTFLAG_ROWSET: DBRESULTFLAGENUM = DBRESULTFLAGENUM(1i32);
pub const DBRESULTFLAG_ROW: DBRESULTFLAGENUM = DBRESULTFLAGENUM(2i32);
#[repr(transparent)]
pub struct DBROWCHANGEKINDENUM(pub i32);
pub const DBROWCHANGEKIND_INSERT: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(0i32);
pub const DBROWCHANGEKIND_DELETE: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(1i32);
pub const DBROWCHANGEKIND_UPDATE: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(2i32);
pub const DBROWCHANGEKIND_COUNT: DBROWCHANGEKINDENUM = DBROWCHANGEKINDENUM(3i32);
#[repr(transparent)]
pub struct DBROWSTATUSENUM(pub i32);
pub const DBROWSTATUS_S_OK: DBROWSTATUSENUM = DBROWSTATUSENUM(0i32);
pub const DBROWSTATUS_S_MULTIPLECHANGES: DBROWSTATUSENUM = DBROWSTATUSENUM(2i32);
pub const DBROWSTATUS_S_PENDINGCHANGES: DBROWSTATUSENUM = DBROWSTATUSENUM(3i32);
pub const DBROWSTATUS_E_CANCELED: DBROWSTATUSENUM = DBROWSTATUSENUM(4i32);
pub const DBROWSTATUS_E_CANTRELEASE: DBROWSTATUSENUM = DBROWSTATUSENUM(6i32);
pub const DBROWSTATUS_E_CONCURRENCYVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(7i32);
pub const DBROWSTATUS_E_DELETED: DBROWSTATUSENUM = DBROWSTATUSENUM(8i32);
pub const DBROWSTATUS_E_PENDINGINSERT: DBROWSTATUSENUM = DBROWSTATUSENUM(9i32);
pub const DBROWSTATUS_E_NEWLYINSERTED: DBROWSTATUSENUM = DBROWSTATUSENUM(10i32);
pub const DBROWSTATUS_E_INTEGRITYVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(11i32);
pub const DBROWSTATUS_E_INVALID: DBROWSTATUSENUM = DBROWSTATUSENUM(12i32);
pub const DBROWSTATUS_E_MAXPENDCHANGESEXCEEDED: DBROWSTATUSENUM = DBROWSTATUSENUM(13i32);
pub const DBROWSTATUS_E_OBJECTOPEN: DBROWSTATUSENUM = DBROWSTATUSENUM(14i32);
pub const DBROWSTATUS_E_OUTOFMEMORY: DBROWSTATUSENUM = DBROWSTATUSENUM(15i32);
pub const DBROWSTATUS_E_PERMISSIONDENIED: DBROWSTATUSENUM = DBROWSTATUSENUM(16i32);
pub const DBROWSTATUS_E_LIMITREACHED: DBROWSTATUSENUM = DBROWSTATUSENUM(17i32);
pub const DBROWSTATUS_E_SCHEMAVIOLATION: DBROWSTATUSENUM = DBROWSTATUSENUM(18i32);
pub const DBROWSTATUS_E_FAIL: DBROWSTATUSENUM = DBROWSTATUSENUM(19i32);
#[repr(transparent)]
pub struct DBROWSTATUSENUM20(pub i32);
pub const DBROWSTATUS_S_NOCHANGE: DBROWSTATUSENUM20 = DBROWSTATUSENUM20(20i32);
pub const DBSCHEMA_LINKEDSERVERS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2425604852, data2: 11948, data3: 4561, data4: [152, 9, 0, 192, 79, 194, 173, 152] };
#[repr(transparent)]
pub struct DBSEEKENUM(pub i32);
pub const DBSEEK_INVALID: DBSEEKENUM = DBSEEKENUM(0i32);
pub const DBSEEK_FIRSTEQ: DBSEEKENUM = DBSEEKENUM(1i32);
pub const DBSEEK_LASTEQ: DBSEEKENUM = DBSEEKENUM(2i32);
pub const DBSEEK_AFTEREQ: DBSEEKENUM = DBSEEKENUM(4i32);
pub const DBSEEK_AFTER: DBSEEKENUM = DBSEEKENUM(8i32);
pub const DBSEEK_BEFOREEQ: DBSEEKENUM = DBSEEKENUM(16i32);
pub const DBSEEK_BEFORE: DBSEEKENUM = DBSEEKENUM(32i32);
#[repr(transparent)]
pub struct DBSORTENUM(pub i32);
pub const DBSORT_ASCENDING: DBSORTENUM = DBSORTENUM(0i32);
pub const DBSORT_DESCENDING: DBSORTENUM = DBSORTENUM(1i32);
#[repr(transparent)]
pub struct DBSOURCETYPEENUM(pub i32);
pub const DBSOURCETYPE_DATASOURCE: DBSOURCETYPEENUM = DBSOURCETYPEENUM(1i32);
pub const DBSOURCETYPE_ENUMERATOR: DBSOURCETYPEENUM = DBSOURCETYPEENUM(2i32);
#[repr(transparent)]
pub struct DBSOURCETYPEENUM20(pub i32);
pub const DBSOURCETYPE_DATASOURCE_TDP: DBSOURCETYPEENUM20 = DBSOURCETYPEENUM20(1i32);
pub const DBSOURCETYPE_DATASOURCE_MDP: DBSOURCETYPEENUM20 = DBSOURCETYPEENUM20(3i32);
#[repr(transparent)]
pub struct DBSOURCETYPEENUM25(pub i32);
pub const DBSOURCETYPE_BINDER: DBSOURCETYPEENUM25 = DBSOURCETYPEENUM25(4i32);
#[repr(transparent)]
pub struct DBSTATUSENUM(pub i32);
pub const DBSTATUS_S_OK: DBSTATUSENUM = DBSTATUSENUM(0i32);
pub const DBSTATUS_E_BADACCESSOR: DBSTATUSENUM = DBSTATUSENUM(1i32);
pub const DBSTATUS_E_CANTCONVERTVALUE: DBSTATUSENUM = DBSTATUSENUM(2i32);
pub const DBSTATUS_S_ISNULL: DBSTATUSENUM = DBSTATUSENUM(3i32);
pub const DBSTATUS_S_TRUNCATED: DBSTATUSENUM = DBSTATUSENUM(4i32);
pub const DBSTATUS_E_SIGNMISMATCH: DBSTATUSENUM = DBSTATUSENUM(5i32);
pub const DBSTATUS_E_DATAOVERFLOW: DBSTATUSENUM = DBSTATUSENUM(6i32);
pub const DBSTATUS_E_CANTCREATE: DBSTATUSENUM = DBSTATUSENUM(7i32);
pub const DBSTATUS_E_UNAVAILABLE: DBSTATUSENUM = DBSTATUSENUM(8i32);
pub const DBSTATUS_E_PERMISSIONDENIED: DBSTATUSENUM = DBSTATUSENUM(9i32);
pub const DBSTATUS_E_INTEGRITYVIOLATION: DBSTATUSENUM = DBSTATUSENUM(10i32);
pub const DBSTATUS_E_SCHEMAVIOLATION: DBSTATUSENUM = DBSTATUSENUM(11i32);
pub const DBSTATUS_E_BADSTATUS: DBSTATUSENUM = DBSTATUSENUM(12i32);
pub const DBSTATUS_S_DEFAULT: DBSTATUSENUM = DBSTATUSENUM(13i32);
#[repr(transparent)]
pub struct DBSTATUSENUM20(pub i32);
pub const MDSTATUS_S_CELLEMPTY: DBSTATUSENUM20 = DBSTATUSENUM20(14i32);
pub const DBSTATUS_S_IGNORE: DBSTATUSENUM20 = DBSTATUSENUM20(15i32);
#[repr(transparent)]
pub struct DBSTATUSENUM21(pub i32);
pub const DBSTATUS_E_DOESNOTEXIST: DBSTATUSENUM21 = DBSTATUSENUM21(16i32);
pub const DBSTATUS_E_INVALIDURL: DBSTATUSENUM21 = DBSTATUSENUM21(17i32);
pub const DBSTATUS_E_RESOURCELOCKED: DBSTATUSENUM21 = DBSTATUSENUM21(18i32);
pub const DBSTATUS_E_RESOURCEEXISTS: DBSTATUSENUM21 = DBSTATUSENUM21(19i32);
pub const DBSTATUS_E_CANNOTCOMPLETE: DBSTATUSENUM21 = DBSTATUSENUM21(20i32);
pub const DBSTATUS_E_VOLUMENOTFOUND: DBSTATUSENUM21 = DBSTATUSENUM21(21i32);
pub const DBSTATUS_E_OUTOFSPACE: DBSTATUSENUM21 = DBSTATUSENUM21(22i32);
pub const DBSTATUS_S_CANNOTDELETESOURCE: DBSTATUSENUM21 = DBSTATUSENUM21(23i32);
pub const DBSTATUS_E_READONLY: DBSTATUSENUM21 = DBSTATUSENUM21(24i32);
pub const DBSTATUS_E_RESOURCEOUTOFSCOPE: DBSTATUSENUM21 = DBSTATUSENUM21(25i32);
pub const DBSTATUS_S_ALREADYEXISTS: DBSTATUSENUM21 = DBSTATUSENUM21(26i32);
#[repr(transparent)]
pub struct DBSTATUSENUM25(pub i32);
pub const DBSTATUS_E_CANCELED: DBSTATUSENUM25 = DBSTATUSENUM25(27i32);
pub const DBSTATUS_E_NOTCOLLECTION: DBSTATUSENUM25 = DBSTATUSENUM25(28i32);
#[repr(transparent)]
pub struct DBSTATUSENUM26(pub i32);
pub const DBSTATUS_S_ROWSETCOLUMN: DBSTATUSENUM26 = DBSTATUSENUM26(29i32);
#[repr(transparent)]
pub struct DBTABLESTATISTICSTYPE26(pub i32);
pub const DBSTAT_HISTOGRAM: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(1i32);
pub const DBSTAT_COLUMN_CARDINALITY: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(2i32);
pub const DBSTAT_TUPLE_CARDINALITY: DBTABLESTATISTICSTYPE26 = DBTABLESTATISTICSTYPE26(4i32);
#[repr(C)]
pub struct DBTIME(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBTIMESTAMP(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBTIMESTAMP(i32);
#[repr(transparent)]
pub struct DBTYPEENUM(pub i32);
pub const DBTYPE_EMPTY: DBTYPEENUM = DBTYPEENUM(0i32);
pub const DBTYPE_NULL: DBTYPEENUM = DBTYPEENUM(1i32);
pub const DBTYPE_I2: DBTYPEENUM = DBTYPEENUM(2i32);
pub const DBTYPE_I4: DBTYPEENUM = DBTYPEENUM(3i32);
pub const DBTYPE_R4: DBTYPEENUM = DBTYPEENUM(4i32);
pub const DBTYPE_R8: DBTYPEENUM = DBTYPEENUM(5i32);
pub const DBTYPE_CY: DBTYPEENUM = DBTYPEENUM(6i32);
pub const DBTYPE_DATE: DBTYPEENUM = DBTYPEENUM(7i32);
pub const DBTYPE_BSTR: DBTYPEENUM = DBTYPEENUM(8i32);
pub const DBTYPE_IDISPATCH: DBTYPEENUM = DBTYPEENUM(9i32);
pub const DBTYPE_ERROR: DBTYPEENUM = DBTYPEENUM(10i32);
pub const DBTYPE_BOOL: DBTYPEENUM = DBTYPEENUM(11i32);
pub const DBTYPE_VARIANT: DBTYPEENUM = DBTYPEENUM(12i32);
pub const DBTYPE_IUNKNOWN: DBTYPEENUM = DBTYPEENUM(13i32);
pub const DBTYPE_DECIMAL: DBTYPEENUM = DBTYPEENUM(14i32);
pub const DBTYPE_UI1: DBTYPEENUM = DBTYPEENUM(17i32);
pub const DBTYPE_ARRAY: DBTYPEENUM = DBTYPEENUM(8192i32);
pub const DBTYPE_BYREF: DBTYPEENUM = DBTYPEENUM(16384i32);
pub const DBTYPE_I1: DBTYPEENUM = DBTYPEENUM(16i32);
pub const DBTYPE_UI2: DBTYPEENUM = DBTYPEENUM(18i32);
pub const DBTYPE_UI4: DBTYPEENUM = DBTYPEENUM(19i32);
pub const DBTYPE_I8: DBTYPEENUM = DBTYPEENUM(20i32);
pub const DBTYPE_UI8: DBTYPEENUM = DBTYPEENUM(21i32);
pub const DBTYPE_GUID: DBTYPEENUM = DBTYPEENUM(72i32);
pub const DBTYPE_VECTOR: DBTYPEENUM = DBTYPEENUM(4096i32);
pub const DBTYPE_RESERVED: DBTYPEENUM = DBTYPEENUM(32768i32);
pub const DBTYPE_BYTES: DBTYPEENUM = DBTYPEENUM(128i32);
pub const DBTYPE_STR: DBTYPEENUM = DBTYPEENUM(129i32);
pub const DBTYPE_WSTR: DBTYPEENUM = DBTYPEENUM(130i32);
pub const DBTYPE_NUMERIC: DBTYPEENUM = DBTYPEENUM(131i32);
pub const DBTYPE_UDT: DBTYPEENUM = DBTYPEENUM(132i32);
pub const DBTYPE_DBDATE: DBTYPEENUM = DBTYPEENUM(133i32);
pub const DBTYPE_DBTIME: DBTYPEENUM = DBTYPEENUM(134i32);
pub const DBTYPE_DBTIMESTAMP: DBTYPEENUM = DBTYPEENUM(135i32);
#[repr(transparent)]
pub struct DBTYPEENUM15(pub i32);
pub const DBTYPE_HCHAPTER: DBTYPEENUM15 = DBTYPEENUM15(136i32);
#[repr(transparent)]
pub struct DBTYPEENUM20(pub i32);
pub const DBTYPE_FILETIME: DBTYPEENUM20 = DBTYPEENUM20(64i32);
pub const DBTYPE_PROPVARIANT: DBTYPEENUM20 = DBTYPEENUM20(138i32);
pub const DBTYPE_VARNUMERIC: DBTYPEENUM20 = DBTYPEENUM20(139i32);
pub const DBTYPE_SQLVARIANT: u32 = 144u32;
#[repr(transparent)]
pub struct DBUPDELRULEENUM(pub i32);
pub const DBUPDELRULE_NOACTION: DBUPDELRULEENUM = DBUPDELRULEENUM(0i32);
pub const DBUPDELRULE_CASCADE: DBUPDELRULEENUM = DBUPDELRULEENUM(1i32);
pub const DBUPDELRULE_SETNULL: DBUPDELRULEENUM = DBUPDELRULEENUM(2i32);
pub const DBUPDELRULE_SETDEFAULT: DBUPDELRULEENUM = DBUPDELRULEENUM(3i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DBVECTOR(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DBVECTOR(i32);
#[repr(transparent)]
pub struct DBWATCHMODEENUM(pub i32);
pub const DBWATCHMODE_ALL: DBWATCHMODEENUM = DBWATCHMODEENUM(1i32);
pub const DBWATCHMODE_EXTEND: DBWATCHMODEENUM = DBWATCHMODEENUM(2i32);
pub const DBWATCHMODE_MOVE: DBWATCHMODEENUM = DBWATCHMODEENUM(4i32);
pub const DBWATCHMODE_COUNT: DBWATCHMODEENUM = DBWATCHMODEENUM(8i32);
#[repr(transparent)]
pub struct DBWATCHNOTIFYENUM(pub i32);
pub const DBWATCHNOTIFY_ROWSCHANGED: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(1i32);
pub const DBWATCHNOTIFY_QUERYDONE: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(2i32);
pub const DBWATCHNOTIFY_QUERYREEXECUTED: DBWATCHNOTIFYENUM = DBWATCHNOTIFYENUM(3i32);
pub const DB_ALL_EXCEPT_LIKE: u32 = 3u32;
pub const DB_BINDFLAGS_COLLECTION: i32 = 16i32;
pub const DB_BINDFLAGS_DELAYFETCHCOLUMNS: i32 = 1i32;
pub const DB_BINDFLAGS_DELAYFETCHSTREAM: i32 = 2i32;
pub const DB_BINDFLAGS_ISSTRUCTUREDDOCUMENT: i32 = 128i32;
pub const DB_BINDFLAGS_OPENIFEXISTS: i32 = 32i32;
pub const DB_BINDFLAGS_OUTPUT: i32 = 8i32;
pub const DB_BINDFLAGS_OVERWRITE: i32 = 64i32;
pub const DB_BINDFLAGS_RECURSIVE: i32 = 4i32;
pub const DB_COLLATION_ASC: u32 = 1u32;
pub const DB_COLLATION_DESC: u32 = 2u32;
pub const DB_COUNTUNAVAILABLE: i32 = -1i32;
pub const DB_E_ABORTLIMITREACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217871i32 as _);
pub const DB_E_ALREADYINITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217838i32 as _);
pub const DB_E_ALTERRESTRICTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217763i32 as _);
pub const DB_E_ASYNCNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217771i32 as _);
pub const DB_E_BADACCESSORFLAGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217850i32 as _);
pub const DB_E_BADACCESSORHANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217920i32 as _);
pub const DB_E_BADACCESSORTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217845i32 as _);
pub const DB_E_BADBINDINFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217912i32 as _);
pub const DB_E_BADBOOKMARK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217906i32 as _);
pub const DB_E_BADCHAPTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217914i32 as _);
pub const DB_E_BADCOLUMNID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217903i32 as _);
pub const DB_E_BADCOMMANDFLAGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217780i32 as _);
pub const DB_E_BADCOMMANDID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217802i32 as _);
pub const DB_E_BADCOMPAREOP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217881i32 as _);
pub const DB_E_BADCONSTRAINTFORM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217800i32 as _);
pub const DB_E_BADCONSTRAINTID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217781i32 as _);
pub const DB_E_BADCONSTRAINTTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217801i32 as _);
pub const DB_E_BADCONVERTFLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217828i32 as _);
pub const DB_E_BADCOPY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217863i32 as _);
pub const DB_E_BADDEFERRABILITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217799i32 as _);
pub const DB_E_BADDYNAMICERRORID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217830i32 as _);
pub const DB_E_BADHRESULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217832i32 as _);
pub const DB_E_BADID: i32 = -2147217860i32;
pub const DB_E_BADINDEXID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217806i32 as _);
pub const DB_E_BADINITSTRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217805i32 as _);
pub const DB_E_BADLOCKMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217905i32 as _);
pub const DB_E_BADLOOKUPID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217831i32 as _);
pub const DB_E_BADMATCHTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217792i32 as _);
pub const DB_E_BADORDINAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217835i32 as _);
pub const DB_E_BADPARAMETERNAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217827i32 as _);
pub const DB_E_BADPRECISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217862i32 as _);
pub const DB_E_BADPROPERTYVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217852i32 as _);
pub const DB_E_BADRATIO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217902i32 as _);
pub const DB_E_BADRECORDNUM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217854i32 as _);
pub const DB_E_BADREGIONHANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217878i32 as _);
pub const DB_E_BADROWHANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217916i32 as _);
pub const DB_E_BADSCALE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217861i32 as _);
pub const DB_E_BADSOURCEHANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217840i32 as _);
pub const DB_E_BADSTARTPOSITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217890i32 as _);
pub const DB_E_BADSTATUSVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217880i32 as _);
pub const DB_E_BADSTORAGEFLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217882i32 as _);
pub const DB_E_BADSTORAGEFLAGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217849i32 as _);
pub const DB_E_BADTABLEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217860i32 as _);
pub const DB_E_BADTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217859i32 as _);
pub const DB_E_BADTYPENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217872i32 as _);
pub const DB_E_BADUPDATEDELETERULE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217782i32 as _);
pub const DB_E_BADVALUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217901i32 as _);
pub const DB_E_BOGUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217665i32 as _);
pub const DB_E_BOOKMARKSKIPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217853i32 as _);
pub const DB_E_BYREFACCESSORNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217848i32 as _);
pub const DB_E_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217842i32 as _);
pub const DB_E_CANNOTCONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217770i32 as _);
pub const DB_E_CANNOTFREE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217894i32 as _);
pub const DB_E_CANNOTRESTART: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217896i32 as _);
pub const DB_E_CANTCANCEL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217899i32 as _);
pub const DB_E_CANTCONVERTVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217913i32 as _);
pub const DB_E_CANTFETCHBACKWARDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217884i32 as _);
pub const DB_E_CANTFILTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217825i32 as _);
pub const DB_E_CANTORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217824i32 as _);
pub const DB_E_CANTSCROLLBACKWARDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217879i32 as _);
pub const DB_E_CANTTRANSLATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217869i32 as _);
pub const DB_E_CHAPTERNOTRELEASED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217841i32 as _);
pub const DB_E_COLUMNUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217760i32 as _);
pub const DB_E_COMMANDNOTPERSISTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217817i32 as _);
pub const DB_E_CONCURRENCYVIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217864i32 as _);
pub const DB_E_COSTLIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217907i32 as _);
pub const DB_E_DATAOVERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217833i32 as _);
pub const DB_E_DELETEDROW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217885i32 as _);
pub const DB_E_DIALECTNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217898i32 as _);
pub const DB_E_DROPRESTRICTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217776i32 as _);
pub const DB_E_DUPLICATECOLUMNID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217858i32 as _);
pub const DB_E_DUPLICATECONSTRAINTID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217767i32 as _);
pub const DB_E_DUPLICATEDATASOURCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217897i32 as _);
pub const DB_E_DUPLICATEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217816i32 as _);
pub const DB_E_DUPLICATEINDEXID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217868i32 as _);
pub const DB_E_DUPLICATETABLEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217857i32 as _);
pub const DB_E_ERRORSINCOMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217900i32 as _);
pub const DB_E_ERRORSOCCURRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217887i32 as _);
pub const DB_E_GOALREJECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217892i32 as _);
pub const DB_E_INDEXINUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217866i32 as _);
pub const DB_E_INTEGRITYVIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217873i32 as _);
pub const DB_E_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217851i32 as _);
pub const DB_E_INVALIDTRANSITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217876i32 as _);
pub const DB_E_LIMITREJECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217909i32 as _);
pub const DB_E_MAXPENDCHANGESEXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217836i32 as _);
pub const DB_E_MISMATCHEDPROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217803i32 as _);
pub const DB_E_MULTIPLESTATEMENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217874i32 as _);
pub const DB_E_MULTIPLESTORAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217826i32 as _);
pub const DB_E_NEWLYINSERTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217893i32 as _);
pub const DB_E_NOAGGREGATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217886i32 as _);
pub const DB_E_NOCOLUMN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217819i32 as _);
pub const DB_E_NOCOMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217908i32 as _);
pub const DB_E_NOCONSTRAINT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217761i32 as _);
pub const DB_E_NOINDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217867i32 as _);
pub const DB_E_NOLOCALE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217855i32 as _);
pub const DB_E_NONCONTIGUOUSRANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217877i32 as _);
pub const DB_E_NOPROVIDERSREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217804i32 as _);
pub const DB_E_NOQUERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217889i32 as _);
pub const DB_E_NOSOURCEOBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217775i32 as _);
pub const DB_E_NOSTATISTIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217764i32 as _);
pub const DB_E_NOTABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217865i32 as _);
pub const DB_E_NOTAREFERENCECOLUMN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217910i32 as _);
pub const DB_E_NOTASUBREGION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217875i32 as _);
pub const DB_E_NOTCOLLECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217773i32 as _);
pub const DB_E_NOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217895i32 as _);
pub const DB_E_NOTPREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217846i32 as _);
pub const DB_E_NOTREENTRANT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217888i32 as _);
pub const DB_E_NOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217837i32 as _);
pub const DB_E_NULLACCESSORNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217847i32 as _);
pub const DB_E_OBJECTCREATIONLIMITREACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217815i32 as _);
pub const DB_E_OBJECTMISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217779i32 as _);
pub const DB_E_OBJECTOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217915i32 as _);
pub const DB_E_OUTOFSPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217766i32 as _);
pub const DB_E_PARAMNOTOPTIONAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217904i32 as _);
pub const DB_E_PARAMUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217839i32 as _);
pub const DB_E_PENDINGCHANGES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217834i32 as _);
pub const DB_E_PENDINGINSERT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217829i32 as _);
pub const DB_E_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217772i32 as _);
pub const DB_E_READONLYACCESSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217918i32 as _);
pub const DB_E_RESOURCEEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217768i32 as _);
pub const DB_E_RESOURCELOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217774i32 as _);
pub const DB_E_RESOURCENOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217762i32 as _);
pub const DB_E_RESOURCEOUTOFSCOPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217778i32 as _);
pub const DB_E_ROWLIMITEXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217919i32 as _);
pub const DB_E_ROWSETINCOMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217870i32 as _);
pub const DB_E_ROWSNOTRELEASED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217883i32 as _);
pub const DB_E_SCHEMAVIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217917i32 as _);
pub const DB_E_TABLEINUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217856i32 as _);
pub const DB_E_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217769i32 as _);
pub const DB_E_UNSUPPORTEDCONVERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217891i32 as _);
pub const DB_E_WRITEONLYACCESSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217844i32 as _);
pub const DB_IMP_LEVEL_ANONYMOUS: u32 = 0u32;
pub const DB_IMP_LEVEL_DELEGATE: u32 = 3u32;
pub const DB_IMP_LEVEL_IDENTIFY: u32 = 1u32;
pub const DB_IMP_LEVEL_IMPERSONATE: u32 = 2u32;
pub const DB_IN: u32 = 1u32;
pub const DB_INVALID_HACCESSOR: u32 = 0u32;
pub const DB_INVALID_HCHAPTER: u32 = 0u32;
pub const DB_LIKE_ONLY: u32 = 2u32;
pub const DB_LOCAL_EXCLUSIVE: u32 = 3u32;
pub const DB_LOCAL_SHARED: u32 = 2u32;
pub const DB_MODE_READ: u32 = 1u32;
pub const DB_MODE_READWRITE: u32 = 3u32;
pub const DB_MODE_SHARE_DENY_NONE: u32 = 16u32;
pub const DB_MODE_SHARE_DENY_READ: u32 = 4u32;
pub const DB_MODE_SHARE_DENY_WRITE: u32 = 8u32;
pub const DB_MODE_SHARE_EXCLUSIVE: u32 = 12u32;
pub const DB_MODE_WRITE: u32 = 2u32;
pub const DB_NULL_HACCESSOR: u32 = 0u32;
pub const DB_NULL_HCHAPTER: u32 = 0u32;
pub const DB_NULL_HROW: u32 = 0u32;
#[repr(C)]
pub struct DB_NUMERIC(i32);
pub const DB_OUT: u32 = 2u32;
pub const DB_PROT_LEVEL_CALL: u32 = 2u32;
pub const DB_PROT_LEVEL_CONNECT: u32 = 1u32;
pub const DB_PROT_LEVEL_NONE: u32 = 0u32;
pub const DB_PROT_LEVEL_PKT: u32 = 3u32;
pub const DB_PROT_LEVEL_PKT_INTEGRITY: u32 = 4u32;
pub const DB_PROT_LEVEL_PKT_PRIVACY: u32 = 5u32;
pub const DB_PT_FUNCTION: u32 = 3u32;
pub const DB_PT_PROCEDURE: u32 = 2u32;
pub const DB_PT_UNKNOWN: u32 = 1u32;
pub const DB_REMOTE: u32 = 1u32;
pub const DB_SEARCHABLE: u32 = 4u32;
pub const DB_SEC_E_AUTH_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217843i32 as _);
pub const DB_SEC_E_PERMISSIONDENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217911i32 as _);
pub const DB_SEC_E_SAFEMODE_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217765i32 as _);
pub const DB_S_ASYNCHRONOUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265936i32 as _);
pub const DB_S_BADROWHANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265939i32 as _);
pub const DB_S_BOOKMARKSKIPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265923i32 as _);
pub const DB_S_BUFFERFULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265928i32 as _);
pub const DB_S_CANTRELEASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265930i32 as _);
pub const DB_S_COLUMNSCHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265937i32 as _);
pub const DB_S_COLUMNTYPEMISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265921i32 as _);
pub const DB_S_COMMANDREEXECUTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265927i32 as _);
pub const DB_S_DELETEDROW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265940i32 as _);
pub const DB_S_DIALECTIGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265933i32 as _);
pub const DB_S_ENDOFROWSET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265926i32 as _);
pub const DB_S_ERRORSOCCURRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265946i32 as _);
pub const DB_S_ERRORSRETURNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265938i32 as _);
pub const DB_S_GOALCHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265931i32 as _);
pub const DB_S_LOCKUPGRADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265944i32 as _);
pub const DB_S_MULTIPLECHANGES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265948i32 as _);
pub const DB_S_NONEXTROWSET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265925i32 as _);
pub const DB_S_NORESULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265929i32 as _);
pub const DB_S_NOROWSPECIFICCOLUMNS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265949i32 as _);
pub const DB_S_NOTSINGLETON: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265943i32 as _);
pub const DB_S_PARAMUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265947i32 as _);
pub const DB_S_PROPERTIESCHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265945i32 as _);
pub const DB_S_ROWLIMITEXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265920i32 as _);
pub const DB_S_STOPLIMITREACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265942i32 as _);
pub const DB_S_TOOMANYCHANGES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265941i32 as _);
pub const DB_S_TYPEINFOOVERRIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265922i32 as _);
pub const DB_S_UNWANTEDOPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265932i32 as _);
pub const DB_S_UNWANTEDPHASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265934i32 as _);
pub const DB_S_UNWANTEDREASON: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(265935i32 as _);
pub const DB_UNSEARCHABLE: u32 = 1u32;
#[repr(C)]
pub struct DB_VARNUMERIC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct DCINFO(i32);
#[repr(transparent)]
pub struct DCINFOTYPEENUM(pub i32);
pub const DCINFOTYPE_VERSION: DCINFOTYPEENUM = DCINFOTYPEENUM(1i32);
#[repr(transparent)]
pub struct DELIVERY_AGENT_FLAGS(pub i32);
pub const DELIVERY_AGENT_FLAG_NO_BROADCAST: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(4i32);
pub const DELIVERY_AGENT_FLAG_NO_RESTRICTIONS: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(8i32);
pub const DELIVERY_AGENT_FLAG_SILENT_DIAL: DELIVERY_AGENT_FLAGS = DELIVERY_AGENT_FLAGS(16i32);
pub const DISPID_QUERY_ALL: u32 = 6u32;
pub const DISPID_QUERY_HITCOUNT: u32 = 4u32;
pub const DISPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const DISPID_QUERY_METADATA_PROPDISPID: u32 = 6u32;
pub const DISPID_QUERY_METADATA_PROPGUID: u32 = 5u32;
pub const DISPID_QUERY_METADATA_PROPMODIFIABLE: u32 = 9u32;
pub const DISPID_QUERY_METADATA_PROPNAME: u32 = 7u32;
pub const DISPID_QUERY_METADATA_STORELEVEL: u32 = 8u32;
pub const DISPID_QUERY_METADATA_VROOTAUTOMATIC: u32 = 3u32;
pub const DISPID_QUERY_METADATA_VROOTMANUAL: u32 = 4u32;
pub const DISPID_QUERY_METADATA_VROOTUSED: u32 = 2u32;
pub const DISPID_QUERY_RANK: u32 = 3u32;
pub const DISPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const DISPID_QUERY_REVNAME: u32 = 8u32;
pub const DISPID_QUERY_UNFILTERED: u32 = 7u32;
pub const DISPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const DISPID_QUERY_WORKID: u32 = 5u32;
pub const DS_E_ALREADYDISABLED: i32 = -2147220447i32;
pub const DS_E_ALREADYENABLED: i32 = -2147220454i32;
pub const DS_E_BADREQUEST: i32 = -2147220475i32;
pub const DS_E_BADRESULT: i32 = -2147220445i32;
pub const DS_E_BADSEQUENCE: i32 = -2147220473i32;
pub const DS_E_BUFFERTOOSMALL: i32 = -2147220449i32;
pub const DS_E_CANNOTREMOVECONCURRENT: i32 = -2147220443i32;
pub const DS_E_CANNOTWRITEREGISTRY: i32 = -2147220444i32;
pub const DS_E_CONFIGBAD: i32 = -2147220470i32;
pub const DS_E_CONFIGNOTRIGHTTYPE: i32 = -2147220456i32;
pub const DS_E_DATANOTPRESENT: i32 = -2147220464i32;
pub const DS_E_DATASOURCENOTAVAILABLE: i32 = -2147220478i32;
pub const DS_E_DATASOURCENOTDISABLED: i32 = -2147220459i32;
pub const DS_E_DUPLICATEID: i32 = -2147220462i32;
pub const DS_E_INDEXDIRECTORY: i32 = -2147220452i32;
pub const DS_E_INVALIDCATALOGNAME: i32 = -2147220457i32;
pub const DS_E_INVALIDDATASOURCE: i32 = -2147220479i32;
pub const DS_E_INVALIDTAGDB: i32 = -2147220458i32;
pub const DS_E_MESSAGETOOLONG: i32 = -2147220472i32;
pub const DS_E_MISSINGCATALOG: i32 = -2147220440i32;
pub const DS_E_NOMOREDATA: i32 = -2147220480i32;
pub const DS_E_PARAMOUTOFRANGE: i32 = -2147220448i32;
pub const DS_E_PROPVERSIONMISMATCH: i32 = -2147220441i32;
pub const DS_E_PROTOCOLVERSION: i32 = -2147220455i32;
pub const DS_E_QUERYCANCELED: i32 = -2147220477i32;
pub const DS_E_QUERYHUNG: i32 = -2147220446i32;
pub const DS_E_REGISTRY: i32 = -2147220460i32;
pub const DS_E_SEARCHCATNAMECOLLISION: i32 = -2147220442i32;
pub const DS_E_SERVERCAPACITY: i32 = -2147220474i32;
pub const DS_E_SERVERERROR: i32 = -2147220471i32;
pub const DS_E_SETSTATUSINPROGRESS: i32 = -2147220463i32;
pub const DS_E_TOOMANYDATASOURCES: i32 = -2147220461i32;
pub const DS_E_UNKNOWNPARAM: i32 = -2147220450i32;
pub const DS_E_UNKNOWNREQUEST: i32 = -2147220476i32;
pub const DS_E_VALUETOOLARGE: i32 = -2147220451i32;
#[repr(C)]
pub struct DataLinks(i32);
#[repr(transparent)]
pub struct DataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataSourceListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataSourceObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EBindInfoOptions(pub i32);
pub const BIO_BINDER: EBindInfoOptions = EBindInfoOptions(1i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct ERRORINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct ERRORINFO(i32);
pub const ERROR_FTE: u32 = 13824u32;
pub const ERROR_FTE_CB: u32 = 51968u32;
pub const ERROR_FTE_FD: u32 = 64768u32;
pub const ERROR_SOURCE_CMDLINE: u32 = 5376u32;
pub const ERROR_SOURCE_COLLATOR: u32 = 1280u32;
pub const ERROR_SOURCE_CONNMGR: u32 = 1536u32;
pub const ERROR_SOURCE_CONTENT_SOURCE: u32 = 13312u32;
pub const ERROR_SOURCE_DATASOURCE: u32 = 1024u32;
pub const ERROR_SOURCE_DAV: u32 = 8960u32;
pub const ERROR_SOURCE_EXSTOREPH: u32 = 9984u32;
pub const ERROR_SOURCE_FLTRDMN: u32 = 9216u32;
pub const ERROR_SOURCE_GATHERER: u32 = 3328u32;
pub const ERROR_SOURCE_INDEXER: u32 = 4352u32;
pub const ERROR_SOURCE_MSS: u32 = 8448u32;
pub const ERROR_SOURCE_NETWORKING: u32 = 768u32;
pub const ERROR_SOURCE_NLADMIN: u32 = 6400u32;
pub const ERROR_SOURCE_NOTESPH: u32 = 9728u32;
pub const ERROR_SOURCE_OLEDB_BINDER: u32 = 9472u32;
pub const ERROR_SOURCE_PEOPLE_IMPORT: u32 = 16384u32;
pub const ERROR_SOURCE_PROTHNDLR: u32 = 4608u32;
pub const ERROR_SOURCE_QUERY: u32 = 1792u32;
pub const ERROR_SOURCE_REMOTE_EXSTOREPH: u32 = 13568u32;
pub const ERROR_SOURCE_SCHEMA: u32 = 3072u32;
pub const ERROR_SOURCE_SCRIPTPI: u32 = 8192u32;
pub const ERROR_SOURCE_SECURITY: u32 = 5120u32;
pub const ERROR_SOURCE_SETUP: u32 = 4864u32;
pub const ERROR_SOURCE_SRCH_SCHEMA_CACHE: u32 = 13056u32;
pub const ERROR_SOURCE_XML: u32 = 8704u32;
pub const EVENT_AUDIENCECOMPUTATION_CANNOTSTART: i32 = -1073738223i32;
pub const EVENT_AUTOCAT_CANT_CREATE_FILE_SHARE: i32 = -1073738726i32;
pub const EVENT_AUTOCAT_PERFMON: i32 = -1073738753i32;
pub const EVENT_CONFIG_ERROR: i32 = -1073738821i32;
pub const EVENT_CONFIG_SYNTAX: i32 = -2147482604i32;
pub const EVENT_CRAWL_SCHEDULED: i32 = 1073744884i32;
pub const EVENT_DETAILED_FILTERPOOL_ADD_FAILED: i32 = -1073738719i32;
pub const EVENT_DSS_NOT_ENABLED: i32 = -2147476572i32;
pub const EVENT_ENUMERATE_SESSIONS_FAILED: i32 = -1073738720i32;
pub const EVENT_EXCEPTION: i32 = -1073740815i32;
pub const EVENT_FAILED_CREATE_GATHERER_LOG: i32 = -2147480587i32;
pub const EVENT_FAILED_INITIALIZE_CRAWL: i32 = -1073738765i32;
pub const EVENT_FILTERPOOL_ADD_FAILED: i32 = -1073738722i32;
pub const EVENT_FILTERPOOL_DELETE_FAILED: i32 = -1073738721i32;
pub const EVENT_FILTER_HOST_FORCE_TERMINATE: i32 = -2147473624i32;
pub const EVENT_FILTER_HOST_NOT_INITIALIZED: i32 = -1073738724i32;
pub const EVENT_FILTER_HOST_NOT_TERMINATED: i32 = -1073738723i32;
pub const EVENT_GATHERER_DATASOURCE: i32 = -1073738727i32;
pub const EVENT_GATHERER_PERFMON: i32 = -1073738817i32;
pub const EVENT_GATHERSVC_PERFMON: i32 = -1073738818i32;
pub const EVENT_GATHER_ADVISE_FAILED: i32 = -1073738798i32;
pub const EVENT_GATHER_APP_INIT_FAILED: i32 = -1073738766i32;
pub const EVENT_GATHER_AUTODESCENCODE_INVALID: i32 = -2147480592i32;
pub const EVENT_GATHER_AUTODESCLEN_ADJUSTED: i32 = -2147480603i32;
pub const EVENT_GATHER_BACKUPAPP_COMPLETE: i32 = 3077i32;
pub const EVENT_GATHER_BACKUPAPP_ERROR: i32 = -1073738748i32;
pub const EVENT_GATHER_CANT_CREATE_DOCID: i32 = -1073738793i32;
pub const EVENT_GATHER_CANT_DELETE_DOCID: i32 = -1073738792i32;
pub const EVENT_GATHER_CHECKPOINT_CORRUPT: i32 = -1073738732i32;
pub const EVENT_GATHER_CHECKPOINT_FAILED: i32 = -1073738736i32;
pub const EVENT_GATHER_CHECKPOINT_FILE_MISSING: i32 = -1073738731i32;
pub const EVENT_GATHER_CRAWL_IN_PROGRESS: i32 = -2147480609i32;
pub const EVENT_GATHER_CRAWL_NOT_STARTED: i32 = -2147480625i32;
pub const EVENT_GATHER_CRAWL_SEED_ERROR: i32 = -2147480624i32;
pub const EVENT_GATHER_CRAWL_SEED_FAILED: i32 = -2147480612i32;
pub const EVENT_GATHER_CRAWL_SEED_FAILED_INIT: i32 = -2147480611i32;
pub const EVENT_GATHER_CRITICAL_ERROR: i32 = -1073738799i32;
pub const EVENT_GATHER_DAEMON_TERMINATED: i32 = -2147480570i32;
pub const EVENT_GATHER_DELETING_HISTORY_ITEMS: i32 = -1073738774i32;
pub const EVENT_GATHER_DIRTY_STARTUP: i32 = -2147480576i32;
pub const EVENT_GATHER_DISK_FULL: i32 = -2147480594i32;
pub const EVENT_GATHER_END_ADAPTIVE: i32 = 1073744891i32;
pub const EVENT_GATHER_END_CRAWL: i32 = 1073744842i32;
pub const EVENT_GATHER_END_INCREMENTAL: i32 = 1073744871i32;
pub const EVENT_GATHER_EXCEPTION: i32 = -1073738810i32;
pub const EVENT_GATHER_FLUSH_FAILED: i32 = -1073738737i32;
pub const EVENT_GATHER_FROM_NOT_SET: i32 = -1073738776i32;
pub const EVENT_GATHER_HISTORY_CORRUPTION_DETECTED: i32 = -2147480575i32;
pub const EVENT_GATHER_INTERNAL: i32 = -1073738804i32;
pub const EVENT_GATHER_INVALID_NETWORK_ACCESS_ACCOUNT: i32 = -1073738739i32;
pub const EVENT_GATHER_LOCK_FAILED: i32 = -1073738784i32;
pub const EVENT_GATHER_NO_CRAWL_SEEDS: i32 = -2147480602i32;
pub const EVENT_GATHER_NO_SCHEMA: i32 = -2147480593i32;
pub const EVENT_GATHER_OBJ_INIT_FAILED: i32 = -1073738796i32;
pub const EVENT_GATHER_PLUGINMGR_INIT_FAILED: i32 = -1073738767i32;
pub const EVENT_GATHER_PLUGIN_INIT_FAILED: i32 = -1073738795i32;
pub const EVENT_GATHER_PROTOCOLHANDLER_INIT_FAILED: i32 = -1073738740i32;
pub const EVENT_GATHER_PROTOCOLHANDLER_LOAD_FAILED: i32 = -1073738741i32;
pub const EVENT_GATHER_READ_CHECKPOINT_FAILED: i32 = -1073738733i32;
pub const EVENT_GATHER_RECOVERY_FAILURE: i32 = -1073738222i32;
pub const EVENT_GATHER_REG_MISSING: i32 = -2147480610i32;
pub const EVENT_GATHER_RESET_START: i32 = 1073744865i32;
pub const EVENT_GATHER_RESTOREAPP_COMPLETE: i32 = 3075i32;
pub const EVENT_GATHER_RESTOREAPP_ERROR: i32 = -1073738750i32;
pub const EVENT_GATHER_RESTORE_CHECKPOINT_FAILED: i32 = -1073738734i32;
pub const EVENT_GATHER_RESTORE_COMPLETE: i32 = 3069i32;
pub const EVENT_GATHER_RESTORE_ERROR: i32 = -1073738754i32;
pub const EVENT_GATHER_RESUME: i32 = 1073744868i32;
pub const EVENT_GATHER_SAVE_FAILED: i32 = -1073738735i32;
pub const EVENT_GATHER_SERVICE_INIT: i32 = -1073738794i32;
pub const EVENT_GATHER_START_CRAWL: i32 = 1073744843i32;
pub const EVENT_GATHER_START_CRAWL_IF_RESET: i32 = -2147480595i32;
pub const EVENT_GATHER_START_PAUSE: i32 = -2147480606i32;
pub const EVENT_GATHER_STOP_START: i32 = 1073744876i32;
pub const EVENT_GATHER_SYSTEM_LCID_CHANGED: i32 = -2147480562i32;
pub const EVENT_GATHER_THROTTLE: i32 = 1073744867i32;
pub const EVENT_GATHER_TRANSACTION_FAIL: i32 = -1073738797i32;
pub const EVENT_HASHMAP_INSERT: i32 = -1073738816i32;
pub const EVENT_HASHMAP_UPDATE: i32 = -1073738811i32;
pub const EVENT_INDEXER_ADD_DSS_DISCONNECT: i32 = -2147476585i32;
pub const EVENT_INDEXER_ADD_DSS_FAILED: i32 = -2147476627i32;
pub const EVENT_INDEXER_ADD_DSS_SUCCEEDED: i32 = 7019i32;
pub const EVENT_INDEXER_BUILD_ENDED: i32 = 1073748873i32;
pub const EVENT_INDEXER_BUILD_FAILED: i32 = -1073734797i32;
pub const EVENT_INDEXER_BUILD_START: i32 = 1073748872i32;
pub const EVENT_INDEXER_CI_LOAD_ERROR: i32 = -1073734785i32;
pub const EVENT_INDEXER_DSS_ALREADY_ADDED: i32 = 1073748870i32;
pub const EVENT_INDEXER_DSS_CONTACT_FAILED: i32 = -1073734800i32;
pub const EVENT_INDEXER_DSS_UNABLE_TO_REMOVE: i32 = -1073734755i32;
pub const EVENT_INDEXER_FAIL_TO_CREATE_PER_USER_CATALOG: i32 = -1073731797i32;
pub const EVENT_INDEXER_FAIL_TO_SET_MAX_JETINSTANCE: i32 = -1073731798i32;
pub const EVENT_INDEXER_FAIL_TO_UNLOAD_PER_USER_CATALOG: i32 = -1073731796i32;
pub const EVENT_INDEXER_INIT_ERROR: i32 = -1073734814i32;
pub const EVENT_INDEXER_INVALID_DIRECTORY: i32 = -1073734813i32;
pub const EVENT_INDEXER_LOAD_FAIL: i32 = -1073734781i32;
pub const EVENT_INDEXER_MISSING_APP_DIRECTORY: i32 = -1073734758i32;
pub const EVENT_INDEXER_NEW_PROJECT: i32 = -1073734754i32;
pub const EVENT_INDEXER_NO_SEARCH_SERVERS: i32 = -2147476630i32;
pub const EVENT_INDEXER_OUT_OF_DATABASE_INSTANCE: i32 = -1073731799i32;
pub const EVENT_INDEXER_PAUSED_FOR_DISKFULL: i32 = -1073734811i32;
pub const EVENT_INDEXER_PERFMON: i32 = -1073734760i32;
pub const EVENT_INDEXER_PROPSTORE_INIT_FAILED: i32 = -1073734787i32;
pub const EVENT_INDEXER_PROP_ABORTED: i32 = 1073748899i32;
pub const EVENT_INDEXER_PROP_COMMITTED: i32 = 1073748898i32;
pub const EVENT_INDEXER_PROP_COMMIT_FAILED: i32 = -1073734747i32;
pub const EVENT_INDEXER_PROP_ERROR: i32 = -1073734812i32;
pub const EVENT_INDEXER_PROP_STARTED: i32 = 1073748841i32;
pub const EVENT_INDEXER_PROP_STATE_CORRUPT: i32 = -1073734780i32;
pub const EVENT_INDEXER_PROP_STOPPED: i32 = -2147476633i32;
pub const EVENT_INDEXER_PROP_SUCCEEDED: i32 = 7016i32;
pub const EVENT_INDEXER_REG_ERROR: i32 = -1073734756i32;
pub const EVENT_INDEXER_REG_MISSING: i32 = -1073734796i32;
pub const EVENT_INDEXER_REMOVED_PROJECT: i32 = -1073734753i32;
pub const EVENT_INDEXER_REMOVE_DSS_FAILED: i32 = -1073734801i32;
pub const EVENT_INDEXER_REMOVE_DSS_SUCCEEDED: i32 = 7020i32;
pub const EVENT_INDEXER_RESET_FOR_CORRUPTION: i32 = -1073734784i32;
pub const EVENT_INDEXER_SCHEMA_COPY_ERROR: i32 = -1073734823i32;
pub const EVENT_INDEXER_SHUTDOWN: i32 = 1073748866i32;
pub const EVENT_INDEXER_STARTED: i32 = 1073748824i32;
pub const EVENT_INDEXER_VERIFY_PROP_ACCOUNT: i32 = -1073734768i32;
pub const EVENT_LEARN_COMPILE_FAILED: i32 = -2147480583i32;
pub const EVENT_LEARN_CREATE_DB_FAILED: i32 = -2147480584i32;
pub const EVENT_LEARN_PROPAGATION_COPY_FAILED: i32 = -2147480585i32;
pub const EVENT_LEARN_PROPAGATION_FAILED: i32 = -2147480582i32;
pub const EVENT_LOCAL_GROUPS_CACHE_FLUSHED: i32 = 1073744920i32;
pub const EVENT_LOCAL_GROUP_NOT_EXPANDED: i32 = 1073744919i32;
pub const EVENT_NOTIFICATION_FAILURE: i32 = -1073738745i32;
pub const EVENT_NOTIFICATION_FAILURE_SCOPE_EXCEEDED_LOGGING: i32 = -2147480568i32;
pub const EVENT_NOTIFICATION_RESTORED: i32 = 1073744905i32;
pub const EVENT_NOTIFICATION_RESTORED_SCOPE_EXCEEDED_LOGGING: i32 = -2147480566i32;
pub const EVENT_NOTIFICATION_THREAD_EXIT_FAILED: i32 = -1073738725i32;
pub const EVENT_OUTOFMEMORY: i32 = -1073740817i32;
pub const EVENT_PERF_COUNTERS_ALREADY_EXISTS: i32 = -2147473626i32;
pub const EVENT_PERF_COUNTERS_NOT_LOADED: i32 = -2147473628i32;
pub const EVENT_PERF_COUNTERS_REGISTRY_TROUBLE: i32 = -2147473627i32;
pub const EVENT_PROTOCOL_HOST_FORCE_TERMINATE: i32 = -2147473625i32;
pub const EVENT_REG_VERSION: i32 = -1073738790i32;
pub const EVENT_SSSEARCH_CREATE_PATH_RULES_FAILED: i32 = -2147482634i32;
pub const EVENT_SSSEARCH_CSM_SAVE_FAILED: i32 = -1073740805i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_FAILED: i32 = -1073740808i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_ROLLBACK_ERRORS: i32 = -2147482630i32;
pub const EVENT_SSSEARCH_DATAFILES_MOVE_SUCCEEDED: i32 = 1073742841i32;
pub const EVENT_SSSEARCH_DROPPED_EVENTS: i32 = -2147482633i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_FAILED: i32 = -1073740813i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_STARTED: i32 = -2147482640i32;
pub const EVENT_SSSEARCH_SETUP_CLEANUP_SUCCEEDED: i32 = 1073742834i32;
pub const EVENT_SSSEARCH_SETUP_FAILED: i32 = -1073740818i32;
pub const EVENT_SSSEARCH_SETUP_SUCCEEDED: i32 = 1073742829i32;
pub const EVENT_SSSEARCH_STARTED: i32 = 1073742827i32;
pub const EVENT_SSSEARCH_STARTING_SETUP: i32 = 1073742828i32;
pub const EVENT_SSSEARCH_STOPPED: i32 = 1073742837i32;
pub const EVENT_STS_INIT_SECURITY_FAILED: i32 = -2147480554i32;
pub const EVENT_SYSTEM_EXCEPTION: i32 = -2147482595i32;
pub const EVENT_TRANSACTION_READ: i32 = -1073738809i32;
pub const EVENT_TRANSLOG_APPEND: i32 = -1073738814i32;
pub const EVENT_TRANSLOG_CREATE: i32 = -1073738791i32;
pub const EVENT_TRANSLOG_CREATE_TRX: i32 = -1073738815i32;
pub const EVENT_TRANSLOG_UPDATE: i32 = -1073738813i32;
pub const EVENT_UNPRIVILEGED_SERVICE_ACCOUNT: i32 = -2147482596i32;
pub const EVENT_USING_DIFFERENT_WORD_BREAKER: i32 = -2147480580i32;
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILE: i32 = -2147473634i32;
pub const EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILES: i32 = -2147473635i32;
pub const EVENT_WBREAKER_NOT_LOADED: i32 = -2147480586i32;
pub const EVENT_WIN32_ERROR: i32 = -2147473633i32;
pub const EXCI_E_ACCESS_DENIED: i32 = -2147216990i32;
pub const EXCI_E_BADCONFIG_OR_ACCESSDENIED: i32 = -2147216988i32;
pub const EXCI_E_INVALID_ACCOUNT_INFO: i32 = -2147216984i32;
pub const EXCI_E_INVALID_EXCHANGE_SERVER: i32 = -2147216989i32;
pub const EXCI_E_INVALID_SERVER_CONFIG: i32 = -2147216991i32;
pub const EXCI_E_NOT_ADMIN_OR_WRONG_SITE: i32 = -2147216986i32;
pub const EXCI_E_NO_CONFIG: i32 = -2147216992i32;
pub const EXCI_E_NO_MAPI: i32 = -2147216985i32;
pub const EXCI_E_WRONG_SERVER_OR_ACCT: i32 = -2147216987i32;
pub const EXSTOREPH_E_UNEXPECTED: i32 = -2147211519i32;
pub const EX_ANY: u32 = 0u32;
pub const EX_CMDFATAL: u32 = 20u32;
pub const EX_CONTROL: u32 = 25u32;
pub const EX_DBCORRUPT: u32 = 23u32;
pub const EX_DBFATAL: u32 = 21u32;
pub const EX_DEADLOCK: u32 = 13u32;
pub const EX_HARDWARE: u32 = 24u32;
pub const EX_INFO: u32 = 10u32;
pub const EX_INTOK: u32 = 18u32;
pub const EX_LIMIT: u32 = 19u32;
pub const EX_MAXISEVERITY: u32 = 10u32;
pub const EX_MISSING: u32 = 11u32;
pub const EX_PERMIT: u32 = 14u32;
pub const EX_RESOURCE: u32 = 17u32;
pub const EX_SYNTAX: u32 = 15u32;
pub const EX_TABCORRUPT: u32 = 22u32;
pub const EX_TYPE: u32 = 12u32;
pub const EX_USER: u32 = 16u32;
pub const FAIL: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILTERED_DATA_SOURCES(i32);
pub const FLTRDMN_E_CANNOT_DECRYPT_PASSWORD: i32 = -2147212282i32;
pub const FLTRDMN_E_ENCRYPTED_DOCUMENT: i32 = -2147212283i32;
pub const FLTRDMN_E_FILTER_INIT_FAILED: i32 = -2147212284i32;
pub const FLTRDMN_E_QI_FILTER_FAILED: i32 = -2147212286i32;
pub const FLTRDMN_E_UNEXPECTED: i32 = -2147212287i32;
#[repr(transparent)]
pub struct FOLLOW_FLAGS(pub i32);
pub const FF_INDEXCOMPLEXURLS: FOLLOW_FLAGS = FOLLOW_FLAGS(1i32);
pub const FF_SUPPRESSINDEXING: FOLLOW_FLAGS = FOLLOW_FLAGS(2i32);
pub const FTE_E_ADMIN_BLOB_CORRUPT: i32 = -2147207676i32;
pub const FTE_E_AFFINITY_MASK: i32 = -2147207651i32;
pub const FTE_E_ALREADY_INITIALIZED: i32 = -2147207604i32;
pub const FTE_E_ANOTHER_STATUS_CHANGE_IS_ALREADY_ACTIVE: i32 = -2147207635i32;
pub const FTE_E_BATCH_ABORTED: i32 = -2147207636i32;
pub const FTE_E_CATALOG_ALREADY_EXISTS: i32 = -2147207656i32;
pub const FTE_E_CATALOG_DOES_NOT_EXIST: i32 = -2147207639i32;
pub const FTE_E_CB_CBID_OUT_OF_BOUND: i32 = -2147169535i32;
pub const FTE_E_CB_NOT_ENOUGH_AVAIL_PHY_MEM: i32 = -2147169534i32;
pub const FTE_E_CB_NOT_ENOUGH_OCC_BUFFER: i32 = -2147169533i32;
pub const FTE_E_CB_OUT_OF_MEMORY: i32 = -2147169536i32;
pub const FTE_E_COM_SIGNATURE_VALIDATION: i32 = -2147207652i32;
pub const FTE_E_CORRUPT_GATHERER_HASH_MAP: i32 = -2147207619i32;
pub const FTE_E_CORRUPT_PROPERTY_STORE: i32 = -2147207622i32;
pub const FTE_E_CORRUPT_WORDLIST: i32 = -2147169532i32;
pub const FTE_E_DATATYPE_MISALIGNMENT: i32 = -2147207605i32;
pub const FTE_E_DEPENDENT_TRAN_FAILED_TO_PERSIST: i32 = -2147207641i32;
pub const FTE_E_DOC_TOO_HUGE: i32 = -2147207606i32;
pub const FTE_E_DUPLICATE_OBJECT: i32 = -2147207644i32;
pub const FTE_E_ERROR_WRITING_REGISTRY: i32 = -2147207674i32;
pub const FTE_E_EXCEEDED_MAX_PLUGINS: i32 = -2147207647i32;
pub const FTE_E_FAILED_TO_CREATE_ACCESSOR: i32 = -2147207625i32;
pub const FTE_E_FAILURE_TO_POST_SETCOMPLETION_STATUS: i32 = -2147207597i32;
pub const FTE_E_FD_DID_NOT_CONNECT: i32 = -2147207660i32;
pub const FTE_E_FD_DOC_TIMEOUT: i32 = -2147156733i32;
pub const FTE_E_FD_DOC_UNEXPECTED_EXIT: i32 = -2147156731i32;
pub const FTE_E_FD_FAILED_TO_LOAD_IFILTER: i32 = -2147156734i32;
pub const FTE_E_FD_FILTER_CAUSED_SHARING_VIOLATION: i32 = -2147156725i32;
pub const FTE_E_FD_IDLE: i32 = -2147207595i32;
pub const FTE_E_FD_IFILTER_INIT_FAILED: i32 = -2147156735i32;
pub const FTE_E_FD_NOISE_NO_IPERSISTSTREAM_ON_TEXT_FILTER: i32 = -2147156729i32;
pub const FTE_E_FD_NOISE_NO_TEXT_FILTER: i32 = -2147156730i32;
pub const FTE_E_FD_NOISE_TEXT_FILTER_INIT_FAILED: i32 = -2147156727i32;
pub const FTE_E_FD_NOISE_TEXT_FILTER_LOAD_FAILED: i32 = -2147156728i32;
pub const FTE_E_FD_NO_IPERSIST_INTERFACE: i32 = -2147156736i32;
pub const FTE_E_FD_OCCURRENCE_OVERFLOW: i32 = -2147156726i32;
pub const FTE_E_FD_OWNERSHIP_OBSOLETE: i32 = -2147207650i32;
pub const FTE_E_FD_SHUTDOWN: i32 = -2147207640i32;
pub const FTE_E_FD_TIMEOUT: i32 = -2147207632i32;
pub const FTE_E_FD_UNEXPECTED_EXIT: i32 = -2147156732i32;
pub const FTE_E_FD_UNRESPONSIVE: i32 = -2147207594i32;
pub const FTE_E_FD_USED_TOO_MUCH_MEMORY: i32 = -2147207603i32;
pub const FTE_E_FILTER_SINGLE_THREADED: i32 = -2147207675i32;
pub const FTE_E_HIGH_MEMORY_PRESSURE: i32 = -2147207601i32;
pub const FTE_E_INVALID_CODEPAGE: i32 = -2147207596i32;
pub const FTE_E_INVALID_DOCID: i32 = -2147207663i32;
pub const FTE_E_INVALID_ISOLATE_ERROR_BATCH: i32 = -2147207600i32;
pub const FTE_E_INVALID_PROG_ID: i32 = -2147207614i32;
pub const FTE_E_INVALID_PROJECT_ID: i32 = -2147207598i32;
pub const FTE_E_INVALID_PROPERTY: i32 = -2147207630i32;
pub const FTE_E_INVALID_TYPE: i32 = -2147207624i32;
pub const FTE_E_KEY_NOT_CACHED: i32 = -2147207618i32;
pub const FTE_E_LIBRARY_NOT_LOADED: i32 = -2147207627i32;
pub const FTE_E_NOT_PROCESSED_DUE_TO_PREVIOUS_ERRORS: i32 = -2147207633i32;
pub const FTE_E_NO_MORE_PROPERTIES: i32 = -2147207629i32;
pub const FTE_E_NO_PLUGINS: i32 = -2147207638i32;
pub const FTE_E_NO_PROPERTY_STORE: i32 = -1073465766i32;
pub const FTE_E_OUT_OF_RANGE: i32 = -2147207623i32;
pub const FTE_E_PATH_TOO_LONG: i32 = -2147207654i32;
pub const FTE_E_PAUSE_EXTERNAL: i32 = -2147207662i32;
pub const FTE_E_PERFMON_FULL: i32 = -2147207626i32;
pub const FTE_E_PERF_NOT_LOADED: i32 = -2147207611i32;
pub const FTE_E_PIPE_DATA_CORRUPTED: i32 = -2147207671i32;
pub const FTE_E_PIPE_NOT_CONNECTED: i32 = -2147207677i32;
pub const FTE_E_PROGID_REQUIRED: i32 = -2147207658i32;
pub const FTE_E_PROJECT_NOT_INITALIZED: i32 = -2147207672i32;
pub const FTE_E_PROJECT_SHUTDOWN: i32 = -2147207673i32;
pub const FTE_E_PROPERTY_STORE_WORKID_NOTVALID: i32 = -2147207621i32;
pub const FTE_E_READONLY_CATALOG: i32 = -2147207612i32;
pub const FTE_E_REDUNDANT_TRAN_FAILURE: i32 = -2147207642i32;
pub const FTE_E_REJECTED_DUE_TO_PROJECT_STATUS: i32 = -2147207661i32;
pub const FTE_E_RESOURCE_SHUTDOWN: i32 = -2147207631i32;
pub const FTE_E_RETRY_HUGE_DOC: i32 = -2147207608i32;
pub const FTE_E_RETRY_SINGLE_DOC_PER_BATCH: i32 = -2147207599i32;
pub const FTE_E_SECRET_NOT_FOUND: i32 = -2147207678i32;
pub const FTE_E_SERIAL_STREAM_CORRUPT: i32 = -2147207613i32;
pub const FTE_E_STACK_CORRUPTED: i32 = -2147207615i32;
pub const FTE_E_STATIC_THREAD_INVALID_ARGUMENTS: i32 = -2147207657i32;
pub const FTE_E_UNEXPECTED_EXIT: i32 = -2147207602i32;
pub const FTE_E_UNKNOWN_FD_TYPE: i32 = -2147207607i32;
pub const FTE_E_UNKNOWN_PLUGIN: i32 = -2147207628i32;
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_INSTANTIATED: i32 = -2147207616i32;
pub const FTE_E_UPGRADE_INTERFACE_ALREADY_SHUTDOWN: i32 = -2147207617i32;
pub const FTE_E_URB_TOO_BIG: i32 = -2147207664i32;
pub const FTE_INVALID_ADMIN_CLIENT: i32 = -2147207653i32;
pub const FTE_S_BEYOND_QUOTA: i32 = 276002i32;
pub const FTE_S_CATALOG_BLOB_MISMATCHED: i32 = 276056i32;
pub const FTE_S_PROPERTY_RESET: i32 = 276057i32;
pub const FTE_S_PROPERTY_STORE_END_OF_ENUMERATION: i32 = 276028i32;
pub const FTE_S_READONLY_CATALOG: i32 = 276038i32;
pub const FTE_S_REDUNDANT: i32 = 276005i32;
pub const FTE_S_RESOURCES_STARTING_TO_GET_LOW: i32 = 275993i32;
pub const FTE_S_RESUME: i32 = 276014i32;
pub const FTE_S_STATUS_CHANGE_REQUEST: i32 = 276011i32;
pub const FTE_S_TRY_TO_FLUSH: i32 = 276055i32;
#[repr(C)]
pub struct FilterRegistration(i32);
pub const GENERATE_METHOD_PREFIXMATCH: u32 = 1u32;
pub const GENERATE_METHOD_STEMMED: u32 = 2u32;
pub const GHTR_E_INSUFFICIENT_DISK_SPACE: i32 = -2147218037i32;
pub const GHTR_E_LOCAL_SERVER_UNAVAILABLE: i32 = -2147218055i32;
pub const GTHR_E_ADDLINKS_FAILED_WILL_RETRY_PARENT: i32 = -2147217989i32;
pub const GTHR_E_APPLICATION_NOT_FOUND: i32 = -2147218079i32;
pub const GTHR_E_AUTOCAT_UNEXPECTED: i32 = -2147218012i32;
pub const GTHR_E_BACKUP_VALIDATION_FAIL: i32 = -2147217994i32;
pub const GTHR_E_BAD_FILTER_DAEMON: i32 = -2147218119i32;
pub const GTHR_E_BAD_FILTER_HOST: i32 = -2147217993i32;
pub const GTHR_E_CANNOT_ENABLE_CHECKPOINT: i32 = -2147218002i32;
pub const GTHR_E_CANNOT_REMOVE_PLUGINMGR: i32 = -2147218078i32;
pub const GTHR_E_CONFIG_DUP_EXTENSION: i32 = -2147218165i32;
pub const GTHR_E_CONFIG_DUP_PROJECT: i32 = -2147218166i32;
pub const GTHR_E_CONTENT_ID_CONFLICT: i32 = -2147218062i32;
pub const GTHR_E_DIRMON_NOT_INITIALZED: i32 = -2147218019i32;
pub const GTHR_E_DUPLICATE_OBJECT: i32 = -2147218174i32;
pub const GTHR_E_DUPLICATE_PROJECT: i32 = -2147218094i32;
pub const GTHR_E_DUPLICATE_URL: i32 = -2147218163i32;
pub const GTHR_E_DUP_PROPERTY_MAPPING: i32 = -2147218134i32;
pub const GTHR_E_EMPTY_DACL: i32 = -2147218006i32;
pub const GTHR_E_ERROR_INITIALIZING_PERFMON: i32 = -2147218171i32;
pub const GTHR_E_ERROR_OBJECT_NOT_FOUND: i32 = -2147218170i32;
pub const GTHR_E_ERROR_WRITING_REGISTRY: i32 = -2147218172i32;
pub const GTHR_E_FILTERPOOL_NOTFOUND: i32 = -2147217990i32;
pub const GTHR_E_FILTER_FAULT: i32 = -2147218075i32;
pub const GTHR_E_FILTER_INIT: i32 = -2147218130i32;
pub const GTHR_E_FILTER_INTERRUPTED: i32 = -2147218092i32;
pub const GTHR_E_FILTER_INVALID_MESSAGE: i32 = -2147218158i32;
pub const GTHR_E_FILTER_NOT_FOUND: i32 = -2147218154i32;
pub const GTHR_E_FILTER_NO_CODEPAGE: i32 = -2147218123i32;
pub const GTHR_E_FILTER_NO_MORE_THREADS: i32 = -2147218153i32;
pub const GTHR_E_FILTER_PROCESS_TERMINATED: i32 = -2147218159i32;
pub const GTHR_E_FILTER_PROCESS_TERMINATED_QUOTA: i32 = -2147218151i32;
pub const GTHR_E_FILTER_SINGLE_THREADED: i32 = -2147218069i32;
pub const GTHR_E_FOLDER_CRAWLED_BY_ANOTHER_WORKSPACE: i32 = -2147218007i32;
pub const GTHR_E_FORCE_NOTIFICATION_RESET: i32 = -2147218065i32;
pub const GTHR_E_FROM_NOT_SPECIFIED: i32 = -2147218109i32;
pub const GTHR_E_IE_OFFLINE: i32 = -2147218120i32;
pub const GTHR_E_INSUFFICIENT_EXAMPLE_CATEGORIES: i32 = -2147218014i32;
pub const GTHR_E_INSUFFICIENT_EXAMPLE_DOCUMENTS: i32 = -2147218013i32;
pub const GTHR_E_INSUFFICIENT_FEATURE_TERMS: i32 = -2147218015i32;
pub const GTHR_E_INVALIDFUNCTION: i32 = -2147218161i32;
pub const GTHR_E_INVALID_ACCOUNT: i32 = -2147218132i32;
pub const GTHR_E_INVALID_ACCOUNT_SYNTAX: i32 = -2147218129i32;
pub const GTHR_E_INVALID_APPLICATION_NAME: i32 = -2147218077i32;
pub const GTHR_E_INVALID_CALL_FROM_WBREAKER: i32 = -2147218058i32;
pub const GTHR_E_INVALID_DIRECTORY: i32 = -2147218093i32;
pub const GTHR_E_INVALID_EXTENSION: i32 = -2147218107i32;
pub const GTHR_E_INVALID_GROW_FACTOR: i32 = -2147218106i32;
pub const GTHR_E_INVALID_HOST_NAME: i32 = -2147218096i32;
pub const GTHR_E_INVALID_LOG_FILE_NAME: i32 = -2147218103i32;
pub const GTHR_E_INVALID_MAPPING: i32 = -2147218112i32;
pub const GTHR_E_INVALID_PATH: i32 = -2147218124i32;
pub const GTHR_E_INVALID_PATH_EXPRESSION: i32 = -2147218088i32;
pub const GTHR_E_INVALID_PATH_SPEC: i32 = -2147218016i32;
pub const GTHR_E_INVALID_PROJECT_NAME: i32 = -2147218142i32;
pub const GTHR_E_INVALID_PROXY_PORT: i32 = -2147218091i32;
pub const GTHR_E_INVALID_RESOURCE_ID: i32 = -2147218035i32;
pub const GTHR_E_INVALID_RETRIES: i32 = -2147218104i32;
pub const GTHR_E_INVALID_START_ADDRESS: i32 = -2147217998i32;
pub const GTHR_E_INVALID_START_PAGE: i32 = -2147218095i32;
pub const GTHR_E_INVALID_START_PAGE_HOST: i32 = -2147218087i32;
pub const GTHR_E_INVALID_START_PAGE_PATH: i32 = -2147218080i32;
pub const GTHR_E_INVALID_STREAM_LOGS_COUNT: i32 = -2147218108i32;
pub const GTHR_E_INVALID_TIME_OUT: i32 = -2147218105i32;
pub const GTHR_E_JET_BACKUP_ERROR: i32 = -2147218026i32;
pub const GTHR_E_JET_RESTORE_ERROR: i32 = -2147218025i32;
pub const GTHR_E_LOCAL_GROUPS_EXPANSION_INTERNAL_ERROR: i32 = -2147216867i32;
pub const GTHR_E_NAME_TOO_LONG: i32 = -2147218156i32;
pub const GTHR_E_NESTED_HIERARCHICAL_START_ADDRESSES: i32 = -2147218034i32;
pub const GTHR_E_NOFILTERSINK: i32 = -2147218160i32;
pub const GTHR_E_NON_FIXED_DRIVE: i32 = -2147218074i32;
pub const GTHR_E_NOTIFICATION_FILE_SHARE_INFO_NOT_AVAILABLE: i32 = -2147218040i32;
pub const GTHR_E_NOTIFICATION_LOCAL_PATH_MUST_USE_FIXED_DRIVE: i32 = -2147218039i32;
pub const GTHR_E_NOTIFICATION_START_ADDRESS_INVALID: i32 = -2147218042i32;
pub const GTHR_E_NOTIFICATION_START_PAGE: i32 = -2147218137i32;
pub const GTHR_E_NOTIFICATION_TYPE_NOT_SUPPORTED: i32 = -2147218041i32;
pub const GTHR_E_NOTIF_ACCESS_TOKEN_UPDATED: i32 = -2147218020i32;
pub const GTHR_E_NOTIF_BEING_REMOVED: i32 = -2147218018i32;
pub const GTHR_E_NOTIF_EXCESSIVE_THROUGHPUT: i32 = -2147218017i32;
pub const GTHR_E_NO_IDENTITY: i32 = -2147218155i32;
pub const GTHR_E_NO_PRTCLHNLR: i32 = -2147218121i32;
pub const GTHR_E_NTF_CLIENT_NOT_SUBSCRIBED: i32 = -1073476167i32;
pub const GTHR_E_OBJECT_NOT_VALID: i32 = -2147218005i32;
pub const GTHR_E_OUT_OF_DOC_ID: i32 = -2147218138i32;
pub const GTHR_E_PIPE_NOT_CONNECTTED: i32 = -2147217996i32;
pub const GTHR_E_PLUGIN_NOT_REGISTERED: i32 = -2147218021i32;
pub const GTHR_E_PROJECT_NOT_INITIALIZED: i32 = -2147218149i32;
pub const GTHR_E_PROPERTIES_EXCEEDED: i32 = -2147218000i32;
pub const GTHR_E_PROPERTY_LIST_NOT_INITIALIZED: i32 = -2147218057i32;
pub const GTHR_E_PROXY_NAME: i32 = -2147218127i32;
pub const GTHR_E_PRT_HNDLR_PROGID_MISSING: i32 = -2147218152i32;
pub const GTHR_E_RECOVERABLE_EXOLEDB_ERROR: i32 = -2147218060i32;
pub const GTHR_E_RETRY: i32 = -2147218027i32;
pub const GTHR_E_SCHEMA_ERRORS_OCCURRED: i32 = -2147218054i32;
pub const GTHR_E_SCOPES_EXCEEDED: i32 = -2147218001i32;
pub const GTHR_E_SECRET_NOT_FOUND: i32 = -2147218089i32;
pub const GTHR_E_SERVER_UNAVAILABLE: i32 = -2147218126i32;
pub const GTHR_E_SHUTTING_DOWN: i32 = -2147218141i32;
pub const GTHR_E_SINGLE_THREADED_EMBEDDING: i32 = -2147218011i32;
pub const GTHR_E_TIMEOUT: i32 = -2147218053i32;
pub const GTHR_E_TOO_MANY_PLUGINS: i32 = -2147218162i32;
pub const GTHR_E_UNABLE_TO_READ_EXCHANGE_STORE: i32 = -2147218061i32;
pub const GTHR_E_UNABLE_TO_READ_REGISTRY: i32 = -2147218173i32;
pub const GTHR_E_UNKNOWN_PROTOCOL: i32 = -2147218150i32;
pub const GTHR_E_UNSUPPORTED_PROPERTY_TYPE: i32 = -2147218157i32;
pub const GTHR_E_URL_EXCLUDED: i32 = -2147218169i32;
pub const GTHR_E_URL_UNIDENTIFIED: i32 = -2147218067i32;
pub const GTHR_E_USER_AGENT_NOT_SPECIFIED: i32 = -2147218111i32;
pub const GTHR_E_VALUE_NOT_AVAILABLE: i32 = -2147218139i32;
pub const GTHR_S_BAD_FILE_LINK: i32 = 265580i32;
pub const GTHR_S_CANNOT_FILTER: i32 = 265520i32;
pub const GTHR_S_CANNOT_WORDBREAK: i32 = 265638i32;
pub const GTHR_S_CONFIG_HAS_ACCOUNTS: i32 = 265558i32;
pub const GTHR_S_CRAWL_ADAPTIVE: i32 = 265605i32;
pub const GTHR_S_CRAWL_FULL: i32 = 265603i32;
pub const GTHR_S_CRAWL_INCREMENTAL: i32 = 265604i32;
pub const GTHR_S_CRAWL_SCHEDULED: i32 = 265576i32;
pub const GTHR_S_END_PROCESS_LOOP_NOTIFY_QUEUE: i32 = 265584i32;
pub const GTHR_S_END_STD_CHUNKS: i32 = 265508i32;
pub const GTHR_S_MODIFIED_PARTS: i32 = 265592i32;
pub const GTHR_S_NOT_ALL_PARTS: i32 = 265582i32;
pub const GTHR_S_NO_CRAWL_SEEDS: i32 = 265515i32;
pub const GTHR_S_NO_INDEX: i32 = 265616i32;
pub const GTHR_S_OFFICE_CHILD: i32 = 265626i32;
pub const GTHR_S_PAUSE_REASON_BACKOFF: i32 = 265620i32;
pub const GTHR_S_PAUSE_REASON_EXTERNAL: i32 = 265618i32;
pub const GTHR_S_PAUSE_REASON_PROFILE_IMPORT: i32 = 265651i32;
pub const GTHR_S_PAUSE_REASON_UPGRADING: i32 = 265619i32;
pub const GTHR_S_PROB_NOT_MODIFIED: i32 = 265575i32;
pub const GTHR_S_START_FILTER_FROM_BODY: i32 = 265585i32;
pub const GTHR_S_START_FILTER_FROM_PROTOCOL: i32 = 265578i32;
pub const GTHR_S_STATUS_CHANGE_IGNORED: i32 = 265500i32;
pub const GTHR_S_STATUS_END_CRAWL: i32 = 265501i32;
pub const GTHR_S_STATUS_PAUSE: i32 = 265505i32;
pub const GTHR_S_STATUS_RESET: i32 = 265502i32;
pub const GTHR_S_STATUS_RESUME: i32 = 265504i32;
pub const GTHR_S_STATUS_START: i32 = 265526i32;
pub const GTHR_S_STATUS_STOP: i32 = 265523i32;
pub const GTHR_S_STATUS_THROTTLE: i32 = 265503i32;
pub const GTHR_S_TRANSACTION_IGNORED: i32 = 265577i32;
pub const GTHR_S_USE_MIME_FILTER: i32 = 265639i32;
#[repr(C)]
pub struct HITRANGE(i32);
#[repr(transparent)]
pub struct IAccessor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlterIndex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlterTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChapteredRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnMapperCreator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnsInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnsRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandCost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandPersist(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandPrepare(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandValidate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandWithParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICondition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICondition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConditionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConditionFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConditionGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConvertType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBAsynchNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBAsynchStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBBinderProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBCreateCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBCreateSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBDataSourceAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBPromptInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBSchemaCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDBSchemaRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCInfo(pub *mut ::core::ffi::c_void);
pub const IDENTIFIER_SDK_ERROR: u32 = 268435456u32;
pub const IDENTIFIER_SDK_MASK: u32 = 4026531840u32;
pub const IDS_MON_BUILTIN_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264511i32 as _);
pub const IDS_MON_BUILTIN_VIEW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264503i32 as _);
pub const IDS_MON_CANNOT_CAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264518i32 as _);
pub const IDS_MON_CANNOT_CONVERT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264507i32 as _);
pub const IDS_MON_COLUMN_NOT_DEFINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264502i32 as _);
pub const IDS_MON_DATE_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264519i32 as _);
pub const IDS_MON_DEFAULT_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264495i32 as _);
pub const IDS_MON_ILLEGAL_PASSTHROUGH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264496i32 as _);
pub const IDS_MON_INVALIDSELECT_COALESCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264517i32 as _);
pub const IDS_MON_INVALID_CATALOG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264516i32 as _);
pub const IDS_MON_INVALID_IN_GROUP_CLAUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264520i32 as _);
pub const IDS_MON_MATCH_STRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264513i32 as _);
pub const IDS_MON_NOT_COLUMN_OF_VIEW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264510i32 as _);
pub const IDS_MON_ORDINAL_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264500i32 as _);
pub const IDS_MON_OR_NOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264506i32 as _);
pub const IDS_MON_OUT_OF_MEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264504i32 as _);
pub const IDS_MON_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264508i32 as _);
pub const IDS_MON_PARSE_ERR_1_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264497i32 as _);
pub const IDS_MON_PARSE_ERR_2_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264498i32 as _);
pub const IDS_MON_PROPERTY_NAME_IN_VIEW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264514i32 as _);
pub const IDS_MON_RELATIVE_INTERVAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264509i32 as _);
pub const IDS_MON_SELECT_STAR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264505i32 as _);
pub const IDS_MON_SEMI_COLON: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264499i32 as _);
pub const IDS_MON_VIEW_ALREADY_DEFINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264515i32 as _);
pub const IDS_MON_VIEW_NOT_DEFINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264501i32 as _);
pub const IDS_MON_WEIGHT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(264512i32 as _);
pub const IDX_E_BUILD_IN_PROGRESS: i32 = -2147217147i32;
pub const IDX_E_CATALOG_DISMOUNTED: i32 = -2147217124i32;
pub const IDX_E_CORRUPT_INDEX: i32 = -2147217136i32;
pub const IDX_E_DISKFULL: i32 = -2147217138i32;
pub const IDX_E_DOCUMENT_ABORTED: i32 = -2147217125i32;
pub const IDX_E_DSS_NOT_CONNECTED: i32 = -2147217126i32;
pub const IDX_E_IDXLSTFILE_CORRUPT: i32 = -2147217146i32;
pub const IDX_E_INVALIDTAG: i32 = -2147217151i32;
pub const IDX_E_INVALID_INDEX: i32 = -2147217137i32;
pub const IDX_E_METAFILE_CORRUPT: i32 = -2147217150i32;
pub const IDX_E_NOISELIST_NOTFOUND: i32 = -2147217141i32;
pub const IDX_E_NOT_LOADED: i32 = -2147217129i32;
pub const IDX_E_OBJECT_NOT_FOUND: i32 = -2147217144i32;
pub const IDX_E_PROPSTORE_INIT_FAILED: i32 = -2147217134i32;
pub const IDX_E_PROP_MAJOR_VERSION_MISMATCH: i32 = -2147217128i32;
pub const IDX_E_PROP_MINOR_VERSION_MISMATCH: i32 = -2147217127i32;
pub const IDX_E_PROP_STATE_CORRUPT: i32 = -2147217133i32;
pub const IDX_E_PROP_STOPPED: i32 = -2147217139i32;
pub const IDX_E_REGISTRY_ENTRY: i32 = -2147217145i32;
pub const IDX_E_SEARCH_SERVER_ALREADY_EXISTS: i32 = -2147217148i32;
pub const IDX_E_SEARCH_SERVER_NOT_FOUND: i32 = -2147217143i32;
pub const IDX_E_STEMMER_NOTFOUND: i32 = -2147217140i32;
pub const IDX_E_TOO_MANY_SEARCH_SERVERS: i32 = -2147217149i32;
pub const IDX_E_USE_APPGLOBAL_PROPTABLE: i32 = -2147217120i32;
pub const IDX_E_USE_DEFAULT_CONTENTCLASS: i32 = -2147217121i32;
pub const IDX_E_WB_NOTFOUND: i32 = -2147217142i32;
pub const IDX_S_DSS_NOT_AVAILABLE: i32 = 266525i32;
pub const IDX_S_NO_BUILD_IN_PROGRESS: i32 = 266516i32;
pub const IDX_S_SEARCH_SERVER_ALREADY_EXISTS: i32 = 266517i32;
pub const IDX_S_SEARCH_SERVER_DOES_NOT_EXIST: i32 = 266518i32;
#[repr(transparent)]
pub struct IDataConvert(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataSourceLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumItemProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSearchRoots(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSearchScopeRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorLookup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorRecords(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetSourceRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIndexDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInterval(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoadFilterWithPrivateComActivation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDDataset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDFind(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDRangeRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMetaData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultipleResults(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INCREMENTAL_ACCESS_INFO(i32);
pub const INET_E_AGENT_CACHE_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693246i32 as _);
pub const INET_E_AGENT_CONNECTION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693245i32 as _);
pub const INET_E_AGENT_EXCEEDING_CACHE_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693232i32 as _);
pub const INET_E_AGENT_MAX_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693248i32 as _);
pub const INET_E_SCHEDULED_EXCLUDE_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693241i32 as _);
pub const INET_E_SCHEDULED_UPDATES_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693244i32 as _);
pub const INET_E_SCHEDULED_UPDATES_RESTRICTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693243i32 as _);
pub const INET_E_SCHEDULED_UPDATE_INTERVAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146693242i32 as _);
pub const INET_S_AGENT_INCREASED_CACHE_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(790416i32 as _);
pub const INET_S_AGENT_PART_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(790401i32 as _);
#[repr(transparent)]
pub struct INTERVAL_LIMIT_KIND(pub i32);
pub const ILK_EXPLICIT_INCLUDED: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(0i32);
pub const ILK_EXPLICIT_EXCLUDED: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(1i32);
pub const ILK_NEGATIVE_INFINITY: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(2i32);
pub const ILK_POSITIVE_INFINITY: INTERVAL_LIMIT_KIND = INTERVAL_LIMIT_KIND(3i32);
#[repr(transparent)]
pub struct INamedEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamedEntityCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectAccessControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpLockStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParentRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtocolHandlerSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideMoniker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryParserManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuerySolution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReadData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisterProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelationship(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichChunk(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowPosition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowPositionChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowSchemaChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetAsynch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetBookmark(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetChangeExtInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetChapterMember(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetCopyRows(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetCurrentIndex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetEvents(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IRowsetExactScroll(i32);
#[repr(transparent)]
pub struct IRowsetFastLoad(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetFind(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetIndex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetLocate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetNewRowAfter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetNextRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetPrioritization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetQueryStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetRefresh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetResynch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetScroll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetUpdate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetWatchAll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetWatchNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetWatchRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowsetWithParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISQLErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISQLGetDiagField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISQLRequestDiagFields(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISQLServerErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISchemaLocalizerSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISchemaLock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISchemaProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScopedOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCatalogManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCatalogManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCrawlScopeManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchCrawlScopeManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchItemsChangedSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchLanguageSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchNotifyInlineSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchPersistentItemsChangedSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchProtocol(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchProtocol2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchProtocolThreadContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchQueryHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchQueryHits(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchScopeRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchViewChangedSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISessionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleCommandCreator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISourcesRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStemmer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubscriptionItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubscriptionMgr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISubscriptionMgr2(pub *mut ::core::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct ITEMPROP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ITEM_INFO(i32);
#[repr(transparent)]
pub struct ITableCreation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableDefinitionWithConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableRename(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITokenCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionJoin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionLocal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrusteeAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrusteeGroupAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUMS(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUMSInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlAccessor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlAccessor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlAccessor3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUrlAccessor4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewChapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewRowset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewSort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordBreaker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordFormSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordSink(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Interval(i32);
pub const JET_GET_PROP_STORE_ERROR: i32 = -1073732822i32;
pub const JET_INIT_ERROR: i32 = -1073732824i32;
pub const JET_MULTIINSTANCE_DISABLED: i32 = -2147474645i32;
pub const JET_NEW_PROP_STORE_ERROR: i32 = -1073732823i32;
pub const JPS_E_CATALOG_DECSRIPTION_MISSING: i32 = -2147217023i32;
pub const JPS_E_INSUFFICIENT_DATABASE_RESOURCES: i32 = -2147217019i32;
pub const JPS_E_INSUFFICIENT_DATABASE_SESSIONS: i32 = -2147217020i32;
pub const JPS_E_INSUFFICIENT_VERSION_STORAGE: i32 = -2147217021i32;
pub const JPS_E_JET_ERR: i32 = -2147217025i32;
pub const JPS_E_MISSING_INFORMATION: i32 = -2147217022i32;
pub const JPS_E_PROPAGATION_CORRUPTION: i32 = -2147217016i32;
pub const JPS_E_PROPAGATION_FILE: i32 = -2147217017i32;
pub const JPS_E_PROPAGATION_VERSION_MISMATCH: i32 = -2147217015i32;
pub const JPS_E_SCHEMA_ERROR: i32 = -2147217018i32;
pub const JPS_E_SHARING_VIOLATION: i32 = -2147217014i32;
pub const JPS_S_DUPLICATE_DOC_DETECTED: i32 = 266624i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct KAGGETDIAG(i32);
pub const KAGPROPVAL_CONCUR_LOCK: u32 = 4u32;
pub const KAGPROPVAL_CONCUR_READ_ONLY: u32 = 8u32;
pub const KAGPROPVAL_CONCUR_ROWVER: u32 = 1u32;
pub const KAGPROPVAL_CONCUR_VALUES: u32 = 2u32;
pub const KAGPROP_ACCESSIBLEPROCEDURES: u32 = 2u32;
pub const KAGPROP_ACCESSIBLETABLES: u32 = 3u32;
pub const KAGPROP_ACTIVESTATEMENTS: u32 = 24u32;
pub const KAGPROP_AUTH_SERVERINTEGRATED: u32 = 3u32;
pub const KAGPROP_AUTH_TRUSTEDCONNECTION: u32 = 2u32;
pub const KAGPROP_BLOBSONFOCURSOR: u32 = 8u32;
pub const KAGPROP_CONCURRENCY: u32 = 7u32;
pub const KAGPROP_CURSOR: u32 = 6u32;
pub const KAGPROP_DRIVERNAME: u32 = 7u32;
pub const KAGPROP_DRIVERODBCVER: u32 = 9u32;
pub const KAGPROP_DRIVERVER: u32 = 8u32;
pub const KAGPROP_FILEUSAGE: u32 = 23u32;
pub const KAGPROP_FORCENOPARAMETERREBIND: u32 = 11u32;
pub const KAGPROP_FORCENOPREPARE: u32 = 12u32;
pub const KAGPROP_FORCENOREEXECUTE: u32 = 13u32;
pub const KAGPROP_FORCESSFIREHOSEMODE: u32 = 10u32;
pub const KAGPROP_INCLUDENONEXACT: u32 = 9u32;
pub const KAGPROP_IRowsetChangeExtInfo: u32 = 5u32;
pub const KAGPROP_LIKEESCAPECLAUSE: u32 = 10u32;
pub const KAGPROP_MARSHALLABLE: u32 = 3u32;
pub const KAGPROP_MAXCOLUMNSINGROUPBY: u32 = 12u32;
pub const KAGPROP_MAXCOLUMNSININDEX: u32 = 13u32;
pub const KAGPROP_MAXCOLUMNSINORDERBY: u32 = 14u32;
pub const KAGPROP_MAXCOLUMNSINSELECT: u32 = 15u32;
pub const KAGPROP_MAXCOLUMNSINTABLE: u32 = 16u32;
pub const KAGPROP_NUMERICFUNCTIONS: u32 = 17u32;
pub const KAGPROP_ODBCSQLCONFORMANCE: u32 = 18u32;
pub const KAGPROP_ODBCSQLOPTIEF: u32 = 4u32;
pub const KAGPROP_OJCAPABILITY: u32 = 5u32;
pub const KAGPROP_OUTERJOINS: u32 = 19u32;
pub const KAGPROP_POSITIONONNEWROW: u32 = 4u32;
pub const KAGPROP_PROCEDURES: u32 = 6u32;
pub const KAGPROP_QUERYBASEDUPDATES: u32 = 2u32;
pub const KAGPROP_SPECIALCHARACTERS: u32 = 11u32;
pub const KAGPROP_STRINGFUNCTIONS: u32 = 20u32;
pub const KAGPROP_SYSTEMFUNCTIONS: u32 = 21u32;
pub const KAGPROP_TIMEDATEFUNCTIONS: u32 = 22u32;
#[repr(C)]
pub struct KAGREQDIAG(i32);
#[repr(transparent)]
pub struct KAGREQDIAGFLAGSENUM(pub i32);
pub const KAGREQDIAGFLAGS_HEADER: KAGREQDIAGFLAGSENUM = KAGREQDIAGFLAGSENUM(1i32);
pub const KAGREQDIAGFLAGS_RECORD: KAGREQDIAGFLAGSENUM = KAGREQDIAGFLAGSENUM(2i32);
#[repr(transparent)]
pub struct LOCKMODEENUM(pub i32);
pub const LOCKMODE_INVALID: LOCKMODEENUM = LOCKMODEENUM(0i32);
pub const LOCKMODE_EXCLUSIVE: LOCKMODEENUM = LOCKMODEENUM(1i32);
pub const LOCKMODE_SHARED: LOCKMODEENUM = LOCKMODEENUM(2i32);
#[repr(C)]
pub struct LeafCondition(i32);
pub const MAXNAME: u32 = 129u32;
pub const MAXNUMERICLEN: u32 = 16u32;
pub const MAXUSEVERITY: u32 = 18u32;
pub const MAX_QUERY_RANK: u32 = 1000u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MDAXISINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MDAXISINFO(i32);
pub const MDAXIS_CHAPTERS: u32 = 4u32;
pub const MDAXIS_COLUMNS: u32 = 0u32;
pub const MDAXIS_PAGES: u32 = 2u32;
pub const MDAXIS_ROWS: u32 = 1u32;
pub const MDAXIS_SECTIONS: u32 = 3u32;
pub const MDAXIS_SLICERS: u32 = 4294967295u32;
pub const MDDISPINFO_DRILLED_DOWN: u32 = 65536u32;
pub const MDDISPINFO_PARENT_SAME_AS_PREV: u32 = 131072u32;
pub const MDFF_BOLD: u32 = 1u32;
pub const MDFF_ITALIC: u32 = 2u32;
pub const MDFF_STRIKEOUT: u32 = 8u32;
pub const MDFF_UNDERLINE: u32 = 4u32;
pub const MDLEVEL_TYPE_ALL: u32 = 1u32;
pub const MDLEVEL_TYPE_CALCULATED: u32 = 2u32;
pub const MDLEVEL_TYPE_REGULAR: u32 = 0u32;
pub const MDLEVEL_TYPE_RESERVED1: u32 = 8u32;
pub const MDLEVEL_TYPE_TIME: u32 = 4u32;
pub const MDLEVEL_TYPE_TIME_DAYS: u32 = 516u32;
pub const MDLEVEL_TYPE_TIME_HALF_YEAR: u32 = 36u32;
pub const MDLEVEL_TYPE_TIME_HOURS: u32 = 772u32;
pub const MDLEVEL_TYPE_TIME_MINUTES: u32 = 1028u32;
pub const MDLEVEL_TYPE_TIME_MONTHS: u32 = 132u32;
pub const MDLEVEL_TYPE_TIME_QUARTERS: u32 = 68u32;
pub const MDLEVEL_TYPE_TIME_SECONDS: u32 = 2052u32;
pub const MDLEVEL_TYPE_TIME_UNDEFINED: u32 = 4100u32;
pub const MDLEVEL_TYPE_TIME_WEEKS: u32 = 260u32;
pub const MDLEVEL_TYPE_TIME_YEARS: u32 = 20u32;
pub const MDLEVEL_TYPE_UNKNOWN: u32 = 0u32;
pub const MDMEASURE_AGGR_AVG: u32 = 5u32;
pub const MDMEASURE_AGGR_CALCULATED: u32 = 127u32;
pub const MDMEASURE_AGGR_COUNT: u32 = 2u32;
pub const MDMEASURE_AGGR_MAX: u32 = 4u32;
pub const MDMEASURE_AGGR_MIN: u32 = 3u32;
pub const MDMEASURE_AGGR_STD: u32 = 7u32;
pub const MDMEASURE_AGGR_SUM: u32 = 1u32;
pub const MDMEASURE_AGGR_UNKNOWN: u32 = 0u32;
pub const MDMEASURE_AGGR_VAR: u32 = 6u32;
pub const MDMEMBER_TYPE_ALL: u32 = 2u32;
pub const MDMEMBER_TYPE_FORMULA: u32 = 4u32;
pub const MDMEMBER_TYPE_MEASURE: u32 = 3u32;
pub const MDMEMBER_TYPE_REGULAR: u32 = 1u32;
pub const MDMEMBER_TYPE_RESERVE1: u32 = 5u32;
pub const MDMEMBER_TYPE_RESERVE2: u32 = 6u32;
pub const MDMEMBER_TYPE_RESERVE3: u32 = 7u32;
pub const MDMEMBER_TYPE_RESERVE4: u32 = 8u32;
pub const MDMEMBER_TYPE_UNKNOWN: u32 = 0u32;
pub const MDPROPVAL_AU_UNCHANGED: i32 = 1i32;
pub const MDPROPVAL_AU_UNKNOWN: i32 = 2i32;
pub const MDPROPVAL_AU_UNSUPPORTED: i32 = 0i32;
pub const MDPROPVAL_FS_FULL_SUPPORT: i32 = 1i32;
pub const MDPROPVAL_FS_GENERATED_COLUMN: i32 = 2i32;
pub const MDPROPVAL_FS_GENERATED_DIMENSION: i32 = 3i32;
pub const MDPROPVAL_FS_NO_SUPPORT: i32 = 4i32;
pub const MDPROPVAL_MC_SEARCHEDCASE: i32 = 2i32;
pub const MDPROPVAL_MC_SINGLECASE: i32 = 1i32;
pub const MDPROPVAL_MD_AFTER: i32 = 4i32;
pub const MDPROPVAL_MD_BEFORE: i32 = 2i32;
pub const MDPROPVAL_MD_SELF: i32 = 1i32;
pub const MDPROPVAL_MF_CREATE_CALCMEMBERS: i32 = 4i32;
pub const MDPROPVAL_MF_CREATE_NAMEDSETS: i32 = 8i32;
pub const MDPROPVAL_MF_SCOPE_GLOBAL: i32 = 32i32;
pub const MDPROPVAL_MF_SCOPE_SESSION: i32 = 16i32;
pub const MDPROPVAL_MF_WITH_CALCMEMBERS: i32 = 1i32;
pub const MDPROPVAL_MF_WITH_NAMEDSETS: i32 = 2i32;
pub const MDPROPVAL_MJC_IMPLICITCUBE: i32 = 4i32;
pub const MDPROPVAL_MJC_MULTICUBES: i32 = 2i32;
pub const MDPROPVAL_MJC_SINGLECUBE: i32 = 1i32;
pub const MDPROPVAL_MMF_CLOSINGPERIOD: i32 = 8i32;
pub const MDPROPVAL_MMF_COUSIN: i32 = 1i32;
pub const MDPROPVAL_MMF_OPENINGPERIOD: i32 = 4i32;
pub const MDPROPVAL_MMF_PARALLELPERIOD: i32 = 2i32;
pub const MDPROPVAL_MNF_AGGREGATE: i32 = 16i32;
pub const MDPROPVAL_MNF_CORRELATION: i32 = 64i32;
pub const MDPROPVAL_MNF_COVARIANCE: i32 = 32i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVEL: i32 = 2048i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
pub const MDPROPVAL_MNF_DRILLDOWNLEVELTOP: i32 = 16384i32;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
pub const MDPROPVAL_MNF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
pub const MDPROPVAL_MNF_DRILLUPLEVEL: i32 = 131072i32;
pub const MDPROPVAL_MNF_DRILLUPMEMBER: i32 = 65536i32;
pub const MDPROPVAL_MNF_LINREG2: i32 = 512i32;
pub const MDPROPVAL_MNF_LINREGPOINT: i32 = 1024i32;
pub const MDPROPVAL_MNF_LINREGSLOPE: i32 = 128i32;
pub const MDPROPVAL_MNF_LINREGVARIANCE: i32 = 256i32;
pub const MDPROPVAL_MNF_MEDIAN: i32 = 1i32;
pub const MDPROPVAL_MNF_RANK: i32 = 8i32;
pub const MDPROPVAL_MNF_STDDEV: i32 = 4i32;
pub const MDPROPVAL_MNF_VAR: i32 = 2i32;
pub const MDPROPVAL_MOQ_CATALOG_CUBE: i32 = 2i32;
pub const MDPROPVAL_MOQ_CUBE_DIM: i32 = 8i32;
pub const MDPROPVAL_MOQ_DATASOURCE_CUBE: i32 = 1i32;
pub const MDPROPVAL_MOQ_DIMHIER_LEVEL: i32 = 32i32;
pub const MDPROPVAL_MOQ_DIMHIER_MEMBER: i32 = 256i32;
pub const MDPROPVAL_MOQ_DIM_HIER: i32 = 16i32;
pub const MDPROPVAL_MOQ_LEVEL_MEMBER: i32 = 64i32;
pub const MDPROPVAL_MOQ_MEMBER_MEMBER: i32 = 128i32;
pub const MDPROPVAL_MOQ_OUTERREFERENCE: i32 = 1i32;
pub const MDPROPVAL_MOQ_SCHEMA_CUBE: i32 = 4i32;
pub const MDPROPVAL_MSC_GREATERTHAN: i32 = 2i32;
pub const MDPROPVAL_MSC_GREATERTHANEQUAL: i32 = 8i32;
pub const MDPROPVAL_MSC_LESSTHAN: i32 = 1i32;
pub const MDPROPVAL_MSC_LESSTHANEQUAL: i32 = 4i32;
pub const MDPROPVAL_MSF_BOTTOMPERCENT: i32 = 2i32;
pub const MDPROPVAL_MSF_BOTTOMSUM: i32 = 8i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVEL: i32 = 2048i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELBOTTOM: i32 = 32768i32;
pub const MDPROPVAL_MSF_DRILLDOWNLEVELTOP: i32 = 16384i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBBER: i32 = 1024i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERBOTTOM: i32 = 8192i32;
pub const MDPROPVAL_MSF_DRILLDOWNMEMBERTOP: i32 = 4096i32;
pub const MDPROPVAL_MSF_DRILLUPLEVEL: i32 = 131072i32;
pub const MDPROPVAL_MSF_DRILLUPMEMBER: i32 = 65536i32;
pub const MDPROPVAL_MSF_LASTPERIODS: i32 = 32i32;
pub const MDPROPVAL_MSF_MTD: i32 = 256i32;
pub const MDPROPVAL_MSF_PERIODSTODATE: i32 = 16i32;
pub const MDPROPVAL_MSF_QTD: i32 = 128i32;
pub const MDPROPVAL_MSF_TOGGLEDRILLSTATE: i32 = 262144i32;
pub const MDPROPVAL_MSF_TOPPERCENT: i32 = 1i32;
pub const MDPROPVAL_MSF_TOPSUM: i32 = 4i32;
pub const MDPROPVAL_MSF_WTD: i32 = 512i32;
pub const MDPROPVAL_MSF_YTD: i32 = 64i32;
pub const MDPROPVAL_MS_MULTIPLETUPLES: i32 = 1i32;
pub const MDPROPVAL_MS_SINGLETUPLE: i32 = 2i32;
pub const MDPROPVAL_NL_NAMEDLEVELS: i32 = 1i32;
pub const MDPROPVAL_NL_NUMBEREDLEVELS: i32 = 2i32;
pub const MDPROPVAL_NL_SCHEMAONLY: i32 = 4i32;
pub const MDPROPVAL_NME_ALLDIMENSIONS: i32 = 0i32;
pub const MDPROPVAL_NME_MEASURESONLY: i32 = 1i32;
pub const MDPROPVAL_RR_NORANGEROWSET: i32 = 1i32;
pub const MDPROPVAL_RR_READONLY: i32 = 2i32;
pub const MDPROPVAL_RR_UPDATE: i32 = 4i32;
pub const MDPROPVAL_VISUAL_MODE_DEFAULT: i32 = 0i32;
pub const MDPROPVAL_VISUAL_MODE_VISUAL: i32 = 1i32;
pub const MDPROPVAL_VISUAL_MODE_VISUAL_OFF: i32 = 2i32;
pub const MDPROP_CELL: u32 = 2u32;
pub const MDPROP_MEMBER: u32 = 1u32;
pub const MDTREEOP_ANCESTORS: u32 = 32u32;
pub const MDTREEOP_CHILDREN: u32 = 1u32;
pub const MDTREEOP_DESCENDANTS: u32 = 16u32;
pub const MDTREEOP_PARENT: u32 = 4u32;
pub const MDTREEOP_SELF: u32 = 8u32;
pub const MDTREEOP_SIBLINGS: u32 = 2u32;
pub const MD_DIMTYPE_MEASURE: u32 = 2u32;
pub const MD_DIMTYPE_OTHER: u32 = 3u32;
pub const MD_DIMTYPE_TIME: u32 = 1u32;
pub const MD_DIMTYPE_UNKNOWN: u32 = 0u32;
pub const MD_E_BADCOORDINATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217822i32 as _);
pub const MD_E_BADTUPLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217823i32 as _);
pub const MD_E_INVALIDAXIS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217821i32 as _);
pub const MD_E_INVALIDCELLRANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217820i32 as _);
pub const MINFATALERR: u32 = 20u32;
pub const MIN_USER_DATATYPE: u32 = 256u32;
#[repr(C)]
pub struct MSDAINITIALIZE(i32);
#[repr(C)]
pub struct MSDAORA(i32);
#[repr(C)]
pub struct MSDAORA8(i32);
#[repr(C)]
pub struct MSDAORA8_ERROR(i32);
#[repr(C)]
pub struct MSDAORA_ERROR(i32);
#[repr(transparent)]
pub struct MSDSDBINITPROPENUM(pub i32);
pub const DBPROP_MSDS_DBINIT_DATAPROVIDER: MSDSDBINITPROPENUM = MSDSDBINITPROPENUM(2i32);
#[repr(transparent)]
pub struct MSDSSESSIONPROPENUM(pub i32);
pub const DBPROP_MSDS_SESS_UNIQUENAMES: MSDSSESSIONPROPENUM = MSDSSESSIONPROPENUM(2i32);
pub const MSG_CI_CORRUPT_INDEX_COMPONENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745962i32 as _);
pub const MSG_CI_CREATE_SEVER_ITEM_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147479480i32 as _);
pub const MSG_CI_MASTER_MERGE_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745928i32 as _);
pub const MSG_CI_MASTER_MERGE_ABORTED_LOW_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745987i32 as _);
pub const MSG_CI_MASTER_MERGE_CANT_RESTART: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073737718i32 as _);
pub const MSG_CI_MASTER_MERGE_CANT_START: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073737719i32 as _);
pub const MSG_CI_MASTER_MERGE_COMPLETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745927i32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_EXPECTED_DOCS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745990i32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_EXTERNAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745988i32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_INDEX_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745989i32 as _);
pub const MSG_CI_MASTER_MERGE_REASON_NUMBER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745991i32 as _);
pub const MSG_CI_MASTER_MERGE_RESTARTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745945i32 as _);
pub const MSG_CI_MASTER_MERGE_STARTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1073745926i32 as _);
pub const MSG_TEST_MESSAGE: i32 = 1074008064i32;
pub const MSS_E_APPALREADYEXISTS: i32 = -2147213054i32;
pub const MSS_E_APPNOTFOUND: i32 = -2147213055i32;
pub const MSS_E_CATALOGALREADYEXISTS: i32 = -2147213050i32;
pub const MSS_E_CATALOGNOTFOUND: i32 = -2147213053i32;
pub const MSS_E_CATALOGSTOPPING: i32 = -2147213052i32;
pub const MSS_E_INVALIDAPPNAME: i32 = -2147213056i32;
pub const MSS_E_UNICODEFILEHEADERMISSING: i32 = -2147213051i32;
#[repr(transparent)]
pub struct NAMED_ENTITY_CERTAINTY(pub i32);
pub const NEC_LOW: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(0i32);
pub const NEC_MEDIUM: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(1i32);
pub const NEC_HIGH: NAMED_ENTITY_CERTAINTY = NAMED_ENTITY_CERTAINTY(2i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct NATLANGUAGERESTRICTION(i32);
pub const NET_E_DISCONNECTED: i32 = -2147220733i32;
pub const NET_E_GENERAL: i32 = -2147220736i32;
pub const NET_E_INVALIDPARAMS: i32 = -2147220728i32;
pub const NET_E_OPERATIONINPROGRESS: i32 = -2147220727i32;
pub const NLADMIN_E_BUILD_CATALOG_NOT_INITIALIZED: i32 = -2147215100i32;
pub const NLADMIN_E_DUPLICATE_CATALOG: i32 = -2147215103i32;
pub const NLADMIN_E_FAILED_TO_GIVE_ACCOUNT_PRIVILEGE: i32 = -2147215101i32;
pub const NLADMIN_S_NOT_ALL_BUILD_CATALOGS_INITIALIZED: i32 = 268546i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct NODERESTRICTION(i32);
pub const NOTESPH_E_ATTACHMENTS: i32 = -2147211770i32;
pub const NOTESPH_E_DB_ACCESS_DENIED: i32 = -2147211768i32;
pub const NOTESPH_E_FAIL: i32 = -2147211759i32;
pub const NOTESPH_E_ITEM_NOT_FOUND: i32 = -2147211772i32;
pub const NOTESPH_E_NOTESSETUP_ID_MAPPING_ERROR: i32 = -2147211767i32;
pub const NOTESPH_E_NO_NTID: i32 = -2147211769i32;
pub const NOTESPH_E_SERVER_CONFIG: i32 = -2147211771i32;
pub const NOTESPH_E_UNEXPECTED_STATE: i32 = -2147211775i32;
pub const NOTESPH_E_UNSUPPORTED_CONTENT_FIELD_TYPE: i32 = -2147211773i32;
pub const NOTESPH_S_IGNORE_ID: i32 = 271874i32;
pub const NOTESPH_S_LISTKNOWNFIELDS: i32 = 271888i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct NOTRESTRICTION(i32);
pub const NOT_N_PARSE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(526638i32 as _);
#[repr(C)]
pub struct NegationCondition(i32);
pub const OCC_INVALID: u32 = 4294967295u32;
pub const ODBCVER: u32 = 896u32;
pub const ODBC_ADD_DSN: u32 = 1u32;
pub const ODBC_ADD_SYS_DSN: u32 = 4u32;
pub const ODBC_BOTH_DSN: u32 = 0u32;
pub const ODBC_CONFIG_DRIVER: u32 = 3u32;
pub const ODBC_CONFIG_DRIVER_MAX: u32 = 100u32;
pub const ODBC_CONFIG_DSN: u32 = 2u32;
pub const ODBC_CONFIG_SYS_DSN: u32 = 5u32;
pub const ODBC_ERROR_COMPONENT_NOT_FOUND: u32 = 6u32;
pub const ODBC_ERROR_CREATE_DSN_FAILED: u32 = 18u32;
pub const ODBC_ERROR_GENERAL_ERR: u32 = 1u32;
pub const ODBC_ERROR_INVALID_BUFF_LEN: u32 = 2u32;
pub const ODBC_ERROR_INVALID_DSN: u32 = 9u32;
pub const ODBC_ERROR_INVALID_HWND: u32 = 3u32;
pub const ODBC_ERROR_INVALID_INF: u32 = 10u32;
pub const ODBC_ERROR_INVALID_KEYWORD_VALUE: u32 = 8u32;
pub const ODBC_ERROR_INVALID_LOG_FILE: u32 = 15u32;
pub const ODBC_ERROR_INVALID_NAME: u32 = 7u32;
pub const ODBC_ERROR_INVALID_PARAM_SEQUENCE: u32 = 14u32;
pub const ODBC_ERROR_INVALID_PATH: u32 = 12u32;
pub const ODBC_ERROR_INVALID_REQUEST_TYPE: u32 = 5u32;
pub const ODBC_ERROR_INVALID_STR: u32 = 4u32;
pub const ODBC_ERROR_LOAD_LIB_FAILED: u32 = 13u32;
pub const ODBC_ERROR_MAX: u32 = 23u32;
pub const ODBC_ERROR_NOTRANINFO: u32 = 23u32;
pub const ODBC_ERROR_OUTPUT_STRING_TRUNCATED: u32 = 22u32;
pub const ODBC_ERROR_OUT_OF_MEM: u32 = 21u32;
pub const ODBC_ERROR_REMOVE_DSN_FAILED: u32 = 20u32;
pub const ODBC_ERROR_REQUEST_FAILED: u32 = 11u32;
pub const ODBC_ERROR_USAGE_UPDATE_FAILED: u32 = 17u32;
pub const ODBC_ERROR_USER_CANCELED: u32 = 16u32;
pub const ODBC_ERROR_WRITING_SYSINFO_FAILED: u32 = 19u32;
pub const ODBC_INSTALL_COMPLETE: u32 = 2u32;
pub const ODBC_INSTALL_DRIVER: u32 = 1u32;
pub const ODBC_INSTALL_INQUIRY: u32 = 1u32;
pub const ODBC_REMOVE_DEFAULT_DSN: u32 = 7u32;
pub const ODBC_REMOVE_DRIVER: u32 = 2u32;
pub const ODBC_REMOVE_DSN: u32 = 3u32;
pub const ODBC_REMOVE_SYS_DSN: u32 = 6u32;
pub const ODBC_SYSTEM_DSN: u32 = 2u32;
pub const ODBC_USER_DSN: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ODBC_VS_ARGS(i32);
pub const ODBC_VS_FLAG_RETCODE: i32 = 4i32;
pub const ODBC_VS_FLAG_STOP: i32 = 8i32;
pub const ODBC_VS_FLAG_UNICODE_ARG: i32 = 1i32;
pub const ODBC_VS_FLAG_UNICODE_COR: i32 = 2i32;
#[repr(transparent)]
pub struct OLEDBSimpleProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OLEDBSimpleProviderListener(pub *mut ::core::ffi::c_void);
pub const OLEDBVER: u32 = 624u32;
pub const OLEDB_BINDER_CUSTOM_ERROR: i32 = -2147212032i32;
#[repr(transparent)]
pub struct OSPCOMP(pub i32);
pub const OSPCOMP_EQ: OSPCOMP = OSPCOMP(1i32);
pub const OSPCOMP_DEFAULT: OSPCOMP = OSPCOMP(1i32);
pub const OSPCOMP_LT: OSPCOMP = OSPCOMP(2i32);
pub const OSPCOMP_LE: OSPCOMP = OSPCOMP(3i32);
pub const OSPCOMP_GE: OSPCOMP = OSPCOMP(4i32);
pub const OSPCOMP_GT: OSPCOMP = OSPCOMP(5i32);
pub const OSPCOMP_NE: OSPCOMP = OSPCOMP(6i32);
#[repr(transparent)]
pub struct OSPFIND(pub i32);
pub const OSPFIND_DEFAULT: OSPFIND = OSPFIND(0i32);
pub const OSPFIND_UP: OSPFIND = OSPFIND(1i32);
pub const OSPFIND_CASESENSITIVE: OSPFIND = OSPFIND(2i32);
pub const OSPFIND_UPCASESENSITIVE: OSPFIND = OSPFIND(3i32);
#[repr(transparent)]
pub struct OSPFORMAT(pub i32);
pub const OSPFORMAT_RAW: OSPFORMAT = OSPFORMAT(0i32);
pub const OSPFORMAT_DEFAULT: OSPFORMAT = OSPFORMAT(0i32);
pub const OSPFORMAT_FORMATTED: OSPFORMAT = OSPFORMAT(1i32);
pub const OSPFORMAT_HTML: OSPFORMAT = OSPFORMAT(2i32);
#[repr(transparent)]
pub struct OSPRW(pub i32);
pub const OSPRW_DEFAULT: OSPRW = OSPRW(1i32);
pub const OSPRW_READONLY: OSPRW = OSPRW(0i32);
pub const OSPRW_READWRITE: OSPRW = OSPRW(1i32);
pub const OSPRW_MIXED: OSPRW = OSPRW(2i32);
#[repr(transparent)]
pub struct OSPXFER(pub i32);
pub const OSPXFER_COMPLETE: OSPXFER = OSPXFER(0i32);
pub const OSPXFER_ABORT: OSPXFER = OSPXFER(1i32);
pub const OSPXFER_ERROR: OSPXFER = OSPXFER(2i32);
pub const OSP_IndexLabel: u32 = 0u32;
#[repr(C)]
pub struct PDPO(i32);
pub const PEOPLE_IMPORT_E_CANONICALURL_TOOLONG: i32 = -2147205110i32;
pub const PEOPLE_IMPORT_E_DATATYPENOTSUPPORTED: i32 = -2147205115i32;
pub const PEOPLE_IMPORT_E_DBCONNFAIL: i32 = -2147205120i32;
pub const PEOPLE_IMPORT_E_DC_NOT_AVAILABLE: i32 = -2147205108i32;
pub const PEOPLE_IMPORT_E_DIRSYNC_NOTREFRESHED: i32 = -2147205103i32;
pub const PEOPLE_IMPORT_E_DIRSYNC_ZERO_COOKIE: i32 = -2147205112i32;
pub const PEOPLE_IMPORT_E_DOMAIN_DISCOVER_FAILED: i32 = -2147205107i32;
pub const PEOPLE_IMPORT_E_DOMAIN_REMOVED: i32 = -2147205105i32;
pub const PEOPLE_IMPORT_E_ENUM_ACCESSDENIED: i32 = -2147205104i32;
pub const PEOPLE_IMPORT_E_FAILTOGETDSDEF: i32 = -2147205118i32;
pub const PEOPLE_IMPORT_E_FAILTOGETDSMAPPING: i32 = -2147205116i32;
pub const PEOPLE_IMPORT_E_FAILTOGETLCID: i32 = -2147205106i32;
pub const PEOPLE_IMPORT_E_LDAPPATH_TOOLONG: i32 = -2147205111i32;
pub const PEOPLE_IMPORT_E_NOCASTINGSUPPORTED: i32 = -2147205114i32;
pub const PEOPLE_IMPORT_E_UPDATE_DIRSYNC_COOKIE: i32 = -2147205113i32;
pub const PEOPLE_IMPORT_E_USERNAME_NOTRESOLVED: i32 = -2147205109i32;
pub const PEOPLE_IMPORT_NODSDEFINED: i32 = -2147205119i32;
pub const PEOPLE_IMPORT_NOMAPPINGDEFINED: i32 = -2147205117i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNFILLTEXTBUFFER = unsafe extern "system" fn(ptextsource: *mut TEXT_SOURCE) -> ::windows_sys::core::HRESULT;
pub const PRAll: u32 = 256u32;
pub const PRAllBits: u32 = 7u32;
pub const PRAny: u32 = 512u32;
#[repr(transparent)]
pub struct PRIORITIZE_FLAGS(pub i32);
pub const PRIORITIZE_FLAG_RETRYFAILEDITEMS: PRIORITIZE_FLAGS = PRIORITIZE_FLAGS(1i32);
pub const PRIORITIZE_FLAG_IGNOREFAILURECOUNT: PRIORITIZE_FLAGS = PRIORITIZE_FLAGS(2i32);
#[repr(transparent)]
pub struct PRIORITY_LEVEL(pub i32);
pub const PRIORITY_LEVEL_FOREGROUND: PRIORITY_LEVEL = PRIORITY_LEVEL(0i32);
pub const PRIORITY_LEVEL_HIGH: PRIORITY_LEVEL = PRIORITY_LEVEL(1i32);
pub const PRIORITY_LEVEL_LOW: PRIORITY_LEVEL = PRIORITY_LEVEL(2i32);
pub const PRIORITY_LEVEL_DEFAULT: PRIORITY_LEVEL = PRIORITY_LEVEL(3i32);
pub const PROGID_MSPersist_Version_W: &'static str = "MSPersist.1";
pub const PROGID_MSPersist_W: &'static str = "MSPersist";
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct PROPERTYRESTRICTION(i32);
pub const PROPID_DBBMK_BOOKMARK: u32 = 2u32;
pub const PROPID_DBBMK_CHAPTER: u32 = 3u32;
pub const PROPID_DBSELF_SELF: u32 = 2u32;
#[repr(transparent)]
pub struct PROXY_ACCESS(pub i32);
pub const PROXY_ACCESS_PRECONFIG: PROXY_ACCESS = PROXY_ACCESS(0i32);
pub const PROXY_ACCESS_DIRECT: PROXY_ACCESS = PROXY_ACCESS(1i32);
pub const PROXY_ACCESS_PROXY: PROXY_ACCESS = PROXY_ACCESS(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROXY_INFO(i32);
pub const PRRE: u32 = 6u32;
pub const PRSomeBits: u32 = 8u32;
pub const PRTH_E_CANT_TRANSFORM_DENIED_ACE: i32 = -2147216881i32;
pub const PRTH_E_CANT_TRANSFORM_EXTERNAL_ACL: i32 = -2147216882i32;
pub const PRTH_E_DATABASE_OPEN_ERROR: i32 = -2147216875i32;
pub const PRTH_E_HTTPS_CERTIFICATE_ERROR: i32 = -2147216861i32;
pub const PRTH_E_HTTPS_REQUIRE_CERTIFICATE: i32 = -2147216860i32;
pub const PRTH_E_INIT_FAILED: i32 = -2147216872i32;
pub const PRTH_E_INTERNAL_ERROR: i32 = -2147216892i32;
pub const PRTH_E_LOAD_FAILED: i32 = -2147216873i32;
pub const PRTH_E_MIME_EXCLUDED: i32 = -2147216883i32;
pub const PRTH_E_NO_PROPERTY: i32 = -2147216877i32;
pub const PRTH_E_OPLOCK_BROKEN: i32 = -2147216874i32;
pub const PRTH_E_RETRY: i32 = -2147216885i32;
pub const PRTH_E_TRUNCATED: i32 = -2147216870i32;
pub const PRTH_E_VOLUME_MOUNT_POINT: i32 = -2147216871i32;
pub const PRTH_E_WININET: i32 = -2147216886i32;
pub const PRTH_S_MAX_DOWNLOAD: i32 = 266764i32;
pub const PRTH_S_MAX_GROWTH: i32 = 266761i32;
pub const PRTH_S_TRY_IMPERSONATING: i32 = 266789i32;
pub const PRTH_S_USE_ROSEBUD: i32 = 266772i32;
pub const PWPROP_OSPVALUE: u32 = 2u32;
pub const QRY_E_COLUMNNOTSEARCHABLE: i32 = -2147219700i32;
pub const QRY_E_COLUMNNOTSORTABLE: i32 = -2147219701i32;
pub const QRY_E_ENGINEFAILED: i32 = -2147219693i32;
pub const QRY_E_INFIXWILDCARD: i32 = -2147219696i32;
pub const QRY_E_INVALIDCATALOG: i32 = -2147219687i32;
pub const QRY_E_INVALIDCOLUMN: i32 = -2147219699i32;
pub const QRY_E_INVALIDINTERVAL: i32 = -2147219682i32;
pub const QRY_E_INVALIDPATH: i32 = -2147219684i32;
pub const QRY_E_INVALIDSCOPES: i32 = -2147219688i32;
pub const QRY_E_LMNOTINITIALIZED: i32 = -2147219683i32;
pub const QRY_E_NOCOLUMNS: i32 = -2147219689i32;
pub const QRY_E_NODATASOURCES: i32 = -2147219703i32;
pub const QRY_E_NOLOGMANAGER: i32 = -2147219681i32;
pub const QRY_E_NULLQUERY: i32 = -2147219691i32;
pub const QRY_E_PREFIXWILDCARD: i32 = -2147219697i32;
pub const QRY_E_QUERYCORRUPT: i32 = -2147219698i32;
pub const QRY_E_QUERYSYNTAX: i32 = -2147219711i32;
pub const QRY_E_SCOPECARDINALIDY: i32 = -2147219686i32;
pub const QRY_E_SEARCHTOOBIG: i32 = -2147219692i32;
pub const QRY_E_STARTHITTOBIG: i32 = -2147219705i32;
pub const QRY_E_TIMEOUT: i32 = -2147219702i32;
pub const QRY_E_TOOMANYCOLUMNS: i32 = -2147219707i32;
pub const QRY_E_TOOMANYDATABASES: i32 = -2147219706i32;
pub const QRY_E_TOOMANYQUERYTERMS: i32 = -2147219704i32;
pub const QRY_E_TYPEMISMATCH: i32 = -2147219710i32;
pub const QRY_E_UNEXPECTED: i32 = -2147219685i32;
pub const QRY_E_UNHANDLEDTYPE: i32 = -2147219709i32;
pub const QRY_E_WILDCARDPREFIXLENGTH: i32 = -2147219695i32;
pub const QRY_S_INEXACTRESULTS: i32 = 263958i32;
pub const QRY_S_NOROWSFOUND: i32 = 263940i32;
pub const QRY_S_TERMIGNORED: i32 = 263954i32;
pub const QUERY_E_AGGREGATE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215847i32 as _);
pub const QUERY_E_ALLNOISE_AND_NO_RELDOC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215859i32 as _);
pub const QUERY_E_ALLNOISE_AND_NO_RELPROP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215857i32 as _);
pub const QUERY_E_DUPLICATE_RANGE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215845i32 as _);
pub const QUERY_E_INCORRECT_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215852i32 as _);
pub const QUERY_E_INVALIDCOALESCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215849i32 as _);
pub const QUERY_E_INVALIDSCOPE_COALESCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215851i32 as _);
pub const QUERY_E_INVALIDSORT_COALESCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215850i32 as _);
pub const QUERY_E_INVALID_DOCUMENT_IDENTIFIER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215853i32 as _);
pub const QUERY_E_NO_RELDOC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215858i32 as _);
pub const QUERY_E_NO_RELPROP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215856i32 as _);
pub const QUERY_E_RELDOC_SYNTAX_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215854i32 as _);
pub const QUERY_E_REPEATED_RELDOC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215855i32 as _);
pub const QUERY_E_TOP_LEVEL_IN_GROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215846i32 as _);
pub const QUERY_E_UPGRADEINPROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215848i32 as _);
#[repr(transparent)]
pub struct QUERY_PARSER_MANAGER_OPTION(pub i32);
pub const QPMO_SCHEMA_BINARY_NAME: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(0i32);
pub const QPMO_PRELOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(1i32);
pub const QPMO_UNLOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(2i32);
pub const QPMO_LOCALIZED_SCHEMA_BINARY_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(3i32);
pub const QPMO_APPEND_LCID_TO_LOCALIZED_PATH: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(4i32);
pub const QPMO_LOCALIZER_SUPPORT: QUERY_PARSER_MANAGER_OPTION = QUERY_PARSER_MANAGER_OPTION(5i32);
pub const QUERY_SORTDEFAULT: u32 = 4u32;
pub const QUERY_SORTXASCEND: u32 = 2u32;
pub const QUERY_SORTXDESCEND: u32 = 3u32;
pub const QUERY_VALIDBITS: u32 = 3u32;
#[repr(C)]
pub struct QueryParser(i32);
#[repr(C)]
pub struct QueryParserManager(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct RANGECATEGORIZE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct RESTRICTION(i32);
pub const REXSPH_E_DUPLICATE_PROPERTY: i32 = -2147207927i32;
pub const REXSPH_E_INVALID_CALL: i32 = -2147207936i32;
pub const REXSPH_E_MULTIPLE_REDIRECT: i32 = -2147207933i32;
pub const REXSPH_E_NO_PROPERTY_ON_ROW: i32 = -2147207932i32;
pub const REXSPH_E_REDIRECT_ON_SECURITY_UPDATE: i32 = -2147207934i32;
pub const REXSPH_E_TYPE_MISMATCH_ON_READ: i32 = -2147207931i32;
pub const REXSPH_E_UNEXPECTED_DATA_STATUS: i32 = -2147207930i32;
pub const REXSPH_E_UNEXPECTED_FILTER_STATE: i32 = -2147207928i32;
pub const REXSPH_E_UNKNOWN_DATA_TYPE: i32 = -2147207929i32;
pub const REXSPH_S_REDIRECTED: i32 = 275713i32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct RMTPACK(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct RMTPACK(i32);
#[repr(transparent)]
pub struct ROWSETEVENT_ITEMSTATE(pub i32);
pub const ROWSETEVENT_ITEMSTATE_NOTINROWSET: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(0i32);
pub const ROWSETEVENT_ITEMSTATE_INROWSET: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(1i32);
pub const ROWSETEVENT_ITEMSTATE_UNKNOWN: ROWSETEVENT_ITEMSTATE = ROWSETEVENT_ITEMSTATE(2i32);
#[repr(transparent)]
pub struct ROWSETEVENT_TYPE(pub i32);
pub const ROWSETEVENT_TYPE_DATAEXPIRED: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(0i32);
pub const ROWSETEVENT_TYPE_FOREGROUNDLOST: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(1i32);
pub const ROWSETEVENT_TYPE_SCOPESTATISTICS: ROWSETEVENT_TYPE = ROWSETEVENT_TYPE(2i32);
pub const RS_COMPLETED: u32 = 2147483648u32;
pub const RS_MAYBOTHERUSER: u32 = 131072u32;
pub const RS_READY: u32 = 1u32;
pub const RS_SUSPENDED: u32 = 2u32;
pub const RS_SUSPENDONIDLE: u32 = 65536u32;
pub const RS_UPDATING: u32 = 4u32;
pub const RTAnd: u32 = 1u32;
pub const RTContent: u32 = 4u32;
pub const RTNatLanguage: u32 = 8u32;
pub const RTNone: u32 = 0u32;
pub const RTNot: u32 = 3u32;
pub const RTOr: u32 = 2u32;
pub const RTProperty: u32 = 5u32;
pub const RTProximity: u32 = 6u32;
pub const RTVector: u32 = 7u32;
#[repr(C)]
pub struct RootBinder(i32);
pub const SCHEMA_E_ADDSTOPWORDS: i32 = -2147218420i32;
pub const SCHEMA_E_BADATTRIBUTE: i32 = -2147218412i32;
pub const SCHEMA_E_BADCOLUMNNAME: i32 = -2147218414i32;
pub const SCHEMA_E_BADFILENAME: i32 = -2147218411i32;
pub const SCHEMA_E_BADPROPPID: i32 = -2147218413i32;
pub const SCHEMA_E_BADPROPSPEC: i32 = -2147218417i32;
pub const SCHEMA_E_CANNOTCREATEFILE: i32 = -2147218426i32;
pub const SCHEMA_E_CANNOTCREATENOISEWORDFILE: i32 = -2147218421i32;
pub const SCHEMA_E_CANNOTWRITEFILE: i32 = -2147218425i32;
pub const SCHEMA_E_DUPLICATENOISE: i32 = -2147218409i32;
pub const SCHEMA_E_EMPTYFILE: i32 = -2147218424i32;
pub const SCHEMA_E_FILECHANGED: i32 = -2147218415i32;
pub const SCHEMA_E_FILENOTFOUND: i32 = -2147218430i32;
pub const SCHEMA_E_INVALIDDATATYPE: i32 = -2147218422i32;
pub const SCHEMA_E_INVALIDFILETYPE: i32 = -2147218423i32;
pub const SCHEMA_E_INVALIDVALUE: i32 = -2147218418i32;
pub const SCHEMA_E_LOAD_SPECIAL: i32 = -2147218431i32;
pub const SCHEMA_E_NAMEEXISTS: i32 = -2147218419i32;
pub const SCHEMA_E_NESTEDTAG: i32 = -2147218429i32;
pub const SCHEMA_E_NOMORECOLUMNS: i32 = -2147218416i32;
pub const SCHEMA_E_PROPEXISTS: i32 = -2147218410i32;
pub const SCHEMA_E_UNEXPECTEDTAG: i32 = -2147218428i32;
pub const SCHEMA_E_VERSIONMISMATCH: i32 = -2147218427i32;
pub const SCRIPTPI_E_ALREADY_COMPLETED: i32 = -2147213307i32;
pub const SCRIPTPI_E_CANNOT_ALTER_CHUNK: i32 = -2147213308i32;
pub const SCRIPTPI_E_CHUNK_NOT_TEXT: i32 = -2147213312i32;
pub const SCRIPTPI_E_CHUNK_NOT_VALUE: i32 = -2147213309i32;
pub const SCRIPTPI_E_PID_NOT_NAME: i32 = -2147213311i32;
pub const SCRIPTPI_E_PID_NOT_NUMERIC: i32 = -2147213310i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct SEARCH_COLUMN_PROPERTIES(i32);
#[repr(transparent)]
pub struct SEARCH_INDEXING_PHASE(pub i32);
pub const SEARCH_INDEXING_PHASE_GATHERER: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(0i32);
pub const SEARCH_INDEXING_PHASE_QUERYABLE: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(1i32);
pub const SEARCH_INDEXING_PHASE_PERSISTED: SEARCH_INDEXING_PHASE = SEARCH_INDEXING_PHASE(2i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SEARCH_ITEM_CHANGE(i32);
#[repr(C)]
pub struct SEARCH_ITEM_INDEXING_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SEARCH_ITEM_PERSISTENT_CHANGE(i32);
#[repr(transparent)]
pub struct SEARCH_KIND_OF_CHANGE(pub i32);
pub const SEARCH_CHANGE_ADD: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(0i32);
pub const SEARCH_CHANGE_DELETE: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(1i32);
pub const SEARCH_CHANGE_MODIFY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(2i32);
pub const SEARCH_CHANGE_MOVE_RENAME: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(3i32);
pub const SEARCH_CHANGE_SEMANTICS_DIRECTORY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(262144i32);
pub const SEARCH_CHANGE_SEMANTICS_SHALLOW: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(524288i32);
pub const SEARCH_CHANGE_SEMANTICS_UPDATE_SECURITY: SEARCH_KIND_OF_CHANGE = SEARCH_KIND_OF_CHANGE(4194304i32);
#[repr(transparent)]
pub struct SEARCH_NOTIFICATION_PRIORITY(pub i32);
pub const SEARCH_NORMAL_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = SEARCH_NOTIFICATION_PRIORITY(0i32);
pub const SEARCH_HIGH_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = SEARCH_NOTIFICATION_PRIORITY(1i32);
#[repr(transparent)]
pub struct SEARCH_QUERY_SYNTAX(pub i32);
pub const SEARCH_NO_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(0i32);
pub const SEARCH_ADVANCED_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(1i32);
pub const SEARCH_NATURAL_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = SEARCH_QUERY_SYNTAX(2i32);
#[repr(transparent)]
pub struct SEARCH_TERM_EXPANSION(pub i32);
pub const SEARCH_TERM_NO_EXPANSION: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(0i32);
pub const SEARCH_TERM_PREFIX_ALL: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(1i32);
pub const SEARCH_TERM_STEM_ALL: SEARCH_TERM_EXPANSION = SEARCH_TERM_EXPANSION(2i32);
pub const SEC_E_ACCESSDENIED: i32 = -2147216129i32;
pub const SEC_E_BADTRUSTEEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217814i32 as _);
pub const SEC_E_INITFAILED: i32 = -2147216383i32;
pub const SEC_E_INVALIDACCESSENTRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217807i32 as _);
pub const SEC_E_INVALIDACCESSENTRYLIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217809i32 as _);
pub const SEC_E_INVALIDCONTEXT: i32 = -2147216381i32;
pub const SEC_E_INVALIDOBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217811i32 as _);
pub const SEC_E_INVALIDOWNER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217808i32 as _);
pub const SEC_E_NOMEMBERSHIPSUPPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217812i32 as _);
pub const SEC_E_NOOWNER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217810i32 as _);
pub const SEC_E_NOTINITIALIZED: i32 = -2147216382i32;
pub const SEC_E_NOTRUSTEEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217813i32 as _);
pub const SEC_E_PERMISSIONDENIED: i32 = -2147217911i32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct SEC_OBJECT(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct SEC_OBJECT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct SEC_OBJECT_ELEMENT(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
#[repr(C)]
pub struct SEC_OBJECT_ELEMENT(i32);
pub const SI_TEMPORARY: u32 = 2147483648u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct SORTKEY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct SORTSET(i32);
pub const SPS_WS_ERROR: i32 = -2147211753i32;
pub const SQLAOPANY: u32 = 83u32;
pub const SQLAOPAVG: u32 = 79u32;
pub const SQLAOPCNT: u32 = 75u32;
pub const SQLAOPMAX: u32 = 82u32;
pub const SQLAOPMIN: u32 = 81u32;
pub const SQLAOPNOOP: u32 = 86u32;
pub const SQLAOPSTDEV: u32 = 48u32;
pub const SQLAOPSTDEVP: u32 = 49u32;
pub const SQLAOPSUM: u32 = 77u32;
pub const SQLAOPVAR: u32 = 50u32;
pub const SQLAOPVARP: u32 = 51u32;
pub const SQLBIGBINARY: u32 = 173u32;
pub const SQLBIGCHAR: u32 = 175u32;
pub const SQLBIGVARBINARY: u32 = 165u32;
pub const SQLBIGVARCHAR: u32 = 167u32;
pub const SQLBINARY: u32 = 45u32;
pub const SQLBIT: u32 = 50u32;
pub const SQLBITN: u32 = 104u32;
pub const SQLCHARACTER: u32 = 47u32;
pub const SQLDATETIM4: u32 = 58u32;
pub const SQLDATETIME: u32 = 61u32;
pub const SQLDATETIMN: u32 = 111u32;
pub const SQLDECIMAL: u32 = 106u32;
pub const SQLDECIMALN: u32 = 106u32;
pub const SQLFLT4: u32 = 59u32;
pub const SQLFLT8: u32 = 62u32;
pub const SQLFLTN: u32 = 109u32;
pub const SQLIMAGE: u32 = 34u32;
pub const SQLINT1: u32 = 48u32;
pub const SQLINT2: u32 = 52u32;
pub const SQLINT4: u32 = 56u32;
pub const SQLINT8: u32 = 127u32;
#[repr(transparent)]
pub struct SQLINTERVAL(pub i32);
pub const SQL_IS_YEAR: SQLINTERVAL = SQLINTERVAL(1i32);
pub const SQL_IS_MONTH: SQLINTERVAL = SQLINTERVAL(2i32);
pub const SQL_IS_DAY: SQLINTERVAL = SQLINTERVAL(3i32);
pub const SQL_IS_HOUR: SQLINTERVAL = SQLINTERVAL(4i32);
pub const SQL_IS_MINUTE: SQLINTERVAL = SQLINTERVAL(5i32);
pub const SQL_IS_SECOND: SQLINTERVAL = SQLINTERVAL(6i32);
pub const SQL_IS_YEAR_TO_MONTH: SQLINTERVAL = SQLINTERVAL(7i32);
pub const SQL_IS_DAY_TO_HOUR: SQLINTERVAL = SQLINTERVAL(8i32);
pub const SQL_IS_DAY_TO_MINUTE: SQLINTERVAL = SQLINTERVAL(9i32);
pub const SQL_IS_DAY_TO_SECOND: SQLINTERVAL = SQLINTERVAL(10i32);
pub const SQL_IS_HOUR_TO_MINUTE: SQLINTERVAL = SQLINTERVAL(11i32);
pub const SQL_IS_HOUR_TO_SECOND: SQLINTERVAL = SQLINTERVAL(12i32);
pub const SQL_IS_MINUTE_TO_SECOND: SQLINTERVAL = SQLINTERVAL(13i32);
pub const SQLINTN: u32 = 38u32;
pub const SQLMONEY: u32 = 60u32;
pub const SQLMONEY4: u32 = 122u32;
pub const SQLMONEYN: u32 = 110u32;
pub const SQLNCHAR: u32 = 239u32;
pub const SQLNTEXT: u32 = 99u32;
pub const SQLNUMERIC: u32 = 108u32;
pub const SQLNUMERICN: u32 = 108u32;
pub const SQLNVARCHAR: u32 = 231u32;
pub const SQLTEXT: u32 = 35u32;
pub const SQLUNIQUEID: u32 = 36u32;
pub const SQLVARBINARY: u32 = 37u32;
pub const SQLVARCHAR: u32 = 39u32;
#[repr(transparent)]
pub struct SQLVARENUM(pub i32);
pub const VT_SS_EMPTY: SQLVARENUM = SQLVARENUM(0i32);
pub const VT_SS_NULL: SQLVARENUM = SQLVARENUM(1i32);
pub const VT_SS_UI1: SQLVARENUM = SQLVARENUM(17i32);
pub const VT_SS_I2: SQLVARENUM = SQLVARENUM(2i32);
pub const VT_SS_I4: SQLVARENUM = SQLVARENUM(3i32);
pub const VT_SS_I8: SQLVARENUM = SQLVARENUM(20i32);
pub const VT_SS_R4: SQLVARENUM = SQLVARENUM(4i32);
pub const VT_SS_R8: SQLVARENUM = SQLVARENUM(5i32);
pub const VT_SS_MONEY: SQLVARENUM = SQLVARENUM(6i32);
pub const VT_SS_SMALLMONEY: SQLVARENUM = SQLVARENUM(200i32);
pub const VT_SS_WSTRING: SQLVARENUM = SQLVARENUM(201i32);
pub const VT_SS_WVARSTRING: SQLVARENUM = SQLVARENUM(202i32);
pub const VT_SS_STRING: SQLVARENUM = SQLVARENUM(203i32);
pub const VT_SS_VARSTRING: SQLVARENUM = SQLVARENUM(204i32);
pub const VT_SS_BIT: SQLVARENUM = SQLVARENUM(11i32);
pub const VT_SS_GUID: SQLVARENUM = SQLVARENUM(72i32);
pub const VT_SS_NUMERIC: SQLVARENUM = SQLVARENUM(131i32);
pub const VT_SS_DECIMAL: SQLVARENUM = SQLVARENUM(205i32);
pub const VT_SS_DATETIME: SQLVARENUM = SQLVARENUM(135i32);
pub const VT_SS_SMALLDATETIME: SQLVARENUM = SQLVARENUM(206i32);
pub const VT_SS_BINARY: SQLVARENUM = SQLVARENUM(207i32);
pub const VT_SS_VARBINARY: SQLVARENUM = SQLVARENUM(208i32);
pub const VT_SS_UNKNOWN: SQLVARENUM = SQLVARENUM(209i32);
pub const SQLVARIANT: u32 = 98u32;
pub const SQL_AA_FALSE: i32 = 0i32;
pub const SQL_AA_TRUE: i32 = 1i32;
pub const SQL_ACCESSIBLE_PROCEDURES: u32 = 20u32;
pub const SQL_ACCESSIBLE_TABLES: u32 = 19u32;
pub const SQL_ACCESS_MODE: u32 = 101u32;
pub const SQL_ACTIVE_CONNECTIONS: u32 = 0u32;
pub const SQL_ACTIVE_ENVIRONMENTS: u32 = 116u32;
pub const SQL_ACTIVE_STATEMENTS: u32 = 1u32;
pub const SQL_ADD: u32 = 4u32;
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: i32 = 2i32;
pub const SQL_AD_ADD_DOMAIN_DEFAULT: i32 = 8i32;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: i32 = 1i32;
pub const SQL_AD_DEFAULT: i32 = 1i32;
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: i32 = 4i32;
pub const SQL_AD_DROP_DOMAIN_DEFAULT: i32 = 16i32;
pub const SQL_AD_OFF: i32 = 0i32;
pub const SQL_AD_ON: i32 = 1i32;
pub const SQL_AF_ALL: i32 = 64i32;
pub const SQL_AF_AVG: i32 = 1i32;
pub const SQL_AF_COUNT: i32 = 2i32;
pub const SQL_AF_DISTINCT: i32 = 32i32;
pub const SQL_AF_MAX: i32 = 4i32;
pub const SQL_AF_MIN: i32 = 8i32;
pub const SQL_AF_SUM: i32 = 16i32;
pub const SQL_AGGREGATE_FUNCTIONS: u32 = 169u32;
pub const SQL_ALL_EXCEPT_LIKE: u32 = 2u32;
pub const SQL_ALL_TYPES: u32 = 0u32;
pub const SQL_ALTER_DOMAIN: u32 = 117u32;
pub const SQL_ALTER_TABLE: u32 = 86u32;
pub const SQL_AM_CONNECTION: u32 = 1u32;
pub const SQL_AM_NONE: u32 = 0u32;
pub const SQL_AM_STATEMENT: u32 = 2u32;
pub const SQL_AO_DEFAULT: i32 = 0i32;
pub const SQL_AO_OFF: i32 = 0i32;
pub const SQL_AO_ON: i32 = 1i32;
pub const SQL_APD_TYPE: i32 = -100i32;
pub const SQL_API_ALL_FUNCTIONS: u32 = 0u32;
pub const SQL_API_LOADBYORDINAL: u32 = 199u32;
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u32 = 999u32;
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: u32 = 250u32;
pub const SQL_API_SQLALLOCCONNECT: u32 = 1u32;
pub const SQL_API_SQLALLOCENV: u32 = 2u32;
pub const SQL_API_SQLALLOCHANDLE: u32 = 1001u32;
pub const SQL_API_SQLALLOCHANDLESTD: u32 = 73u32;
pub const SQL_API_SQLALLOCSTMT: u32 = 3u32;
pub const SQL_API_SQLBINDCOL: u32 = 4u32;
pub const SQL_API_SQLBINDPARAM: u32 = 1002u32;
pub const SQL_API_SQLBINDPARAMETER: u32 = 72u32;
pub const SQL_API_SQLBROWSECONNECT: u32 = 55u32;
pub const SQL_API_SQLBULKOPERATIONS: u32 = 24u32;
pub const SQL_API_SQLCANCEL: u32 = 5u32;
pub const SQL_API_SQLCANCELHANDLE: u32 = 1550u32;
pub const SQL_API_SQLCLOSECURSOR: u32 = 1003u32;
pub const SQL_API_SQLCOLATTRIBUTE: u32 = 6u32;
pub const SQL_API_SQLCOLATTRIBUTES: u32 = 6u32;
pub const SQL_API_SQLCOLUMNPRIVILEGES: u32 = 56u32;
pub const SQL_API_SQLCOLUMNS: u32 = 40u32;
pub const SQL_API_SQLCOMPLETEASYNC: u32 = 1551u32;
pub const SQL_API_SQLCONNECT: u32 = 7u32;
pub const SQL_API_SQLCOPYDESC: u32 = 1004u32;
pub const SQL_API_SQLDATASOURCES: u32 = 57u32;
pub const SQL_API_SQLDESCRIBECOL: u32 = 8u32;
pub const SQL_API_SQLDESCRIBEPARAM: u32 = 58u32;
pub const SQL_API_SQLDISCONNECT: u32 = 9u32;
pub const SQL_API_SQLDRIVERCONNECT: u32 = 41u32;
pub const SQL_API_SQLDRIVERS: u32 = 71u32;
pub const SQL_API_SQLENDTRAN: u32 = 1005u32;
pub const SQL_API_SQLERROR: u32 = 10u32;
pub const SQL_API_SQLEXECDIRECT: u32 = 11u32;
pub const SQL_API_SQLEXECUTE: u32 = 12u32;
pub const SQL_API_SQLEXTENDEDFETCH: u32 = 59u32;
pub const SQL_API_SQLFETCH: u32 = 13u32;
pub const SQL_API_SQLFETCHSCROLL: u32 = 1021u32;
pub const SQL_API_SQLFOREIGNKEYS: u32 = 60u32;
pub const SQL_API_SQLFREECONNECT: u32 = 14u32;
pub const SQL_API_SQLFREEENV: u32 = 15u32;
pub const SQL_API_SQLFREEHANDLE: u32 = 1006u32;
pub const SQL_API_SQLFREESTMT: u32 = 16u32;
pub const SQL_API_SQLGETCONNECTATTR: u32 = 1007u32;
pub const SQL_API_SQLGETCONNECTOPTION: u32 = 42u32;
pub const SQL_API_SQLGETCURSORNAME: u32 = 17u32;
pub const SQL_API_SQLGETDATA: u32 = 43u32;
pub const SQL_API_SQLGETDESCFIELD: u32 = 1008u32;
pub const SQL_API_SQLGETDESCREC: u32 = 1009u32;
pub const SQL_API_SQLGETDIAGFIELD: u32 = 1010u32;
pub const SQL_API_SQLGETDIAGREC: u32 = 1011u32;
pub const SQL_API_SQLGETENVATTR: u32 = 1012u32;
pub const SQL_API_SQLGETFUNCTIONS: u32 = 44u32;
pub const SQL_API_SQLGETINFO: u32 = 45u32;
pub const SQL_API_SQLGETSTMTATTR: u32 = 1014u32;
pub const SQL_API_SQLGETSTMTOPTION: u32 = 46u32;
pub const SQL_API_SQLGETTYPEINFO: u32 = 47u32;
pub const SQL_API_SQLMORERESULTS: u32 = 61u32;
pub const SQL_API_SQLNATIVESQL: u32 = 62u32;
pub const SQL_API_SQLNUMPARAMS: u32 = 63u32;
pub const SQL_API_SQLNUMRESULTCOLS: u32 = 18u32;
pub const SQL_API_SQLPARAMDATA: u32 = 48u32;
pub const SQL_API_SQLPARAMOPTIONS: u32 = 64u32;
pub const SQL_API_SQLPREPARE: u32 = 19u32;
pub const SQL_API_SQLPRIMARYKEYS: u32 = 65u32;
pub const SQL_API_SQLPRIVATEDRIVERS: u32 = 79u32;
pub const SQL_API_SQLPROCEDURECOLUMNS: u32 = 66u32;
pub const SQL_API_SQLPROCEDURES: u32 = 67u32;
pub const SQL_API_SQLPUTDATA: u32 = 49u32;
pub const SQL_API_SQLROWCOUNT: u32 = 20u32;
pub const SQL_API_SQLSETCONNECTATTR: u32 = 1016u32;
pub const SQL_API_SQLSETCONNECTOPTION: u32 = 50u32;
pub const SQL_API_SQLSETCURSORNAME: u32 = 21u32;
pub const SQL_API_SQLSETDESCFIELD: u32 = 1017u32;
pub const SQL_API_SQLSETDESCREC: u32 = 1018u32;
pub const SQL_API_SQLSETENVATTR: u32 = 1019u32;
pub const SQL_API_SQLSETPARAM: u32 = 22u32;
pub const SQL_API_SQLSETPOS: u32 = 68u32;
pub const SQL_API_SQLSETSCROLLOPTIONS: u32 = 69u32;
pub const SQL_API_SQLSETSTMTATTR: u32 = 1020u32;
pub const SQL_API_SQLSETSTMTOPTION: u32 = 51u32;
pub const SQL_API_SQLSPECIALCOLUMNS: u32 = 52u32;
pub const SQL_API_SQLSTATISTICS: u32 = 53u32;
pub const SQL_API_SQLTABLEPRIVILEGES: u32 = 70u32;
pub const SQL_API_SQLTABLES: u32 = 54u32;
pub const SQL_API_SQLTRANSACT: u32 = 23u32;
pub const SQL_ARD_TYPE: i32 = -99i32;
pub const SQL_ASYNC_DBC_CAPABLE: i32 = 1i32;
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0u32;
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0u32;
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1u32;
pub const SQL_ASYNC_DBC_FUNCTIONS: u32 = 10023u32;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: i32 = 0i32;
pub const SQL_ASYNC_ENABLE: u32 = 4u32;
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0u32;
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0u32;
pub const SQL_ASYNC_ENABLE_ON: u32 = 1u32;
pub const SQL_ASYNC_MODE: u32 = 10021u32;
pub const SQL_ASYNC_NOTIFICATION: u32 = 10025u32;
#[cfg(feature = "Win32_Foundation")]
pub type SQL_ASYNC_NOTIFICATION_CALLBACK = unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, flast: super::super::Foundation::BOOL) -> i16;
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: i32 = 1i32;
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: i32 = 0i32;
pub const SQL_ATTR_ACCESS_MODE: u32 = 101u32;
pub const SQL_ATTR_ANSI_APP: u32 = 115u32;
pub const SQL_ATTR_APPLICATION_KEY: u32 = 203u32;
pub const SQL_ATTR_APP_PARAM_DESC: u32 = 10011u32;
pub const SQL_ATTR_APP_ROW_DESC: u32 = 10010u32;
pub const SQL_ATTR_ASYNC_DBC_EVENT: u32 = 119u32;
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: u32 = 117u32;
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CALLBACK: u32 = 120u32;
pub const SQL_ATTR_ASYNC_DBC_NOTIFICATION_CONTEXT: u32 = 121u32;
pub const SQL_ATTR_ASYNC_ENABLE: u32 = 4u32;
pub const SQL_ATTR_ASYNC_STMT_EVENT: u32 = 29u32;
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CALLBACK: u32 = 30u32;
pub const SQL_ATTR_ASYNC_STMT_NOTIFICATION_CONTEXT: u32 = 31u32;
pub const SQL_ATTR_AUTOCOMMIT: u32 = 102u32;
pub const SQL_ATTR_AUTO_IPD: u32 = 10001u32;
pub const SQL_ATTR_CONCURRENCY: u32 = 7u32;
pub const SQL_ATTR_CONNECTION_DEAD: u32 = 1209u32;
pub const SQL_ATTR_CONNECTION_POOLING: u32 = 201u32;
pub const SQL_ATTR_CONNECTION_TIMEOUT: u32 = 113u32;
pub const SQL_ATTR_CP_MATCH: u32 = 202u32;
pub const SQL_ATTR_CURRENT_CATALOG: u32 = 109u32;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1i32;
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2i32;
pub const SQL_ATTR_CURSOR_TYPE: u32 = 6u32;
pub const SQL_ATTR_DBC_INFO_TOKEN: u32 = 118u32;
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: u32 = 114u32;
pub const SQL_ATTR_ENABLE_AUTO_IPD: u32 = 15u32;
pub const SQL_ATTR_ENLIST_IN_DTC: u32 = 1207u32;
pub const SQL_ATTR_ENLIST_IN_XA: u32 = 1208u32;
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: u32 = 16u32;
pub const SQL_ATTR_IMP_PARAM_DESC: u32 = 10013u32;
pub const SQL_ATTR_IMP_ROW_DESC: u32 = 10012u32;
pub const SQL_ATTR_KEYSET_SIZE: u32 = 8u32;
pub const SQL_ATTR_LOGIN_TIMEOUT: u32 = 103u32;
pub const SQL_ATTR_MAX_LENGTH: u32 = 3u32;
pub const SQL_ATTR_MAX_ROWS: u32 = 1u32;
pub const SQL_ATTR_METADATA_ID: u32 = 10014u32;
pub const SQL_ATTR_NOSCAN: u32 = 2u32;
pub const SQL_ATTR_ODBC_CURSORS: u32 = 110u32;
pub const SQL_ATTR_ODBC_VERSION: u32 = 200u32;
pub const SQL_ATTR_OUTPUT_NTS: u32 = 10001u32;
pub const SQL_ATTR_PACKET_SIZE: u32 = 112u32;
pub const SQL_ATTR_PARAMSET_SIZE: u32 = 22u32;
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: u32 = 21u32;
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: u32 = 17u32;
pub const SQL_ATTR_PARAM_BIND_TYPE: u32 = 18u32;
pub const SQL_ATTR_PARAM_OPERATION_PTR: u32 = 19u32;
pub const SQL_ATTR_PARAM_STATUS_PTR: u32 = 20u32;
pub const SQL_ATTR_QUERY_TIMEOUT: u32 = 0u32;
pub const SQL_ATTR_QUIET_MODE: u32 = 111u32;
pub const SQL_ATTR_READONLY: u32 = 0u32;
pub const SQL_ATTR_READWRITE_UNKNOWN: u32 = 2u32;
pub const SQL_ATTR_RESET_CONNECTION: u32 = 116u32;
pub const SQL_ATTR_RETRIEVE_DATA: u32 = 11u32;
pub const SQL_ATTR_ROWS_FETCHED_PTR: u32 = 26u32;
pub const SQL_ATTR_ROW_ARRAY_SIZE: u32 = 27u32;
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: u32 = 23u32;
pub const SQL_ATTR_ROW_BIND_TYPE: u32 = 5u32;
pub const SQL_ATTR_ROW_NUMBER: u32 = 14u32;
pub const SQL_ATTR_ROW_OPERATION_PTR: u32 = 24u32;
pub const SQL_ATTR_ROW_STATUS_PTR: u32 = 25u32;
pub const SQL_ATTR_SIMULATE_CURSOR: u32 = 10u32;
pub const SQL_ATTR_TRACE: u32 = 104u32;
pub const SQL_ATTR_TRACEFILE: u32 = 105u32;
pub const SQL_ATTR_TRANSLATE_LIB: u32 = 106u32;
pub const SQL_ATTR_TRANSLATE_OPTION: u32 = 107u32;
pub const SQL_ATTR_TXN_ISOLATION: u32 = 108u32;
pub const SQL_ATTR_USE_BOOKMARKS: u32 = 12u32;
pub const SQL_ATTR_WRITE: u32 = 1u32;
pub const SQL_AT_ADD_COLUMN: i32 = 1i32;
pub const SQL_AT_ADD_COLUMN_COLLATION: i32 = 128i32;
pub const SQL_AT_ADD_COLUMN_DEFAULT: i32 = 64i32;
pub const SQL_AT_ADD_COLUMN_SINGLE: i32 = 32i32;
pub const SQL_AT_ADD_CONSTRAINT: i32 = 8i32;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: i32 = 4096i32;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: i32 = 262144i32;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 65536i32;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 131072i32;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: i32 = 32768i32;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: i32 = 524288i32;
pub const SQL_AT_DROP_COLUMN: i32 = 2i32;
pub const SQL_AT_DROP_COLUMN_CASCADE: i32 = 1024i32;
pub const SQL_AT_DROP_COLUMN_DEFAULT: i32 = 512i32;
pub const SQL_AT_DROP_COLUMN_RESTRICT: i32 = 2048i32;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: i32 = 8192i32;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: i32 = 16384i32;
pub const SQL_AT_SET_COLUMN_DEFAULT: i32 = 256i32;
pub const SQL_AUTOCOMMIT: u32 = 102u32;
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1u32;
pub const SQL_AUTOCOMMIT_OFF: u32 = 0u32;
pub const SQL_AUTOCOMMIT_ON: u32 = 1u32;
pub const SQL_BATCH_ROW_COUNT: u32 = 120u32;
pub const SQL_BATCH_SUPPORT: u32 = 121u32;
pub const SQL_BCP_DEFAULT: i32 = 0i32;
pub const SQL_BCP_OFF: i32 = 0i32;
pub const SQL_BCP_ON: i32 = 1i32;
pub const SQL_BEST_ROWID: u32 = 1u32;
pub const SQL_BIGINT: i32 = -5i32;
pub const SQL_BINARY: i32 = -2i32;
pub const SQL_BIND_BY_COLUMN: u32 = 0u32;
pub const SQL_BIND_TYPE: u32 = 5u32;
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_BIT: i32 = -7i32;
pub const SQL_BOOKMARK_PERSISTENCE: u32 = 82u32;
pub const SQL_BP_CLOSE: i32 = 1i32;
pub const SQL_BP_DELETE: i32 = 2i32;
pub const SQL_BP_DROP: i32 = 4i32;
pub const SQL_BP_OTHER_HSTMT: i32 = 32i32;
pub const SQL_BP_SCROLL: i32 = 64i32;
pub const SQL_BP_TRANSACTION: i32 = 8i32;
pub const SQL_BP_UPDATE: i32 = 16i32;
pub const SQL_BRC_EXPLICIT: u32 = 2u32;
pub const SQL_BRC_PROCEDURES: u32 = 1u32;
pub const SQL_BRC_ROLLED_UP: u32 = 4u32;
pub const SQL_BS_ROW_COUNT_EXPLICIT: i32 = 2i32;
pub const SQL_BS_ROW_COUNT_PROC: i32 = 8i32;
pub const SQL_BS_SELECT_EXPLICIT: i32 = 1i32;
pub const SQL_BS_SELECT_PROC: i32 = 4i32;
pub const SQL_CA1_ABSOLUTE: i32 = 2i32;
pub const SQL_CA1_BOOKMARK: i32 = 8i32;
pub const SQL_CA1_BULK_ADD: i32 = 65536i32;
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: i32 = 262144i32;
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: i32 = 524288i32;
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: i32 = 131072i32;
pub const SQL_CA1_LOCK_EXCLUSIVE: i32 = 128i32;
pub const SQL_CA1_LOCK_NO_CHANGE: i32 = 64i32;
pub const SQL_CA1_LOCK_UNLOCK: i32 = 256i32;
pub const SQL_CA1_NEXT: i32 = 1i32;
pub const SQL_CA1_POSITIONED_DELETE: i32 = 16384i32;
pub const SQL_CA1_POSITIONED_UPDATE: i32 = 8192i32;
pub const SQL_CA1_POS_DELETE: i32 = 2048i32;
pub const SQL_CA1_POS_POSITION: i32 = 512i32;
pub const SQL_CA1_POS_REFRESH: i32 = 4096i32;
pub const SQL_CA1_POS_UPDATE: i32 = 1024i32;
pub const SQL_CA1_RELATIVE: i32 = 4i32;
pub const SQL_CA1_SELECT_FOR_UPDATE: i32 = 32768i32;
pub const SQL_CA2_CRC_APPROXIMATE: i32 = 8192i32;
pub const SQL_CA2_CRC_EXACT: i32 = 4096i32;
pub const SQL_CA2_LOCK_CONCURRENCY: i32 = 2i32;
pub const SQL_CA2_MAX_ROWS_CATALOG: i32 = 2048i32;
pub const SQL_CA2_MAX_ROWS_DELETE: i32 = 512i32;
pub const SQL_CA2_MAX_ROWS_INSERT: i32 = 256i32;
pub const SQL_CA2_MAX_ROWS_SELECT: i32 = 128i32;
pub const SQL_CA2_MAX_ROWS_UPDATE: i32 = 1024i32;
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: i32 = 4i32;
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: i32 = 8i32;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: i32 = 1i32;
pub const SQL_CA2_SENSITIVITY_ADDITIONS: i32 = 16i32;
pub const SQL_CA2_SENSITIVITY_DELETIONS: i32 = 32i32;
pub const SQL_CA2_SENSITIVITY_UPDATES: i32 = 64i32;
pub const SQL_CA2_SIMULATE_NON_UNIQUE: i32 = 16384i32;
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: i32 = 32768i32;
pub const SQL_CA2_SIMULATE_UNIQUE: i32 = 65536i32;
pub const SQL_CACHE_DATA_NO: i32 = 0i32;
pub const SQL_CACHE_DATA_YES: i32 = 1i32;
pub const SQL_CASCADE: u32 = 0u32;
pub const SQL_CATALOG_LOCATION: u32 = 114u32;
pub const SQL_CATALOG_NAME: u32 = 10003u32;
pub const SQL_CATALOG_NAME_SEPARATOR: u32 = 41u32;
pub const SQL_CATALOG_TERM: u32 = 42u32;
pub const SQL_CATALOG_USAGE: u32 = 92u32;
pub const SQL_CA_CONSTRAINT_DEFERRABLE: i32 = 64i32;
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: i32 = 16i32;
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 32i32;
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: i32 = 128i32;
pub const SQL_CA_CREATE_ASSERTION: i32 = 1i32;
pub const SQL_CA_SS_BASE: u32 = 1200u32;
pub const SQL_CA_SS_COLUMN_COLLATION: u32 = 1214u32;
pub const SQL_CA_SS_COLUMN_HIDDEN: u32 = 1211u32;
pub const SQL_CA_SS_COLUMN_ID: u32 = 1208u32;
pub const SQL_CA_SS_COLUMN_KEY: u32 = 1212u32;
pub const SQL_CA_SS_COLUMN_OP: u32 = 1209u32;
pub const SQL_CA_SS_COLUMN_ORDER: u32 = 1203u32;
pub const SQL_CA_SS_COLUMN_SIZE: u32 = 1210u32;
pub const SQL_CA_SS_COLUMN_SSTYPE: u32 = 1200u32;
pub const SQL_CA_SS_COLUMN_UTYPE: u32 = 1201u32;
pub const SQL_CA_SS_COLUMN_VARYLEN: u32 = 1204u32;
pub const SQL_CA_SS_COMPUTE_BYLIST: u32 = 1207u32;
pub const SQL_CA_SS_COMPUTE_ID: u32 = 1206u32;
pub const SQL_CA_SS_MAX_USED: u32 = 1218u32;
pub const SQL_CA_SS_NUM_COMPUTES: u32 = 1205u32;
pub const SQL_CA_SS_NUM_ORDERS: u32 = 1202u32;
pub const SQL_CA_SS_VARIANT_SERVER_TYPE: u32 = 1217u32;
pub const SQL_CA_SS_VARIANT_SQL_TYPE: u32 = 1216u32;
pub const SQL_CA_SS_VARIANT_TYPE: u32 = 1215u32;
pub const SQL_CB_CLOSE: u32 = 1u32;
pub const SQL_CB_DELETE: u32 = 0u32;
pub const SQL_CB_NON_NULL: u32 = 1u32;
pub const SQL_CB_NULL: u32 = 0u32;
pub const SQL_CB_PRESERVE: u32 = 2u32;
pub const SQL_CCOL_CREATE_COLLATION: i32 = 1i32;
pub const SQL_CCS_COLLATE_CLAUSE: i32 = 2i32;
pub const SQL_CCS_CREATE_CHARACTER_SET: i32 = 1i32;
pub const SQL_CCS_LIMITED_COLLATION: i32 = 4i32;
pub const SQL_CC_CLOSE: u32 = 1u32;
pub const SQL_CC_DELETE: u32 = 0u32;
pub const SQL_CC_PRESERVE: u32 = 2u32;
pub const SQL_CDO_COLLATION: i32 = 8i32;
pub const SQL_CDO_CONSTRAINT: i32 = 4i32;
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: i32 = 16i32;
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_CDO_CREATE_DOMAIN: i32 = 1i32;
pub const SQL_CDO_DEFAULT: i32 = 2i32;
pub const SQL_CD_FALSE: i32 = 0i32;
pub const SQL_CD_TRUE: i32 = 1i32;
pub const SQL_CHAR: u32 = 1u32;
pub const SQL_CLOSE: u32 = 0u32;
pub const SQL_CL_END: u32 = 2u32;
pub const SQL_CL_START: u32 = 1u32;
pub const SQL_CN_ANY: u32 = 2u32;
pub const SQL_CN_DEFAULT: i32 = 1i32;
pub const SQL_CN_DIFFERENT: u32 = 1u32;
pub const SQL_CN_NONE: u32 = 0u32;
pub const SQL_CN_OFF: i32 = 0i32;
pub const SQL_CN_ON: i32 = 1i32;
pub const SQL_CODE_DATE: u32 = 1u32;
pub const SQL_CODE_DAY: u32 = 3u32;
pub const SQL_CODE_DAY_TO_HOUR: u32 = 8u32;
pub const SQL_CODE_DAY_TO_MINUTE: u32 = 9u32;
pub const SQL_CODE_DAY_TO_SECOND: u32 = 10u32;
pub const SQL_CODE_HOUR: u32 = 4u32;
pub const SQL_CODE_HOUR_TO_MINUTE: u32 = 11u32;
pub const SQL_CODE_HOUR_TO_SECOND: u32 = 12u32;
pub const SQL_CODE_MINUTE: u32 = 5u32;
pub const SQL_CODE_MINUTE_TO_SECOND: u32 = 13u32;
pub const SQL_CODE_MONTH: u32 = 2u32;
pub const SQL_CODE_SECOND: u32 = 6u32;
pub const SQL_CODE_TIME: u32 = 2u32;
pub const SQL_CODE_TIMESTAMP: u32 = 3u32;
pub const SQL_CODE_YEAR: u32 = 1u32;
pub const SQL_CODE_YEAR_TO_MONTH: u32 = 7u32;
pub const SQL_COLATT_OPT_MAX: u32 = 18u32;
pub const SQL_COLATT_OPT_MIN: u32 = 0u32;
pub const SQL_COLLATION_SEQ: u32 = 10004u32;
pub const SQL_COLUMN_ALIAS: u32 = 87u32;
pub const SQL_COLUMN_AUTO_INCREMENT: u32 = 11u32;
pub const SQL_COLUMN_CASE_SENSITIVE: u32 = 12u32;
pub const SQL_COLUMN_COUNT: u32 = 0u32;
pub const SQL_COLUMN_DISPLAY_SIZE: u32 = 6u32;
pub const SQL_COLUMN_DRIVER_START: u32 = 1000u32;
pub const SQL_COLUMN_IGNORE: i32 = -6i32;
pub const SQL_COLUMN_LABEL: u32 = 18u32;
pub const SQL_COLUMN_LENGTH: u32 = 3u32;
pub const SQL_COLUMN_MONEY: u32 = 9u32;
pub const SQL_COLUMN_NAME: u32 = 1u32;
pub const SQL_COLUMN_NULLABLE: u32 = 7u32;
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2i32;
pub const SQL_COLUMN_OWNER_NAME: u32 = 16u32;
pub const SQL_COLUMN_PRECISION: u32 = 4u32;
pub const SQL_COLUMN_QUALIFIER_NAME: u32 = 17u32;
pub const SQL_COLUMN_SCALE: u32 = 5u32;
pub const SQL_COLUMN_SEARCHABLE: u32 = 13u32;
pub const SQL_COLUMN_TABLE_NAME: u32 = 15u32;
pub const SQL_COLUMN_TYPE: u32 = 2u32;
pub const SQL_COLUMN_TYPE_NAME: u32 = 14u32;
pub const SQL_COLUMN_UNSIGNED: u32 = 8u32;
pub const SQL_COLUMN_UPDATABLE: u32 = 10u32;
pub const SQL_COMMIT: u32 = 0u32;
pub const SQL_CONCAT_NULL_BEHAVIOR: u32 = 22u32;
pub const SQL_CONCURRENCY: u32 = 7u32;
pub const SQL_CONCUR_DEFAULT: u32 = 1u32;
pub const SQL_CONCUR_LOCK: u32 = 2u32;
pub const SQL_CONCUR_READ_ONLY: u32 = 1u32;
pub const SQL_CONCUR_ROWVER: u32 = 3u32;
pub const SQL_CONCUR_TIMESTAMP: u32 = 3u32;
pub const SQL_CONCUR_VALUES: u32 = 4u32;
pub const SQL_CONNECT_OPT_DRVR_START: u32 = 1000u32;
pub const SQL_CONN_OPT_MAX: u32 = 112u32;
pub const SQL_CONN_OPT_MIN: u32 = 101u32;
pub const SQL_CONN_POOL_RATING_BEST: u32 = 100u32;
pub const SQL_CONN_POOL_RATING_GOOD_ENOUGH: u32 = 99u32;
pub const SQL_CONN_POOL_RATING_USELESS: u32 = 0u32;
pub const SQL_CONVERT_BIGINT: u32 = 53u32;
pub const SQL_CONVERT_BINARY: u32 = 54u32;
pub const SQL_CONVERT_BIT: u32 = 55u32;
pub const SQL_CONVERT_CHAR: u32 = 56u32;
pub const SQL_CONVERT_DATE: u32 = 57u32;
pub const SQL_CONVERT_DECIMAL: u32 = 58u32;
pub const SQL_CONVERT_DOUBLE: u32 = 59u32;
pub const SQL_CONVERT_FLOAT: u32 = 60u32;
pub const SQL_CONVERT_FUNCTIONS: u32 = 48u32;
pub const SQL_CONVERT_GUID: u32 = 173u32;
pub const SQL_CONVERT_INTEGER: u32 = 61u32;
pub const SQL_CONVERT_INTERVAL_DAY_TIME: u32 = 123u32;
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: u32 = 124u32;
pub const SQL_CONVERT_LONGVARBINARY: u32 = 71u32;
pub const SQL_CONVERT_LONGVARCHAR: u32 = 62u32;
pub const SQL_CONVERT_NUMERIC: u32 = 63u32;
pub const SQL_CONVERT_REAL: u32 = 64u32;
pub const SQL_CONVERT_SMALLINT: u32 = 65u32;
pub const SQL_CONVERT_TIME: u32 = 66u32;
pub const SQL_CONVERT_TIMESTAMP: u32 = 67u32;
pub const SQL_CONVERT_TINYINT: u32 = 68u32;
pub const SQL_CONVERT_VARBINARY: u32 = 69u32;
pub const SQL_CONVERT_VARCHAR: u32 = 70u32;
pub const SQL_CONVERT_WCHAR: u32 = 122u32;
pub const SQL_CONVERT_WLONGVARCHAR: u32 = 125u32;
pub const SQL_CONVERT_WVARCHAR: u32 = 126u32;
pub const SQL_COPT_SS_ANSI_NPW: u32 = 1218u32;
pub const SQL_COPT_SS_ANSI_OEM: u32 = 1206u32;
pub const SQL_COPT_SS_ATTACHDBFILENAME: u32 = 1221u32;
pub const SQL_COPT_SS_BASE: u32 = 1200u32;
pub const SQL_COPT_SS_BASE_EX: u32 = 1240u32;
pub const SQL_COPT_SS_BCP: u32 = 1219u32;
pub const SQL_COPT_SS_BROWSE_CACHE_DATA: u32 = 1245u32;
pub const SQL_COPT_SS_BROWSE_CONNECT: u32 = 1241u32;
pub const SQL_COPT_SS_BROWSE_SERVER: u32 = 1242u32;
pub const SQL_COPT_SS_CONCAT_NULL: u32 = 1222u32;
pub const SQL_COPT_SS_CONNECTION_DEAD: u32 = 1244u32;
pub const SQL_COPT_SS_ENCRYPT: u32 = 1223u32;
pub const SQL_COPT_SS_EX_MAX_USED: u32 = 1246u32;
pub const SQL_COPT_SS_FALLBACK_CONNECT: u32 = 1210u32;
pub const SQL_COPT_SS_INTEGRATED_SECURITY: u32 = 1203u32;
pub const SQL_COPT_SS_MAX_USED: u32 = 1223u32;
pub const SQL_COPT_SS_PERF_DATA: u32 = 1211u32;
pub const SQL_COPT_SS_PERF_DATA_LOG: u32 = 1212u32;
pub const SQL_COPT_SS_PERF_DATA_LOG_NOW: u32 = 1216u32;
pub const SQL_COPT_SS_PERF_QUERY: u32 = 1215u32;
pub const SQL_COPT_SS_PERF_QUERY_INTERVAL: u32 = 1213u32;
pub const SQL_COPT_SS_PERF_QUERY_LOG: u32 = 1214u32;
pub const SQL_COPT_SS_PRESERVE_CURSORS: u32 = 1204u32;
pub const SQL_COPT_SS_QUOTED_IDENT: u32 = 1217u32;
pub const SQL_COPT_SS_REMOTE_PWD: u32 = 1201u32;
pub const SQL_COPT_SS_RESET_CONNECTION: u32 = 1246u32;
pub const SQL_COPT_SS_TRANSLATE: u32 = 1220u32;
pub const SQL_COPT_SS_USER_DATA: u32 = 1205u32;
pub const SQL_COPT_SS_USE_PROC_FOR_PREP: u32 = 1202u32;
pub const SQL_COPT_SS_WARN_ON_CP_ERROR: u32 = 1243u32;
pub const SQL_CORRELATION_NAME: u32 = 74u32;
pub const SQL_CO_AF: i32 = 2i32;
pub const SQL_CO_DEFAULT: i32 = 0i32;
pub const SQL_CO_FFO: i32 = 1i32;
pub const SQL_CO_FIREHOSE_AF: i32 = 4i32;
pub const SQL_CO_OFF: i32 = 0i32;
pub const SQL_CP_DEFAULT: u32 = 0u32;
pub const SQL_CP_DRIVER_AWARE: u32 = 3u32;
pub const SQL_CP_MATCH_DEFAULT: u32 = 0u32;
pub const SQL_CP_OFF: u32 = 0u32;
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1u32;
pub const SQL_CP_ONE_PER_HENV: u32 = 2u32;
pub const SQL_CP_RELAXED_MATCH: u32 = 1u32;
pub const SQL_CP_STRICT_MATCH: u32 = 0u32;
pub const SQL_CREATE_ASSERTION: u32 = 127u32;
pub const SQL_CREATE_CHARACTER_SET: u32 = 128u32;
pub const SQL_CREATE_COLLATION: u32 = 129u32;
pub const SQL_CREATE_DOMAIN: u32 = 130u32;
pub const SQL_CREATE_SCHEMA: u32 = 131u32;
pub const SQL_CREATE_TABLE: u32 = 132u32;
pub const SQL_CREATE_TRANSLATION: u32 = 133u32;
pub const SQL_CREATE_VIEW: u32 = 134u32;
pub const SQL_CR_CLOSE: u32 = 1u32;
pub const SQL_CR_DELETE: u32 = 0u32;
pub const SQL_CR_PRESERVE: u32 = 2u32;
pub const SQL_CS_AUTHORIZATION: i32 = 2i32;
pub const SQL_CS_CREATE_SCHEMA: i32 = 1i32;
pub const SQL_CS_DEFAULT_CHARACTER_SET: i32 = 4i32;
pub const SQL_CTR_CREATE_TRANSLATION: i32 = 1i32;
pub const SQL_CT_COLUMN_COLLATION: i32 = 2048i32;
pub const SQL_CT_COLUMN_CONSTRAINT: i32 = 512i32;
pub const SQL_CT_COLUMN_DEFAULT: i32 = 1024i32;
pub const SQL_CT_COMMIT_DELETE: i32 = 4i32;
pub const SQL_CT_COMMIT_PRESERVE: i32 = 2i32;
pub const SQL_CT_CONSTRAINT_DEFERRABLE: i32 = 128i32;
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: i32 = 32i32;
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: i32 = 64i32;
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: i32 = 8192i32;
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: i32 = 256i32;
pub const SQL_CT_CREATE_TABLE: i32 = 1i32;
pub const SQL_CT_GLOBAL_TEMPORARY: i32 = 8i32;
pub const SQL_CT_LOCAL_TEMPORARY: i32 = 16i32;
pub const SQL_CT_TABLE_CONSTRAINT: i32 = 4096i32;
pub const SQL_CURRENT_QUALIFIER: u32 = 109u32;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u32 = 23u32;
pub const SQL_CURSOR_DYNAMIC: u32 = 2u32;
pub const SQL_CURSOR_FAST_FORWARD_ONLY: u32 = 8u32;
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0u32;
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1u32;
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: u32 = 24u32;
pub const SQL_CURSOR_SENSITIVITY: u32 = 10001u32;
pub const SQL_CURSOR_STATIC: u32 = 3u32;
pub const SQL_CURSOR_TYPE: u32 = 6u32;
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_CUR_DEFAULT: u32 = 2u32;
pub const SQL_CUR_USE_DRIVER: u32 = 2u32;
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0u32;
pub const SQL_CUR_USE_ODBC: u32 = 1u32;
pub const SQL_CU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_CU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_CU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_CU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_CU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_CVT_BIGINT: i32 = 16384i32;
pub const SQL_CVT_BINARY: i32 = 1024i32;
pub const SQL_CVT_BIT: i32 = 4096i32;
pub const SQL_CVT_CHAR: i32 = 1i32;
pub const SQL_CVT_DATE: i32 = 32768i32;
pub const SQL_CVT_DECIMAL: i32 = 4i32;
pub const SQL_CVT_DOUBLE: i32 = 128i32;
pub const SQL_CVT_FLOAT: i32 = 32i32;
pub const SQL_CVT_GUID: i32 = 16777216i32;
pub const SQL_CVT_INTEGER: i32 = 8i32;
pub const SQL_CVT_INTERVAL_DAY_TIME: i32 = 1048576i32;
pub const SQL_CVT_INTERVAL_YEAR_MONTH: i32 = 524288i32;
pub const SQL_CVT_LONGVARBINARY: i32 = 262144i32;
pub const SQL_CVT_LONGVARCHAR: i32 = 512i32;
pub const SQL_CVT_NUMERIC: i32 = 2i32;
pub const SQL_CVT_REAL: i32 = 64i32;
pub const SQL_CVT_SMALLINT: i32 = 16i32;
pub const SQL_CVT_TIME: i32 = 65536i32;
pub const SQL_CVT_TIMESTAMP: i32 = 131072i32;
pub const SQL_CVT_TINYINT: i32 = 8192i32;
pub const SQL_CVT_VARBINARY: i32 = 2048i32;
pub const SQL_CVT_VARCHAR: i32 = 256i32;
pub const SQL_CVT_WCHAR: i32 = 2097152i32;
pub const SQL_CVT_WLONGVARCHAR: i32 = 4194304i32;
pub const SQL_CVT_WVARCHAR: i32 = 8388608i32;
pub const SQL_CV_CASCADED: i32 = 4i32;
pub const SQL_CV_CHECK_OPTION: i32 = 2i32;
pub const SQL_CV_CREATE_VIEW: i32 = 1i32;
pub const SQL_CV_LOCAL: i32 = 8i32;
pub const SQL_C_BINARY: i32 = -2i32;
pub const SQL_C_BIT: i32 = -7i32;
pub const SQL_C_CHAR: u32 = 1u32;
pub const SQL_C_DATE: u32 = 9u32;
pub const SQL_C_DEFAULT: u32 = 99u32;
pub const SQL_C_DOUBLE: u32 = 8u32;
pub const SQL_C_FLOAT: u32 = 7u32;
pub const SQL_C_GUID: i32 = -11i32;
pub const SQL_C_INTERVAL_DAY: i32 = -83i32;
pub const SQL_C_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
pub const SQL_C_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
pub const SQL_C_INTERVAL_HOUR: i32 = -84i32;
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
pub const SQL_C_INTERVAL_MINUTE: i32 = -85i32;
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
pub const SQL_C_INTERVAL_MONTH: i32 = -81i32;
pub const SQL_C_INTERVAL_SECOND: i32 = -86i32;
pub const SQL_C_INTERVAL_YEAR: i32 = -80i32;
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
pub const SQL_C_LONG: u32 = 4u32;
pub const SQL_C_NUMERIC: u32 = 2u32;
pub const SQL_C_SHORT: u32 = 5u32;
pub const SQL_C_TCHAR: i32 = -8i32;
pub const SQL_C_TIME: u32 = 10u32;
pub const SQL_C_TIMESTAMP: u32 = 11u32;
pub const SQL_C_TINYINT: i32 = -6i32;
pub const SQL_C_TYPE_DATE: u32 = 91u32;
pub const SQL_C_TYPE_TIME: u32 = 92u32;
pub const SQL_C_TYPE_TIMESTAMP: u32 = 93u32;
pub const SQL_C_VARBOOKMARK: i32 = -2i32;
pub const SQL_C_WCHAR: i32 = -8i32;
pub const SQL_DATABASE_NAME: u32 = 16u32;
pub const SQL_DATA_AT_EXEC: i32 = -2i32;
pub const SQL_DATA_SOURCE_NAME: u32 = 2u32;
pub const SQL_DATA_SOURCE_READ_ONLY: u32 = 25u32;
pub const SQL_DATE: u32 = 9u32;
pub const SQL_DATETIME: u32 = 9u32;
pub const SQL_DATETIME_LITERALS: u32 = 119u32;
pub const SQL_DATE_LEN: u32 = 10u32;
pub const SQL_DAY: u32 = 3u32;
pub const SQL_DAY_TO_HOUR: u32 = 8u32;
pub const SQL_DAY_TO_MINUTE: u32 = 9u32;
pub const SQL_DAY_TO_SECOND: u32 = 10u32;
pub const SQL_DA_DROP_ASSERTION: i32 = 1i32;
pub const SQL_DBMS_NAME: u32 = 17u32;
pub const SQL_DBMS_VER: u32 = 18u32;
pub const SQL_DB_DEFAULT: u32 = 0u32;
pub const SQL_DB_DISCONNECT: u32 = 1u32;
pub const SQL_DB_RETURN_TO_POOL: u32 = 0u32;
pub const SQL_DCS_DROP_CHARACTER_SET: i32 = 1i32;
pub const SQL_DC_DROP_COLLATION: i32 = 1i32;
pub const SQL_DDL_INDEX: u32 = 170u32;
pub const SQL_DD_CASCADE: i32 = 4i32;
pub const SQL_DD_DROP_DOMAIN: i32 = 1i32;
pub const SQL_DD_RESTRICT: i32 = 2i32;
pub const SQL_DECIMAL: u32 = 3u32;
pub const SQL_DEFAULT: u32 = 99u32;
pub const SQL_DEFAULT_PARAM: i32 = -5i32;
pub const SQL_DEFAULT_TXN_ISOLATION: u32 = 26u32;
pub const SQL_DELETE: u32 = 3u32;
pub const SQL_DELETE_BY_BOOKMARK: u32 = 6u32;
pub const SQL_DESCRIBE_PARAMETER: u32 = 10002u32;
pub const SQL_DESC_ALLOC_AUTO: u32 = 1u32;
pub const SQL_DESC_ALLOC_TYPE: u32 = 1099u32;
pub const SQL_DESC_ALLOC_USER: u32 = 2u32;
pub const SQL_DESC_ARRAY_SIZE: u32 = 20u32;
pub const SQL_DESC_ARRAY_STATUS_PTR: u32 = 21u32;
pub const SQL_DESC_BASE_COLUMN_NAME: u32 = 22u32;
pub const SQL_DESC_BASE_TABLE_NAME: u32 = 23u32;
pub const SQL_DESC_BIND_OFFSET_PTR: u32 = 24u32;
pub const SQL_DESC_BIND_TYPE: u32 = 25u32;
pub const SQL_DESC_COUNT: u32 = 1001u32;
pub const SQL_DESC_DATA_PTR: u32 = 1010u32;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u32 = 1007u32;
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: u32 = 26u32;
pub const SQL_DESC_INDICATOR_PTR: u32 = 1009u32;
pub const SQL_DESC_LENGTH: u32 = 1003u32;
pub const SQL_DESC_LITERAL_PREFIX: u32 = 27u32;
pub const SQL_DESC_LITERAL_SUFFIX: u32 = 28u32;
pub const SQL_DESC_LOCAL_TYPE_NAME: u32 = 29u32;
pub const SQL_DESC_MAXIMUM_SCALE: u32 = 30u32;
pub const SQL_DESC_MINIMUM_SCALE: u32 = 31u32;
pub const SQL_DESC_NAME: u32 = 1011u32;
pub const SQL_DESC_NULLABLE: u32 = 1008u32;
pub const SQL_DESC_NUM_PREC_RADIX: u32 = 32u32;
pub const SQL_DESC_OCTET_LENGTH: u32 = 1013u32;
pub const SQL_DESC_OCTET_LENGTH_PTR: u32 = 1004u32;
pub const SQL_DESC_PARAMETER_TYPE: u32 = 33u32;
pub const SQL_DESC_PRECISION: u32 = 1005u32;
pub const SQL_DESC_ROWS_PROCESSED_PTR: u32 = 34u32;
pub const SQL_DESC_ROWVER: u32 = 35u32;
pub const SQL_DESC_SCALE: u32 = 1006u32;
pub const SQL_DESC_TYPE: u32 = 1002u32;
pub const SQL_DESC_UNNAMED: u32 = 1012u32;
pub const SQL_DIAG_ALTER_DOMAIN: u32 = 3u32;
pub const SQL_DIAG_ALTER_TABLE: u32 = 4u32;
pub const SQL_DIAG_CALL: u32 = 7u32;
pub const SQL_DIAG_CLASS_ORIGIN: u32 = 8u32;
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247i32;
pub const SQL_DIAG_CONNECTION_NAME: u32 = 10u32;
pub const SQL_DIAG_CREATE_ASSERTION: u32 = 6u32;
pub const SQL_DIAG_CREATE_CHARACTER_SET: u32 = 8u32;
pub const SQL_DIAG_CREATE_COLLATION: u32 = 10u32;
pub const SQL_DIAG_CREATE_DOMAIN: u32 = 23u32;
pub const SQL_DIAG_CREATE_INDEX: i32 = -1i32;
pub const SQL_DIAG_CREATE_SCHEMA: u32 = 64u32;
pub const SQL_DIAG_CREATE_TABLE: u32 = 77u32;
pub const SQL_DIAG_CREATE_TRANSLATION: u32 = 79u32;
pub const SQL_DIAG_CREATE_VIEW: u32 = 84u32;
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249i32;
pub const SQL_DIAG_DELETE_WHERE: u32 = 19u32;
pub const SQL_DIAG_DFC_SS_BASE: i32 = -200i32;
pub const SQL_DIAG_DROP_ASSERTION: u32 = 24u32;
pub const SQL_DIAG_DROP_CHARACTER_SET: u32 = 25u32;
pub const SQL_DIAG_DROP_COLLATION: u32 = 26u32;
pub const SQL_DIAG_DROP_DOMAIN: u32 = 27u32;
pub const SQL_DIAG_DROP_INDEX: i32 = -2i32;
pub const SQL_DIAG_DROP_SCHEMA: u32 = 31u32;
pub const SQL_DIAG_DROP_TABLE: u32 = 32u32;
pub const SQL_DIAG_DROP_TRANSLATION: u32 = 33u32;
pub const SQL_DIAG_DROP_VIEW: u32 = 36u32;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u32 = 38u32;
pub const SQL_DIAG_DYNAMIC_FUNCTION: u32 = 7u32;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u32 = 12u32;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u32 = 81u32;
pub const SQL_DIAG_GRANT: u32 = 48u32;
pub const SQL_DIAG_INSERT: u32 = 50u32;
pub const SQL_DIAG_MESSAGE_TEXT: u32 = 6u32;
pub const SQL_DIAG_NATIVE: u32 = 5u32;
pub const SQL_DIAG_NUMBER: u32 = 2u32;
pub const SQL_DIAG_RETURNCODE: u32 = 1u32;
pub const SQL_DIAG_REVOKE: u32 = 59u32;
pub const SQL_DIAG_ROW_COUNT: u32 = 3u32;
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248i32;
pub const SQL_DIAG_SELECT_CURSOR: u32 = 85u32;
pub const SQL_DIAG_SERVER_NAME: u32 = 11u32;
pub const SQL_DIAG_SQLSTATE: u32 = 4u32;
pub const SQL_DIAG_SS_BASE: i32 = -1150i32;
pub const SQL_DIAG_SS_MSGSTATE: i32 = -1150i32;
pub const SQL_DIAG_SUBCLASS_ORIGIN: u32 = 9u32;
pub const SQL_DIAG_UNKNOWN_STATEMENT: u32 = 0u32;
pub const SQL_DIAG_UPDATE_WHERE: u32 = 82u32;
pub const SQL_DI_CREATE_INDEX: i32 = 1i32;
pub const SQL_DI_DROP_INDEX: i32 = 2i32;
pub const SQL_DL_SQL92_DATE: i32 = 1i32;
pub const SQL_DL_SQL92_INTERVAL_DAY: i32 = 32i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: i32 = 1024i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: i32 = 2048i32;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: i32 = 4096i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR: i32 = 64i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: i32 = 8192i32;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: i32 = 16384i32;
pub const SQL_DL_SQL92_INTERVAL_MINUTE: i32 = 128i32;
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: i32 = 32768i32;
pub const SQL_DL_SQL92_INTERVAL_MONTH: i32 = 16i32;
pub const SQL_DL_SQL92_INTERVAL_SECOND: i32 = 256i32;
pub const SQL_DL_SQL92_INTERVAL_YEAR: i32 = 8i32;
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: i32 = 512i32;
pub const SQL_DL_SQL92_TIME: i32 = 2i32;
pub const SQL_DL_SQL92_TIMESTAMP: i32 = 4i32;
pub const SQL_DM_VER: u32 = 171u32;
pub const SQL_DOUBLE: u32 = 8u32;
pub const SQL_DP_OFF: i32 = 0i32;
pub const SQL_DP_ON: i32 = 1i32;
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: i32 = 1i32;
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: i32 = 0i32;
pub const SQL_DRIVER_AWARE_POOLING_SUPPORTED: u32 = 10024u32;
pub const SQL_DRIVER_COMPLETE: u32 = 1u32;
pub const SQL_DRIVER_COMPLETE_REQUIRED: u32 = 3u32;
pub const SQL_DRIVER_CONN_ATTR_BASE: u32 = 16384u32;
pub const SQL_DRIVER_C_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_DESC_FIELD_BASE: u32 = 16384u32;
pub const SQL_DRIVER_DIAG_FIELD_BASE: u32 = 16384u32;
pub const SQL_DRIVER_HDBC: u32 = 3u32;
pub const SQL_DRIVER_HDESC: u32 = 135u32;
pub const SQL_DRIVER_HENV: u32 = 4u32;
pub const SQL_DRIVER_HLIB: u32 = 76u32;
pub const SQL_DRIVER_HSTMT: u32 = 5u32;
pub const SQL_DRIVER_INFO_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_NAME: u32 = 6u32;
pub const SQL_DRIVER_NOPROMPT: u32 = 0u32;
pub const SQL_DRIVER_ODBC_VER: u32 = 77u32;
pub const SQL_DRIVER_PROMPT: u32 = 2u32;
pub const SQL_DRIVER_SQL_TYPE_BASE: u32 = 16384u32;
pub const SQL_DRIVER_STMT_ATTR_BASE: u32 = 16384u32;
pub const SQL_DRIVER_VER: u32 = 7u32;
pub const SQL_DROP: u32 = 1u32;
pub const SQL_DROP_ASSERTION: u32 = 136u32;
pub const SQL_DROP_CHARACTER_SET: u32 = 137u32;
pub const SQL_DROP_COLLATION: u32 = 138u32;
pub const SQL_DROP_DOMAIN: u32 = 139u32;
pub const SQL_DROP_SCHEMA: u32 = 140u32;
pub const SQL_DROP_TABLE: u32 = 141u32;
pub const SQL_DROP_TRANSLATION: u32 = 142u32;
pub const SQL_DROP_VIEW: u32 = 143u32;
pub const SQL_DS_CASCADE: i32 = 4i32;
pub const SQL_DS_DROP_SCHEMA: i32 = 1i32;
pub const SQL_DS_RESTRICT: i32 = 2i32;
pub const SQL_DTC_DONE: i32 = 0i32;
pub const SQL_DTC_ENLIST_EXPENSIVE: i32 = 1i32;
pub const SQL_DTC_TRANSITION_COST: u32 = 1750u32;
pub const SQL_DTC_UNENLIST_EXPENSIVE: i32 = 2i32;
pub const SQL_DTR_DROP_TRANSLATION: i32 = 1i32;
pub const SQL_DT_CASCADE: i32 = 4i32;
pub const SQL_DT_DROP_TABLE: i32 = 1i32;
pub const SQL_DT_RESTRICT: i32 = 2i32;
pub const SQL_DV_CASCADE: i32 = 4i32;
pub const SQL_DV_DROP_VIEW: i32 = 1i32;
pub const SQL_DV_RESTRICT: i32 = 2i32;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: u32 = 144u32;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: u32 = 145u32;
pub const SQL_ENSURE: u32 = 1u32;
pub const SQL_ENTIRE_ROWSET: u32 = 0u32;
pub const SQL_EN_OFF: i32 = 0i32;
pub const SQL_EN_ON: i32 = 1i32;
pub const SQL_ERROR: i32 = -1i32;
pub const SQL_EXPRESSIONS_IN_ORDERBY: u32 = 27u32;
pub const SQL_EXT_API_LAST: u32 = 72u32;
pub const SQL_EXT_API_START: u32 = 40u32;
pub const SQL_FALSE: u32 = 0u32;
pub const SQL_FAST_CONNECT: u32 = 1200u32;
pub const SQL_FB_DEFAULT: i32 = 0i32;
pub const SQL_FB_OFF: i32 = 0i32;
pub const SQL_FB_ON: i32 = 1i32;
pub const SQL_FC_DEFAULT: i32 = 0i32;
pub const SQL_FC_OFF: i32 = 0i32;
pub const SQL_FC_ON: i32 = 1i32;
pub const SQL_FD_FETCH_ABSOLUTE: i32 = 16i32;
pub const SQL_FD_FETCH_BOOKMARK: i32 = 128i32;
pub const SQL_FD_FETCH_FIRST: i32 = 2i32;
pub const SQL_FD_FETCH_LAST: i32 = 4i32;
pub const SQL_FD_FETCH_NEXT: i32 = 1i32;
pub const SQL_FD_FETCH_PREV: i32 = 8i32;
pub const SQL_FD_FETCH_PRIOR: i32 = 8i32;
pub const SQL_FD_FETCH_RELATIVE: i32 = 32i32;
pub const SQL_FD_FETCH_RESUME: i32 = 64i32;
pub const SQL_FETCH_ABSOLUTE: u32 = 5u32;
pub const SQL_FETCH_BOOKMARK: u32 = 8u32;
pub const SQL_FETCH_BY_BOOKMARK: u32 = 7u32;
pub const SQL_FETCH_DIRECTION: u32 = 8u32;
pub const SQL_FETCH_FIRST: u32 = 2u32;
pub const SQL_FETCH_FIRST_SYSTEM: u32 = 32u32;
pub const SQL_FETCH_FIRST_USER: u32 = 31u32;
pub const SQL_FETCH_LAST: u32 = 3u32;
pub const SQL_FETCH_NEXT: u32 = 1u32;
pub const SQL_FETCH_PREV: u32 = 4u32;
pub const SQL_FETCH_PRIOR: u32 = 4u32;
pub const SQL_FETCH_RELATIVE: u32 = 6u32;
pub const SQL_FETCH_RESUME: u32 = 7u32;
pub const SQL_FILE_CATALOG: u32 = 2u32;
pub const SQL_FILE_NOT_SUPPORTED: u32 = 0u32;
pub const SQL_FILE_QUALIFIER: u32 = 2u32;
pub const SQL_FILE_TABLE: u32 = 1u32;
pub const SQL_FILE_USAGE: u32 = 84u32;
pub const SQL_FLOAT: u32 = 6u32;
pub const SQL_FN_CVT_CAST: i32 = 2i32;
pub const SQL_FN_CVT_CONVERT: i32 = 1i32;
pub const SQL_FN_NUM_ABS: i32 = 1i32;
pub const SQL_FN_NUM_ACOS: i32 = 2i32;
pub const SQL_FN_NUM_ASIN: i32 = 4i32;
pub const SQL_FN_NUM_ATAN: i32 = 8i32;
pub const SQL_FN_NUM_ATAN2: i32 = 16i32;
pub const SQL_FN_NUM_CEILING: i32 = 32i32;
pub const SQL_FN_NUM_COS: i32 = 64i32;
pub const SQL_FN_NUM_COT: i32 = 128i32;
pub const SQL_FN_NUM_DEGREES: i32 = 262144i32;
pub const SQL_FN_NUM_EXP: i32 = 256i32;
pub const SQL_FN_NUM_FLOOR: i32 = 512i32;
pub const SQL_FN_NUM_LOG: i32 = 1024i32;
pub const SQL_FN_NUM_LOG10: i32 = 524288i32;
pub const SQL_FN_NUM_MOD: i32 = 2048i32;
pub const SQL_FN_NUM_PI: i32 = 65536i32;
pub const SQL_FN_NUM_POWER: i32 = 1048576i32;
pub const SQL_FN_NUM_RADIANS: i32 = 2097152i32;
pub const SQL_FN_NUM_RAND: i32 = 131072i32;
pub const SQL_FN_NUM_ROUND: i32 = 4194304i32;
pub const SQL_FN_NUM_SIGN: i32 = 4096i32;
pub const SQL_FN_NUM_SIN: i32 = 8192i32;
pub const SQL_FN_NUM_SQRT: i32 = 16384i32;
pub const SQL_FN_NUM_TAN: i32 = 32768i32;
pub const SQL_FN_NUM_TRUNCATE: i32 = 8388608i32;
pub const SQL_FN_STR_ASCII: i32 = 8192i32;
pub const SQL_FN_STR_BIT_LENGTH: i32 = 524288i32;
pub const SQL_FN_STR_CHAR: i32 = 16384i32;
pub const SQL_FN_STR_CHARACTER_LENGTH: i32 = 2097152i32;
pub const SQL_FN_STR_CHAR_LENGTH: i32 = 1048576i32;
pub const SQL_FN_STR_CONCAT: i32 = 1i32;
pub const SQL_FN_STR_DIFFERENCE: i32 = 32768i32;
pub const SQL_FN_STR_INSERT: i32 = 2i32;
pub const SQL_FN_STR_LCASE: i32 = 64i32;
pub const SQL_FN_STR_LEFT: i32 = 4i32;
pub const SQL_FN_STR_LENGTH: i32 = 16i32;
pub const SQL_FN_STR_LOCATE: i32 = 32i32;
pub const SQL_FN_STR_LOCATE_2: i32 = 65536i32;
pub const SQL_FN_STR_LTRIM: i32 = 8i32;
pub const SQL_FN_STR_OCTET_LENGTH: i32 = 4194304i32;
pub const SQL_FN_STR_POSITION: i32 = 8388608i32;
pub const SQL_FN_STR_REPEAT: i32 = 128i32;
pub const SQL_FN_STR_REPLACE: i32 = 256i32;
pub const SQL_FN_STR_RIGHT: i32 = 512i32;
pub const SQL_FN_STR_RTRIM: i32 = 1024i32;
pub const SQL_FN_STR_SOUNDEX: i32 = 131072i32;
pub const SQL_FN_STR_SPACE: i32 = 262144i32;
pub const SQL_FN_STR_SUBSTRING: i32 = 2048i32;
pub const SQL_FN_STR_UCASE: i32 = 4096i32;
pub const SQL_FN_SYS_DBNAME: i32 = 2i32;
pub const SQL_FN_SYS_IFNULL: i32 = 4i32;
pub const SQL_FN_SYS_USERNAME: i32 = 1i32;
pub const SQL_FN_TD_CURDATE: i32 = 2i32;
pub const SQL_FN_TD_CURRENT_DATE: i32 = 131072i32;
pub const SQL_FN_TD_CURRENT_TIME: i32 = 262144i32;
pub const SQL_FN_TD_CURRENT_TIMESTAMP: i32 = 524288i32;
pub const SQL_FN_TD_CURTIME: i32 = 512i32;
pub const SQL_FN_TD_DAYNAME: i32 = 32768i32;
pub const SQL_FN_TD_DAYOFMONTH: i32 = 4i32;
pub const SQL_FN_TD_DAYOFWEEK: i32 = 8i32;
pub const SQL_FN_TD_DAYOFYEAR: i32 = 16i32;
pub const SQL_FN_TD_EXTRACT: i32 = 1048576i32;
pub const SQL_FN_TD_HOUR: i32 = 1024i32;
pub const SQL_FN_TD_MINUTE: i32 = 2048i32;
pub const SQL_FN_TD_MONTH: i32 = 32i32;
pub const SQL_FN_TD_MONTHNAME: i32 = 65536i32;
pub const SQL_FN_TD_NOW: i32 = 1i32;
pub const SQL_FN_TD_QUARTER: i32 = 64i32;
pub const SQL_FN_TD_SECOND: i32 = 4096i32;
pub const SQL_FN_TD_TIMESTAMPADD: i32 = 8192i32;
pub const SQL_FN_TD_TIMESTAMPDIFF: i32 = 16384i32;
pub const SQL_FN_TD_WEEK: i32 = 128i32;
pub const SQL_FN_TD_YEAR: i32 = 256i32;
pub const SQL_FN_TSI_DAY: i32 = 16i32;
pub const SQL_FN_TSI_FRAC_SECOND: i32 = 1i32;
pub const SQL_FN_TSI_HOUR: i32 = 8i32;
pub const SQL_FN_TSI_MINUTE: i32 = 4i32;
pub const SQL_FN_TSI_MONTH: i32 = 64i32;
pub const SQL_FN_TSI_QUARTER: i32 = 128i32;
pub const SQL_FN_TSI_SECOND: i32 = 2i32;
pub const SQL_FN_TSI_WEEK: i32 = 32i32;
pub const SQL_FN_TSI_YEAR: i32 = 256i32;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: u32 = 146u32;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: u32 = 147u32;
pub const SQL_GB_COLLATE: u32 = 4u32;
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: u32 = 2u32;
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: u32 = 1u32;
pub const SQL_GB_NOT_SUPPORTED: u32 = 0u32;
pub const SQL_GB_NO_RELATION: u32 = 3u32;
pub const SQL_GD_ANY_COLUMN: i32 = 1i32;
pub const SQL_GD_ANY_ORDER: i32 = 2i32;
pub const SQL_GD_BLOCK: i32 = 4i32;
pub const SQL_GD_BOUND: i32 = 8i32;
pub const SQL_GD_OUTPUT_PARAMS: i32 = 16i32;
pub const SQL_GETDATA_EXTENSIONS: u32 = 81u32;
pub const SQL_GET_BOOKMARK: u32 = 13u32;
pub const SQL_GROUP_BY: u32 = 88u32;
pub const SQL_GUID: i32 = -11i32;
pub const SQL_HANDLE_DBC: u32 = 2u32;
pub const SQL_HANDLE_DBC_INFO_TOKEN: u32 = 6u32;
pub const SQL_HANDLE_DESC: u32 = 4u32;
pub const SQL_HANDLE_ENV: u32 = 1u32;
pub const SQL_HANDLE_SENV: u32 = 5u32;
pub const SQL_HANDLE_STMT: u32 = 3u32;
pub const SQL_HC_DEFAULT: i32 = 0i32;
pub const SQL_HC_OFF: i32 = 0i32;
pub const SQL_HC_ON: i32 = 1i32;
pub const SQL_HOUR: u32 = 4u32;
pub const SQL_HOUR_TO_MINUTE: u32 = 11u32;
pub const SQL_HOUR_TO_SECOND: u32 = 12u32;
pub const SQL_IC_LOWER: u32 = 2u32;
pub const SQL_IC_MIXED: u32 = 4u32;
pub const SQL_IC_SENSITIVE: u32 = 3u32;
pub const SQL_IC_UPPER: u32 = 1u32;
pub const SQL_IDENTIFIER_CASE: u32 = 28u32;
pub const SQL_IDENTIFIER_QUOTE_CHAR: u32 = 29u32;
pub const SQL_IGNORE: i32 = -6i32;
pub const SQL_IK_ASC: i32 = 1i32;
pub const SQL_IK_DESC: i32 = 2i32;
pub const SQL_IK_NONE: i32 = 0i32;
pub const SQL_INDEX_ALL: u32 = 1u32;
pub const SQL_INDEX_CLUSTERED: u32 = 1u32;
pub const SQL_INDEX_HASHED: u32 = 2u32;
pub const SQL_INDEX_KEYWORDS: u32 = 148u32;
pub const SQL_INDEX_OTHER: u32 = 3u32;
pub const SQL_INDEX_UNIQUE: u32 = 0u32;
pub const SQL_INFO_DRIVER_START: u32 = 1000u32;
pub const SQL_INFO_FIRST: u32 = 0u32;
pub const SQL_INFO_LAST: u32 = 114u32;
pub const SQL_INFO_SCHEMA_VIEWS: u32 = 149u32;
pub const SQL_INFO_SS_FIRST: u32 = 1199u32;
pub const SQL_INFO_SS_MAX_USED: u32 = 1200u32;
pub const SQL_INFO_SS_NETLIB_NAME: u32 = 1199u32;
pub const SQL_INFO_SS_NETLIB_NAMEA: u32 = 1200u32;
pub const SQL_INFO_SS_NETLIB_NAMEW: u32 = 1199u32;
pub const SQL_INITIALLY_DEFERRED: u32 = 5u32;
pub const SQL_INITIALLY_IMMEDIATE: u32 = 6u32;
pub const SQL_INSENSITIVE: u32 = 1u32;
pub const SQL_INSERT_STATEMENT: u32 = 172u32;
pub const SQL_INTEGER: u32 = 4u32;
pub const SQL_INTEGRATED_SECURITY: u32 = 1203u32;
pub const SQL_INTEGRITY: u32 = 73u32;
pub const SQL_INTERVAL: u32 = 10u32;
pub const SQL_INTERVAL_DAY: i32 = -83i32;
pub const SQL_INTERVAL_DAY_TO_HOUR: i32 = -87i32;
pub const SQL_INTERVAL_DAY_TO_MINUTE: i32 = -88i32;
pub const SQL_INTERVAL_DAY_TO_SECOND: i32 = -89i32;
pub const SQL_INTERVAL_HOUR: i32 = -84i32;
pub const SQL_INTERVAL_HOUR_TO_MINUTE: i32 = -90i32;
pub const SQL_INTERVAL_HOUR_TO_SECOND: i32 = -91i32;
pub const SQL_INTERVAL_MINUTE: i32 = -85i32;
pub const SQL_INTERVAL_MINUTE_TO_SECOND: i32 = -92i32;
pub const SQL_INTERVAL_MONTH: i32 = -81i32;
pub const SQL_INTERVAL_SECOND: i32 = -86i32;
#[repr(C)]
pub struct SQL_INTERVAL_STRUCT(i32);
pub const SQL_INTERVAL_YEAR: i32 = -80i32;
pub const SQL_INTERVAL_YEAR_TO_MONTH: i32 = -82i32;
pub const SQL_INVALID_HANDLE: i32 = -2i32;
pub const SQL_ISV_ASSERTIONS: i32 = 1i32;
pub const SQL_ISV_CHARACTER_SETS: i32 = 2i32;
pub const SQL_ISV_CHECK_CONSTRAINTS: i32 = 4i32;
pub const SQL_ISV_COLLATIONS: i32 = 8i32;
pub const SQL_ISV_COLUMNS: i32 = 64i32;
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: i32 = 16i32;
pub const SQL_ISV_COLUMN_PRIVILEGES: i32 = 32i32;
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: i32 = 128i32;
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: i32 = 256i32;
pub const SQL_ISV_DOMAINS: i32 = 1024i32;
pub const SQL_ISV_DOMAIN_CONSTRAINTS: i32 = 512i32;
pub const SQL_ISV_KEY_COLUMN_USAGE: i32 = 2048i32;
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: i32 = 4096i32;
pub const SQL_ISV_SCHEMATA: i32 = 8192i32;
pub const SQL_ISV_SQL_LANGUAGES: i32 = 16384i32;
pub const SQL_ISV_TABLES: i32 = 131072i32;
pub const SQL_ISV_TABLE_CONSTRAINTS: i32 = 32768i32;
pub const SQL_ISV_TABLE_PRIVILEGES: i32 = 65536i32;
pub const SQL_ISV_TRANSLATIONS: i32 = 262144i32;
pub const SQL_ISV_USAGE_PRIVILEGES: i32 = 524288i32;
pub const SQL_ISV_VIEWS: i32 = 4194304i32;
pub const SQL_ISV_VIEW_COLUMN_USAGE: i32 = 1048576i32;
pub const SQL_ISV_VIEW_TABLE_USAGE: i32 = 2097152i32;
pub const SQL_IS_DEFAULT: i32 = 0i32;
pub const SQL_IS_INSERT_LITERALS: i32 = 1i32;
pub const SQL_IS_INSERT_SEARCHED: i32 = 2i32;
pub const SQL_IS_INTEGER: i32 = -6i32;
pub const SQL_IS_OFF: i32 = 0i32;
pub const SQL_IS_ON: i32 = 1i32;
pub const SQL_IS_POINTER: i32 = -4i32;
pub const SQL_IS_SELECT_INTO: i32 = 4i32;
pub const SQL_IS_SMALLINT: i32 = -8i32;
pub const SQL_IS_UINTEGER: i32 = -5i32;
pub const SQL_IS_USMALLINT: i32 = -7i32;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: u32 = 150u32;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: u32 = 151u32;
pub const SQL_KEYSET_SIZE: u32 = 8u32;
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0u32;
pub const SQL_KEYWORDS: u32 = 89u32;
pub const SQL_LCK_EXCLUSIVE: i32 = 2i32;
pub const SQL_LCK_NO_CHANGE: i32 = 1i32;
pub const SQL_LCK_UNLOCK: i32 = 4i32;
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100i32;
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100i32;
pub const SQL_LIKE_ESCAPE_CLAUSE: u32 = 113u32;
pub const SQL_LIKE_ONLY: u32 = 1u32;
pub const SQL_LOCK_EXCLUSIVE: u32 = 1u32;
pub const SQL_LOCK_NO_CHANGE: u32 = 0u32;
pub const SQL_LOCK_TYPES: u32 = 78u32;
pub const SQL_LOCK_UNLOCK: u32 = 2u32;
pub const SQL_LOGIN_TIMEOUT: u32 = 103u32;
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15u32;
pub const SQL_LONGVARBINARY: i32 = -4i32;
pub const SQL_LONGVARCHAR: i32 = -1i32;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u32 = 34u32;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u32 = 97u32;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u32 = 98u32;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u32 = 99u32;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u32 = 100u32;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u32 = 30u32;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u32 = 1u32;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u32 = 31u32;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u32 = 0u32;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u32 = 10005u32;
pub const SQL_MAXIMUM_INDEX_SIZE: u32 = 102u32;
pub const SQL_MAXIMUM_ROW_SIZE: u32 = 104u32;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u32 = 32u32;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u32 = 105u32;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u32 = 106u32;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u32 = 107u32;
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: u32 = 10022u32;
pub const SQL_MAX_BINARY_LITERAL_LEN: u32 = 112u32;
pub const SQL_MAX_CATALOG_NAME_LEN: u32 = 34u32;
pub const SQL_MAX_CHAR_LITERAL_LEN: u32 = 108u32;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u32 = 97u32;
pub const SQL_MAX_COLUMNS_IN_INDEX: u32 = 98u32;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u32 = 99u32;
pub const SQL_MAX_COLUMNS_IN_SELECT: u32 = 100u32;
pub const SQL_MAX_COLUMNS_IN_TABLE: u32 = 101u32;
pub const SQL_MAX_COLUMN_NAME_LEN: u32 = 30u32;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u32 = 1u32;
pub const SQL_MAX_CURSOR_NAME_LEN: u32 = 31u32;
pub const SQL_MAX_DRIVER_CONNECTIONS: u32 = 0u32;
pub const SQL_MAX_DSN_LENGTH: u32 = 32u32;
pub const SQL_MAX_IDENTIFIER_LEN: u32 = 10005u32;
pub const SQL_MAX_INDEX_SIZE: u32 = 102u32;
pub const SQL_MAX_LENGTH: u32 = 3u32;
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0u32;
pub const SQL_MAX_MESSAGE_LENGTH: u32 = 512u32;
pub const SQL_MAX_NUMERIC_LEN: u32 = 16u32;
pub const SQL_MAX_OPTION_STRING_LENGTH: u32 = 256u32;
pub const SQL_MAX_OWNER_NAME_LEN: u32 = 32u32;
pub const SQL_MAX_PROCEDURE_NAME_LEN: u32 = 33u32;
pub const SQL_MAX_QUALIFIER_NAME_LEN: u32 = 34u32;
pub const SQL_MAX_ROWS: u32 = 1u32;
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0u32;
pub const SQL_MAX_ROW_SIZE: u32 = 104u32;
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: u32 = 103u32;
pub const SQL_MAX_SCHEMA_NAME_LEN: u32 = 32u32;
pub const SQL_MAX_SQLSERVERNAME: u32 = 128u32;
pub const SQL_MAX_STATEMENT_LEN: u32 = 105u32;
pub const SQL_MAX_TABLES_IN_SELECT: u32 = 106u32;
pub const SQL_MAX_TABLE_NAME_LEN: u32 = 35u32;
pub const SQL_MAX_USER_NAME_LEN: u32 = 107u32;
pub const SQL_MINUTE: u32 = 5u32;
pub const SQL_MINUTE_TO_SECOND: u32 = 13u32;
pub const SQL_MODE_DEFAULT: u32 = 0u32;
pub const SQL_MODE_READ_ONLY: u32 = 1u32;
pub const SQL_MODE_READ_WRITE: u32 = 0u32;
pub const SQL_MONTH: u32 = 2u32;
pub const SQL_MORE_INFO_NO: i32 = 0i32;
pub const SQL_MORE_INFO_YES: i32 = 1i32;
pub const SQL_MULTIPLE_ACTIVE_TXN: u32 = 37u32;
pub const SQL_MULT_RESULT_SETS: u32 = 36u32;
pub const SQL_NAMED: u32 = 0u32;
pub const SQL_NB_DEFAULT: i32 = 0i32;
pub const SQL_NB_OFF: i32 = 0i32;
pub const SQL_NB_ON: i32 = 1i32;
pub const SQL_NC_END: u32 = 4u32;
pub const SQL_NC_HIGH: u32 = 0u32;
pub const SQL_NC_LOW: u32 = 1u32;
pub const SQL_NC_OFF: i32 = 0i32;
pub const SQL_NC_ON: i32 = 1i32;
pub const SQL_NC_START: u32 = 2u32;
pub const SQL_NEED_DATA: u32 = 99u32;
pub const SQL_NEED_LONG_DATA_LEN: u32 = 111u32;
pub const SQL_NNC_NON_NULL: u32 = 1u32;
pub const SQL_NNC_NULL: u32 = 0u32;
pub const SQL_NONSCROLLABLE: u32 = 0u32;
pub const SQL_NON_NULLABLE_COLUMNS: u32 = 75u32;
pub const SQL_NOSCAN: u32 = 2u32;
pub const SQL_NOSCAN_DEFAULT: u32 = 0u32;
pub const SQL_NOSCAN_OFF: u32 = 0u32;
pub const SQL_NOSCAN_ON: u32 = 1u32;
pub const SQL_NOT_DEFERRABLE: u32 = 7u32;
pub const SQL_NO_ACTION: u32 = 3u32;
pub const SQL_NO_COLUMN_NUMBER: i32 = -1i32;
pub const SQL_NO_DATA: u32 = 100u32;
pub const SQL_NO_DATA_FOUND: u32 = 100u32;
pub const SQL_NO_NULLS: u32 = 0u32;
pub const SQL_NO_ROW_NUMBER: i32 = -1i32;
pub const SQL_NO_TOTAL: i32 = -4i32;
pub const SQL_NTS: i32 = -3i32;
pub const SQL_NTSL: i32 = -3i32;
pub const SQL_NULLABLE: u32 = 1u32;
pub const SQL_NULLABLE_UNKNOWN: u32 = 2u32;
pub const SQL_NULL_COLLATION: u32 = 85u32;
pub const SQL_NULL_DATA: i32 = -1i32;
pub const SQL_NULL_HANDLE: i32 = 0i32;
pub const SQL_NULL_HDBC: u32 = 0u32;
pub const SQL_NULL_HDESC: u32 = 0u32;
pub const SQL_NULL_HENV: u32 = 0u32;
pub const SQL_NULL_HSTMT: u32 = 0u32;
pub const SQL_NUMERIC: u32 = 2u32;
pub const SQL_NUMERIC_FUNCTIONS: u32 = 49u32;
#[repr(C)]
pub struct SQL_NUMERIC_STRUCT(i32);
pub const SQL_NUM_FUNCTIONS: u32 = 23u32;
pub const SQL_OAC_LEVEL1: u32 = 1u32;
pub const SQL_OAC_LEVEL2: u32 = 2u32;
pub const SQL_OAC_NONE: u32 = 0u32;
pub const SQL_ODBC_API_CONFORMANCE: u32 = 9u32;
pub const SQL_ODBC_CURSORS: u32 = 110u32;
pub const SQL_ODBC_INTERFACE_CONFORMANCE: u32 = 152u32;
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: u32 = 12u32;
pub const SQL_ODBC_SQL_CONFORMANCE: u32 = 15u32;
pub const SQL_ODBC_SQL_OPT_IEF: u32 = 73u32;
pub const SQL_ODBC_VER: u32 = 10u32;
pub const SQL_OIC_CORE: u32 = 1u32;
pub const SQL_OIC_LEVEL1: u32 = 2u32;
pub const SQL_OIC_LEVEL2: u32 = 3u32;
pub const SQL_OJ_ALL_COMPARISON_OPS: i32 = 64i32;
pub const SQL_OJ_CAPABILITIES: u32 = 115u32;
pub const SQL_OJ_FULL: i32 = 4i32;
pub const SQL_OJ_INNER: i32 = 32i32;
pub const SQL_OJ_LEFT: i32 = 1i32;
pub const SQL_OJ_NESTED: i32 = 8i32;
pub const SQL_OJ_NOT_ORDERED: i32 = 16i32;
pub const SQL_OJ_RIGHT: i32 = 2i32;
pub const SQL_OPT_TRACE: u32 = 104u32;
pub const SQL_OPT_TRACEFILE: u32 = 105u32;
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0u32;
pub const SQL_OPT_TRACE_OFF: u32 = 0u32;
pub const SQL_OPT_TRACE_ON: u32 = 1u32;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u32 = 90u32;
pub const SQL_OSCC_COMPLIANT: u32 = 1u32;
pub const SQL_OSCC_NOT_COMPLIANT: u32 = 0u32;
pub const SQL_OSC_CORE: u32 = 1u32;
pub const SQL_OSC_EXTENDED: u32 = 2u32;
pub const SQL_OSC_MINIMUM: u32 = 0u32;
pub const SQL_OUTER_JOINS: u32 = 38u32;
pub const SQL_OUTER_JOIN_CAPABILITIES: u32 = 115u32;
pub const SQL_OU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_OU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_OU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_OU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_OU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_OV_ODBC2: u32 = 2u32;
pub const SQL_OV_ODBC3: u32 = 3u32;
pub const SQL_OV_ODBC3_80: u32 = 380u32;
pub const SQL_OWNER_TERM: u32 = 39u32;
pub const SQL_OWNER_USAGE: u32 = 91u32;
pub const SQL_PACKET_SIZE: u32 = 112u32;
pub const SQL_PARAM_ARRAY_ROW_COUNTS: u32 = 153u32;
pub const SQL_PARAM_ARRAY_SELECTS: u32 = 154u32;
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0u32;
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0u32;
pub const SQL_PARAM_DATA_AVAILABLE: u32 = 101u32;
pub const SQL_PARAM_DIAG_UNAVAILABLE: u32 = 1u32;
pub const SQL_PARAM_ERROR: u32 = 5u32;
pub const SQL_PARAM_IGNORE: u32 = 1u32;
pub const SQL_PARAM_INPUT: u32 = 1u32;
pub const SQL_PARAM_INPUT_OUTPUT: u32 = 2u32;
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: u32 = 8u32;
pub const SQL_PARAM_OUTPUT: u32 = 4u32;
pub const SQL_PARAM_OUTPUT_STREAM: u32 = 16u32;
pub const SQL_PARAM_PROCEED: u32 = 0u32;
pub const SQL_PARAM_SUCCESS: u32 = 0u32;
pub const SQL_PARAM_SUCCESS_WITH_INFO: u32 = 6u32;
pub const SQL_PARAM_TYPE_UNKNOWN: u32 = 0u32;
pub const SQL_PARAM_UNUSED: u32 = 7u32;
pub const SQL_PARC_BATCH: u32 = 1u32;
pub const SQL_PARC_NO_BATCH: u32 = 2u32;
pub const SQL_PAS_BATCH: u32 = 1u32;
pub const SQL_PAS_NO_BATCH: u32 = 2u32;
pub const SQL_PAS_NO_SELECT: u32 = 3u32;
pub const SQL_PC_DEFAULT: i32 = 0i32;
pub const SQL_PC_NON_PSEUDO: u32 = 1u32;
pub const SQL_PC_NOT_PSEUDO: u32 = 1u32;
pub const SQL_PC_OFF: i32 = 0i32;
pub const SQL_PC_ON: i32 = 1i32;
pub const SQL_PC_PSEUDO: u32 = 2u32;
pub const SQL_PC_UNKNOWN: u32 = 0u32;
pub const SQL_PERF_START: u32 = 1u32;
pub const SQL_PERF_STOP: u32 = 2u32;
pub const SQL_POSITION: u32 = 0u32;
pub const SQL_POSITIONED_STATEMENTS: u32 = 80u32;
pub const SQL_POS_ADD: i32 = 16i32;
pub const SQL_POS_DELETE: i32 = 8i32;
pub const SQL_POS_OPERATIONS: u32 = 79u32;
pub const SQL_POS_POSITION: i32 = 1i32;
pub const SQL_POS_REFRESH: i32 = 2i32;
pub const SQL_POS_UPDATE: i32 = 4i32;
pub const SQL_PRED_BASIC: u32 = 2u32;
pub const SQL_PRED_CHAR: u32 = 1u32;
pub const SQL_PRED_NONE: u32 = 0u32;
pub const SQL_PRED_SEARCHABLE: u32 = 3u32;
pub const SQL_PRESERVE_CURSORS: u32 = 1204u32;
pub const SQL_PROCEDURES: u32 = 21u32;
pub const SQL_PROCEDURE_TERM: u32 = 40u32;
pub const SQL_PS_POSITIONED_DELETE: i32 = 1i32;
pub const SQL_PS_POSITIONED_UPDATE: i32 = 2i32;
pub const SQL_PS_SELECT_FOR_UPDATE: i32 = 4i32;
pub const SQL_PT_FUNCTION: u32 = 2u32;
pub const SQL_PT_PROCEDURE: u32 = 1u32;
pub const SQL_PT_UNKNOWN: u32 = 0u32;
pub const SQL_QI_DEFAULT: i32 = 1i32;
pub const SQL_QI_OFF: i32 = 0i32;
pub const SQL_QI_ON: i32 = 1i32;
pub const SQL_QL_END: u32 = 2u32;
pub const SQL_QL_START: u32 = 1u32;
pub const SQL_QUALIFIER_LOCATION: u32 = 114u32;
pub const SQL_QUALIFIER_NAME_SEPARATOR: u32 = 41u32;
pub const SQL_QUALIFIER_TERM: u32 = 42u32;
pub const SQL_QUALIFIER_USAGE: u32 = 92u32;
pub const SQL_QUERY_TIMEOUT: u32 = 0u32;
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0u32;
pub const SQL_QUICK: u32 = 0u32;
pub const SQL_QUIET_MODE: u32 = 111u32;
pub const SQL_QUOTED_IDENTIFIER_CASE: u32 = 93u32;
pub const SQL_QU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_QU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_QU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_QU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_QU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_RD_DEFAULT: u32 = 1u32;
pub const SQL_RD_OFF: u32 = 0u32;
pub const SQL_RD_ON: u32 = 1u32;
pub const SQL_REAL: u32 = 7u32;
pub const SQL_REFRESH: u32 = 1u32;
pub const SQL_REMOTE_PWD: u32 = 1201u32;
pub const SQL_RESET_CONNECTION_YES: u32 = 1u32;
pub const SQL_RESET_PARAMS: u32 = 3u32;
pub const SQL_RESET_YES: i32 = 1i32;
pub const SQL_RESTRICT: u32 = 1u32;
pub const SQL_RESULT_COL: u32 = 3u32;
pub const SQL_RETRIEVE_DATA: u32 = 11u32;
pub const SQL_RETURN_VALUE: u32 = 5u32;
pub const SQL_RE_DEFAULT: i32 = 0i32;
pub const SQL_RE_OFF: i32 = 0i32;
pub const SQL_RE_ON: i32 = 1i32;
pub const SQL_ROLLBACK: u32 = 1u32;
pub const SQL_ROWSET_SIZE: u32 = 9u32;
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1u32;
pub const SQL_ROWVER: u32 = 2u32;
pub const SQL_ROW_ADDED: u32 = 4u32;
pub const SQL_ROW_DELETED: u32 = 1u32;
pub const SQL_ROW_ERROR: u32 = 5u32;
pub const SQL_ROW_IDENTIFIER: u32 = 1u32;
pub const SQL_ROW_IGNORE: u32 = 1u32;
pub const SQL_ROW_NOROW: u32 = 3u32;
pub const SQL_ROW_NUMBER: u32 = 14u32;
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2i32;
pub const SQL_ROW_PROCEED: u32 = 0u32;
pub const SQL_ROW_SUCCESS: u32 = 0u32;
pub const SQL_ROW_SUCCESS_WITH_INFO: u32 = 6u32;
pub const SQL_ROW_UPDATED: u32 = 2u32;
pub const SQL_ROW_UPDATES: u32 = 11u32;
pub const SQL_SCCO_LOCK: i32 = 2i32;
pub const SQL_SCCO_OPT_ROWVER: i32 = 4i32;
pub const SQL_SCCO_OPT_TIMESTAMP: i32 = 4i32;
pub const SQL_SCCO_OPT_VALUES: i32 = 8i32;
pub const SQL_SCCO_READ_ONLY: i32 = 1i32;
pub const SQL_SCC_ISO92_CLI: i32 = 2i32;
pub const SQL_SCC_XOPEN_CLI_VERSION1: i32 = 1i32;
pub const SQL_SCHEMA_TERM: u32 = 39u32;
pub const SQL_SCHEMA_USAGE: u32 = 91u32;
pub const SQL_SCOPE_CURROW: u32 = 0u32;
pub const SQL_SCOPE_SESSION: u32 = 2u32;
pub const SQL_SCOPE_TRANSACTION: u32 = 1u32;
pub const SQL_SCROLLABLE: u32 = 1u32;
pub const SQL_SCROLL_CONCURRENCY: u32 = 43u32;
pub const SQL_SCROLL_DYNAMIC: i32 = -2i32;
pub const SQL_SCROLL_FORWARD_ONLY: i32 = 0i32;
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1i32;
pub const SQL_SCROLL_OPTIONS: u32 = 44u32;
pub const SQL_SCROLL_STATIC: i32 = -3i32;
pub const SQL_SC_FIPS127_2_TRANSITIONAL: i32 = 2i32;
pub const SQL_SC_NON_UNIQUE: u32 = 0u32;
pub const SQL_SC_SQL92_ENTRY: i32 = 1i32;
pub const SQL_SC_SQL92_FULL: i32 = 8i32;
pub const SQL_SC_SQL92_INTERMEDIATE: i32 = 4i32;
pub const SQL_SC_TRY_UNIQUE: u32 = 1u32;
pub const SQL_SC_UNIQUE: u32 = 2u32;
pub const SQL_SDF_CURRENT_DATE: i32 = 1i32;
pub const SQL_SDF_CURRENT_TIME: i32 = 2i32;
pub const SQL_SDF_CURRENT_TIMESTAMP: i32 = 4i32;
pub const SQL_SEARCHABLE: u32 = 3u32;
pub const SQL_SEARCH_PATTERN_ESCAPE: u32 = 14u32;
pub const SQL_SECOND: u32 = 6u32;
pub const SQL_SENSITIVE: u32 = 2u32;
pub const SQL_SERVER_NAME: u32 = 13u32;
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1i32;
pub const SQL_SETPOS_MAX_LOCK_VALUE: u32 = 2u32;
pub const SQL_SETPOS_MAX_OPTION_VALUE: u32 = 4u32;
pub const SQL_SET_DEFAULT: u32 = 4u32;
pub const SQL_SET_NULL: u32 = 2u32;
pub const SQL_SFKD_CASCADE: i32 = 1i32;
pub const SQL_SFKD_NO_ACTION: i32 = 2i32;
pub const SQL_SFKD_SET_DEFAULT: i32 = 4i32;
pub const SQL_SFKD_SET_NULL: i32 = 8i32;
pub const SQL_SFKU_CASCADE: i32 = 1i32;
pub const SQL_SFKU_NO_ACTION: i32 = 2i32;
pub const SQL_SFKU_SET_DEFAULT: i32 = 4i32;
pub const SQL_SFKU_SET_NULL: i32 = 8i32;
pub const SQL_SG_DELETE_TABLE: i32 = 32i32;
pub const SQL_SG_INSERT_COLUMN: i32 = 128i32;
pub const SQL_SG_INSERT_TABLE: i32 = 64i32;
pub const SQL_SG_REFERENCES_COLUMN: i32 = 512i32;
pub const SQL_SG_REFERENCES_TABLE: i32 = 256i32;
pub const SQL_SG_SELECT_TABLE: i32 = 1024i32;
pub const SQL_SG_UPDATE_COLUMN: i32 = 4096i32;
pub const SQL_SG_UPDATE_TABLE: i32 = 2048i32;
pub const SQL_SG_USAGE_ON_CHARACTER_SET: i32 = 2i32;
pub const SQL_SG_USAGE_ON_COLLATION: i32 = 4i32;
pub const SQL_SG_USAGE_ON_DOMAIN: i32 = 1i32;
pub const SQL_SG_USAGE_ON_TRANSLATION: i32 = 8i32;
pub const SQL_SG_WITH_GRANT_OPTION: i32 = 16i32;
pub const SQL_SIGNED_OFFSET: i32 = -20i32;
pub const SQL_SIMULATE_CURSOR: u32 = 10u32;
pub const SQL_SMALLINT: u32 = 5u32;
pub const SQL_SNVF_BIT_LENGTH: i32 = 1i32;
pub const SQL_SNVF_CHARACTER_LENGTH: i32 = 4i32;
pub const SQL_SNVF_CHAR_LENGTH: i32 = 2i32;
pub const SQL_SNVF_EXTRACT: i32 = 8i32;
pub const SQL_SNVF_OCTET_LENGTH: i32 = 16i32;
pub const SQL_SNVF_POSITION: i32 = 32i32;
pub const SQL_SOPT_SS_BASE: u32 = 1225u32;
pub const SQL_SOPT_SS_CURRENT_COMMAND: u32 = 1226u32;
pub const SQL_SOPT_SS_CURSOR_OPTIONS: u32 = 1230u32;
pub const SQL_SOPT_SS_DEFER_PREPARE: u32 = 1232u32;
pub const SQL_SOPT_SS_HIDDEN_COLUMNS: u32 = 1227u32;
pub const SQL_SOPT_SS_MAX_USED: u32 = 1232u32;
pub const SQL_SOPT_SS_NOBROWSETABLE: u32 = 1228u32;
pub const SQL_SOPT_SS_NOCOUNT_STATUS: u32 = 1231u32;
pub const SQL_SOPT_SS_REGIONALIZE: u32 = 1229u32;
pub const SQL_SOPT_SS_TEXTPTR_LOGGING: u32 = 1225u32;
pub const SQL_SO_DYNAMIC: i32 = 4i32;
pub const SQL_SO_FORWARD_ONLY: i32 = 1i32;
pub const SQL_SO_KEYSET_DRIVEN: i32 = 2i32;
pub const SQL_SO_MIXED: i32 = 8i32;
pub const SQL_SO_STATIC: i32 = 16i32;
pub const SQL_SPECIAL_CHARACTERS: u32 = 94u32;
pub const SQL_SPEC_MAJOR: u32 = 3u32;
pub const SQL_SPEC_MINOR: u32 = 80u32;
pub const SQL_SP_BETWEEN: i32 = 2048i32;
pub const SQL_SP_COMPARISON: i32 = 4096i32;
pub const SQL_SP_EXISTS: i32 = 1i32;
pub const SQL_SP_IN: i32 = 1024i32;
pub const SQL_SP_ISNOTNULL: i32 = 2i32;
pub const SQL_SP_ISNULL: i32 = 4i32;
pub const SQL_SP_LIKE: i32 = 512i32;
pub const SQL_SP_MATCH_FULL: i32 = 8i32;
pub const SQL_SP_MATCH_PARTIAL: i32 = 16i32;
pub const SQL_SP_MATCH_UNIQUE_FULL: i32 = 32i32;
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: i32 = 64i32;
pub const SQL_SP_OVERLAPS: i32 = 128i32;
pub const SQL_SP_QUANTIFIED_COMPARISON: i32 = 8192i32;
pub const SQL_SP_UNIQUE: i32 = 256i32;
pub const SQL_SQL92_DATETIME_FUNCTIONS: u32 = 155u32;
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: u32 = 156u32;
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: u32 = 157u32;
pub const SQL_SQL92_GRANT: u32 = 158u32;
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: u32 = 159u32;
pub const SQL_SQL92_PREDICATES: u32 = 160u32;
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: u32 = 161u32;
pub const SQL_SQL92_REVOKE: u32 = 162u32;
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: u32 = 163u32;
pub const SQL_SQL92_STRING_FUNCTIONS: u32 = 164u32;
pub const SQL_SQL92_VALUE_EXPRESSIONS: u32 = 165u32;
pub const SQL_SQLSTATE_SIZE: u32 = 5u32;
pub const SQL_SQLSTATE_SIZEW: u32 = 10u32;
pub const SQL_SQL_CONFORMANCE: u32 = 118u32;
pub const SQL_SQ_COMPARISON: i32 = 1i32;
pub const SQL_SQ_CORRELATED_SUBQUERIES: i32 = 16i32;
pub const SQL_SQ_EXISTS: i32 = 2i32;
pub const SQL_SQ_IN: i32 = 4i32;
pub const SQL_SQ_QUANTIFIED: i32 = 8i32;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: i32 = 1i32;
pub const SQL_SRJO_CROSS_JOIN: i32 = 2i32;
pub const SQL_SRJO_EXCEPT_JOIN: i32 = 4i32;
pub const SQL_SRJO_FULL_OUTER_JOIN: i32 = 8i32;
pub const SQL_SRJO_INNER_JOIN: i32 = 16i32;
pub const SQL_SRJO_INTERSECT_JOIN: i32 = 32i32;
pub const SQL_SRJO_LEFT_OUTER_JOIN: i32 = 64i32;
pub const SQL_SRJO_NATURAL_JOIN: i32 = 128i32;
pub const SQL_SRJO_RIGHT_OUTER_JOIN: i32 = 256i32;
pub const SQL_SRJO_UNION_JOIN: i32 = 512i32;
pub const SQL_SRVC_DEFAULT: i32 = 4i32;
pub const SQL_SRVC_NULL: i32 = 2i32;
pub const SQL_SRVC_ROW_SUBQUERY: i32 = 8i32;
pub const SQL_SRVC_VALUE_EXPRESSION: i32 = 1i32;
pub const SQL_SR_CASCADE: i32 = 32i32;
pub const SQL_SR_DELETE_TABLE: i32 = 128i32;
pub const SQL_SR_GRANT_OPTION_FOR: i32 = 16i32;
pub const SQL_SR_INSERT_COLUMN: i32 = 512i32;
pub const SQL_SR_INSERT_TABLE: i32 = 256i32;
pub const SQL_SR_REFERENCES_COLUMN: i32 = 2048i32;
pub const SQL_SR_REFERENCES_TABLE: i32 = 1024i32;
pub const SQL_SR_RESTRICT: i32 = 64i32;
pub const SQL_SR_SELECT_TABLE: i32 = 4096i32;
pub const SQL_SR_UPDATE_COLUMN: i32 = 16384i32;
pub const SQL_SR_UPDATE_TABLE: i32 = 8192i32;
pub const SQL_SR_USAGE_ON_CHARACTER_SET: i32 = 2i32;
pub const SQL_SR_USAGE_ON_COLLATION: i32 = 4i32;
pub const SQL_SR_USAGE_ON_DOMAIN: i32 = 1i32;
pub const SQL_SR_USAGE_ON_TRANSLATION: i32 = 8i32;
pub const SQL_SSF_CONVERT: i32 = 1i32;
pub const SQL_SSF_LOWER: i32 = 2i32;
pub const SQL_SSF_SUBSTRING: i32 = 8i32;
pub const SQL_SSF_TRANSLATE: i32 = 16i32;
pub const SQL_SSF_TRIM_BOTH: i32 = 32i32;
pub const SQL_SSF_TRIM_LEADING: i32 = 64i32;
pub const SQL_SSF_TRIM_TRAILING: i32 = 128i32;
pub const SQL_SSF_UPPER: i32 = 4i32;
pub const SQL_SS_ADDITIONS: i32 = 1i32;
pub const SQL_SS_DELETIONS: i32 = 2i32;
pub const SQL_SS_QI_DEFAULT: u32 = 30000u32;
pub const SQL_SS_UPDATES: i32 = 4i32;
pub const SQL_SS_VARIANT: i32 = -150i32;
pub const SQL_STANDARD_CLI_CONFORMANCE: u32 = 166u32;
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: u32 = 167u32;
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: u32 = 168u32;
pub const SQL_STATIC_SENSITIVITY: u32 = 83u32;
pub const SQL_STILL_EXECUTING: u32 = 2u32;
pub const SQL_STMT_OPT_MAX: u32 = 14u32;
pub const SQL_STMT_OPT_MIN: u32 = 0u32;
pub const SQL_STRING_FUNCTIONS: u32 = 50u32;
pub const SQL_SUBQUERIES: u32 = 95u32;
pub const SQL_SUCCESS: u32 = 0u32;
pub const SQL_SUCCESS_WITH_INFO: u32 = 1u32;
pub const SQL_SU_DML_STATEMENTS: i32 = 1i32;
pub const SQL_SU_INDEX_DEFINITION: i32 = 8i32;
pub const SQL_SU_PRIVILEGE_DEFINITION: i32 = 16i32;
pub const SQL_SU_PROCEDURE_INVOCATION: i32 = 2i32;
pub const SQL_SU_TABLE_DEFINITION: i32 = 4i32;
pub const SQL_SVE_CASE: i32 = 1i32;
pub const SQL_SVE_CAST: i32 = 2i32;
pub const SQL_SVE_COALESCE: i32 = 4i32;
pub const SQL_SVE_NULLIF: i32 = 8i32;
pub const SQL_SYSTEM_FUNCTIONS: u32 = 51u32;
pub const SQL_TABLE_STAT: u32 = 0u32;
pub const SQL_TABLE_TERM: u32 = 45u32;
pub const SQL_TC_ALL: u32 = 2u32;
pub const SQL_TC_DDL_COMMIT: u32 = 3u32;
pub const SQL_TC_DDL_IGNORE: u32 = 4u32;
pub const SQL_TC_DML: u32 = 1u32;
pub const SQL_TC_NONE: u32 = 0u32;
pub const SQL_TEXTPTR_LOGGING: u32 = 1225u32;
pub const SQL_TIME: u32 = 10u32;
pub const SQL_TIMEDATE_ADD_INTERVALS: u32 = 109u32;
pub const SQL_TIMEDATE_DIFF_INTERVALS: u32 = 110u32;
pub const SQL_TIMEDATE_FUNCTIONS: u32 = 52u32;
pub const SQL_TIMESTAMP: u32 = 11u32;
pub const SQL_TIMESTAMP_LEN: u32 = 19u32;
pub const SQL_TIME_LEN: u32 = 8u32;
pub const SQL_TINYINT: i32 = -6i32;
pub const SQL_TL_DEFAULT: i32 = 1i32;
pub const SQL_TL_OFF: i32 = 0i32;
pub const SQL_TL_ON: i32 = 1i32;
pub const SQL_TRANSACTION_CAPABLE: u32 = 46u32;
pub const SQL_TRANSACTION_ISOLATION_OPTION: u32 = 72u32;
pub const SQL_TRANSACTION_READ_COMMITTED: i32 = 2i32;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: i32 = 1i32;
pub const SQL_TRANSACTION_REPEATABLE_READ: i32 = 4i32;
pub const SQL_TRANSACTION_SERIALIZABLE: i32 = 8i32;
pub const SQL_TRANSLATE_DLL: u32 = 106u32;
pub const SQL_TRANSLATE_OPTION: u32 = 107u32;
pub const SQL_TRUE: u32 = 1u32;
pub const SQL_TXN_CAPABLE: u32 = 46u32;
pub const SQL_TXN_ISOLATION: u32 = 108u32;
pub const SQL_TXN_ISOLATION_OPTION: u32 = 72u32;
pub const SQL_TXN_READ_COMMITTED: i32 = 2i32;
pub const SQL_TXN_READ_UNCOMMITTED: i32 = 1i32;
pub const SQL_TXN_REPEATABLE_READ: i32 = 4i32;
pub const SQL_TXN_SERIALIZABLE: i32 = 8i32;
pub const SQL_TXN_VERSIONING: i32 = 16i32;
pub const SQL_TYPE_DATE: u32 = 91u32;
pub const SQL_TYPE_DRIVER_END: i32 = -97i32;
pub const SQL_TYPE_DRIVER_START: i32 = -80i32;
pub const SQL_TYPE_MAX: u32 = 12u32;
pub const SQL_TYPE_MIN: i32 = -7i32;
pub const SQL_TYPE_NULL: u32 = 0u32;
pub const SQL_TYPE_TIME: u32 = 92u32;
pub const SQL_TYPE_TIMESTAMP: u32 = 93u32;
pub const SQL_UB_DEFAULT: u32 = 0u32;
pub const SQL_UB_FIXED: u32 = 1u32;
pub const SQL_UB_OFF: u32 = 0u32;
pub const SQL_UB_ON: u32 = 1u32;
pub const SQL_UB_VARIABLE: u32 = 2u32;
pub const SQL_UNBIND: u32 = 2u32;
pub const SQL_UNICODE: i32 = -95i32;
pub const SQL_UNICODE_CHAR: i32 = -95i32;
pub const SQL_UNICODE_LONGVARCHAR: i32 = -97i32;
pub const SQL_UNICODE_VARCHAR: i32 = -96i32;
pub const SQL_UNION: u32 = 96u32;
pub const SQL_UNION_STATEMENT: u32 = 96u32;
pub const SQL_UNKNOWN_TYPE: u32 = 0u32;
pub const SQL_UNNAMED: u32 = 1u32;
pub const SQL_UNSEARCHABLE: u32 = 0u32;
pub const SQL_UNSIGNED_OFFSET: i32 = -22i32;
pub const SQL_UNSPECIFIED: u32 = 0u32;
pub const SQL_UPDATE: u32 = 2u32;
pub const SQL_UPDATE_BY_BOOKMARK: u32 = 5u32;
pub const SQL_UP_DEFAULT: i32 = 1i32;
pub const SQL_UP_OFF: i32 = 0i32;
pub const SQL_UP_ON: i32 = 1i32;
pub const SQL_UP_ON_DROP: i32 = 2i32;
pub const SQL_USER_NAME: u32 = 47u32;
pub const SQL_USE_BOOKMARKS: u32 = 12u32;
pub const SQL_USE_PROCEDURE_FOR_PREPARE: u32 = 1202u32;
pub const SQL_US_UNION: i32 = 1i32;
pub const SQL_US_UNION_ALL: i32 = 2i32;
pub const SQL_U_UNION: i32 = 1i32;
pub const SQL_U_UNION_ALL: i32 = 2i32;
pub const SQL_VARBINARY: i32 = -3i32;
pub const SQL_VARCHAR: u32 = 12u32;
pub const SQL_VARLEN_DATA: i32 = -10i32;
pub const SQL_WARN_NO: i32 = 0i32;
pub const SQL_WARN_YES: i32 = 1i32;
pub const SQL_WCHAR: i32 = -8i32;
pub const SQL_WLONGVARCHAR: i32 = -10i32;
pub const SQL_WVARCHAR: i32 = -9i32;
pub const SQL_XL_DEFAULT: i32 = 1i32;
pub const SQL_XL_OFF: i32 = 0i32;
pub const SQL_XL_ON: i32 = 1i32;
pub const SQL_XOPEN_CLI_YEAR: u32 = 10000u32;
pub const SQL_YEAR: u32 = 1u32;
pub const SQL_YEAR_TO_MONTH: u32 = 7u32;
pub const SQLudtBINARY: u32 = 3u32;
pub const SQLudtBIT: u32 = 16u32;
pub const SQLudtBITN: u32 = 0u32;
pub const SQLudtCHAR: u32 = 1u32;
pub const SQLudtDATETIM4: u32 = 22u32;
pub const SQLudtDATETIME: u32 = 12u32;
pub const SQLudtDATETIMN: u32 = 15u32;
pub const SQLudtDECML: u32 = 24u32;
pub const SQLudtDECMLN: u32 = 26u32;
pub const SQLudtFLT4: u32 = 23u32;
pub const SQLudtFLT8: u32 = 8u32;
pub const SQLudtFLTN: u32 = 14u32;
pub const SQLudtIMAGE: u32 = 20u32;
pub const SQLudtINT1: u32 = 5u32;
pub const SQLudtINT2: u32 = 6u32;
pub const SQLudtINT4: u32 = 7u32;
pub const SQLudtINTN: u32 = 13u32;
pub const SQLudtMONEY: u32 = 11u32;
pub const SQLudtMONEY4: u32 = 21u32;
pub const SQLudtMONEYN: u32 = 17u32;
pub const SQLudtNUM: u32 = 10u32;
pub const SQLudtNUMN: u32 = 25u32;
pub const SQLudtSYSNAME: u32 = 18u32;
pub const SQLudtTEXT: u32 = 19u32;
pub const SQLudtTIMESTAMP: u32 = 80u32;
pub const SQLudtUNIQUEIDENTIFIER: u32 = 0u32;
pub const SQLudtVARBINARY: u32 = 4u32;
pub const SQLudtVARCHAR: u32 = 2u32;
pub const SRCH_SCHEMA_CACHE_E_UNEXPECTED: i32 = -2147208447i32;
pub const SSPROPVAL_COMMANDTYPE_BULKLOAD: u32 = 22u32;
pub const SSPROPVAL_COMMANDTYPE_REGULAR: u32 = 21u32;
pub const SSPROPVAL_USEPROCFORPREP_OFF: u32 = 0u32;
pub const SSPROPVAL_USEPROCFORPREP_ON: u32 = 1u32;
pub const SSPROPVAL_USEPROCFORPREP_ON_DROP: u32 = 2u32;
pub const SSPROP_ALLOWNATIVEVARIANT: u32 = 3u32;
pub const SSPROP_AUTH_REPL_SERVER_NAME: u32 = 14u32;
pub const SSPROP_CHARACTERSET: u32 = 5u32;
pub const SSPROP_COLUMNLEVELCOLLATION: u32 = 4u32;
pub const SSPROP_COL_COLLATIONNAME: u32 = 14u32;
pub const SSPROP_CURRENTCOLLATION: u32 = 7u32;
pub const SSPROP_CURSORAUTOFETCH: u32 = 12u32;
pub const SSPROP_DEFERPREPARE: u32 = 13u32;
pub const SSPROP_ENABLEFASTLOAD: u32 = 2u32;
pub const SSPROP_FASTLOADKEEPIDENTITY: u32 = 11u32;
pub const SSPROP_FASTLOADKEEPNULLS: u32 = 10u32;
pub const SSPROP_FASTLOADOPTIONS: u32 = 9u32;
pub const SSPROP_INIT_APPNAME: u32 = 10u32;
pub const SSPROP_INIT_AUTOTRANSLATE: u32 = 8u32;
pub const SSPROP_INIT_CURRENTLANGUAGE: u32 = 4u32;
pub const SSPROP_INIT_ENCRYPT: u32 = 13u32;
pub const SSPROP_INIT_FILENAME: u32 = 12u32;
pub const SSPROP_INIT_NETWORKADDRESS: u32 = 5u32;
pub const SSPROP_INIT_NETWORKLIBRARY: u32 = 6u32;
pub const SSPROP_INIT_PACKETSIZE: u32 = 9u32;
pub const SSPROP_INIT_TAGCOLUMNCOLLATION: u32 = 15u32;
pub const SSPROP_INIT_USEPROCFORPREP: u32 = 7u32;
pub const SSPROP_INIT_WSID: u32 = 11u32;
pub const SSPROP_IRowsetFastLoad: u32 = 14u32;
pub const SSPROP_MAXBLOBLENGTH: u32 = 8u32;
pub const SSPROP_QUOTEDCATALOGNAMES: u32 = 2u32;
pub const SSPROP_SORTORDER: u32 = 6u32;
pub const SSPROP_SQLXMLXPROGID: u32 = 4u32;
pub const SSPROP_STREAM_BASEPATH: u32 = 17u32;
pub const SSPROP_STREAM_COMMANDTYPE: u32 = 18u32;
pub const SSPROP_STREAM_CONTENTTYPE: u32 = 23u32;
pub const SSPROP_STREAM_FLAGS: u32 = 20u32;
pub const SSPROP_STREAM_MAPPINGSCHEMA: u32 = 15u32;
pub const SSPROP_STREAM_XMLROOT: u32 = 19u32;
pub const SSPROP_STREAM_XSL: u32 = 16u32;
pub const SSPROP_UNICODECOMPARISONSTYLE: u32 = 3u32;
pub const SSPROP_UNICODELCID: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SSVARIANT(i32);
pub const STD_BOOKMARKLENGTH: u32 = 1u32;
pub const STGM_COLLECTION: i32 = 8192i32;
pub const STGM_OPEN: i32 = -2147483648i32;
pub const STGM_OUTPUT: i32 = 32768i32;
pub const STGM_RECURSIVE: i32 = 16777216i32;
pub const STGM_STRICTOPEN: i32 = 1073741824i32;
pub const STREAM_FLAGS_DISALLOW_ABSOLUTE_PATH: u32 = 2u32;
pub const STREAM_FLAGS_DISALLOW_QUERY: u32 = 4u32;
pub const STREAM_FLAGS_DISALLOW_UPDATEGRAMS: u32 = 64u32;
pub const STREAM_FLAGS_DISALLOW_URL: u32 = 1u32;
pub const STREAM_FLAGS_DONTCACHEMAPPINGSCHEMA: u32 = 8u32;
pub const STREAM_FLAGS_DONTCACHETEMPLATE: u32 = 16u32;
pub const STREAM_FLAGS_DONTCACHEXSL: u32 = 32u32;
pub const STREAM_FLAGS_RESERVED: u32 = 4294901760u32;
#[repr(transparent)]
pub struct STRUCTURED_QUERY_MULTIOPTION(pub i32);
pub const SQMO_VIRTUAL_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(0i32);
pub const SQMO_DEFAULT_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(1i32);
pub const SQMO_GENERATOR_FOR_TYPE: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(2i32);
pub const SQMO_MAP_PROPERTY: STRUCTURED_QUERY_MULTIOPTION = STRUCTURED_QUERY_MULTIOPTION(3i32);
#[repr(transparent)]
pub struct STRUCTURED_QUERY_PARSE_ERROR(pub i32);
pub const SQPE_NONE: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(0i32);
pub const SQPE_EXTRA_OPENING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(1i32);
pub const SQPE_EXTRA_CLOSING_PARENTHESIS: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(2i32);
pub const SQPE_IGNORED_MODIFIER: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(3i32);
pub const SQPE_IGNORED_CONNECTOR: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(4i32);
pub const SQPE_IGNORED_KEYWORD: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(5i32);
pub const SQPE_UNHANDLED: STRUCTURED_QUERY_PARSE_ERROR = STRUCTURED_QUERY_PARSE_ERROR(6i32);
#[repr(transparent)]
pub struct STRUCTURED_QUERY_RESOLVE_OPTION(pub u32);
pub const SQRO_DEFAULT: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(0u32);
pub const SQRO_DONT_RESOLVE_DATETIME: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(1u32);
pub const SQRO_ALWAYS_ONE_INTERVAL: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(2u32);
pub const SQRO_DONT_SIMPLIFY_CONDITION_TREES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(4u32);
pub const SQRO_DONT_MAP_RELATIONS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(8u32);
pub const SQRO_DONT_RESOLVE_RANGES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(16u32);
pub const SQRO_DONT_REMOVE_UNRESTRICTED_KEYWORDS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(32u32);
pub const SQRO_DONT_SPLIT_WORDS: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(64u32);
pub const SQRO_IGNORE_PHRASE_ORDER: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(128u32);
pub const SQRO_ADD_VALUE_TYPE_FOR_PLAIN_VALUES: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(256u32);
pub const SQRO_ADD_ROBUST_ITEM_NAME: STRUCTURED_QUERY_RESOLVE_OPTION = STRUCTURED_QUERY_RESOLVE_OPTION(512u32);
#[repr(transparent)]
pub struct STRUCTURED_QUERY_SINGLE_OPTION(pub i32);
pub const SQSO_SCHEMA: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(0i32);
pub const SQSO_LOCALE_WORD_BREAKING: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(1i32);
pub const SQSO_WORD_BREAKER: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(2i32);
pub const SQSO_NATURAL_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(3i32);
pub const SQSO_AUTOMATIC_WILDCARD: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(4i32);
pub const SQSO_TRACE_LEVEL: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(5i32);
pub const SQSO_LANGUAGE_KEYWORDS: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(6i32);
pub const SQSO_SYNTAX: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(7i32);
pub const SQSO_TIME_ZONE: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(8i32);
pub const SQSO_IMPLICIT_CONNECTOR: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(9i32);
pub const SQSO_CONNECTOR_CASE: STRUCTURED_QUERY_SINGLE_OPTION = STRUCTURED_QUERY_SINGLE_OPTION(10i32);
#[repr(transparent)]
pub struct STRUCTURED_QUERY_SYNTAX(pub i32);
pub const SQS_NO_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(0i32);
pub const SQS_ADVANCED_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(1i32);
pub const SQS_NATURAL_QUERY_SYNTAX: STRUCTURED_QUERY_SYNTAX = STRUCTURED_QUERY_SYNTAX(2i32);
pub const STS_ABORTXMLPARSE: i32 = -2147211756i32;
pub const STS_WS_ERROR: i32 = -2147211754i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SUBSCRIPTIONINFO(i32);
#[repr(transparent)]
pub struct SUBSCRIPTIONINFOFLAGS(pub i32);
pub const SUBSINFO_SCHEDULE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(1i32);
pub const SUBSINFO_RECURSE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(2i32);
pub const SUBSINFO_WEBCRAWL: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(4i32);
pub const SUBSINFO_MAILNOT: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(8i32);
pub const SUBSINFO_MAXSIZEKB: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(16i32);
pub const SUBSINFO_USER: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(32i32);
pub const SUBSINFO_PASSWORD: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(64i32);
pub const SUBSINFO_TASKFLAGS: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(256i32);
pub const SUBSINFO_GLEAM: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(512i32);
pub const SUBSINFO_CHANGESONLY: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(1024i32);
pub const SUBSINFO_CHANNELFLAGS: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(2048i32);
pub const SUBSINFO_FRIENDLYNAME: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(8192i32);
pub const SUBSINFO_NEEDPASSWORD: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(16384i32);
pub const SUBSINFO_TYPE: SUBSCRIPTIONINFOFLAGS = SUBSCRIPTIONINFOFLAGS(32768i32);
#[repr(C)]
pub struct SUBSCRIPTIONITEMINFO(i32);
#[repr(transparent)]
pub struct SUBSCRIPTIONSCHEDULE(pub i32);
pub const SUBSSCHED_AUTO: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(0i32);
pub const SUBSSCHED_DAILY: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(1i32);
pub const SUBSSCHED_WEEKLY: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(2i32);
pub const SUBSSCHED_CUSTOM: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(3i32);
pub const SUBSSCHED_MANUAL: SUBSCRIPTIONSCHEDULE = SUBSCRIPTIONSCHEDULE(4i32);
#[repr(transparent)]
pub struct SUBSCRIPTIONTYPE(pub i32);
pub const SUBSTYPE_URL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(0i32);
pub const SUBSTYPE_CHANNEL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(1i32);
pub const SUBSTYPE_DESKTOPURL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(2i32);
pub const SUBSTYPE_EXTERNAL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(3i32);
pub const SUBSTYPE_DESKTOPCHANNEL: SUBSCRIPTIONTYPE = SUBSCRIPTIONTYPE(4i32);
pub const SUBSINFO_ALLFLAGS: u32 = 61311u32;
pub const SUBSMGRENUM_MASK: u32 = 1u32;
pub const SUBSMGRENUM_TEMP: u32 = 1u32;
pub const SUBSMGRUPDATE_MASK: u32 = 1u32;
pub const SUBSMGRUPDATE_MINIMIZE: u32 = 1u32;
pub const SUCCEED: u32 = 1u32;
pub const SUCCEED_ABORT: u32 = 2u32;
pub const SUCCEED_ASYNC: u32 = 3u32;
#[repr(C)]
pub struct SubscriptionMgr(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TEXT_SOURCE(i32);
#[repr(C)]
pub struct TIMEOUT_INFO(i32);
#[repr(C)]
pub struct TIMESTAMP_STRUCT(i32);
#[repr(C)]
pub struct TIME_STRUCT(i32);
pub const TRACE_ON: i32 = 1i32;
pub const TRACE_VERSION: u32 = 1000u32;
pub const TRACE_VS_EVENT_ON: i32 = 2i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct VECTORRESTRICTION(i32);
#[repr(transparent)]
pub struct WEBCRAWL_RECURSEFLAGS(pub i32);
pub const WEBCRAWL_DONT_MAKE_STICKY: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(1i32);
pub const WEBCRAWL_GET_IMAGES: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(2i32);
pub const WEBCRAWL_GET_VIDEOS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(4i32);
pub const WEBCRAWL_GET_BGSOUNDS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(8i32);
pub const WEBCRAWL_GET_CONTROLS: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(16i32);
pub const WEBCRAWL_LINKS_ELSEWHERE: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(32i32);
pub const WEBCRAWL_IGNORE_ROBOTSTXT: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(128i32);
pub const WEBCRAWL_ONLY_LINKS_TO_HTML: WEBCRAWL_RECURSEFLAGS = WEBCRAWL_RECURSEFLAGS(256i32);
pub const XML_E_BADSXQL: i32 = -2147212799i32;
pub const XML_E_NODEFAULTNS: i32 = -2147212800i32;
pub const _MAPI_E_ACCOUNT_DISABLED: i32 = -2147221212i32;
pub const _MAPI_E_BAD_CHARWIDTH: i32 = -2147221245i32;
pub const _MAPI_E_BAD_COLUMN: i32 = -2147221224i32;
pub const _MAPI_E_BUSY: i32 = -2147221237i32;
pub const _MAPI_E_COMPUTED: i32 = -2147221222i32;
pub const _MAPI_E_CORRUPT_DATA: i32 = -2147221221i32;
pub const _MAPI_E_DISK_ERROR: i32 = -2147221226i32;
pub const _MAPI_E_END_OF_SESSION: i32 = -2147220992i32;
pub const _MAPI_E_EXTENDED_ERROR: i32 = -2147221223i32;
pub const _MAPI_E_FAILONEPROVIDER: i32 = -2147221219i32;
pub const _MAPI_E_INVALID_ACCESS_TIME: i32 = -2147221213i32;
pub const _MAPI_E_INVALID_ENTRYID: i32 = -2147221241i32;
pub const _MAPI_E_INVALID_OBJECT: i32 = -2147221240i32;
pub const _MAPI_E_INVALID_WORKSTATION_ACCOUNT: i32 = -2147221214i32;
pub const _MAPI_E_LOGON_FAILED: i32 = -2147221231i32;
pub const _MAPI_E_MISSING_REQUIRED_COLUMN: i32 = -2147220990i32;
pub const _MAPI_E_NETWORK_ERROR: i32 = -2147221227i32;
pub const _MAPI_E_NOT_ENOUGH_DISK: i32 = -2147221235i32;
pub const _MAPI_E_NOT_ENOUGH_RESOURCES: i32 = -2147221234i32;
pub const _MAPI_E_NOT_FOUND: i32 = -2147221233i32;
pub const _MAPI_E_NO_SUPPORT: i32 = -2147221246i32;
pub const _MAPI_E_OBJECT_CHANGED: i32 = -2147221239i32;
pub const _MAPI_E_OBJECT_DELETED: i32 = -2147221238i32;
pub const _MAPI_E_PASSWORD_CHANGE_REQUIRED: i32 = -2147221216i32;
pub const _MAPI_E_PASSWORD_EXPIRED: i32 = -2147221215i32;
pub const _MAPI_E_SESSION_LIMIT: i32 = -2147221230i32;
pub const _MAPI_E_STRING_TOO_LONG: i32 = -2147221243i32;
pub const _MAPI_E_TOO_COMPLEX: i32 = -2147221225i32;
pub const _MAPI_E_UNABLE_TO_ABORT: i32 = -2147221228i32;
pub const _MAPI_E_UNCONFIGURED: i32 = -2147221220i32;
pub const _MAPI_E_UNKNOWN_CPID: i32 = -2147221218i32;
pub const _MAPI_E_UNKNOWN_ENTRYID: i32 = -2147220991i32;
pub const _MAPI_E_UNKNOWN_FLAGS: i32 = -2147221242i32;
pub const _MAPI_E_UNKNOWN_LCID: i32 = -2147221217i32;
pub const _MAPI_E_USER_CANCEL: i32 = -2147221229i32;
pub const _MAPI_E_VERSION: i32 = -2147221232i32;
pub const _MAPI_W_NO_SERVICE: i32 = 262659i32;
#[repr(C)]
pub struct dbdatetime(i32);
#[repr(C)]
pub struct dbdatetime4(i32);
#[repr(C)]
pub struct dbmoney(i32);
#[repr(C)]
pub struct dbvarybin(i32);
#[repr(C)]
pub struct dbvarychar(i32);
#[repr(C)]
pub struct sqlperf(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct tagDBROWWATCHRANGE(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct tagDBROWWATCHRANGE(i32);
#[repr(C)]
pub struct tagSQL_DAY_SECOND(i32);
#[repr(C)]
pub struct tagSQL_YEAR_MONTH(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct tagSSErrorInfo(i32);
