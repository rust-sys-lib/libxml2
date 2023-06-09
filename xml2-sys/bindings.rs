pub const LIBXML_DOTTED_VERSION: &[u8; 7usize] = b"2.10.4\0";
pub const LIBXML_VERSION: u32 = 21003;
pub const LIBXML_VERSION_STRING: &[u8; 6usize] = b"21003\0";
pub const LIBXML_VERSION_EXTRA: &[u8; 1usize] = b"\0";
pub const LIBXML_MODULE_EXTENSION: &[u8; 4usize] = b".so\0";
pub const BASE_BUFFER_SIZE: u32 = 4096;
pub const XML_DOCB_DOCUMENT_NODE: u32 = 21;
pub const XML_DEFAULT_VERSION: &[u8; 4usize] = b"1.0\0";
pub const XML_DETECT_IDS: u32 = 2;
pub const XML_COMPLETE_ATTRS: u32 = 4;
pub const XML_SKIP_IDS: u32 = 8;
pub const XML_SAX2_MAGIC: u32 = 3740122799;
pub const XPATH_POINT: u32 = 5;
pub const XPATH_RANGE: u32 = 6;
pub const XPATH_LOCATIONSET: u32 = 7;
pub const XML_XPATH_CHECKNS: u32 = 1;
pub const XML_XPATH_NOVAR: u32 = 2;
pub const XML_SCHEMAS_ANYATTR_SKIP: u32 = 1;
pub const XML_SCHEMAS_ANYATTR_LAX: u32 = 2;
pub const XML_SCHEMAS_ANYATTR_STRICT: u32 = 3;
pub const XML_SCHEMAS_ANY_SKIP: u32 = 1;
pub const XML_SCHEMAS_ANY_LAX: u32 = 2;
pub const XML_SCHEMAS_ANY_STRICT: u32 = 3;
pub const XML_SCHEMAS_ATTR_USE_PROHIBITED: u32 = 0;
pub const XML_SCHEMAS_ATTR_USE_REQUIRED: u32 = 1;
pub const XML_SCHEMAS_ATTR_USE_OPTIONAL: u32 = 2;
pub const XML_SCHEMAS_ATTR_GLOBAL: u32 = 1;
pub const XML_SCHEMAS_ATTR_NSDEFAULT: u32 = 128;
pub const XML_SCHEMAS_ATTR_INTERNAL_RESOLVED: u32 = 256;
pub const XML_SCHEMAS_ATTR_FIXED: u32 = 512;
pub const XML_SCHEMAS_WILDCARD_COMPLETE: u32 = 1;
pub const XML_SCHEMAS_ATTRGROUP_WILDCARD_BUILDED: u32 = 1;
pub const XML_SCHEMAS_ATTRGROUP_GLOBAL: u32 = 2;
pub const XML_SCHEMAS_ATTRGROUP_MARKED: u32 = 4;
pub const XML_SCHEMAS_ATTRGROUP_REDEFINED: u32 = 8;
pub const XML_SCHEMAS_ATTRGROUP_HAS_REFS: u32 = 16;
pub const XML_SCHEMAS_TYPE_MIXED: u32 = 1;
pub const XML_SCHEMAS_TYPE_DERIVATION_METHOD_EXTENSION: u32 = 2;
pub const XML_SCHEMAS_TYPE_DERIVATION_METHOD_RESTRICTION: u32 = 4;
pub const XML_SCHEMAS_TYPE_GLOBAL: u32 = 8;
pub const XML_SCHEMAS_TYPE_OWNED_ATTR_WILDCARD: u32 = 16;
pub const XML_SCHEMAS_TYPE_VARIETY_ABSENT: u32 = 32;
pub const XML_SCHEMAS_TYPE_VARIETY_LIST: u32 = 64;
pub const XML_SCHEMAS_TYPE_VARIETY_UNION: u32 = 128;
pub const XML_SCHEMAS_TYPE_VARIETY_ATOMIC: u32 = 256;
pub const XML_SCHEMAS_TYPE_FINAL_EXTENSION: u32 = 512;
pub const XML_SCHEMAS_TYPE_FINAL_RESTRICTION: u32 = 1024;
pub const XML_SCHEMAS_TYPE_FINAL_LIST: u32 = 2048;
pub const XML_SCHEMAS_TYPE_FINAL_UNION: u32 = 4096;
pub const XML_SCHEMAS_TYPE_FINAL_DEFAULT: u32 = 8192;
pub const XML_SCHEMAS_TYPE_BUILTIN_PRIMITIVE: u32 = 16384;
pub const XML_SCHEMAS_TYPE_MARKED: u32 = 65536;
pub const XML_SCHEMAS_TYPE_BLOCK_DEFAULT: u32 = 131072;
pub const XML_SCHEMAS_TYPE_BLOCK_EXTENSION: u32 = 262144;
pub const XML_SCHEMAS_TYPE_BLOCK_RESTRICTION: u32 = 524288;
pub const XML_SCHEMAS_TYPE_ABSTRACT: u32 = 1048576;
pub const XML_SCHEMAS_TYPE_FACETSNEEDVALUE: u32 = 2097152;
pub const XML_SCHEMAS_TYPE_INTERNAL_RESOLVED: u32 = 4194304;
pub const XML_SCHEMAS_TYPE_INTERNAL_INVALID: u32 = 8388608;
pub const XML_SCHEMAS_TYPE_WHITESPACE_PRESERVE: u32 = 16777216;
pub const XML_SCHEMAS_TYPE_WHITESPACE_REPLACE: u32 = 33554432;
pub const XML_SCHEMAS_TYPE_WHITESPACE_COLLAPSE: u32 = 67108864;
pub const XML_SCHEMAS_TYPE_HAS_FACETS: u32 = 134217728;
pub const XML_SCHEMAS_TYPE_NORMVALUENEEDED: u32 = 268435456;
pub const XML_SCHEMAS_TYPE_FIXUP_1: u32 = 536870912;
pub const XML_SCHEMAS_TYPE_REDEFINED: u32 = 1073741824;
pub const XML_SCHEMAS_ELEM_NILLABLE: u32 = 1;
pub const XML_SCHEMAS_ELEM_GLOBAL: u32 = 2;
pub const XML_SCHEMAS_ELEM_DEFAULT: u32 = 4;
pub const XML_SCHEMAS_ELEM_FIXED: u32 = 8;
pub const XML_SCHEMAS_ELEM_ABSTRACT: u32 = 16;
pub const XML_SCHEMAS_ELEM_TOPLEVEL: u32 = 32;
pub const XML_SCHEMAS_ELEM_REF: u32 = 64;
pub const XML_SCHEMAS_ELEM_NSDEFAULT: u32 = 128;
pub const XML_SCHEMAS_ELEM_INTERNAL_RESOLVED: u32 = 256;
pub const XML_SCHEMAS_ELEM_CIRCULAR: u32 = 512;
pub const XML_SCHEMAS_ELEM_BLOCK_ABSENT: u32 = 1024;
pub const XML_SCHEMAS_ELEM_BLOCK_EXTENSION: u32 = 2048;
pub const XML_SCHEMAS_ELEM_BLOCK_RESTRICTION: u32 = 4096;
pub const XML_SCHEMAS_ELEM_BLOCK_SUBSTITUTION: u32 = 8192;
pub const XML_SCHEMAS_ELEM_FINAL_ABSENT: u32 = 16384;
pub const XML_SCHEMAS_ELEM_FINAL_EXTENSION: u32 = 32768;
pub const XML_SCHEMAS_ELEM_FINAL_RESTRICTION: u32 = 65536;
pub const XML_SCHEMAS_ELEM_SUBST_GROUP_HEAD: u32 = 131072;
pub const XML_SCHEMAS_ELEM_INTERNAL_CHECKED: u32 = 262144;
pub const XML_SCHEMAS_FACET_UNKNOWN: u32 = 0;
pub const XML_SCHEMAS_FACET_PRESERVE: u32 = 1;
pub const XML_SCHEMAS_FACET_REPLACE: u32 = 2;
pub const XML_SCHEMAS_FACET_COLLAPSE: u32 = 3;
pub const XML_SCHEMAS_QUALIF_ELEM: u32 = 1;
pub const XML_SCHEMAS_QUALIF_ATTR: u32 = 2;
pub const XML_SCHEMAS_FINAL_DEFAULT_EXTENSION: u32 = 4;
pub const XML_SCHEMAS_FINAL_DEFAULT_RESTRICTION: u32 = 8;
pub const XML_SCHEMAS_FINAL_DEFAULT_LIST: u32 = 16;
pub const XML_SCHEMAS_FINAL_DEFAULT_UNION: u32 = 32;
pub const XML_SCHEMAS_BLOCK_DEFAULT_EXTENSION: u32 = 64;
pub const XML_SCHEMAS_BLOCK_DEFAULT_RESTRICTION: u32 = 128;
pub const XML_SCHEMAS_BLOCK_DEFAULT_SUBSTITUTION: u32 = 256;
pub const XML_SCHEMAS_INCLUDING_CONVERT_NS: u32 = 512;
pub const XML_MAX_TEXT_LENGTH: u32 = 10000000;
pub const XML_MAX_NAME_LENGTH: u32 = 50000;
pub const XML_MAX_DICTIONARY_LIMIT: u32 = 10000000;
pub const XML_MAX_LOOKUP_LIMIT: u32 = 10000000;
pub const XML_MAX_NAMELEN: u32 = 100;
pub const INPUT_CHUNK: u32 = 250;
pub const XML_SUBSTITUTE_NONE: u32 = 0;
pub const XML_SUBSTITUTE_REF: u32 = 1;
pub const XML_SUBSTITUTE_PEREF: u32 = 2;
pub const XML_SUBSTITUTE_BOTH: u32 = 3;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sbuf() {
    const UNINIT: ::std::mem::MaybeUninit<__sbuf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sbuf>(),
        16usize,
        concat!("Size of: ", stringify!(__sbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<__sbuf>(),
        8usize,
        concat!("Alignment of ", stringify!(__sbuf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._base) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
#[test]
fn bindgen_test_layout___sFILE() {
    const UNINIT: ::std::mem::MaybeUninit<__sFILE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sFILE>(),
        152usize,
        concat!("Size of: ", stringify!(__sFILE))
    );
    assert_eq!(
        ::std::mem::align_of::<__sFILE>(),
        8usize,
        concat!("Alignment of ", stringify!(__sFILE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._r) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_w)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._file) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_file)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._bf) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_bf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lbfsize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lbfsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._cookie) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_cookie)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._close) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_close)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._read) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._seek) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_seek)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._write) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_write)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ub) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ub)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._extra) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_extra)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ur) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ubuf) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ubuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nbuf) as usize - ptr as usize },
        119usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_nbuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lb) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lb)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._blksize) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_blksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_offset)
        )
    );
}
pub type FILE = __sFILE;
extern "C" {
    pub fn xmlCheckVersion(version: ::std::os::raw::c_int);
}
pub type xmlChar = ::std::os::raw::c_uchar;
extern "C" {
    pub fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrndup(cur: *const xmlChar, len: ::std::os::raw::c_int) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCharStrndup(
        cur: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCharStrdup(cur: *const ::std::os::raw::c_char) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrsub(
        str_: *const xmlChar,
        start: ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrchr(str_: *const xmlChar, val: xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlStrstr(str_: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlStrcasestr(str_: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrncmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrncasecmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str_: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrlen(str_: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrncat(
        cur: *mut xmlChar,
        add: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrncatNew(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStrPrintf(
        buf: *mut xmlChar,
        len: ::std::os::raw::c_int,
        msg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStrVPrintf(
        buf: *mut xmlChar,
        len: ::std::os::raw::c_int,
        msg: *const ::std::os::raw::c_char,
        ap: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGetUTF8Char(
        utf: *const ::std::os::raw::c_uchar,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCheckUTF8(utf: *const ::std::os::raw::c_uchar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUTF8Strsize(utf: *const xmlChar, len: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUTF8Strndup(utf: *const xmlChar, len: ::std::os::raw::c_int) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlUTF8Strpos(utf: *const xmlChar, pos: ::std::os::raw::c_int) -> *const xmlChar;
}
extern "C" {
    pub fn xmlUTF8Strloc(utf: *const xmlChar, utfchar: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUTF8Strsub(
        utf: *const xmlChar,
        start: ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlUTF8Strlen(utf: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUTF8Size(utf: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUTF8Charcmp(utf1: *const xmlChar, utf2: *const xmlChar) -> ::std::os::raw::c_int;
}
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXLocator = _xmlSAXLocator;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlEntity = _xmlEntity;
pub type xmlEntityPtr = *mut xmlEntity;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const xmlBufferAllocationScheme_XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub type xmlBufferAllocationScheme = ::std::os::raw::c_uint;
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
#[test]
fn bindgen_test_layout__xmlBuffer() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlBuffer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlBuffer>(),
        32usize,
        concat!("Size of: ", stringify!(_xmlBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlBuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlBuffer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).content) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlBuffer),
            "::",
            stringify!(content)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).use_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlBuffer),
            "::",
            stringify!(use_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlBuffer),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alloc) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlBuffer),
            "::",
            stringify!(alloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contentIO) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlBuffer),
            "::",
            stringify!(contentIO)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlBuf {
    _unused: [u8; 0],
}
pub type xmlBuf = _xmlBuf;
pub type xmlBufPtr = *mut xmlBuf;
extern "C" {
    pub fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlBufUse(buf: xmlBufPtr) -> usize;
}
extern "C" {
    pub fn xmlBufShrink(buf: xmlBufPtr, len: usize) -> usize;
}
pub const xmlElementType_XML_ELEMENT_NODE: xmlElementType = 1;
pub const xmlElementType_XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const xmlElementType_XML_TEXT_NODE: xmlElementType = 3;
pub const xmlElementType_XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const xmlElementType_XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const xmlElementType_XML_ENTITY_NODE: xmlElementType = 6;
pub const xmlElementType_XML_PI_NODE: xmlElementType = 7;
pub const xmlElementType_XML_COMMENT_NODE: xmlElementType = 8;
pub const xmlElementType_XML_DOCUMENT_NODE: xmlElementType = 9;
pub const xmlElementType_XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const xmlElementType_XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const xmlElementType_XML_NOTATION_NODE: xmlElementType = 12;
pub const xmlElementType_XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const xmlElementType_XML_DTD_NODE: xmlElementType = 14;
pub const xmlElementType_XML_ELEMENT_DECL: xmlElementType = 15;
pub const xmlElementType_XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const xmlElementType_XML_ENTITY_DECL: xmlElementType = 17;
pub const xmlElementType_XML_NAMESPACE_DECL: xmlElementType = 18;
pub const xmlElementType_XML_XINCLUDE_START: xmlElementType = 19;
pub const xmlElementType_XML_XINCLUDE_END: xmlElementType = 20;
pub type xmlElementType = ::std::os::raw::c_uint;
pub type xmlNotation = _xmlNotation;
pub type xmlNotationPtr = *mut xmlNotation;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlNotation {
    pub name: *const xmlChar,
    pub PublicID: *const xmlChar,
    pub SystemID: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlNotation() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlNotation> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlNotation>(),
        24usize,
        concat!("Size of: ", stringify!(_xmlNotation))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlNotation>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlNotation))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNotation),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PublicID) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNotation),
            "::",
            stringify!(PublicID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SystemID) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNotation),
            "::",
            stringify!(SystemID)
        )
    );
}
pub const xmlAttributeType_XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub const xmlAttributeType_XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const xmlAttributeType_XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const xmlAttributeType_XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const xmlAttributeType_XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const xmlAttributeType_XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const xmlAttributeType_XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const xmlAttributeType_XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const xmlAttributeType_XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const xmlAttributeType_XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub type xmlAttributeType = ::std::os::raw::c_uint;
pub const xmlAttributeDefault_XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
pub const xmlAttributeDefault_XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const xmlAttributeDefault_XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const xmlAttributeDefault_XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub type xmlAttributeDefault = ::std::os::raw::c_uint;
pub type xmlEnumeration = _xmlEnumeration;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlEnumeration() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlEnumeration> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlEnumeration>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlEnumeration))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlEnumeration>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlEnumeration))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEnumeration),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEnumeration),
            "::",
            stringify!(name)
        )
    );
}
pub type xmlAttribute = _xmlAttribute;
pub type xmlAttributePtr = *mut xmlAttribute;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlAttribute {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub nexth: *mut _xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: xmlAttributeDefault,
    pub defaultValue: *const xmlChar,
    pub tree: xmlEnumerationPtr,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlAttribute() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlAttribute> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlAttribute>(),
        120usize,
        concat!("Size of: ", stringify!(_xmlAttribute))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlAttribute>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlAttribute))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nexth) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(nexth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atype) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(atype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).defaultValue) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(defaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tree) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(tree)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefix) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elem) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttribute),
            "::",
            stringify!(elem)
        )
    );
}
pub const xmlElementContentType_XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub const xmlElementContentType_XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const xmlElementContentType_XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const xmlElementContentType_XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub type xmlElementContentType = ::std::os::raw::c_uint;
pub const xmlElementContentOccur_XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub const xmlElementContentOccur_XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const xmlElementContentOccur_XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const xmlElementContentOccur_XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub type xmlElementContentOccur = ::std::os::raw::c_uint;
pub type xmlElementContent = _xmlElementContent;
pub type xmlElementContentPtr = *mut xmlElementContent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlElementContent {
    pub type_: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlElementContent() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlElementContent> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlElementContent>(),
        48usize,
        concat!("Size of: ", stringify!(_xmlElementContent))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlElementContent>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlElementContent))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ocur) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(ocur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c1) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(c1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c2) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(c2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefix) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElementContent),
            "::",
            stringify!(prefix)
        )
    );
}
pub const xmlElementTypeVal_XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
pub const xmlElementTypeVal_XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const xmlElementTypeVal_XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const xmlElementTypeVal_XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const xmlElementTypeVal_XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub type xmlElementTypeVal = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRegexp {
    _unused: [u8; 0],
}
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRegExecCtxt {
    _unused: [u8; 0],
}
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlDict {
    _unused: [u8; 0],
}
pub type xmlDict = _xmlDict;
pub type xmlDictPtr = *mut xmlDict;
extern "C" {
    pub fn xmlInitializeDict() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDictCreate() -> xmlDictPtr;
}
extern "C" {
    pub fn xmlDictSetLimit(dict: xmlDictPtr, limit: usize) -> usize;
}
extern "C" {
    pub fn xmlDictGetUsage(dict: xmlDictPtr) -> usize;
}
extern "C" {
    pub fn xmlDictCreateSub(sub: xmlDictPtr) -> xmlDictPtr;
}
extern "C" {
    pub fn xmlDictReference(dict: xmlDictPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDictFree(dict: xmlDictPtr);
}
extern "C" {
    pub fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> *const xmlChar;
}
extern "C" {
    pub fn xmlDictExists(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> *const xmlChar;
}
extern "C" {
    pub fn xmlDictQLookup(
        dict: xmlDictPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
    ) -> *const xmlChar;
}
extern "C" {
    pub fn xmlDictOwns(dict: xmlDictPtr, str_: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDictSize(dict: xmlDictPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDictCleanup();
}
extern "C" {
    pub fn xmlRegexpCompile(regexp: *const xmlChar) -> xmlRegexpPtr;
}
extern "C" {
    pub fn xmlRegFreeRegexp(regexp: xmlRegexpPtr);
}
extern "C" {
    pub fn xmlRegexpExec(comp: xmlRegexpPtr, value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegexpPrint(output: *mut FILE, regexp: xmlRegexpPtr);
}
extern "C" {
    pub fn xmlRegexpIsDeterminist(comp: xmlRegexpPtr) -> ::std::os::raw::c_int;
}
pub type xmlRegExecCallbacks = ::std::option::Option<
    unsafe extern "C" fn(
        exec: xmlRegExecCtxtPtr,
        token: *const xmlChar,
        transdata: *mut ::std::os::raw::c_void,
        inputdata: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn xmlRegNewExecCtxt(
        comp: xmlRegexpPtr,
        callback: xmlRegExecCallbacks,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlRegExecCtxtPtr;
}
extern "C" {
    pub fn xmlRegFreeExecCtxt(exec: xmlRegExecCtxtPtr);
}
extern "C" {
    pub fn xmlRegExecPushString(
        exec: xmlRegExecCtxtPtr,
        value: *const xmlChar,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegExecPushString2(
        exec: xmlRegExecCtxtPtr,
        value: *const xmlChar,
        value2: *const xmlChar,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegExecNextValues(
        exec: xmlRegExecCtxtPtr,
        nbval: *mut ::std::os::raw::c_int,
        nbneg: *mut ::std::os::raw::c_int,
        values: *mut *mut xmlChar,
        terminal: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegExecErrInfo(
        exec: xmlRegExecCtxtPtr,
        string: *mut *const xmlChar,
        nbval: *mut ::std::os::raw::c_int,
        nbneg: *mut ::std::os::raw::c_int,
        values: *mut *mut xmlChar,
        terminal: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type xmlElement = _xmlElement;
pub type xmlElementPtr = *mut xmlElement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlElement {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub etype: xmlElementTypeVal,
    pub content: xmlElementContentPtr,
    pub attributes: xmlAttributePtr,
    pub prefix: *const xmlChar,
    pub contModel: xmlRegexpPtr,
}
#[test]
fn bindgen_test_layout__xmlElement() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlElement> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlElement>(),
        112usize,
        concat!("Size of: ", stringify!(_xmlElement))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlElement>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlElement))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).etype) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(etype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).content) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(content)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(attributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefix) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contModel) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlElement),
            "::",
            stringify!(contModel)
        )
    );
}
pub use self::xmlElementType as xmlNsType;
pub type xmlNs = _xmlNs;
pub type xmlNsPtr = *mut xmlNs;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut ::std::os::raw::c_void,
    pub context: *mut _xmlDoc,
}
#[test]
fn bindgen_test_layout__xmlNs() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlNs> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlNs>(),
        48usize,
        concat!("Size of: ", stringify!(_xmlNs))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlNs>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlNs))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).href) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(href)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prefix) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(prefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNs),
            "::",
            stringify!(context)
        )
    );
}
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlDtd {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut ::std::os::raw::c_void,
    pub elements: *mut ::std::os::raw::c_void,
    pub attributes: *mut ::std::os::raw::c_void,
    pub entities: *mut ::std::os::raw::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlDtd() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlDtd> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlDtd>(),
        128usize,
        concat!("Size of: ", stringify!(_xmlDtd))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlDtd>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlDtd))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).notations) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(notations)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elements) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(elements)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(attributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).entities) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(entities)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ExternalID) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(ExternalID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SystemID) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(SystemID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pentities) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDtd),
            "::",
            stringify!(pentities)
        )
    );
}
pub type xmlAttr = _xmlAttr;
pub type xmlAttrPtr = *mut xmlAttr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlAttr {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlAttr() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlAttr> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlAttr>(),
        96usize,
        concat!("Size of: ", stringify!(_xmlAttr))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlAttr>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlAttr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ns) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(ns)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atype) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(atype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).psvi) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlAttr),
            "::",
            stringify!(psvi)
        )
    );
}
pub type xmlID = _xmlID;
pub type xmlIDPtr = *mut xmlID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlID {
    pub next: *mut _xmlID,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: ::std::os::raw::c_int,
    pub doc: *mut _xmlDoc,
}
#[test]
fn bindgen_test_layout__xmlID() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlID> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlID>(),
        48usize,
        concat!("Size of: ", stringify!(_xmlID))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlID>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlID))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attr) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(attr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lineno) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(lineno)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlID),
            "::",
            stringify!(doc)
        )
    );
}
pub type xmlRef = _xmlRef;
pub type xmlRefPtr = *mut xmlRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRef {
    pub next: *mut _xmlRef,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlRef() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlRef> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlRef>(),
        40usize,
        concat!("Size of: ", stringify!(_xmlRef))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlRef>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlRef))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlRef),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlRef),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attr) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlRef),
            "::",
            stringify!(attr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlRef),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lineno) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlRef),
            "::",
            stringify!(lineno)
        )
    );
}
pub type xmlNode = _xmlNode;
pub type xmlNodePtr = *mut xmlNode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlNode {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut ::std::os::raw::c_void,
    pub line: ::std::os::raw::c_ushort,
    pub extra: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__xmlNode() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlNode> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlNode>(),
        120usize,
        concat!("Size of: ", stringify!(_xmlNode))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlNode>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlNode))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ns) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(ns)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).content) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(content)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).properties) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(properties)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsDef) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(nsDef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).psvi) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(psvi)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extra) as usize - ptr as usize },
        114usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNode),
            "::",
            stringify!(extra)
        )
    );
}
pub const xmlDocProperties_XML_DOC_WELLFORMED: xmlDocProperties = 1;
pub const xmlDocProperties_XML_DOC_NSVALID: xmlDocProperties = 2;
pub const xmlDocProperties_XML_DOC_OLD10: xmlDocProperties = 4;
pub const xmlDocProperties_XML_DOC_DTDVALID: xmlDocProperties = 8;
pub const xmlDocProperties_XML_DOC_XINCLUDE: xmlDocProperties = 16;
pub const xmlDocProperties_XML_DOC_USERBUILT: xmlDocProperties = 32;
pub const xmlDocProperties_XML_DOC_INTERNAL: xmlDocProperties = 64;
pub const xmlDocProperties_XML_DOC_HTML: xmlDocProperties = 128;
pub type xmlDocProperties = ::std::os::raw::c_uint;
pub type xmlDoc = _xmlDoc;
pub type xmlDocPtr = *mut xmlDoc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlDoc {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *mut ::std::os::raw::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: ::std::os::raw::c_int,
    pub standalone: ::std::os::raw::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::std::os::raw::c_void,
    pub refs: *mut ::std::os::raw::c_void,
    pub URL: *const xmlChar,
    pub charset: ::std::os::raw::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::std::os::raw::c_void,
    pub parseFlags: ::std::os::raw::c_int,
    pub properties: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlDoc() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlDoc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlDoc>(),
        176usize,
        concat!("Size of: ", stringify!(_xmlDoc))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlDoc>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlDoc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compression) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(compression)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).standalone) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(standalone)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).intSubset) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(intSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extSubset) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(extSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).oldNs) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(oldNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encoding) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(encoding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ids) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(ids)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refs) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(refs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).URL) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(URL)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).charset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(charset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dict) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(dict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).psvi) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(psvi)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parseFlags) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(parseFlags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).properties) as usize - ptr as usize },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDoc),
            "::",
            stringify!(properties)
        )
    );
}
pub type xmlDOMWrapCtxt = _xmlDOMWrapCtxt;
pub type xmlDOMWrapCtxtPtr = *mut xmlDOMWrapCtxt;
pub type xmlDOMWrapAcquireNsFunction = ::std::option::Option<
    unsafe extern "C" fn(
        ctxt: xmlDOMWrapCtxtPtr,
        node: xmlNodePtr,
        nsName: *const xmlChar,
        nsPrefix: *const xmlChar,
    ) -> xmlNsPtr,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlDOMWrapCtxt {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: ::std::os::raw::c_int,
    pub namespaceMap: *mut ::std::os::raw::c_void,
    pub getNsForNodeFunc: xmlDOMWrapAcquireNsFunction,
}
#[test]
fn bindgen_test_layout__xmlDOMWrapCtxt() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlDOMWrapCtxt> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlDOMWrapCtxt>(),
        32usize,
        concat!("Size of: ", stringify!(_xmlDOMWrapCtxt))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlDOMWrapCtxt>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlDOMWrapCtxt))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDOMWrapCtxt),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDOMWrapCtxt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namespaceMap) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDOMWrapCtxt),
            "::",
            stringify!(namespaceMap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getNsForNodeFunc) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlDOMWrapCtxt),
            "::",
            stringify!(getNsForNodeFunc)
        )
    );
}
extern "C" {
    pub fn xmlValidateNCName(
        value: *const xmlChar,
        space: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateQName(
        value: *const xmlChar,
        space: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateName(
        value: *const xmlChar,
        space: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNMToken(
        value: *const xmlChar,
        space: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBuildQName(
        ncname: *const xmlChar,
        prefix: *const xmlChar,
        memory: *mut xmlChar,
        len: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlSplitQName3(name: *const xmlChar, len: *mut ::std::os::raw::c_int) -> *const xmlChar;
}
extern "C" {
    pub fn xmlSetBufferAllocationScheme(scheme: xmlBufferAllocationScheme);
}
extern "C" {
    pub fn xmlGetBufferAllocationScheme() -> xmlBufferAllocationScheme;
}
extern "C" {
    pub fn xmlBufferCreate() -> xmlBufferPtr;
}
extern "C" {
    pub fn xmlBufferCreateSize(size: usize) -> xmlBufferPtr;
}
extern "C" {
    pub fn xmlBufferCreateStatic(mem: *mut ::std::os::raw::c_void, size: usize) -> xmlBufferPtr;
}
extern "C" {
    pub fn xmlBufferResize(
        buf: xmlBufferPtr,
        size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferFree(buf: xmlBufferPtr);
}
extern "C" {
    pub fn xmlBufferDump(file: *mut FILE, buf: xmlBufferPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferAdd(
        buf: xmlBufferPtr,
        str_: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferAddHead(
        buf: xmlBufferPtr,
        str_: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferCat(buf: xmlBufferPtr, str_: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferCCat(
        buf: xmlBufferPtr,
        str_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferShrink(buf: xmlBufferPtr, len: ::std::os::raw::c_uint)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferGrow(buf: xmlBufferPtr, len: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferEmpty(buf: xmlBufferPtr);
}
extern "C" {
    pub fn xmlBufferContent(buf: *const xmlBuffer) -> *const xmlChar;
}
extern "C" {
    pub fn xmlBufferDetach(buf: xmlBufferPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlBufferSetAllocationScheme(buf: xmlBufferPtr, scheme: xmlBufferAllocationScheme);
}
extern "C" {
    pub fn xmlBufferLength(buf: *const xmlBuffer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlFreeDtd(cur: xmlDtdPtr);
}
extern "C" {
    pub fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar, prefix: *const xmlChar) -> xmlNsPtr;
}
extern "C" {
    pub fn xmlFreeNs(cur: xmlNsPtr);
}
extern "C" {
    pub fn xmlFreeNsList(cur: xmlNsPtr);
}
extern "C" {
    pub fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlFreeDoc(cur: xmlDocPtr);
}
extern "C" {
    pub fn xmlNewDocProp(doc: xmlDocPtr, name: *const xmlChar, value: *const xmlChar)
        -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlNewProp(node: xmlNodePtr, name: *const xmlChar, value: *const xmlChar) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlNewNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlNewNsPropEatName(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlFreePropList(cur: xmlAttrPtr);
}
extern "C" {
    pub fn xmlFreeProp(cur: xmlAttrPtr);
}
extern "C" {
    pub fn xmlCopyProp(target: xmlNodePtr, cur: xmlAttrPtr) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlCopyPropList(target: xmlNodePtr, cur: xmlAttrPtr) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlCopyDoc(doc: xmlDocPtr, recursive: ::std::os::raw::c_int) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocNodeEatName(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewNode(ns: xmlNsPtr, name: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewNodeEatName(ns: xmlNsPtr, name: *mut xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewChild(
        parent: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocPI(doc: xmlDocPtr, name: *const xmlChar, content: *const xmlChar)
        -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewPI(name: *const xmlChar, content: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocTextLen(
        doc: xmlDocPtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewTextLen(content: *const xmlChar, len: ::std::os::raw::c_int) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocComment(doc: xmlDocPtr, content: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewComment(content: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewCDataBlock(
        doc: xmlDocPtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewCharRef(doc: xmlDocPtr, name: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewReference(doc: *const xmlDoc, name: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlCopyNode(node: xmlNodePtr, recursive: ::std::os::raw::c_int) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: ::std::os::raw::c_int,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlDocCopyNodeList(doc: xmlDocPtr, node: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlCopyNodeList(node: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewTextChild(
        parent: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocRawNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNewDocFragment(doc: xmlDocPtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlGetLineNo(node: *const xmlNode) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNodeIsText(node: *const xmlNode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsBlankNode(node: *const xmlNode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNodeSetName(cur: xmlNodePtr, name: *const xmlChar);
}
extern "C" {
    pub fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlReplaceNode(old: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlAddSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlUnlinkNode(cur: xmlNodePtr);
}
extern "C" {
    pub fn xmlTextMerge(first: xmlNodePtr, second: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlTextConcat(
        node: xmlNodePtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlFreeNodeList(cur: xmlNodePtr);
}
extern "C" {
    pub fn xmlFreeNode(cur: xmlNodePtr);
}
extern "C" {
    pub fn xmlSetTreeDoc(tree: xmlNodePtr, doc: xmlDocPtr);
}
extern "C" {
    pub fn xmlSetListDoc(list: xmlNodePtr, doc: xmlDocPtr);
}
extern "C" {
    pub fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
}
extern "C" {
    pub fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;
}
extern "C" {
    pub fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;
}
extern "C" {
    pub fn xmlSetNs(node: xmlNodePtr, ns: xmlNsPtr);
}
extern "C" {
    pub fn xmlCopyNamespace(cur: xmlNsPtr) -> xmlNsPtr;
}
extern "C" {
    pub fn xmlCopyNamespaceList(cur: xmlNsPtr) -> xmlNsPtr;
}
extern "C" {
    pub fn xmlSetProp(node: xmlNodePtr, name: *const xmlChar, value: *const xmlChar) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlSetNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlHasProp(node: *const xmlNode, name: *const xmlChar) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlHasNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStringGetNodeList(doc: *const xmlDoc, value: *const xmlChar) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlStringLenGetNodeList(
        doc: *const xmlDoc,
        value: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlNodeListGetRawString(
        doc: *const xmlDoc,
        list: *const xmlNode,
        inLine: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar);
}
extern "C" {
    pub fn xmlNodeSetContentLen(
        cur: xmlNodePtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlNodeAddContent(cur: xmlNodePtr, content: *const xmlChar);
}
extern "C" {
    pub fn xmlNodeAddContentLen(
        cur: xmlNodePtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlNodeBufGetContent(buffer: xmlBufferPtr, cur: *const xmlNode)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNodeSetLang(cur: xmlNodePtr, lang: *const xmlChar);
}
extern "C" {
    pub fn xmlNodeSetSpacePreserve(cur: xmlNodePtr, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
}
extern "C" {
    pub fn xmlRemoveProp(cur: xmlAttrPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUnsetNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUnsetProp(node: xmlNodePtr, name: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufferWriteCHAR(buf: xmlBufferPtr, string: *const xmlChar);
}
extern "C" {
    pub fn xmlBufferWriteChar(buf: xmlBufferPtr, string: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn xmlBufferWriteQuotedString(buf: xmlBufferPtr, string: *const xmlChar);
}
extern "C" {
    pub fn xmlAttrSerializeTxtContent(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        attr: xmlAttrPtr,
        string: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlReconciliateNs(doc: xmlDocPtr, tree: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDocDumpFormatMemory(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlDocDumpMemory(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlDocDumpMemoryEnc(
        out_doc: xmlDocPtr,
        doc_txt_ptr: *mut *mut xmlChar,
        doc_txt_len: *mut ::std::os::raw::c_int,
        txt_encoding: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn xmlDocDumpFormatMemoryEnc(
        out_doc: xmlDocPtr,
        doc_txt_ptr: *mut *mut xmlChar,
        doc_txt_len: *mut ::std::os::raw::c_int,
        txt_encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlDocFormatDump(
        f: *mut FILE,
        cur: xmlDocPtr,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlElemDump(f: *mut FILE, doc: xmlDocPtr, cur: xmlNodePtr);
}
extern "C" {
    pub fn xmlSaveFile(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveFormatFile(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBufNodeDump(
        buf: xmlBufPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    pub fn xmlNodeDump(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveFileTo(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveFormatFileTo(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNodeDumpOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        encoding: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn xmlSaveFormatFileEnc(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveFileEnc(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsXHTML(systemID: *const xmlChar, publicID: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGetDocCompressMode(doc: *const xmlDoc) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSetDocCompressMode(doc: xmlDocPtr, mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlGetCompressMode() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSetCompressMode(mode: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDOMWrapNewCtxt() -> xmlDOMWrapCtxtPtr;
}
extern "C" {
    pub fn xmlDOMWrapFreeCtxt(ctxt: xmlDOMWrapCtxtPtr);
}
extern "C" {
    pub fn xmlDOMWrapReconcileNamespaces(
        ctxt: xmlDOMWrapCtxtPtr,
        elem: xmlNodePtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDOMWrapAdoptNode(
        ctxt: xmlDOMWrapCtxtPtr,
        sourceDoc: xmlDocPtr,
        node: xmlNodePtr,
        destDoc: xmlDocPtr,
        destParent: xmlNodePtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDOMWrapRemoveNode(
        ctxt: xmlDOMWrapCtxtPtr,
        doc: xmlDocPtr,
        node: xmlNodePtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDOMWrapCloneNode(
        ctxt: xmlDOMWrapCtxtPtr,
        sourceDoc: xmlDocPtr,
        node: xmlNodePtr,
        clonedNode: *mut xmlNodePtr,
        destDoc: xmlDocPtr,
        destParent: xmlNodePtr,
        deep: ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlChildElementCount(parent: xmlNodePtr) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn xmlNextElementSibling(node: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlFirstElementChild(parent: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlLastElementChild(parent: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlPreviousElementSibling(node: xmlNodePtr) -> xmlNodePtr;
}
pub type xmlFreeFunc =
    ::std::option::Option<unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void)>;
pub type xmlMallocFunc =
    ::std::option::Option<unsafe extern "C" fn(size: usize) -> *mut ::std::os::raw::c_void>;
pub type xmlReallocFunc = ::std::option::Option<
    unsafe extern "C" fn(
        mem: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type xmlStrdupFunc = ::std::option::Option<
    unsafe extern "C" fn(str_: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
extern "C" {
    pub fn xmlMemSetup(
        freeFunc: xmlFreeFunc,
        mallocFunc: xmlMallocFunc,
        reallocFunc: xmlReallocFunc,
        strdupFunc: xmlStrdupFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlMemGet(
        freeFunc: *mut xmlFreeFunc,
        mallocFunc: *mut xmlMallocFunc,
        reallocFunc: *mut xmlReallocFunc,
        strdupFunc: *mut xmlStrdupFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGcMemSetup(
        freeFunc: xmlFreeFunc,
        mallocFunc: xmlMallocFunc,
        mallocAtomicFunc: xmlMallocFunc,
        reallocFunc: xmlReallocFunc,
        strdupFunc: xmlStrdupFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGcMemGet(
        freeFunc: *mut xmlFreeFunc,
        mallocFunc: *mut xmlMallocFunc,
        mallocAtomicFunc: *mut xmlMallocFunc,
        reallocFunc: *mut xmlReallocFunc,
        strdupFunc: *mut xmlStrdupFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlInitMemory() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCleanupMemory();
}
extern "C" {
    pub fn xmlMemUsed() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlMemBlocks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlMemDisplay(fp: *mut FILE);
}
extern "C" {
    pub fn xmlMemDisplayLast(fp: *mut FILE, nbBytes: ::std::os::raw::c_long);
}
extern "C" {
    pub fn xmlMemShow(fp: *mut FILE, nr: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlMemoryDump();
}
extern "C" {
    pub fn xmlMemMalloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlMemRealloc(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlMemFree(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlMemoryStrdup(str_: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlMallocLoc(
        size: usize,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlReallocLoc(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlMallocAtomicLoc(
        size: usize,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlMemStrdupLoc(
        str_: *const ::std::os::raw::c_char,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlMutex {
    _unused: [u8; 0],
}
pub type xmlMutex = _xmlMutex;
pub type xmlMutexPtr = *mut xmlMutex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRMutex {
    _unused: [u8; 0],
}
pub type xmlRMutex = _xmlRMutex;
pub type xmlRMutexPtr = *mut xmlRMutex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlHashTable {
    _unused: [u8; 0],
}
pub type xmlHashTable = _xmlHashTable;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashDeallocator = ::std::option::Option<
    unsafe extern "C" fn(payload: *mut ::std::os::raw::c_void, name: *const xmlChar),
>;
pub type xmlHashCopier = ::std::option::Option<
    unsafe extern "C" fn(
        payload: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type xmlHashScanner = ::std::option::Option<
    unsafe extern "C" fn(
        payload: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
    ),
>;
pub type xmlHashScannerFull = ::std::option::Option<
    unsafe extern "C" fn(
        payload: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
    ),
>;
extern "C" {
    pub fn xmlHashCreate(size: ::std::os::raw::c_int) -> xmlHashTablePtr;
}
extern "C" {
    pub fn xmlHashCreateDict(size: ::std::os::raw::c_int, dict: xmlDictPtr) -> xmlHashTablePtr;
}
extern "C" {
    pub fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
}
extern "C" {
    pub fn xmlHashDefaultDeallocator(entry: *mut ::std::os::raw::c_void, name: *const xmlChar);
}
extern "C" {
    pub fn xmlHashAddEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashUpdateEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashAddEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashUpdateEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashAddEntry3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashUpdateEntry3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        userdata: *mut ::std::os::raw::c_void,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashRemoveEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashRemoveEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashRemoveEntry3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashLookup(
        table: xmlHashTablePtr,
        name: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashLookup3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashQLookup(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashQLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
        name2: *const xmlChar,
        prefix2: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashQLookup3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
        name2: *const xmlChar,
        prefix2: *const xmlChar,
        name3: *const xmlChar,
        prefix3: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlHashCopy(table: xmlHashTablePtr, f: xmlHashCopier) -> xmlHashTablePtr;
}
extern "C" {
    pub fn xmlHashSize(table: xmlHashTablePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlHashScan(
        table: xmlHashTablePtr,
        f: xmlHashScanner,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlHashScan3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        f: xmlHashScanner,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlHashScanFull(
        table: xmlHashTablePtr,
        f: xmlHashScannerFull,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlHashScanFull3(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        f: xmlHashScannerFull,
        data: *mut ::std::os::raw::c_void,
    );
}
pub const xmlErrorLevel_XML_ERR_NONE: xmlErrorLevel = 0;
pub const xmlErrorLevel_XML_ERR_WARNING: xmlErrorLevel = 1;
pub const xmlErrorLevel_XML_ERR_ERROR: xmlErrorLevel = 2;
pub const xmlErrorLevel_XML_ERR_FATAL: xmlErrorLevel = 3;
pub type xmlErrorLevel = ::std::os::raw::c_uint;
pub const xmlErrorDomain_XML_FROM_NONE: xmlErrorDomain = 0;
pub const xmlErrorDomain_XML_FROM_PARSER: xmlErrorDomain = 1;
pub const xmlErrorDomain_XML_FROM_TREE: xmlErrorDomain = 2;
pub const xmlErrorDomain_XML_FROM_NAMESPACE: xmlErrorDomain = 3;
pub const xmlErrorDomain_XML_FROM_DTD: xmlErrorDomain = 4;
pub const xmlErrorDomain_XML_FROM_HTML: xmlErrorDomain = 5;
pub const xmlErrorDomain_XML_FROM_MEMORY: xmlErrorDomain = 6;
pub const xmlErrorDomain_XML_FROM_OUTPUT: xmlErrorDomain = 7;
pub const xmlErrorDomain_XML_FROM_IO: xmlErrorDomain = 8;
pub const xmlErrorDomain_XML_FROM_FTP: xmlErrorDomain = 9;
pub const xmlErrorDomain_XML_FROM_HTTP: xmlErrorDomain = 10;
pub const xmlErrorDomain_XML_FROM_XINCLUDE: xmlErrorDomain = 11;
pub const xmlErrorDomain_XML_FROM_XPATH: xmlErrorDomain = 12;
pub const xmlErrorDomain_XML_FROM_XPOINTER: xmlErrorDomain = 13;
pub const xmlErrorDomain_XML_FROM_REGEXP: xmlErrorDomain = 14;
pub const xmlErrorDomain_XML_FROM_DATATYPE: xmlErrorDomain = 15;
pub const xmlErrorDomain_XML_FROM_SCHEMASP: xmlErrorDomain = 16;
pub const xmlErrorDomain_XML_FROM_SCHEMASV: xmlErrorDomain = 17;
pub const xmlErrorDomain_XML_FROM_RELAXNGP: xmlErrorDomain = 18;
pub const xmlErrorDomain_XML_FROM_RELAXNGV: xmlErrorDomain = 19;
pub const xmlErrorDomain_XML_FROM_CATALOG: xmlErrorDomain = 20;
pub const xmlErrorDomain_XML_FROM_C14N: xmlErrorDomain = 21;
pub const xmlErrorDomain_XML_FROM_XSLT: xmlErrorDomain = 22;
pub const xmlErrorDomain_XML_FROM_VALID: xmlErrorDomain = 23;
pub const xmlErrorDomain_XML_FROM_CHECK: xmlErrorDomain = 24;
pub const xmlErrorDomain_XML_FROM_WRITER: xmlErrorDomain = 25;
pub const xmlErrorDomain_XML_FROM_MODULE: xmlErrorDomain = 26;
pub const xmlErrorDomain_XML_FROM_I18N: xmlErrorDomain = 27;
pub const xmlErrorDomain_XML_FROM_SCHEMATRONV: xmlErrorDomain = 28;
pub const xmlErrorDomain_XML_FROM_BUFFER: xmlErrorDomain = 29;
pub const xmlErrorDomain_XML_FROM_URI: xmlErrorDomain = 30;
pub type xmlErrorDomain = ::std::os::raw::c_uint;
pub type xmlError = _xmlError;
pub type xmlErrorPtr = *mut xmlError;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlError {
    pub domain: ::std::os::raw::c_int,
    pub code: ::std::os::raw::c_int,
    pub message: *mut ::std::os::raw::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut ::std::os::raw::c_char,
    pub line: ::std::os::raw::c_int,
    pub str1: *mut ::std::os::raw::c_char,
    pub str2: *mut ::std::os::raw::c_char,
    pub str3: *mut ::std::os::raw::c_char,
    pub int1: ::std::os::raw::c_int,
    pub int2: ::std::os::raw::c_int,
    pub ctxt: *mut ::std::os::raw::c_void,
    pub node: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlError() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlError> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlError>(),
        88usize,
        concat!("Size of: ", stringify!(_xmlError))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlError>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlError))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).domain) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(domain)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).code) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).message) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(message)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).level) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(level)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).file) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(file)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str1) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(str1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str2) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(str2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str3) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(str3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).int1) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(int1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).int2) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(int2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ctxt) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(ctxt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlError),
            "::",
            stringify!(node)
        )
    );
}
pub const xmlParserErrors_XML_ERR_OK: xmlParserErrors = 0;
pub const xmlParserErrors_XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const xmlParserErrors_XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const xmlParserErrors_XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const xmlParserErrors_XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const xmlParserErrors_XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const xmlParserErrors_XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const xmlParserErrors_XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const xmlParserErrors_XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const xmlParserErrors_XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const xmlParserErrors_XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const xmlParserErrors_XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const xmlParserErrors_XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const xmlParserErrors_XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const xmlParserErrors_XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const xmlParserErrors_XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const xmlParserErrors_XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const xmlParserErrors_XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const xmlParserErrors_XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const xmlParserErrors_XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const xmlParserErrors_XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const xmlParserErrors_XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const xmlParserErrors_XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const xmlParserErrors_XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const xmlParserErrors_XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const xmlParserErrors_XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const xmlParserErrors_XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const xmlParserErrors_XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const xmlParserErrors_XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const xmlParserErrors_XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const xmlParserErrors_XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const xmlParserErrors_XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const xmlParserErrors_XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const xmlParserErrors_XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const xmlParserErrors_XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const xmlParserErrors_XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const xmlParserErrors_XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const xmlParserErrors_XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const xmlParserErrors_XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const xmlParserErrors_XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const xmlParserErrors_XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const xmlParserErrors_XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const xmlParserErrors_XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const xmlParserErrors_XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const xmlParserErrors_XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const xmlParserErrors_XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const xmlParserErrors_XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const xmlParserErrors_XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const xmlParserErrors_XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const xmlParserErrors_XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const xmlParserErrors_XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const xmlParserErrors_XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const xmlParserErrors_XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const xmlParserErrors_XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const xmlParserErrors_XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const xmlParserErrors_XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const xmlParserErrors_XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const xmlParserErrors_XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const xmlParserErrors_XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const xmlParserErrors_XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const xmlParserErrors_XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const xmlParserErrors_XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const xmlParserErrors_XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const xmlParserErrors_XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const xmlParserErrors_XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const xmlParserErrors_XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const xmlParserErrors_XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const xmlParserErrors_XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const xmlParserErrors_XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const xmlParserErrors_XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const xmlParserErrors_XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const xmlParserErrors_XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const xmlParserErrors_XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const xmlParserErrors_XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const xmlParserErrors_XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const xmlParserErrors_XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const xmlParserErrors_XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const xmlParserErrors_XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const xmlParserErrors_XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const xmlParserErrors_XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const xmlParserErrors_XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const xmlParserErrors_XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const xmlParserErrors_XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const xmlParserErrors_XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const xmlParserErrors_XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const xmlParserErrors_XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const xmlParserErrors_XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const xmlParserErrors_XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const xmlParserErrors_XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const xmlParserErrors_XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const xmlParserErrors_XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const xmlParserErrors_XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const xmlParserErrors_XML_WAR_NS_URI: xmlParserErrors = 99;
pub const xmlParserErrors_XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const xmlParserErrors_XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const xmlParserErrors_XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const xmlParserErrors_XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const xmlParserErrors_XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const xmlParserErrors_XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const xmlParserErrors_XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const xmlParserErrors_XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const xmlParserErrors_XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const xmlParserErrors_XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const xmlParserErrors_XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const xmlParserErrors_XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const xmlParserErrors_XML_ERR_COMMENT_ABRUPTLY_ENDED: xmlParserErrors = 112;
pub const xmlParserErrors_XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const xmlParserErrors_XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const xmlParserErrors_XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const xmlParserErrors_XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const xmlParserErrors_XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const xmlParserErrors_XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const xmlParserErrors_XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const xmlParserErrors_XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const xmlParserErrors_XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const xmlParserErrors_XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const xmlParserErrors_XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const xmlParserErrors_XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const xmlParserErrors_XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const xmlParserErrors_XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const xmlParserErrors_XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const xmlParserErrors_XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const xmlParserErrors_XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const xmlParserErrors_XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const xmlParserErrors_XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const xmlParserErrors_XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const xmlParserErrors_XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const xmlParserErrors_XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const xmlParserErrors_XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const xmlParserErrors_XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const xmlParserErrors_XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const xmlParserErrors_XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const xmlParserErrors_XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const xmlParserErrors_XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const xmlParserErrors_XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const xmlParserErrors_XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const xmlParserErrors_XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const xmlParserErrors_XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const xmlParserErrors_XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const xmlParserErrors_XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const xmlParserErrors_XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const xmlParserErrors_XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const xmlParserErrors_XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const xmlParserErrors_XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const xmlParserErrors_XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const xmlParserErrors_XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const xmlParserErrors_XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const xmlParserErrors_XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const xmlParserErrors_XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const xmlParserErrors_XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const xmlParserErrors_XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const xmlParserErrors_XML_HTML_INCORRECTLY_OPENED_COMMENT: xmlParserErrors = 802;
pub const xmlParserErrors_XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const xmlParserErrors_XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const xmlParserErrors_XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const xmlParserErrors_XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const xmlParserErrors_XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const xmlParserErrors_XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const xmlParserErrors_XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const xmlParserErrors_XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const xmlParserErrors_XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const xmlParserErrors_XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const xmlParserErrors_XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const xmlParserErrors_XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const xmlParserErrors_XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const xmlParserErrors_XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const xmlParserErrors_XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const xmlParserErrors_XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const xmlParserErrors_XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const xmlParserErrors_XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const xmlParserErrors_XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const xmlParserErrors_XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const xmlParserErrors_XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const xmlParserErrors_XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const xmlParserErrors_XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const xmlParserErrors_XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const xmlParserErrors_XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const xmlParserErrors_XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const xmlParserErrors_XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const xmlParserErrors_XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const xmlParserErrors_XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const xmlParserErrors_XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const xmlParserErrors_XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const xmlParserErrors_XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const xmlParserErrors_XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const xmlParserErrors_XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const xmlParserErrors_XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const xmlParserErrors_XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const xmlParserErrors_XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const xmlParserErrors_XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const xmlParserErrors_XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const xmlParserErrors_XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const xmlParserErrors_XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const xmlParserErrors_XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const xmlParserErrors_XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const xmlParserErrors_XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const xmlParserErrors_XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const xmlParserErrors_XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const xmlParserErrors_XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const xmlParserErrors_XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const xmlParserErrors_XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const xmlParserErrors_XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const xmlParserErrors_XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const xmlParserErrors_XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const xmlParserErrors_XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const xmlParserErrors_XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const xmlParserErrors_XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const xmlParserErrors_XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const xmlParserErrors_XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const xmlParserErrors_XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const xmlParserErrors_XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const xmlParserErrors_XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const xmlParserErrors_XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const xmlParserErrors_XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const xmlParserErrors_XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const xmlParserErrors_XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const xmlParserErrors_XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const xmlParserErrors_XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const xmlParserErrors_XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const xmlParserErrors_XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const xmlParserErrors_XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const xmlParserErrors_XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const xmlParserErrors_XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const xmlParserErrors_XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const xmlParserErrors_XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const xmlParserErrors_XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const xmlParserErrors_XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const xmlParserErrors_XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const xmlParserErrors_XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const xmlParserErrors_XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const xmlParserErrors_XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const xmlParserErrors_XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const xmlParserErrors_XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const xmlParserErrors_XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const xmlParserErrors_XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const xmlParserErrors_XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const xmlParserErrors_XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const xmlParserErrors_XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const xmlParserErrors_XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const xmlParserErrors_XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const xmlParserErrors_XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const xmlParserErrors_XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const xmlParserErrors_XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const xmlParserErrors_XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const xmlParserErrors_XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const xmlParserErrors_XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const xmlParserErrors_XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const xmlParserErrors_XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const xmlParserErrors_XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const xmlParserErrors_XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const xmlParserErrors_XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const xmlParserErrors_XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const xmlParserErrors_XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const xmlParserErrors_XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const xmlParserErrors_XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const xmlParserErrors_XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const xmlParserErrors_XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const xmlParserErrors_XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const xmlParserErrors_XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const xmlParserErrors_XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const xmlParserErrors_XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const xmlParserErrors_XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const xmlParserErrors_XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const xmlParserErrors_XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const xmlParserErrors_XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const xmlParserErrors_XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const xmlParserErrors_XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const xmlParserErrors_XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const xmlParserErrors_XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const xmlParserErrors_XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const xmlParserErrors_XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const xmlParserErrors_XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const xmlParserErrors_XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const xmlParserErrors_XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const xmlParserErrors_XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const xmlParserErrors_XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const xmlParserErrors_XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const xmlParserErrors_XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const xmlParserErrors_XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const xmlParserErrors_XML_IO_EACCES: xmlParserErrors = 1501;
pub const xmlParserErrors_XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const xmlParserErrors_XML_IO_EBADF: xmlParserErrors = 1503;
pub const xmlParserErrors_XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const xmlParserErrors_XML_IO_EBUSY: xmlParserErrors = 1505;
pub const xmlParserErrors_XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const xmlParserErrors_XML_IO_ECHILD: xmlParserErrors = 1507;
pub const xmlParserErrors_XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const xmlParserErrors_XML_IO_EDOM: xmlParserErrors = 1509;
pub const xmlParserErrors_XML_IO_EEXIST: xmlParserErrors = 1510;
pub const xmlParserErrors_XML_IO_EFAULT: xmlParserErrors = 1511;
pub const xmlParserErrors_XML_IO_EFBIG: xmlParserErrors = 1512;
pub const xmlParserErrors_XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const xmlParserErrors_XML_IO_EINTR: xmlParserErrors = 1514;
pub const xmlParserErrors_XML_IO_EINVAL: xmlParserErrors = 1515;
pub const xmlParserErrors_XML_IO_EIO: xmlParserErrors = 1516;
pub const xmlParserErrors_XML_IO_EISDIR: xmlParserErrors = 1517;
pub const xmlParserErrors_XML_IO_EMFILE: xmlParserErrors = 1518;
pub const xmlParserErrors_XML_IO_EMLINK: xmlParserErrors = 1519;
pub const xmlParserErrors_XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const xmlParserErrors_XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const xmlParserErrors_XML_IO_ENFILE: xmlParserErrors = 1522;
pub const xmlParserErrors_XML_IO_ENODEV: xmlParserErrors = 1523;
pub const xmlParserErrors_XML_IO_ENOENT: xmlParserErrors = 1524;
pub const xmlParserErrors_XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const xmlParserErrors_XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const xmlParserErrors_XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const xmlParserErrors_XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const xmlParserErrors_XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const xmlParserErrors_XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const xmlParserErrors_XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const xmlParserErrors_XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const xmlParserErrors_XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const xmlParserErrors_XML_IO_ENXIO: xmlParserErrors = 1534;
pub const xmlParserErrors_XML_IO_EPERM: xmlParserErrors = 1535;
pub const xmlParserErrors_XML_IO_EPIPE: xmlParserErrors = 1536;
pub const xmlParserErrors_XML_IO_ERANGE: xmlParserErrors = 1537;
pub const xmlParserErrors_XML_IO_EROFS: xmlParserErrors = 1538;
pub const xmlParserErrors_XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const xmlParserErrors_XML_IO_ESRCH: xmlParserErrors = 1540;
pub const xmlParserErrors_XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const xmlParserErrors_XML_IO_EXDEV: xmlParserErrors = 1542;
pub const xmlParserErrors_XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const xmlParserErrors_XML_IO_ENCODER: xmlParserErrors = 1544;
pub const xmlParserErrors_XML_IO_FLUSH: xmlParserErrors = 1545;
pub const xmlParserErrors_XML_IO_WRITE: xmlParserErrors = 1546;
pub const xmlParserErrors_XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const xmlParserErrors_XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const xmlParserErrors_XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const xmlParserErrors_XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const xmlParserErrors_XML_IO_EISCONN: xmlParserErrors = 1551;
pub const xmlParserErrors_XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const xmlParserErrors_XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const xmlParserErrors_XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const xmlParserErrors_XML_IO_EALREADY: xmlParserErrors = 1555;
pub const xmlParserErrors_XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const xmlParserErrors_XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const xmlParserErrors_XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const xmlParserErrors_XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const xmlParserErrors_XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const xmlParserErrors_XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const xmlParserErrors_XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const xmlParserErrors_XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const xmlParserErrors_XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const xmlParserErrors_XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const xmlParserErrors_XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const xmlParserErrors_XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const xmlParserErrors_XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const xmlParserErrors_XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const xmlParserErrors_XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const xmlParserErrors_XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const xmlParserErrors_XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const xmlParserErrors_XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const xmlParserErrors_XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const xmlParserErrors_XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const xmlParserErrors_XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const xmlParserErrors_XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const xmlParserErrors_XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const xmlParserErrors_XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const xmlParserErrors_XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const xmlParserErrors_XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const xmlParserErrors_XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const xmlParserErrors_XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const xmlParserErrors_XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const xmlParserErrors_XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const xmlParserErrors_XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const xmlParserErrors_XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const xmlParserErrors_XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const xmlParserErrors_XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const xmlParserErrors_XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const xmlParserErrors_XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const xmlParserErrors_XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const xmlParserErrors_XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const xmlParserErrors_XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const xmlParserErrors_XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const xmlParserErrors_XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const xmlParserErrors_XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const xmlParserErrors_XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const xmlParserErrors_XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const xmlParserErrors_XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const xmlParserErrors_XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const xmlParserErrors_XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const xmlParserErrors_XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const xmlParserErrors_XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const xmlParserErrors_XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const xmlParserErrors_XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const xmlParserErrors_XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const xmlParserErrors_XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const xmlParserErrors_XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const xmlParserErrors_XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const xmlParserErrors_XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const xmlParserErrors_XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const xmlParserErrors_XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const xmlParserErrors_XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const xmlParserErrors_XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const xmlParserErrors_XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const xmlParserErrors_XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const xmlParserErrors_XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const xmlParserErrors_XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const xmlParserErrors_XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const xmlParserErrors_XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const xmlParserErrors_XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const xmlParserErrors_XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const xmlParserErrors_XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const xmlParserErrors_XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const xmlParserErrors_XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const xmlParserErrors_XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const xmlParserErrors_XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const xmlParserErrors_XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const xmlParserErrors_XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const xmlParserErrors_XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const xmlParserErrors_XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const xmlParserErrors_XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const xmlParserErrors_XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const xmlParserErrors_XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const xmlParserErrors_XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const xmlParserErrors_XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const xmlParserErrors_XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const xmlParserErrors_XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const xmlParserErrors_XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const xmlParserErrors_XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const xmlParserErrors_XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const xmlParserErrors_XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const xmlParserErrors_XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const xmlParserErrors_XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const xmlParserErrors_XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const xmlParserErrors_XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const xmlParserErrors_XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const xmlParserErrors_XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const xmlParserErrors_XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const xmlParserErrors_XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const xmlParserErrors_XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const xmlParserErrors_XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const xmlParserErrors_XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const xmlParserErrors_XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const xmlParserErrors_XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const xmlParserErrors_XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const xmlParserErrors_XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const xmlParserErrors_XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const xmlParserErrors_XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const xmlParserErrors_XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const xmlParserErrors_XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const xmlParserErrors_XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const xmlParserErrors_XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const xmlParserErrors_XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const xmlParserErrors_XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const xmlParserErrors_XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const xmlParserErrors_XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const xmlParserErrors_XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const xmlParserErrors_XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const xmlParserErrors_XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const xmlParserErrors_XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const xmlParserErrors_XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const xmlParserErrors_XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const xmlParserErrors_XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const xmlParserErrors_XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors = 3005;
pub const xmlParserErrors_XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const xmlParserErrors_XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors = 3007;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const xmlParserErrors_XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const xmlParserErrors_XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const xmlParserErrors_XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const xmlParserErrors_XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const xmlParserErrors_XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const xmlParserErrors_XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const xmlParserErrors_XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const xmlParserErrors_XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const xmlParserErrors_XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const xmlParserErrors_XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const xmlParserErrors_XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const xmlParserErrors_XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const xmlParserErrors_XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const xmlParserErrors_XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const xmlParserErrors_XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const xmlParserErrors_XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const xmlParserErrors_XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const xmlParserErrors_XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const xmlParserErrors_XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const xmlParserErrors_XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const xmlParserErrors_XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const xmlParserErrors_XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const xmlParserErrors_XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const xmlParserErrors_XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const xmlParserErrors_XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const xmlParserErrors_XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const xmlParserErrors_XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const xmlParserErrors_XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const xmlParserErrors_XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const xmlParserErrors_XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const xmlParserErrors_XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const xmlParserErrors_XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const xmlParserErrors_XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const xmlParserErrors_XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const xmlParserErrors_XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const xmlParserErrors_XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const xmlParserErrors_XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const xmlParserErrors_XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const xmlParserErrors_XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const xmlParserErrors_XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const xmlParserErrors_XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const xmlParserErrors_XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const xmlParserErrors_XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const xmlParserErrors_XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const xmlParserErrors_XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const xmlParserErrors_XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const xmlParserErrors_XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const xmlParserErrors_XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const xmlParserErrors_XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const xmlParserErrors_XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const xmlParserErrors_XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const xmlParserErrors_XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const xmlParserErrors_XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const xmlParserErrors_XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const xmlParserErrors_XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const xmlParserErrors_XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const xmlParserErrors_XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const xmlParserErrors_XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const xmlParserErrors_XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const xmlParserErrors_XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const xmlParserErrors_XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const xmlParserErrors_XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const xmlParserErrors_XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const xmlParserErrors_XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const xmlParserErrors_XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const xmlParserErrors_XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const xmlParserErrors_XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const xmlParserErrors_XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const xmlParserErrors_XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const xmlParserErrors_XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const xmlParserErrors_XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const xmlParserErrors_XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const xmlParserErrors_XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const xmlParserErrors_XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const xmlParserErrors_XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const xmlParserErrors_XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const xmlParserErrors_XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const xmlParserErrors_XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const xmlParserErrors_XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub type xmlParserErrors = ::std::os::raw::c_uint;
pub type xmlGenericErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlStructuredErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void, error: xmlErrorPtr),
>;
extern "C" {
    pub fn xmlSetGenericErrorFunc(ctx: *mut ::std::os::raw::c_void, handler: xmlGenericErrorFunc);
}
extern "C" {
    pub fn initGenericErrorDefaultFunc(handler: *mut xmlGenericErrorFunc);
}
extern "C" {
    pub fn xmlSetStructuredErrorFunc(
        ctx: *mut ::std::os::raw::c_void,
        handler: xmlStructuredErrorFunc,
    );
}
extern "C" {
    pub fn xmlParserError(
        ctx: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn xmlParserWarning(
        ctx: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn xmlParserValidityError(
        ctx: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn xmlParserValidityWarning(
        ctx: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn xmlParserPrintFileInfo(input: xmlParserInputPtr);
}
extern "C" {
    pub fn xmlParserPrintFileContext(input: xmlParserInputPtr);
}
extern "C" {
    pub fn xmlGetLastError() -> xmlErrorPtr;
}
extern "C" {
    pub fn xmlResetLastError();
}
extern "C" {
    pub fn xmlCtxtGetLastError(ctx: *mut ::std::os::raw::c_void) -> xmlErrorPtr;
}
extern "C" {
    pub fn xmlCtxtResetLastError(ctx: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlResetError(err: xmlErrorPtr);
}
extern "C" {
    pub fn xmlCopyError(from: xmlErrorPtr, to: xmlErrorPtr) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlLink {
    _unused: [u8; 0],
}
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlList {
    _unused: [u8; 0],
}
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = ::std::option::Option<unsafe extern "C" fn(lk: xmlLinkPtr)>;
pub type xmlListDataCompare = ::std::option::Option<
    unsafe extern "C" fn(
        data0: *const ::std::os::raw::c_void,
        data1: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlListWalker = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const ::std::os::raw::c_void,
        user: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn xmlListCreate(
        deallocator: xmlListDeallocator,
        compare: xmlListDataCompare,
    ) -> xmlListPtr;
}
extern "C" {
    pub fn xmlListDelete(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListSearch(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlListReverseSearch(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlListInsert(l: xmlListPtr, data: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListAppend(l: xmlListPtr, data: *mut ::std::os::raw::c_void)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListRemoveFirst(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListRemoveLast(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListRemoveAll(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListClear(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListEmpty(l: xmlListPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListFront(l: xmlListPtr) -> xmlLinkPtr;
}
extern "C" {
    pub fn xmlListEnd(l: xmlListPtr) -> xmlLinkPtr;
}
extern "C" {
    pub fn xmlListSize(l: xmlListPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListPopFront(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListPopBack(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListPushFront(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListPushBack(
        l: xmlListPtr,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlListReverse(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListSort(l: xmlListPtr);
}
extern "C" {
    pub fn xmlListWalk(l: xmlListPtr, walker: xmlListWalker, user: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlListReverseWalk(
        l: xmlListPtr,
        walker: xmlListWalker,
        user: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlListMerge(l1: xmlListPtr, l2: xmlListPtr);
}
extern "C" {
    pub fn xmlListDup(old: xmlListPtr) -> xmlListPtr;
}
extern "C" {
    pub fn xmlListCopy(cur: xmlListPtr, old: xmlListPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlLinkGetData(lk: xmlLinkPtr) -> *mut ::std::os::raw::c_void;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlAutomata {
    _unused: [u8; 0],
}
pub type xmlAutomata = _xmlAutomata;
pub type xmlAutomataPtr = *mut xmlAutomata;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlAutomataState {
    _unused: [u8; 0],
}
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
extern "C" {
    pub fn xmlNewAutomata() -> xmlAutomataPtr;
}
extern "C" {
    pub fn xmlFreeAutomata(am: xmlAutomataPtr);
}
extern "C" {
    pub fn xmlAutomataGetInitState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataSetFinalState(
        am: xmlAutomataPtr,
        state: xmlAutomataStatePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlAutomataNewState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewTransition(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewTransition2(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        token2: *const xmlChar,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewNegTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        token2: *const xmlChar,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewCountTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewCountTrans2(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        token2: *const xmlChar,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewOnceTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewOnceTrans2(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        token2: *const xmlChar,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewAllTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        lax: ::std::os::raw::c_int,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewEpsilon(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewCountedTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        counter: ::std::os::raw::c_int,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewCounterTrans(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        counter: ::std::os::raw::c_int,
    ) -> xmlAutomataStatePtr;
}
extern "C" {
    pub fn xmlAutomataNewCounter(
        am: xmlAutomataPtr,
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlAutomataCompile(am: xmlAutomataPtr) -> xmlRegexpPtr;
}
extern "C" {
    pub fn xmlAutomataIsDeterminist(am: xmlAutomataPtr) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlValidState {
    _unused: [u8; 0],
}
pub type xmlValidState = _xmlValidState;
pub type xmlValidStatePtr = *mut xmlValidState;
pub type xmlValidityErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlValidityWarningFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlValidCtxt = _xmlValidCtxt;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlValidCtxt {
    pub userData: *mut ::std::os::raw::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: ::std::os::raw::c_int,
    pub nodeMax: ::std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: ::std::os::raw::c_uint,
    pub doc: xmlDocPtr,
    pub valid: ::std::os::raw::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: ::std::os::raw::c_int,
    pub vstateMax: ::std::os::raw::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
#[test]
fn bindgen_test_layout__xmlValidCtxt() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlValidCtxt> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlValidCtxt>(),
        112usize,
        concat!("Size of: ", stringify!(_xmlValidCtxt))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlValidCtxt>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlValidCtxt))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).userData) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(userData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).warning) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(warning)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeNr) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(nodeNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeMax) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(nodeMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeTab) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(nodeTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valid) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(valid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vstate) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(vstate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vstateNr) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(vstateNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vstateMax) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(vstateMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vstateTab) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(vstateTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).am) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(am)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlValidCtxt),
            "::",
            stringify!(state)
        )
    );
}
pub type xmlNotationTable = _xmlHashTable;
pub type xmlNotationTablePtr = *mut xmlNotationTable;
pub type xmlElementTable = _xmlHashTable;
pub type xmlElementTablePtr = *mut xmlElementTable;
pub type xmlAttributeTable = _xmlHashTable;
pub type xmlAttributeTablePtr = *mut xmlAttributeTable;
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
extern "C" {
    pub fn xmlAddNotationDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        PublicID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlNotationPtr;
}
extern "C" {
    pub fn xmlCopyNotationTable(table: xmlNotationTablePtr) -> xmlNotationTablePtr;
}
extern "C" {
    pub fn xmlFreeNotationTable(table: xmlNotationTablePtr);
}
extern "C" {
    pub fn xmlDumpNotationDecl(buf: xmlBufferPtr, nota: xmlNotationPtr);
}
extern "C" {
    pub fn xmlDumpNotationTable(buf: xmlBufferPtr, table: xmlNotationTablePtr);
}
extern "C" {
    pub fn xmlNewElementContent(
        name: *const xmlChar,
        type_: xmlElementContentType,
    ) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlCopyElementContent(content: xmlElementContentPtr) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlFreeElementContent(cur: xmlElementContentPtr);
}
extern "C" {
    pub fn xmlNewDocElementContent(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_: xmlElementContentType,
    ) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlCopyDocElementContent(
        doc: xmlDocPtr,
        content: xmlElementContentPtr,
    ) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlFreeDocElementContent(doc: xmlDocPtr, cur: xmlElementContentPtr);
}
extern "C" {
    pub fn xmlSnprintfElementContent(
        buf: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        content: xmlElementContentPtr,
        englob: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlSprintfElementContent(
        buf: *mut ::std::os::raw::c_char,
        content: xmlElementContentPtr,
        englob: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlAddElementDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        type_: xmlElementTypeVal,
        content: xmlElementContentPtr,
    ) -> xmlElementPtr;
}
extern "C" {
    pub fn xmlCopyElementTable(table: xmlElementTablePtr) -> xmlElementTablePtr;
}
extern "C" {
    pub fn xmlFreeElementTable(table: xmlElementTablePtr);
}
extern "C" {
    pub fn xmlDumpElementTable(buf: xmlBufferPtr, table: xmlElementTablePtr);
}
extern "C" {
    pub fn xmlDumpElementDecl(buf: xmlBufferPtr, elem: xmlElementPtr);
}
extern "C" {
    pub fn xmlCreateEnumeration(name: *const xmlChar) -> xmlEnumerationPtr;
}
extern "C" {
    pub fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
}
extern "C" {
    pub fn xmlCopyEnumeration(cur: xmlEnumerationPtr) -> xmlEnumerationPtr;
}
extern "C" {
    pub fn xmlAddAttributeDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
        ns: *const xmlChar,
        type_: xmlAttributeType,
        def: xmlAttributeDefault,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    ) -> xmlAttributePtr;
}
extern "C" {
    pub fn xmlCopyAttributeTable(table: xmlAttributeTablePtr) -> xmlAttributeTablePtr;
}
extern "C" {
    pub fn xmlFreeAttributeTable(table: xmlAttributeTablePtr);
}
extern "C" {
    pub fn xmlDumpAttributeTable(buf: xmlBufferPtr, table: xmlAttributeTablePtr);
}
extern "C" {
    pub fn xmlDumpAttributeDecl(buf: xmlBufferPtr, attr: xmlAttributePtr);
}
extern "C" {
    pub fn xmlAddID(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlIDPtr;
}
extern "C" {
    pub fn xmlFreeIDTable(table: xmlIDTablePtr);
}
extern "C" {
    pub fn xmlGetID(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr;
}
extern "C" {
    pub fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRemoveID(doc: xmlDocPtr, attr: xmlAttrPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlAddRef(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlRefPtr;
}
extern "C" {
    pub fn xmlFreeRefTable(table: xmlRefTablePtr);
}
extern "C" {
    pub fn xmlIsRef(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRemoveRef(doc: xmlDocPtr, attr: xmlAttrPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGetRefs(doc: xmlDocPtr, ID: *const xmlChar) -> xmlListPtr;
}
extern "C" {
    pub fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
}
extern "C" {
    pub fn xmlFreeValidCtxt(arg1: xmlValidCtxtPtr);
}
extern "C" {
    pub fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateElementDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlElementPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidNormalizeAttributeValue(
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlValidCtxtNormalizeAttributeValue(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlValidateAttributeDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        attr: xmlAttributePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateAttributeValue(
        type_: xmlAttributeType,
        value: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNotationDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        nota: xmlNotationPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateDtd(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        dtd: xmlDtdPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateDtdFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateOneElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateOneAttribute(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        attr: xmlAttrPtr,
        value: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateOneNamespace(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        prefix: *const xmlChar,
        ns: xmlNsPtr,
        value: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateDocumentFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNotationUse(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        notationName: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsMixedElement(doc: xmlDocPtr, name: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGetDtdAttrDesc(
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
    ) -> xmlAttributePtr;
}
extern "C" {
    pub fn xmlGetDtdQAttrDesc(
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
        prefix: *const xmlChar,
    ) -> xmlAttributePtr;
}
extern "C" {
    pub fn xmlGetDtdNotationDesc(dtd: xmlDtdPtr, name: *const xmlChar) -> xmlNotationPtr;
}
extern "C" {
    pub fn xmlGetDtdQElementDesc(
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
    ) -> xmlElementPtr;
}
extern "C" {
    pub fn xmlGetDtdElementDesc(dtd: xmlDtdPtr, name: *const xmlChar) -> xmlElementPtr;
}
extern "C" {
    pub fn xmlValidGetPotentialChildren(
        ctree: *mut xmlElementContent,
        names: *mut *const xmlChar,
        len: *mut ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidGetValidElements(
        prev: *mut xmlNode,
        next: *mut xmlNode,
        names: *mut *const xmlChar,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNameValue(value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNamesValue(value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNmtokenValue(value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidateNmtokensValue(value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidBuildContentModel(
        ctxt: xmlValidCtxtPtr,
        elem: xmlElementPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidatePushElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidatePushCData(
        ctxt: xmlValidCtxtPtr,
        data: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlValidatePopElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
pub const xmlEntityType_XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub const xmlEntityType_XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const xmlEntityType_XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const xmlEntityType_XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const xmlEntityType_XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const xmlEntityType_XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub type xmlEntityType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlEntity {
    pub _private: *mut ::std::os::raw::c_void,
    pub type_: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: ::std::os::raw::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: ::std::os::raw::c_int,
    pub checked: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlEntity() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlEntity> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlEntity>(),
        136usize,
        concat!("Size of: ", stringify!(_xmlEntity))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlEntity>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlEntity))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).children) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(children)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parent) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prev) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).orig) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(orig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).content) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(content)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).etype) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(etype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ExternalID) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(ExternalID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SystemID) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(SystemID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nexte) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(nexte)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).URI) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(URI)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).owner) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).checked) as usize - ptr as usize },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlEntity),
            "::",
            stringify!(checked)
        )
    );
}
pub type xmlEntitiesTable = _xmlHashTable;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
extern "C" {
    pub fn xmlNewEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlAddDocEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlAddDtdEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlGetDtdEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlGetParameterEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlEncodeSpecialChars(doc: *const xmlDoc, input: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCreateEntitiesTable() -> xmlEntitiesTablePtr;
}
extern "C" {
    pub fn xmlCopyEntitiesTable(table: xmlEntitiesTablePtr) -> xmlEntitiesTablePtr;
}
extern "C" {
    pub fn xmlFreeEntitiesTable(table: xmlEntitiesTablePtr);
}
extern "C" {
    pub fn xmlDumpEntitiesTable(buf: xmlBufferPtr, table: xmlEntitiesTablePtr);
}
extern "C" {
    pub fn xmlDumpEntityDecl(buf: xmlBufferPtr, ent: xmlEntityPtr);
}
pub type xmlParserInputDeallocate = ::std::option::Option<unsafe extern "C" fn(str_: *mut xmlChar)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const ::std::os::raw::c_char,
    pub directory: *const ::std::os::raw::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: ::std::os::raw::c_int,
    pub line: ::std::os::raw::c_int,
    pub col: ::std::os::raw::c_int,
    pub consumed: ::std::os::raw::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: ::std::os::raw::c_int,
    pub id: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlParserInput() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlParserInput> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlParserInput>(),
        104usize,
        concat!("Size of: ", stringify!(_xmlParserInput))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlParserInput>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlParserInput))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buf) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).filename) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(filename)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).directory) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(directory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).base) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cur) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(cur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).end) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).col) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(col)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).consumed) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(consumed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).free) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encoding) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(encoding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).standalone) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(standalone)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInput),
            "::",
            stringify!(id)
        )
    );
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
pub type xmlParserNodeInfoPtr = *mut xmlParserNodeInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: ::std::os::raw::c_ulong,
    pub begin_line: ::std::os::raw::c_ulong,
    pub end_pos: ::std::os::raw::c_ulong,
    pub end_line: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout__xmlParserNodeInfo() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlParserNodeInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlParserNodeInfo>(),
        40usize,
        concat!("Size of: ", stringify!(_xmlParserNodeInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlParserNodeInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlParserNodeInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfo),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).begin_pos) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfo),
            "::",
            stringify!(begin_pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).begin_line) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfo),
            "::",
            stringify!(begin_line)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).end_pos) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfo),
            "::",
            stringify!(end_pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).end_line) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfo),
            "::",
            stringify!(end_line)
        )
    );
}
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
pub type xmlParserNodeInfoSeqPtr = *mut xmlParserNodeInfoSeq;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: ::std::os::raw::c_ulong,
    pub length: ::std::os::raw::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[test]
fn bindgen_test_layout__xmlParserNodeInfoSeq() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlParserNodeInfoSeq> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlParserNodeInfoSeq>(),
        24usize,
        concat!("Size of: ", stringify!(_xmlParserNodeInfoSeq))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlParserNodeInfoSeq>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlParserNodeInfoSeq))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maximum) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfoSeq),
            "::",
            stringify!(maximum)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfoSeq),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserNodeInfoSeq),
            "::",
            stringify!(buffer)
        )
    );
}
pub const xmlParserInputState_XML_PARSER_EOF: xmlParserInputState = -1;
pub const xmlParserInputState_XML_PARSER_START: xmlParserInputState = 0;
pub const xmlParserInputState_XML_PARSER_MISC: xmlParserInputState = 1;
pub const xmlParserInputState_XML_PARSER_PI: xmlParserInputState = 2;
pub const xmlParserInputState_XML_PARSER_DTD: xmlParserInputState = 3;
pub const xmlParserInputState_XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const xmlParserInputState_XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const xmlParserInputState_XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const xmlParserInputState_XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const xmlParserInputState_XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const xmlParserInputState_XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const xmlParserInputState_XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const xmlParserInputState_XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const xmlParserInputState_XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const xmlParserInputState_XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const xmlParserInputState_XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const xmlParserInputState_XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const xmlParserInputState_XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub type xmlParserInputState = ::std::os::raw::c_int;
pub const xmlParserMode_XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub const xmlParserMode_XML_PARSE_DOM: xmlParserMode = 1;
pub const xmlParserMode_XML_PARSE_SAX: xmlParserMode = 2;
pub const xmlParserMode_XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const xmlParserMode_XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const xmlParserMode_XML_PARSE_READER: xmlParserMode = 5;
pub type xmlParserMode = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlStartTag {
    _unused: [u8; 0],
}
pub type xmlStartTag = _xmlStartTag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut ::std::os::raw::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: ::std::os::raw::c_int,
    pub replaceEntities: ::std::os::raw::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: ::std::os::raw::c_int,
    pub html: ::std::os::raw::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: ::std::os::raw::c_int,
    pub inputMax: ::std::os::raw::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: ::std::os::raw::c_int,
    pub nodeMax: ::std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: ::std::os::raw::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: ::std::os::raw::c_int,
    pub hasExternalSubset: ::std::os::raw::c_int,
    pub hasPErefs: ::std::os::raw::c_int,
    pub external: ::std::os::raw::c_int,
    pub valid: ::std::os::raw::c_int,
    pub validate: ::std::os::raw::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: ::std::os::raw::c_int,
    pub directory: *mut ::std::os::raw::c_char,
    pub name: *const xmlChar,
    pub nameNr: ::std::os::raw::c_int,
    pub nameMax: ::std::os::raw::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: ::std::os::raw::c_long,
    pub checkIndex: ::std::os::raw::c_long,
    pub keepBlanks: ::std::os::raw::c_int,
    pub disableSAX: ::std::os::raw::c_int,
    pub inSubset: ::std::os::raw::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut ::std::os::raw::c_int,
    pub spaceNr: ::std::os::raw::c_int,
    pub spaceMax: ::std::os::raw::c_int,
    pub spaceTab: *mut ::std::os::raw::c_int,
    pub depth: ::std::os::raw::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: ::std::os::raw::c_int,
    pub nodelen: ::std::os::raw::c_int,
    pub nodemem: ::std::os::raw::c_int,
    pub pedantic: ::std::os::raw::c_int,
    pub _private: *mut ::std::os::raw::c_void,
    pub loadsubset: ::std::os::raw::c_int,
    pub linenumbers: ::std::os::raw::c_int,
    pub catalogs: *mut ::std::os::raw::c_void,
    pub recovery: ::std::os::raw::c_int,
    pub progressive: ::std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: ::std::os::raw::c_int,
    pub docdict: ::std::os::raw::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: ::std::os::raw::c_int,
    pub nsNr: ::std::os::raw::c_int,
    pub nsMax: ::std::os::raw::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut ::std::os::raw::c_int,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: ::std::os::raw::c_int,
    pub options: ::std::os::raw::c_int,
    pub dictNames: ::std::os::raw::c_int,
    pub freeElemsNr: ::std::os::raw::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: ::std::os::raw::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: ::std::os::raw::c_ulong,
    pub sizeentities: ::std::os::raw::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: ::std::os::raw::c_int,
    pub nodeInfoMax: ::std::os::raw::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: ::std::os::raw::c_int,
    pub sizeentcopy: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout__xmlParserCtxt() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlParserCtxt> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlParserCtxt>(),
        752usize,
        concat!("Size of: ", stringify!(_xmlParserCtxt))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlParserCtxt>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlParserCtxt))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sax) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(sax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).userData) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(userData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).myDoc) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(myDoc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wellFormed) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(wellFormed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).replaceEntities) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(replaceEntities)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encoding) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(encoding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).standalone) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(standalone)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).html) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(html)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inputNr) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(inputNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inputMax) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(inputMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inputTab) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(inputTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeNr) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeMax) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeTab) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).record_info) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(record_info)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node_seq) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(node_seq)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).errNo) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(errNo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasExternalSubset) as usize - ptr as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(hasExternalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasPErefs) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(hasPErefs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).external) as usize - ptr as usize },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(external)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valid) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(valid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).validate) as usize - ptr as usize },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(validate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vctxt) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(vctxt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).instate) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(instate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).token) as usize - ptr as usize },
        276usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(token)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).directory) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(directory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nameNr) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nameNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nameMax) as usize - ptr as usize },
        300usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nameMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nameTab) as usize - ptr as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nameTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nbChars) as usize - ptr as usize },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nbChars)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).checkIndex) as usize - ptr as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(checkIndex)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).keepBlanks) as usize - ptr as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(keepBlanks)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).disableSAX) as usize - ptr as usize },
        332usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(disableSAX)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inSubset) as usize - ptr as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(inSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).intSubName) as usize - ptr as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(intSubName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extSubURI) as usize - ptr as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(extSubURI)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extSubSystem) as usize - ptr as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(extSubSystem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).space) as usize - ptr as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(space)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).spaceNr) as usize - ptr as usize },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(spaceNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).spaceMax) as usize - ptr as usize },
        380usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(spaceMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).spaceTab) as usize - ptr as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(spaceTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).entity) as usize - ptr as usize },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(entity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).charset) as usize - ptr as usize },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(charset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodelen) as usize - ptr as usize },
        412usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodelen)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodemem) as usize - ptr as usize },
        416usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodemem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pedantic) as usize - ptr as usize },
        420usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(pedantic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).loadsubset) as usize - ptr as usize },
        432usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(loadsubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).linenumbers) as usize - ptr as usize },
        436usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(linenumbers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).catalogs) as usize - ptr as usize },
        440usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(catalogs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).recovery) as usize - ptr as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(recovery)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).progressive) as usize - ptr as usize },
        452usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(progressive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dict) as usize - ptr as usize },
        456usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(dict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atts) as usize - ptr as usize },
        464usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(atts)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxatts) as usize - ptr as usize },
        472usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(maxatts)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).docdict) as usize - ptr as usize },
        476usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(docdict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_xml) as usize - ptr as usize },
        480usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(str_xml)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_xmlns) as usize - ptr as usize },
        488usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(str_xmlns)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_xml_ns) as usize - ptr as usize },
        496usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(str_xml_ns)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sax2) as usize - ptr as usize },
        504usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(sax2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsNr) as usize - ptr as usize },
        508usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nsNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsMax) as usize - ptr as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nsMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsTab) as usize - ptr as usize },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nsTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attallocs) as usize - ptr as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(attallocs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pushTab) as usize - ptr as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(pushTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attsDefault) as usize - ptr as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(attsDefault)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attsSpecial) as usize - ptr as usize },
        552usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(attsSpecial)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsWellFormed) as usize - ptr as usize },
        560usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nsWellFormed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).options) as usize - ptr as usize },
        564usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dictNames) as usize - ptr as usize },
        568usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(dictNames)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freeElemsNr) as usize - ptr as usize },
        572usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(freeElemsNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freeElems) as usize - ptr as usize },
        576usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(freeElems)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freeAttrsNr) as usize - ptr as usize },
        584usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(freeAttrsNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freeAttrs) as usize - ptr as usize },
        592usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(freeAttrs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lastError) as usize - ptr as usize },
        600usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(lastError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).parseMode) as usize - ptr as usize },
        688usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(parseMode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nbentities) as usize - ptr as usize },
        696usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nbentities)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sizeentities) as usize - ptr as usize },
        704usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(sizeentities)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeInfo) as usize - ptr as usize },
        712usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeInfo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeInfoNr) as usize - ptr as usize },
        720usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeInfoNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeInfoMax) as usize - ptr as usize },
        724usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeInfoMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeInfoTab) as usize - ptr as usize },
        728usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(nodeInfoTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input_id) as usize - ptr as usize },
        736usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(input_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sizeentcopy) as usize - ptr as usize },
        744usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserCtxt),
            "::",
            stringify!(sizeentcopy)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSAXLocator {
    pub getPublicId: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> *const xmlChar,
    >,
    pub getSystemId: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> *const xmlChar,
    >,
    pub getLineNumber: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub getColumnNumber: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
}
#[test]
fn bindgen_test_layout__xmlSAXLocator() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSAXLocator> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSAXLocator>(),
        32usize,
        concat!("Size of: ", stringify!(_xmlSAXLocator))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSAXLocator>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSAXLocator))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getPublicId) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXLocator),
            "::",
            stringify!(getPublicId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getSystemId) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXLocator),
            "::",
            stringify!(getSystemId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getLineNumber) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXLocator),
            "::",
            stringify!(getLineNumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getColumnNumber) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXLocator),
            "::",
            stringify!(getColumnNumber)
        )
    );
}
pub type resolveEntitySAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type internalSubsetSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ),
>;
pub type externalSubsetSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ),
>;
pub type getEntitySAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar) -> xmlEntityPtr,
>;
pub type getParameterEntitySAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar) -> xmlEntityPtr,
>;
pub type entityDeclSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    ),
>;
pub type notationDeclSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ),
>;
pub type attributeDeclSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_: ::std::os::raw::c_int,
        def: ::std::os::raw::c_int,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    ),
>;
pub type elementDeclSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        content: xmlElementContentPtr,
    ),
>;
pub type unparsedEntityDeclSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    ),
>;
pub type setDocumentLocatorSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, loc: xmlSAXLocatorPtr),
>;
pub type startDocumentSAXFunc =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>;
pub type endDocumentSAXFunc =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>;
pub type startElementSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        atts: *mut *const xmlChar,
    ),
>;
pub type endElementSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar),
>;
pub type attributeSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        value: *const xmlChar,
    ),
>;
pub type referenceSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar),
>;
pub type charactersSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        ch: *const xmlChar,
        len: ::std::os::raw::c_int,
    ),
>;
pub type ignorableWhitespaceSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        ch: *const xmlChar,
        len: ::std::os::raw::c_int,
    ),
>;
pub type processingInstructionSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    ),
>;
pub type commentSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, value: *const xmlChar),
>;
pub type cdataBlockSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        value: *const xmlChar,
        len: ::std::os::raw::c_int,
    ),
>;
pub type warningSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type errorSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type fatalErrorSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type isStandaloneSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
pub type hasInternalSubsetSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
pub type hasExternalSubsetSAXFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
pub type startElementNsSAX2Func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        localname: *const xmlChar,
        prefix: *const xmlChar,
        URI: *const xmlChar,
        nb_namespaces: ::std::os::raw::c_int,
        namespaces: *mut *const xmlChar,
        nb_attributes: ::std::os::raw::c_int,
        nb_defaulted: ::std::os::raw::c_int,
        attributes: *mut *const xmlChar,
    ),
>;
pub type endElementNsSAX2Func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        localname: *const xmlChar,
        prefix: *const xmlChar,
        URI: *const xmlChar,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: ::std::os::raw::c_uint,
    pub _private: *mut ::std::os::raw::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
#[test]
fn bindgen_test_layout__xmlSAXHandler() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSAXHandler> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSAXHandler>(),
        256usize,
        concat!("Size of: ", stringify!(_xmlSAXHandler))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSAXHandler>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSAXHandler))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).internalSubset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(internalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).isStandalone) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(isStandalone)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasInternalSubset) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(hasInternalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasExternalSubset) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(hasExternalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resolveEntity) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(resolveEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getEntity) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(getEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).entityDecl) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(entityDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).notationDecl) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(notationDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeDecl) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(attributeDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elementDecl) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(elementDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unparsedEntityDecl) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(unparsedEntityDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).setDocumentLocator) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(setDocumentLocator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startDocument) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(startDocument)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endDocument) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(endDocument)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startElement) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(startElement)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endElement) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(endElement)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reference) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(reference)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).characters) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(characters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ignorableWhitespace) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(ignorableWhitespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).processingInstruction) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(processingInstruction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).comment) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(comment)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).warning) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(warning)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fatalError) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(fatalError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getParameterEntity) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(getParameterEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cdataBlock) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(cdataBlock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).externalSubset) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(externalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).initialized) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(initialized)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startElementNs) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(startElementNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endElementNs) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(endElementNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).serror) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandler),
            "::",
            stringify!(serror)
        )
    );
}
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
pub type xmlSAXHandlerV1Ptr = *mut xmlSAXHandlerV1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSAXHandlerV1 {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout__xmlSAXHandlerV1() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSAXHandlerV1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSAXHandlerV1>(),
        224usize,
        concat!("Size of: ", stringify!(_xmlSAXHandlerV1))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSAXHandlerV1>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSAXHandlerV1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).internalSubset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(internalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).isStandalone) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(isStandalone)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasInternalSubset) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(hasInternalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hasExternalSubset) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(hasExternalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resolveEntity) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(resolveEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getEntity) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(getEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).entityDecl) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(entityDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).notationDecl) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(notationDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeDecl) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(attributeDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elementDecl) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(elementDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unparsedEntityDecl) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(unparsedEntityDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).setDocumentLocator) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(setDocumentLocator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startDocument) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(startDocument)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endDocument) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(endDocument)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startElement) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(startElement)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endElement) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(endElement)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reference) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(reference)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).characters) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(characters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ignorableWhitespace) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(ignorableWhitespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).processingInstruction) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(processingInstruction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).comment) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(comment)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).warning) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(warning)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fatalError) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(fatalError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getParameterEntity) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(getParameterEntity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cdataBlock) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(cdataBlock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).externalSubset) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(externalSubset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).initialized) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSAXHandlerV1),
            "::",
            stringify!(initialized)
        )
    );
}
pub type xmlExternalEntityLoader = ::std::option::Option<
    unsafe extern "C" fn(
        URL: *const ::std::os::raw::c_char,
        ID: *const ::std::os::raw::c_char,
        context: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr,
>;
pub type iconv_t = *mut ::std::os::raw::c_void;
pub const xmlCharEncoding_XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
pub const xmlCharEncoding_XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const xmlCharEncoding_XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const xmlCharEncoding_XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const xmlCharEncoding_XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const xmlCharEncoding_XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const xmlCharEncoding_XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const xmlCharEncoding_XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const xmlCharEncoding_XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub type xmlCharEncoding = ::std::os::raw::c_int;
pub type xmlCharEncodingInputFunc = ::std::option::Option<
    unsafe extern "C" fn(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlCharEncodingOutputFunc = ::std::option::Option<
    unsafe extern "C" fn(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut ::std::os::raw::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
#[test]
fn bindgen_test_layout__xmlCharEncodingHandler() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlCharEncodingHandler> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlCharEncodingHandler>(),
        40usize,
        concat!("Size of: ", stringify!(_xmlCharEncodingHandler))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlCharEncodingHandler>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlCharEncodingHandler))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlCharEncodingHandler),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlCharEncodingHandler),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).output) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlCharEncodingHandler),
            "::",
            stringify!(output)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iconv_in) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlCharEncodingHandler),
            "::",
            stringify!(iconv_in)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iconv_out) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlCharEncodingHandler),
            "::",
            stringify!(iconv_out)
        )
    );
}
extern "C" {
    pub fn xmlInitCharEncodingHandlers();
}
extern "C" {
    pub fn xmlCleanupCharEncodingHandlers();
}
extern "C" {
    pub fn xmlRegisterCharEncodingHandler(handler: xmlCharEncodingHandlerPtr);
}
extern "C" {
    pub fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
}
extern "C" {
    pub fn xmlFindCharEncodingHandler(
        name: *const ::std::os::raw::c_char,
    ) -> xmlCharEncodingHandlerPtr;
}
extern "C" {
    pub fn xmlNewCharEncodingHandler(
        name: *const ::std::os::raw::c_char,
        input: xmlCharEncodingInputFunc,
        output: xmlCharEncodingOutputFunc,
    ) -> xmlCharEncodingHandlerPtr;
}
extern "C" {
    pub fn xmlAddEncodingAlias(
        name: *const ::std::os::raw::c_char,
        alias: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDelEncodingAlias(alias: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlGetEncodingAlias(
        alias: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlCleanupEncodingAliases();
}
extern "C" {
    pub fn xmlParseCharEncoding(name: *const ::std::os::raw::c_char) -> xmlCharEncoding;
}
extern "C" {
    pub fn xmlGetCharEncodingName(enc: xmlCharEncoding) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlDetectCharEncoding(
        in_: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    ) -> xmlCharEncoding;
}
extern "C" {
    pub fn xmlCharEncOutFunc(
        handler: *mut xmlCharEncodingHandler,
        out: xmlBufferPtr,
        in_: xmlBufferPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCharEncInFunc(
        handler: *mut xmlCharEncodingHandler,
        out: xmlBufferPtr,
        in_: xmlBufferPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCharEncFirstLine(
        handler: *mut xmlCharEncodingHandler,
        out: xmlBufferPtr,
        in_: xmlBufferPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn UTF8Toisolat1(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isolat1ToUTF8(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type xmlInputMatchCallback = ::std::option::Option<
    unsafe extern "C" fn(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
>;
pub type xmlInputOpenCallback = ::std::option::Option<
    unsafe extern "C" fn(filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type xmlInputReadCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlInputCloseCallback = ::std::option::Option<
    unsafe extern "C" fn(context: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
pub type xmlOutputMatchCallback = ::std::option::Option<
    unsafe extern "C" fn(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
>;
pub type xmlOutputOpenCallback = ::std::option::Option<
    unsafe extern "C" fn(filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type xmlOutputWriteCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlOutputCloseCallback = ::std::option::Option<
    unsafe extern "C" fn(context: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlParserInputBuffer {
    pub context: *mut ::std::os::raw::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: ::std::os::raw::c_int,
    pub error: ::std::os::raw::c_int,
    pub rawconsumed: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout__xmlParserInputBuffer() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlParserInputBuffer> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlParserInputBuffer>(),
        64usize,
        concat!("Size of: ", stringify!(_xmlParserInputBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlParserInputBuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlParserInputBuffer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).readcallback) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(readcallback)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).closecallback) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(closecallback)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encoder) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(encoder)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).raw) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(raw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).compressed) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(compressed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rawconsumed) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlParserInputBuffer),
            "::",
            stringify!(rawconsumed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlOutputBuffer {
    pub context: *mut ::std::os::raw::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: ::std::os::raw::c_int,
    pub error: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlOutputBuffer() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlOutputBuffer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlOutputBuffer>(),
        56usize,
        concat!("Size of: ", stringify!(_xmlOutputBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlOutputBuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlOutputBuffer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).writecallback) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(writecallback)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).closecallback) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(closecallback)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).encoder) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(encoder)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).conv) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(conv)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).written) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(written)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlOutputBuffer),
            "::",
            stringify!(error)
        )
    );
}
extern "C" {
    pub fn xmlCleanupInputCallbacks();
}
extern "C" {
    pub fn xmlPopInputCallbacks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegisterDefaultInputCallbacks();
}
extern "C" {
    pub fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateFilename(
        URI: *const ::std::os::raw::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateFile(
        file: *mut FILE,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateFd(
        fd: ::std::os::raw::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateMem(
        mem: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateStatic(
        mem: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlParserInputBufferRead(
        in_: xmlParserInputBufferPtr,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParserInputBufferGrow(
        in_: xmlParserInputBufferPtr,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParserInputBufferPush(
        in_: xmlParserInputBufferPtr,
        len: ::std::os::raw::c_int,
        buf: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlFreeParserInputBuffer(in_: xmlParserInputBufferPtr);
}
extern "C" {
    pub fn xmlParserGetDirectory(
        filename: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlRegisterInputCallbacks(
        matchFunc: xmlInputMatchCallback,
        openFunc: xmlInputOpenCallback,
        readFunc: xmlInputReadCallback,
        closeFunc: xmlInputCloseCallback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlParserInputBufferCreateFilename(
        URI: *const ::std::os::raw::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlCleanupOutputCallbacks();
}
extern "C" {
    pub fn xmlPopOutputCallbacks() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegisterDefaultOutputCallbacks();
}
extern "C" {
    pub fn xmlAllocOutputBuffer(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferCreateFilename(
        URI: *const ::std::os::raw::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferCreateBuffer(
        buffer: xmlBufferPtr,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferCreateFd(
        fd: ::std::os::raw::c_int,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferCreateIO(
        iowrite: xmlOutputWriteCallback,
        ioclose: xmlOutputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlOutputBufferGetContent(out: xmlOutputBufferPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlOutputBufferGetSize(out: xmlOutputBufferPtr) -> usize;
}
extern "C" {
    pub fn xmlOutputBufferWrite(
        out: xmlOutputBufferPtr,
        len: ::std::os::raw::c_int,
        buf: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlOutputBufferWriteString(
        out: xmlOutputBufferPtr,
        str_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlOutputBufferWriteEscape(
        out: xmlOutputBufferPtr,
        str_: *const xmlChar,
        escaping: xmlCharEncodingOutputFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRegisterOutputCallbacks(
        matchFunc: xmlOutputMatchCallback,
        openFunc: xmlOutputOpenCallback,
        writeFunc: xmlOutputWriteCallback,
        closeFunc: xmlOutputCloseCallback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlOutputBufferCreateFilename(
        URI: *const ::std::os::raw::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlOutputBufferPtr;
}
extern "C" {
    pub fn xmlRegisterHTTPPostCallbacks();
}
extern "C" {
    pub fn xmlCheckHTTPInput(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlNoNetExternalEntityLoader(
        URL: *const ::std::os::raw::c_char,
        ID: *const ::std::os::raw::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlNormalizeWindowsPath(path: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCheckFilename(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlFileMatch(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlFileOpen(filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlFileRead(
        context: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlFileClose(context: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIOHTTPMatch(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIOHTTPOpen(filename: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlIOHTTPOpenW(
        post_uri: *const ::std::os::raw::c_char,
        compression: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlIOHTTPRead(
        context: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIOHTTPClose(context: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlInitParser();
}
extern "C" {
    pub fn xmlCleanupParser();
}
extern "C" {
    pub fn xmlParserInputRead(
        in_: xmlParserInputPtr,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParserInputGrow(
        in_: xmlParserInputPtr,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseDoc(cur: *const xmlChar) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlParseFile(filename: *const ::std::os::raw::c_char) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlParseMemory(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSubstituteEntitiesDefault(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlKeepBlanksDefault(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStopParser(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlPedanticParserDefault(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlLineNumbersDefault(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRecoverDoc(cur: *const xmlChar) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlRecoverMemory(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlRecoverFile(filename: *const ::std::os::raw::c_char) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseExtParsedEnt(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAXUserParseFile(
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAXUserParseMemory(
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAXParseDoc(
        sax: xmlSAXHandlerPtr,
        cur: *const xmlChar,
        recovery: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseMemory(
        sax: xmlSAXHandlerPtr,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        recovery: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseMemoryWithData(
        sax: xmlSAXHandlerPtr,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        recovery: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseFile(
        sax: xmlSAXHandlerPtr,
        filename: *const ::std::os::raw::c_char,
        recovery: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseFileWithData(
        sax: xmlSAXHandlerPtr,
        filename: *const ::std::os::raw::c_char,
        recovery: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseEntity(
        sax: xmlSAXHandlerPtr,
        filename: *const ::std::os::raw::c_char,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlParseEntity(filename: *const ::std::os::raw::c_char) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlSAXParseDTD(
        sax: xmlSAXHandlerPtr,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlIOParseDTD(
        sax: xmlSAXHandlerPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
    ) -> xmlDtdPtr;
}
extern "C" {
    pub fn xmlParseBalancedChunkMemory(
        doc: xmlDocPtr,
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        string: *const xmlChar,
        lst: *mut xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseInNodeContext(
        node: xmlNodePtr,
        data: *const ::std::os::raw::c_char,
        datalen: ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
        lst: *mut xmlNodePtr,
    ) -> xmlParserErrors;
}
extern "C" {
    pub fn xmlParseBalancedChunkMemoryRecover(
        doc: xmlDocPtr,
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        string: *const xmlChar,
        lst: *mut xmlNodePtr,
        recover: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseExternalEntity(
        doc: xmlDocPtr,
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        URL: *const xmlChar,
        ID: *const xmlChar,
        lst: *mut xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseCtxtExternalEntity(
        ctx: xmlParserCtxtPtr,
        URL: *const xmlChar,
        ID: *const xmlChar,
        lst: *mut xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlInitParserCtxt(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlClearParserCtxt(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlSetupParserForBuffer(
        ctxt: xmlParserCtxtPtr,
        buffer: *const xmlChar,
        filename: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn xmlCreateDocParserCtxt(cur: *const xmlChar) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlCreatePushParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        chunk: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
    ) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        terminate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCreateIOParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlNewIOInputStream(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlParserFindNodeInfo(
        ctxt: xmlParserCtxtPtr,
        node: xmlNodePtr,
    ) -> *const xmlParserNodeInfo;
}
extern "C" {
    pub fn xmlInitNodeInfoSeq(seq: xmlParserNodeInfoSeqPtr);
}
extern "C" {
    pub fn xmlClearNodeInfoSeq(seq: xmlParserNodeInfoSeqPtr);
}
extern "C" {
    pub fn xmlParserFindNodeInfoIndex(
        seq: xmlParserNodeInfoSeqPtr,
        node: xmlNodePtr,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn xmlParserAddNodeInfo(ctxt: xmlParserCtxtPtr, info: xmlParserNodeInfoPtr);
}
extern "C" {
    pub fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
}
extern "C" {
    pub fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader;
}
extern "C" {
    pub fn xmlLoadExternalEntity(
        URL: *const ::std::os::raw::c_char,
        ID: *const ::std::os::raw::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_long;
}
pub const xmlParserOption_XML_PARSE_RECOVER: xmlParserOption = 1;
pub const xmlParserOption_XML_PARSE_NOENT: xmlParserOption = 2;
pub const xmlParserOption_XML_PARSE_DTDLOAD: xmlParserOption = 4;
pub const xmlParserOption_XML_PARSE_DTDATTR: xmlParserOption = 8;
pub const xmlParserOption_XML_PARSE_DTDVALID: xmlParserOption = 16;
pub const xmlParserOption_XML_PARSE_NOERROR: xmlParserOption = 32;
pub const xmlParserOption_XML_PARSE_NOWARNING: xmlParserOption = 64;
pub const xmlParserOption_XML_PARSE_PEDANTIC: xmlParserOption = 128;
pub const xmlParserOption_XML_PARSE_NOBLANKS: xmlParserOption = 256;
pub const xmlParserOption_XML_PARSE_SAX1: xmlParserOption = 512;
pub const xmlParserOption_XML_PARSE_XINCLUDE: xmlParserOption = 1024;
pub const xmlParserOption_XML_PARSE_NONET: xmlParserOption = 2048;
pub const xmlParserOption_XML_PARSE_NODICT: xmlParserOption = 4096;
pub const xmlParserOption_XML_PARSE_NSCLEAN: xmlParserOption = 8192;
pub const xmlParserOption_XML_PARSE_NOCDATA: xmlParserOption = 16384;
pub const xmlParserOption_XML_PARSE_NOXINCNODE: xmlParserOption = 32768;
pub const xmlParserOption_XML_PARSE_COMPACT: xmlParserOption = 65536;
pub const xmlParserOption_XML_PARSE_OLD10: xmlParserOption = 131072;
pub const xmlParserOption_XML_PARSE_NOBASEFIX: xmlParserOption = 262144;
pub const xmlParserOption_XML_PARSE_HUGE: xmlParserOption = 524288;
pub const xmlParserOption_XML_PARSE_OLDSAX: xmlParserOption = 1048576;
pub const xmlParserOption_XML_PARSE_IGNORE_ENC: xmlParserOption = 2097152;
pub const xmlParserOption_XML_PARSE_BIG_LINES: xmlParserOption = 4194304;
pub type xmlParserOption = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlCtxtResetPush(
        ctxt: xmlParserCtxtPtr,
        chunk: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCtxtUseOptions(
        ctxt: xmlParserCtxtPtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReadDoc(
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlReadFile(
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlReadMemory(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlReadFd(
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlReadIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCtxtReadDoc(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCtxtReadFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCtxtReadMemory(
        ctxt: xmlParserCtxtPtr,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCtxtReadFd(
        ctxt: xmlParserCtxtPtr,
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCtxtReadIO(
        ctxt: xmlParserCtxtPtr,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlDocPtr;
}
pub const xmlFeature_XML_WITH_THREAD: xmlFeature = 1;
pub const xmlFeature_XML_WITH_TREE: xmlFeature = 2;
pub const xmlFeature_XML_WITH_OUTPUT: xmlFeature = 3;
pub const xmlFeature_XML_WITH_PUSH: xmlFeature = 4;
pub const xmlFeature_XML_WITH_READER: xmlFeature = 5;
pub const xmlFeature_XML_WITH_PATTERN: xmlFeature = 6;
pub const xmlFeature_XML_WITH_WRITER: xmlFeature = 7;
pub const xmlFeature_XML_WITH_SAX1: xmlFeature = 8;
pub const xmlFeature_XML_WITH_FTP: xmlFeature = 9;
pub const xmlFeature_XML_WITH_HTTP: xmlFeature = 10;
pub const xmlFeature_XML_WITH_VALID: xmlFeature = 11;
pub const xmlFeature_XML_WITH_HTML: xmlFeature = 12;
pub const xmlFeature_XML_WITH_LEGACY: xmlFeature = 13;
pub const xmlFeature_XML_WITH_C14N: xmlFeature = 14;
pub const xmlFeature_XML_WITH_CATALOG: xmlFeature = 15;
pub const xmlFeature_XML_WITH_XPATH: xmlFeature = 16;
pub const xmlFeature_XML_WITH_XPTR: xmlFeature = 17;
pub const xmlFeature_XML_WITH_XINCLUDE: xmlFeature = 18;
pub const xmlFeature_XML_WITH_ICONV: xmlFeature = 19;
pub const xmlFeature_XML_WITH_ISO8859X: xmlFeature = 20;
pub const xmlFeature_XML_WITH_UNICODE: xmlFeature = 21;
pub const xmlFeature_XML_WITH_REGEXP: xmlFeature = 22;
pub const xmlFeature_XML_WITH_AUTOMATA: xmlFeature = 23;
pub const xmlFeature_XML_WITH_EXPR: xmlFeature = 24;
pub const xmlFeature_XML_WITH_SCHEMAS: xmlFeature = 25;
pub const xmlFeature_XML_WITH_SCHEMATRON: xmlFeature = 26;
pub const xmlFeature_XML_WITH_MODULES: xmlFeature = 27;
pub const xmlFeature_XML_WITH_DEBUG: xmlFeature = 28;
pub const xmlFeature_XML_WITH_DEBUG_MEM: xmlFeature = 29;
pub const xmlFeature_XML_WITH_DEBUG_RUN: xmlFeature = 30;
pub const xmlFeature_XML_WITH_ZLIB: xmlFeature = 31;
pub const xmlFeature_XML_WITH_ICU: xmlFeature = 32;
pub const xmlFeature_XML_WITH_LZMA: xmlFeature = 33;
pub const xmlFeature_XML_WITH_NONE: xmlFeature = 99999;
pub type xmlFeature = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlHasFeature(feature: xmlFeature) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2GetPublicId(ctx: *mut ::std::os::raw::c_void) -> *const xmlChar;
}
extern "C" {
    pub fn xmlSAX2GetSystemId(ctx: *mut ::std::os::raw::c_void) -> *const xmlChar;
}
extern "C" {
    pub fn xmlSAX2SetDocumentLocator(ctx: *mut ::std::os::raw::c_void, loc: xmlSAXLocatorPtr);
}
extern "C" {
    pub fn xmlSAX2GetLineNumber(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2GetColumnNumber(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2IsStandalone(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2HasInternalSubset(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2HasExternalSubset(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2InternalSubset(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2ExternalSubset(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2GetEntity(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar)
        -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlSAX2GetParameterEntity(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlSAX2ResolveEntity(
        ctx: *mut ::std::os::raw::c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlSAX2EntityDecl(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2AttributeDecl(
        ctx: *mut ::std::os::raw::c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_: ::std::os::raw::c_int,
        def: ::std::os::raw::c_int,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    );
}
extern "C" {
    pub fn xmlSAX2ElementDecl(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        type_: ::std::os::raw::c_int,
        content: xmlElementContentPtr,
    );
}
extern "C" {
    pub fn xmlSAX2NotationDecl(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2UnparsedEntityDecl(
        ctx: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2StartDocument(ctx: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlSAX2EndDocument(ctx: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlSAX2StartElement(
        ctx: *mut ::std::os::raw::c_void,
        fullname: *const xmlChar,
        atts: *mut *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2EndElement(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar);
}
extern "C" {
    pub fn xmlSAX2StartElementNs(
        ctx: *mut ::std::os::raw::c_void,
        localname: *const xmlChar,
        prefix: *const xmlChar,
        URI: *const xmlChar,
        nb_namespaces: ::std::os::raw::c_int,
        namespaces: *mut *const xmlChar,
        nb_attributes: ::std::os::raw::c_int,
        nb_defaulted: ::std::os::raw::c_int,
        attributes: *mut *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2EndElementNs(
        ctx: *mut ::std::os::raw::c_void,
        localname: *const xmlChar,
        prefix: *const xmlChar,
        URI: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2Reference(ctx: *mut ::std::os::raw::c_void, name: *const xmlChar);
}
extern "C" {
    pub fn xmlSAX2Characters(
        ctx: *mut ::std::os::raw::c_void,
        ch: *const xmlChar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlSAX2IgnorableWhitespace(
        ctx: *mut ::std::os::raw::c_void,
        ch: *const xmlChar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlSAX2ProcessingInstruction(
        ctx: *mut ::std::os::raw::c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlSAX2Comment(ctx: *mut ::std::os::raw::c_void, value: *const xmlChar);
}
extern "C" {
    pub fn xmlSAX2CDataBlock(
        ctx: *mut ::std::os::raw::c_void,
        value: *const xmlChar,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlSAXDefaultVersion(version: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAXVersion(
        hdlr: *mut xmlSAXHandler,
        version: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSAX2InitDefaultSAXHandler(hdlr: *mut xmlSAXHandler, warning: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlSAX2InitHtmlDefaultSAXHandler(hdlr: *mut xmlSAXHandler);
}
extern "C" {
    pub fn htmlDefaultSAXHandlerInit();
}
extern "C" {
    pub fn xmlDefaultSAXHandlerInit();
}
extern "C" {
    pub fn xmlInitGlobals();
}
extern "C" {
    pub fn xmlCleanupGlobals();
}
pub type xmlParserInputBufferCreateFilenameFunc = ::std::option::Option<
    unsafe extern "C" fn(
        URI: *const ::std::os::raw::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr,
>;
pub type xmlOutputBufferCreateFilenameFunc = ::std::option::Option<
    unsafe extern "C" fn(
        URI: *const ::std::os::raw::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlOutputBufferPtr,
>;
extern "C" {
    pub fn xmlParserInputBufferCreateFilenameDefault(
        func: xmlParserInputBufferCreateFilenameFunc,
    ) -> xmlParserInputBufferCreateFilenameFunc;
}
extern "C" {
    pub fn xmlOutputBufferCreateFilenameDefault(
        func: xmlOutputBufferCreateFilenameFunc,
    ) -> xmlOutputBufferCreateFilenameFunc;
}
pub type xmlRegisterNodeFunc = ::std::option::Option<unsafe extern "C" fn(node: xmlNodePtr)>;
pub type xmlDeregisterNodeFunc = ::std::option::Option<unsafe extern "C" fn(node: xmlNodePtr)>;
pub type xmlGlobalState = _xmlGlobalState;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlGlobalState {
    pub xmlParserVersion: *const ::std::os::raw::c_char,
    pub xmlDefaultSAXLocator: xmlSAXLocator,
    pub xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub docbDefaultSAXHandler: xmlSAXHandlerV1,
    pub htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub xmlFree: xmlFreeFunc,
    pub xmlMalloc: xmlMallocFunc,
    pub xmlMemStrdup: xmlStrdupFunc,
    pub xmlRealloc: xmlReallocFunc,
    pub xmlGenericError: xmlGenericErrorFunc,
    pub xmlStructuredError: xmlStructuredErrorFunc,
    pub xmlGenericErrorContext: *mut ::std::os::raw::c_void,
    pub oldXMLWDcompatibility: ::std::os::raw::c_int,
    pub xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub xmlDefaultBufferSize: ::std::os::raw::c_int,
    pub xmlSubstituteEntitiesDefaultValue: ::std::os::raw::c_int,
    pub xmlDoValidityCheckingDefaultValue: ::std::os::raw::c_int,
    pub xmlGetWarningsDefaultValue: ::std::os::raw::c_int,
    pub xmlKeepBlanksDefaultValue: ::std::os::raw::c_int,
    pub xmlLineNumbersDefaultValue: ::std::os::raw::c_int,
    pub xmlLoadExtDtdDefaultValue: ::std::os::raw::c_int,
    pub xmlParserDebugEntities: ::std::os::raw::c_int,
    pub xmlPedanticParserDefaultValue: ::std::os::raw::c_int,
    pub xmlSaveNoEmptyTags: ::std::os::raw::c_int,
    pub xmlIndentTreeOutput: ::std::os::raw::c_int,
    pub xmlTreeIndentString: *const ::std::os::raw::c_char,
    pub xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
    pub xmlMallocAtomic: xmlMallocFunc,
    pub xmlLastError: xmlError,
    pub xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub xmlStructuredErrorContext: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlGlobalState() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlGlobalState> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlGlobalState>(),
        968usize,
        concat!("Size of: ", stringify!(_xmlGlobalState))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlGlobalState>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlGlobalState))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlParserVersion) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlParserVersion)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlDefaultSAXLocator) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlDefaultSAXLocator)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlDefaultSAXHandler) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlDefaultSAXHandler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).docbDefaultSAXHandler) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(docbDefaultSAXHandler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).htmlDefaultSAXHandler) as usize - ptr as usize },
        488usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(htmlDefaultSAXHandler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlFree) as usize - ptr as usize },
        712usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlFree)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlMalloc) as usize - ptr as usize },
        720usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlMalloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlMemStrdup) as usize - ptr as usize },
        728usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlMemStrdup)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlRealloc) as usize - ptr as usize },
        736usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlRealloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlGenericError) as usize - ptr as usize },
        744usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlGenericError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlStructuredError) as usize - ptr as usize },
        752usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlStructuredError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlGenericErrorContext) as usize - ptr as usize },
        760usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlGenericErrorContext)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).oldXMLWDcompatibility) as usize - ptr as usize },
        768usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(oldXMLWDcompatibility)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlBufferAllocScheme) as usize - ptr as usize },
        772usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlBufferAllocScheme)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlDefaultBufferSize) as usize - ptr as usize },
        776usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlDefaultBufferSize)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlSubstituteEntitiesDefaultValue) as usize - ptr as usize
        },
        780usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlSubstituteEntitiesDefaultValue)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlDoValidityCheckingDefaultValue) as usize - ptr as usize
        },
        784usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlDoValidityCheckingDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlGetWarningsDefaultValue) as usize - ptr as usize },
        788usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlGetWarningsDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlKeepBlanksDefaultValue) as usize - ptr as usize },
        792usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlKeepBlanksDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlLineNumbersDefaultValue) as usize - ptr as usize },
        796usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlLineNumbersDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlLoadExtDtdDefaultValue) as usize - ptr as usize },
        800usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlLoadExtDtdDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlParserDebugEntities) as usize - ptr as usize },
        804usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlParserDebugEntities)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlPedanticParserDefaultValue) as usize - ptr as usize
        },
        808usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlPedanticParserDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlSaveNoEmptyTags) as usize - ptr as usize },
        812usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlSaveNoEmptyTags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlIndentTreeOutput) as usize - ptr as usize },
        816usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlIndentTreeOutput)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlTreeIndentString) as usize - ptr as usize },
        824usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlTreeIndentString)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlRegisterNodeDefaultValue) as usize - ptr as usize },
        832usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlRegisterNodeDefaultValue)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlDeregisterNodeDefaultValue) as usize - ptr as usize
        },
        840usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlDeregisterNodeDefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlMallocAtomic) as usize - ptr as usize },
        848usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlMallocAtomic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlLastError) as usize - ptr as usize },
        856usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlLastError)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlParserInputBufferCreateFilenameValue) as usize
                - ptr as usize
        },
        944usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlParserInputBufferCreateFilenameValue)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).xmlOutputBufferCreateFilenameValue) as usize - ptr as usize
        },
        952usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlOutputBufferCreateFilenameValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xmlStructuredErrorContext) as usize - ptr as usize },
        960usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlGlobalState),
            "::",
            stringify!(xmlStructuredErrorContext)
        )
    );
}
extern "C" {
    pub fn xmlInitializeGlobalState(gs: xmlGlobalStatePtr);
}
extern "C" {
    pub fn xmlThrDefSetGenericErrorFunc(
        ctx: *mut ::std::os::raw::c_void,
        handler: xmlGenericErrorFunc,
    );
}
extern "C" {
    pub fn xmlThrDefSetStructuredErrorFunc(
        ctx: *mut ::std::os::raw::c_void,
        handler: xmlStructuredErrorFunc,
    );
}
extern "C" {
    pub fn xmlRegisterNodeDefault(func: xmlRegisterNodeFunc) -> xmlRegisterNodeFunc;
}
extern "C" {
    pub fn xmlThrDefRegisterNodeDefault(func: xmlRegisterNodeFunc) -> xmlRegisterNodeFunc;
}
extern "C" {
    pub fn xmlDeregisterNodeDefault(func: xmlDeregisterNodeFunc) -> xmlDeregisterNodeFunc;
}
extern "C" {
    pub fn xmlThrDefDeregisterNodeDefault(func: xmlDeregisterNodeFunc) -> xmlDeregisterNodeFunc;
}
extern "C" {
    pub fn xmlThrDefOutputBufferCreateFilenameDefault(
        func: xmlOutputBufferCreateFilenameFunc,
    ) -> xmlOutputBufferCreateFilenameFunc;
}
extern "C" {
    pub fn xmlThrDefParserInputBufferCreateFilenameDefault(
        func: xmlParserInputBufferCreateFilenameFunc,
    ) -> xmlParserInputBufferCreateFilenameFunc;
}
extern "C" {
    pub static mut xmlMalloc: xmlMallocFunc;
}
extern "C" {
    pub static mut xmlMallocAtomic: xmlMallocFunc;
}
extern "C" {
    pub static mut xmlRealloc: xmlReallocFunc;
}
extern "C" {
    pub static mut xmlFree: xmlFreeFunc;
}
extern "C" {
    pub static mut xmlMemStrdup: xmlStrdupFunc;
}
extern "C" {
    pub fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
}
extern "C" {
    pub fn __xmlLastError() -> *mut xmlError;
}
extern "C" {
    pub fn __oldXMLWDcompatibility() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme;
}
extern "C" {
    pub fn xmlThrDefBufferAllocScheme(v: xmlBufferAllocationScheme) -> xmlBufferAllocationScheme;
}
extern "C" {
    pub fn __xmlDefaultBufferSize() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefDefaultBufferSize(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
}
extern "C" {
    pub fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator;
}
extern "C" {
    pub fn __xmlDoValidityCheckingDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefDoValidityCheckingDefaultValue(
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
}
extern "C" {
    pub fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc;
}
extern "C" {
    pub fn __xmlGenericErrorContext() -> *mut *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn __xmlStructuredErrorContext() -> *mut *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn __xmlGetWarningsDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefGetWarningsDefaultValue(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlIndentTreeOutput() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefIndentTreeOutput(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlTreeIndentString() -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlThrDefTreeIndentString(
        v: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn __xmlKeepBlanksDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefKeepBlanksDefaultValue(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlLineNumbersDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefLineNumbersDefaultValue(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlLoadExtDtdDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefLoadExtDtdDefaultValue(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlParserDebugEntities() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefParserDebugEntities(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlParserVersion() -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn __xmlPedanticParserDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefPedanticParserDefaultValue(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlSaveNoEmptyTags() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefSaveNoEmptyTags(v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlSubstituteEntitiesDefaultValue() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlThrDefSubstituteEntitiesDefaultValue(
        v: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc;
}
extern "C" {
    pub fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
}
extern "C" {
    pub fn __xmlParserInputBufferCreateFilenameValue() -> *mut xmlParserInputBufferCreateFilenameFunc;
}
extern "C" {
    pub fn __xmlOutputBufferCreateFilenameValue() -> *mut xmlOutputBufferCreateFilenameFunc;
}
extern "C" {
    pub fn xmlNewMutex() -> xmlMutexPtr;
}
extern "C" {
    pub fn xmlMutexLock(tok: xmlMutexPtr);
}
extern "C" {
    pub fn xmlMutexUnlock(tok: xmlMutexPtr);
}
extern "C" {
    pub fn xmlFreeMutex(tok: xmlMutexPtr);
}
extern "C" {
    pub fn xmlNewRMutex() -> xmlRMutexPtr;
}
extern "C" {
    pub fn xmlRMutexLock(tok: xmlRMutexPtr);
}
extern "C" {
    pub fn xmlRMutexUnlock(tok: xmlRMutexPtr);
}
extern "C" {
    pub fn xmlFreeRMutex(tok: xmlRMutexPtr);
}
extern "C" {
    pub fn xmlInitThreads();
}
extern "C" {
    pub fn xmlLockLibrary();
}
extern "C" {
    pub fn xmlUnlockLibrary();
}
extern "C" {
    pub fn xmlGetThreadId() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsMainThread() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCleanupThreads();
}
extern "C" {
    pub fn xmlGetGlobalState() -> xmlGlobalStatePtr;
}
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub const xmlXPathError_XPATH_EXPRESSION_OK: xmlXPathError = 0;
pub const xmlXPathError_XPATH_NUMBER_ERROR: xmlXPathError = 1;
pub const xmlXPathError_XPATH_UNFINISHED_LITERAL_ERROR: xmlXPathError = 2;
pub const xmlXPathError_XPATH_START_LITERAL_ERROR: xmlXPathError = 3;
pub const xmlXPathError_XPATH_VARIABLE_REF_ERROR: xmlXPathError = 4;
pub const xmlXPathError_XPATH_UNDEF_VARIABLE_ERROR: xmlXPathError = 5;
pub const xmlXPathError_XPATH_INVALID_PREDICATE_ERROR: xmlXPathError = 6;
pub const xmlXPathError_XPATH_EXPR_ERROR: xmlXPathError = 7;
pub const xmlXPathError_XPATH_UNCLOSED_ERROR: xmlXPathError = 8;
pub const xmlXPathError_XPATH_UNKNOWN_FUNC_ERROR: xmlXPathError = 9;
pub const xmlXPathError_XPATH_INVALID_OPERAND: xmlXPathError = 10;
pub const xmlXPathError_XPATH_INVALID_TYPE: xmlXPathError = 11;
pub const xmlXPathError_XPATH_INVALID_ARITY: xmlXPathError = 12;
pub const xmlXPathError_XPATH_INVALID_CTXT_SIZE: xmlXPathError = 13;
pub const xmlXPathError_XPATH_INVALID_CTXT_POSITION: xmlXPathError = 14;
pub const xmlXPathError_XPATH_MEMORY_ERROR: xmlXPathError = 15;
pub const xmlXPathError_XPTR_SYNTAX_ERROR: xmlXPathError = 16;
pub const xmlXPathError_XPTR_RESOURCE_ERROR: xmlXPathError = 17;
pub const xmlXPathError_XPTR_SUB_RESOURCE_ERROR: xmlXPathError = 18;
pub const xmlXPathError_XPATH_UNDEF_PREFIX_ERROR: xmlXPathError = 19;
pub const xmlXPathError_XPATH_ENCODING_ERROR: xmlXPathError = 20;
pub const xmlXPathError_XPATH_INVALID_CHAR_ERROR: xmlXPathError = 21;
pub const xmlXPathError_XPATH_INVALID_CTXT: xmlXPathError = 22;
pub const xmlXPathError_XPATH_STACK_ERROR: xmlXPathError = 23;
pub const xmlXPathError_XPATH_FORBID_VARIABLE_ERROR: xmlXPathError = 24;
pub const xmlXPathError_XPATH_OP_LIMIT_EXCEEDED: xmlXPathError = 25;
pub const xmlXPathError_XPATH_RECURSION_LIMIT_EXCEEDED: xmlXPathError = 26;
pub type xmlXPathError = ::std::os::raw::c_uint;
pub type xmlNodeSet = _xmlNodeSet;
pub type xmlNodeSetPtr = *mut xmlNodeSet;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlNodeSet {
    pub nodeNr: ::std::os::raw::c_int,
    pub nodeMax: ::std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
#[test]
fn bindgen_test_layout__xmlNodeSet() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlNodeSet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlNodeSet>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlNodeSet))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlNodeSet>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlNodeSet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeNr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNodeSet),
            "::",
            stringify!(nodeNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeMax) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNodeSet),
            "::",
            stringify!(nodeMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodeTab) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlNodeSet),
            "::",
            stringify!(nodeTab)
        )
    );
}
pub const xmlXPathObjectType_XPATH_UNDEFINED: xmlXPathObjectType = 0;
pub const xmlXPathObjectType_XPATH_NODESET: xmlXPathObjectType = 1;
pub const xmlXPathObjectType_XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const xmlXPathObjectType_XPATH_NUMBER: xmlXPathObjectType = 3;
pub const xmlXPathObjectType_XPATH_STRING: xmlXPathObjectType = 4;
pub const xmlXPathObjectType_XPATH_USERS: xmlXPathObjectType = 8;
pub const xmlXPathObjectType_XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub type xmlXPathObjectType = ::std::os::raw::c_uint;
pub type xmlXPathObject = _xmlXPathObject;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathObject {
    pub type_: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: ::std::os::raw::c_int,
    pub floatval: f64,
    pub stringval: *mut xmlChar,
    pub user: *mut ::std::os::raw::c_void,
    pub index: ::std::os::raw::c_int,
    pub user2: *mut ::std::os::raw::c_void,
    pub index2: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlXPathObject() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathObject> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathObject>(),
        72usize,
        concat!("Size of: ", stringify!(_xmlXPathObject))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathObject>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathObject))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nodesetval) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(nodesetval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).boolval) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(boolval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).floatval) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(floatval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stringval) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(stringval)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).user) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).user2) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(user2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).index2) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathObject),
            "::",
            stringify!(index2)
        )
    );
}
pub type xmlXPathConvertFunc = ::std::option::Option<
    unsafe extern "C" fn(
        obj: xmlXPathObjectPtr,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type xmlXPathType = _xmlXPathType;
pub type xmlXPathTypePtr = *mut xmlXPathType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
#[test]
fn bindgen_test_layout__xmlXPathType() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathType> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathType>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlXPathType))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathType>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathType))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathType),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathType),
            "::",
            stringify!(func)
        )
    );
}
pub type xmlXPathVariable = _xmlXPathVariable;
pub type xmlXPathVariablePtr = *mut xmlXPathVariable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathVariable {
    pub name: *const xmlChar,
    pub value: xmlXPathObjectPtr,
}
#[test]
fn bindgen_test_layout__xmlXPathVariable() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathVariable> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathVariable>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlXPathVariable))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathVariable>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathVariable))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathVariable),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathVariable),
            "::",
            stringify!(value)
        )
    );
}
pub type xmlXPathEvalFunc = ::std::option::Option<
    unsafe extern "C" fn(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int),
>;
pub type xmlXPathFunct = _xmlXPathFunct;
pub type xmlXPathFuncPtr = *mut xmlXPathFunct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathFunct {
    pub name: *const xmlChar,
    pub func: xmlXPathEvalFunc,
}
#[test]
fn bindgen_test_layout__xmlXPathFunct() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathFunct> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathFunct>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlXPathFunct))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathFunct>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathFunct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathFunct),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathFunct),
            "::",
            stringify!(func)
        )
    );
}
pub type xmlXPathAxisFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctxt: xmlXPathParserContextPtr,
        cur: xmlXPathObjectPtr,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathAxis = _xmlXPathAxis;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
#[test]
fn bindgen_test_layout__xmlXPathAxis() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathAxis> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathAxis>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlXPathAxis))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathAxis>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathAxis))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathAxis),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathAxis),
            "::",
            stringify!(func)
        )
    );
}
pub type xmlXPathFunction = ::std::option::Option<
    unsafe extern "C" fn(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int),
>;
pub type xmlXPathVariableLookupFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctxt: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathFuncLookupFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctxt: *mut ::std::os::raw::c_void,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
    ) -> xmlXPathFunction,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: ::std::os::raw::c_int,
    pub max_variables_unused: ::std::os::raw::c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: ::std::os::raw::c_int,
    pub max_types: ::std::os::raw::c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: ::std::os::raw::c_int,
    pub max_funcs_unused: ::std::os::raw::c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: ::std::os::raw::c_int,
    pub max_axis: ::std::os::raw::c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: ::std::os::raw::c_int,
    pub user: *mut ::std::os::raw::c_void,
    pub contextSize: ::std::os::raw::c_int,
    pub proximityPosition: ::std::os::raw::c_int,
    pub xptr: ::std::os::raw::c_int,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut ::std::os::raw::c_void,
    pub extra: *mut ::std::os::raw::c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut ::std::os::raw::c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: ::std::os::raw::c_int,
    pub userData: *mut ::std::os::raw::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: ::std::os::raw::c_int,
    pub cache: *mut ::std::os::raw::c_void,
    pub opLimit: ::std::os::raw::c_ulong,
    pub opCount: ::std::os::raw::c_ulong,
    pub depth: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlXPathContext() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathContext> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathContext>(),
        376usize,
        concat!("Size of: ", stringify!(_xmlXPathContext))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathContext>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathContext))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nb_variables_unused) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nb_variables_unused)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_variables_unused) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(max_variables_unused)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).varHash) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(varHash)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nb_types) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nb_types)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_types) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(max_types)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).types) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(types)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nb_funcs_unused) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nb_funcs_unused)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_funcs_unused) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(max_funcs_unused)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).funcHash) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(funcHash)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nb_axis) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nb_axis)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_axis) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(max_axis)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).axis) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(axis)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namespaces) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(namespaces)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsNr) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nsNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).user) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contextSize) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(contextSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).proximityPosition) as usize - ptr as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(proximityPosition)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xptr) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(xptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).here) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(here)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).origin) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(origin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsHash) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(nsHash)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).varLookupFunc) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(varLookupFunc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).varLookupData) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(varLookupData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extra) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(extra)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).function) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).functionURI) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(functionURI)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).funcLookupFunc) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(funcLookupFunc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).funcLookupData) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(funcLookupData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tmpNsList) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(tmpNsList)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tmpNsNr) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(tmpNsNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).userData) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(userData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lastError) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(lastError)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).debugNode) as usize - ptr as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(debugNode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dict) as usize - ptr as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(dict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cache) as usize - ptr as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(cache)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opLimit) as usize - ptr as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(opLimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opCount) as usize - ptr as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(opCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depth) as usize - ptr as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathContext),
            "::",
            stringify!(depth)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathCompExpr {
    _unused: [u8; 0],
}
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: ::std::os::raw::c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: ::std::os::raw::c_int,
    pub valueMax: ::std::os::raw::c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: ::std::os::raw::c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlXPathParserContext() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlXPathParserContext> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlXPathParserContext>(),
        88usize,
        concat!("Size of: ", stringify!(_xmlXPathParserContext))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlXPathParserContext>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlXPathParserContext))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cur) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(cur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).base) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).context) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valueNr) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(valueNr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valueMax) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(valueMax)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valueTab) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(valueTab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).comp) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(comp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).xptr) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(xptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ancestor) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(ancestor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).valueFrame) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlXPathParserContext),
            "::",
            stringify!(valueFrame)
        )
    );
}
extern "C" {
    pub static mut xmlXPathNAN: f64;
}
extern "C" {
    pub static mut xmlXPathPINF: f64;
}
extern "C" {
    pub static mut xmlXPathNINF: f64;
}
extern "C" {
    pub fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
}
extern "C" {
    pub fn xmlXPathNodeSetCreate(val: xmlNodePtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathFreeNodeSetList(obj: xmlXPathObjectPtr);
}
extern "C" {
    pub fn xmlXPathFreeNodeSet(obj: xmlNodeSetPtr);
}
extern "C" {
    pub fn xmlXPathObjectCopy(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathCmpNodes(node1: xmlNodePtr, node2: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCastNumberToBoolean(val: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCastStringToBoolean(val: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCastNodeSetToBoolean(ns: xmlNodeSetPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCastToBoolean(val: xmlXPathObjectPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCastBooleanToNumber(val: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn xmlXPathCastStringToNumber(val: *const xmlChar) -> f64;
}
extern "C" {
    pub fn xmlXPathCastNodeToNumber(node: xmlNodePtr) -> f64;
}
extern "C" {
    pub fn xmlXPathCastNodeSetToNumber(ns: xmlNodeSetPtr) -> f64;
}
extern "C" {
    pub fn xmlXPathCastToNumber(val: xmlXPathObjectPtr) -> f64;
}
extern "C" {
    pub fn xmlXPathCastBooleanToString(val: ::std::os::raw::c_int) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathCastNumberToString(val: f64) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathCastNodeToString(node: xmlNodePtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathCastNodeSetToString(ns: xmlNodeSetPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathCastToString(val: xmlXPathObjectPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathConvertBoolean(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathConvertNumber(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathConvertString(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
}
extern "C" {
    pub fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
}
extern "C" {
    pub fn xmlXPathContextSetCache(
        ctxt: xmlXPathContextPtr,
        active: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathOrderDocElems(doc: xmlDocPtr) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn xmlXPathSetContextNode(
        node: xmlNodePtr,
        ctx: xmlXPathContextPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNodeEval(
        node: xmlNodePtr,
        str_: *const xmlChar,
        ctx: xmlXPathContextPtr,
    ) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathEval(str_: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathEvalExpression(
        str_: *const xmlChar,
        ctxt: xmlXPathContextPtr,
    ) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathEvalPredicate(
        ctxt: xmlXPathContextPtr,
        res: xmlXPathObjectPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCompile(str_: *const xmlChar) -> xmlXPathCompExprPtr;
}
extern "C" {
    pub fn xmlXPathCtxtCompile(
        ctxt: xmlXPathContextPtr,
        str_: *const xmlChar,
    ) -> xmlXPathCompExprPtr;
}
extern "C" {
    pub fn xmlXPathCompiledEval(
        comp: xmlXPathCompExprPtr,
        ctx: xmlXPathContextPtr,
    ) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathCompiledEvalToBoolean(
        comp: xmlXPathCompExprPtr,
        ctxt: xmlXPathContextPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathFreeCompExpr(comp: xmlXPathCompExprPtr);
}
extern "C" {
    pub fn xmlXPathInit();
}
extern "C" {
    pub fn xmlXPathIsNaN(val: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathIsInf(val: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlDebugDumpString(output: *mut FILE, str_: *const xmlChar);
}
extern "C" {
    pub fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDebugDumpAttrList(output: *mut FILE, attr: xmlAttrPtr, depth: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDebugDumpNode(output: *mut FILE, node: xmlNodePtr, depth: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDebugDumpNodeList(output: *mut FILE, node: xmlNodePtr, depth: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlDebugDumpDocumentHead(output: *mut FILE, doc: xmlDocPtr);
}
extern "C" {
    pub fn xmlDebugDumpDocument(output: *mut FILE, doc: xmlDocPtr);
}
extern "C" {
    pub fn xmlDebugDumpDTD(output: *mut FILE, dtd: xmlDtdPtr);
}
extern "C" {
    pub fn xmlDebugDumpEntities(output: *mut FILE, doc: xmlDocPtr);
}
extern "C" {
    pub fn xmlDebugCheckDocument(output: *mut FILE, doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlLsOneNode(output: *mut FILE, node: xmlNodePtr);
}
extern "C" {
    pub fn xmlLsCountNode(node: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlBoolToText(boolval: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
pub type xmlShellReadlineFunc = ::std::option::Option<
    unsafe extern "C" fn(prompt: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char,
>;
pub type xmlShellCtxt = _xmlShellCtxt;
pub type xmlShellCtxtPtr = *mut xmlShellCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlShellCtxt {
    pub filename: *mut ::std::os::raw::c_char,
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub pctxt: xmlXPathContextPtr,
    pub loaded: ::std::os::raw::c_int,
    pub output: *mut FILE,
    pub input: xmlShellReadlineFunc,
}
#[test]
fn bindgen_test_layout__xmlShellCtxt() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlShellCtxt> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlShellCtxt>(),
        56usize,
        concat!("Size of: ", stringify!(_xmlShellCtxt))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlShellCtxt>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlShellCtxt))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).filename) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(filename)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pctxt) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(pctxt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).loaded) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(loaded)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).output) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(output)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlShellCtxt),
            "::",
            stringify!(input)
        )
    );
}
pub type xmlShellCmd = ::std::option::Option<
    unsafe extern "C" fn(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn xmlShellPrintXPathError(
        errorType: ::std::os::raw::c_int,
        arg: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn xmlShellPrintXPathResult(list: xmlXPathObjectPtr);
}
extern "C" {
    pub fn xmlShellList(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellBase(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellDir(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellLoad(
        ctxt: xmlShellCtxtPtr,
        filename: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellPrintNode(node: xmlNodePtr);
}
extern "C" {
    pub fn xmlShellCat(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellWrite(
        ctxt: xmlShellCtxtPtr,
        filename: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellSave(
        ctxt: xmlShellCtxtPtr,
        filename: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellValidate(
        ctxt: xmlShellCtxtPtr,
        dtd: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellDu(
        ctxt: xmlShellCtxtPtr,
        arg: *mut ::std::os::raw::c_char,
        tree: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShellPwd(
        ctxt: xmlShellCtxtPtr,
        buffer: *mut ::std::os::raw::c_char,
        node: xmlNodePtr,
        node2: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlShell(
        doc: xmlDocPtr,
        filename: *mut ::std::os::raw::c_char,
        input: xmlShellReadlineFunc,
        output: *mut FILE,
    );
}
pub type xmlChSRange = _xmlChSRange;
pub type xmlChSRangePtr = *mut xmlChSRange;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlChSRange {
    pub low: ::std::os::raw::c_ushort,
    pub high: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__xmlChSRange() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlChSRange> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlChSRange>(),
        4usize,
        concat!("Size of: ", stringify!(_xmlChSRange))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlChSRange>(),
        2usize,
        concat!("Alignment of ", stringify!(_xmlChSRange))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).low) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChSRange),
            "::",
            stringify!(low)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).high) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChSRange),
            "::",
            stringify!(high)
        )
    );
}
pub type xmlChLRange = _xmlChLRange;
pub type xmlChLRangePtr = *mut xmlChLRange;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlChLRange {
    pub low: ::std::os::raw::c_uint,
    pub high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout__xmlChLRange() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlChLRange> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlChLRange>(),
        8usize,
        concat!("Size of: ", stringify!(_xmlChLRange))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlChLRange>(),
        4usize,
        concat!("Alignment of ", stringify!(_xmlChLRange))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).low) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChLRange),
            "::",
            stringify!(low)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).high) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChLRange),
            "::",
            stringify!(high)
        )
    );
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
pub type xmlChRangeGroupPtr = *mut xmlChRangeGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: ::std::os::raw::c_int,
    pub nbLongRange: ::std::os::raw::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
#[test]
fn bindgen_test_layout__xmlChRangeGroup() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlChRangeGroup> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlChRangeGroup>(),
        24usize,
        concat!("Size of: ", stringify!(_xmlChRangeGroup))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlChRangeGroup>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlChRangeGroup))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nbShortRange) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChRangeGroup),
            "::",
            stringify!(nbShortRange)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nbLongRange) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChRangeGroup),
            "::",
            stringify!(nbLongRange)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shortRange) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChRangeGroup),
            "::",
            stringify!(shortRange)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).longRange) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlChRangeGroup),
            "::",
            stringify!(longRange)
        )
    );
}
extern "C" {
    pub fn xmlCharInRange(
        val: ::std::os::raw::c_uint,
        group: *const xmlChRangeGroup,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static xmlIsBaseCharGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsCharGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsCombiningGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsDigitGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsExtenderGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsIdeographicGroup: xmlChRangeGroup;
}
extern "C" {
    pub static xmlIsPubidChar_tab: [::std::os::raw::c_uchar; 256usize];
}
extern "C" {
    pub fn xmlIsBaseChar(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsBlank(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsChar(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsCombining(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsDigit(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsExtender(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsIdeographic(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlIsPubidChar(ch: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPtrNewContext(
        doc: xmlDocPtr,
        here: xmlNodePtr,
        origin: xmlNodePtr,
    ) -> xmlXPathContextPtr;
}
extern "C" {
    pub fn xmlXPtrEval(str_: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
}
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_QUIET: xmlSchematronValidOptions = 1;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_TEXT: xmlSchematronValidOptions = 2;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_XML: xmlSchematronValidOptions = 4;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_ERROR: xmlSchematronValidOptions = 8;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_FILE: xmlSchematronValidOptions = 256;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_BUFFER: xmlSchematronValidOptions = 512;
pub const xmlSchematronValidOptions_XML_SCHEMATRON_OUT_IO: xmlSchematronValidOptions = 1024;
pub type xmlSchematronValidOptions = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchematron {
    _unused: [u8; 0],
}
pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
pub type xmlSchematronValidityErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlSchematronValidityWarningFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchematronParserCtxt {
    _unused: [u8; 0],
}
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchematronValidCtxt {
    _unused: [u8; 0],
}
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;
extern "C" {
    pub fn xmlSchematronNewParserCtxt(
        URL: *const ::std::os::raw::c_char,
    ) -> xmlSchematronParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchematronNewMemParserCtxt(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlSchematronParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchematronNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchematronParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr);
}
extern "C" {
    pub fn xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr;
}
extern "C" {
    pub fn xmlSchematronFree(schema: xmlSchematronPtr);
}
extern "C" {
    pub fn xmlSchematronSetValidStructuredErrors(
        ctxt: xmlSchematronValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlSchematronNewValidCtxt(
        schema: xmlSchematronPtr,
        options: ::std::os::raw::c_int,
    ) -> xmlSchematronValidCtxtPtr;
}
extern "C" {
    pub fn xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr);
}
extern "C" {
    pub fn xmlSchematronValidateDoc(
        ctxt: xmlSchematronValidCtxtPtr,
        instance: xmlDocPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathPopBoolean(ctxt: xmlXPathParserContextPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathPopNumber(ctxt: xmlXPathParserContextPtr) -> f64;
}
extern "C" {
    pub fn xmlXPathPopString(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathPopNodeSet(ctxt: xmlXPathParserContextPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathPopExternal(ctxt: xmlXPathParserContextPtr) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlXPathRegisterVariableLookup(
        ctxt: xmlXPathContextPtr,
        f: xmlXPathVariableLookupFunc,
        data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlXPathRegisterFuncLookup(
        ctxt: xmlXPathContextPtr,
        f: xmlXPathFuncLookupFunc,
        funcCtxt: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlXPatherror(
        ctxt: xmlXPathParserContextPtr,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        no: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathErr(ctxt: xmlXPathParserContextPtr, error: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathDebugDumpObject(
        output: *mut FILE,
        cur: xmlXPathObjectPtr,
        depth: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathDebugDumpCompExpr(
        output: *mut FILE,
        comp: xmlXPathCompExprPtr,
        depth: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathNodeSetContains(cur: xmlNodeSetPtr, val: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathDifference(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathIntersection(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathDistinctSorted(nodes: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathDistinct(nodes: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathHasSameNodes(
        nodes1: xmlNodeSetPtr,
        nodes2: xmlNodeSetPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNodeLeadingSorted(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathLeadingSorted(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathNodeLeading(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathLeading(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathNodeTrailingSorted(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathTrailingSorted(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathNodeTrailing(nodes: xmlNodeSetPtr, node: xmlNodePtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathTrailing(nodes1: xmlNodeSetPtr, nodes2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathRegisterNs(
        ctxt: xmlXPathContextPtr,
        prefix: *const xmlChar,
        ns_uri: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNsLookup(ctxt: xmlXPathContextPtr, prefix: *const xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlXPathRegisteredNsCleanup(ctxt: xmlXPathContextPtr);
}
extern "C" {
    pub fn xmlXPathRegisterFunc(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        f: xmlXPathFunction,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathRegisterFuncNS(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
        f: xmlXPathFunction,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathRegisterVariable(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        value: xmlXPathObjectPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathRegisterVariableNS(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
        value: xmlXPathObjectPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathFunctionLookup(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
    ) -> xmlXPathFunction;
}
extern "C" {
    pub fn xmlXPathFunctionLookupNS(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
    ) -> xmlXPathFunction;
}
extern "C" {
    pub fn xmlXPathRegisteredFuncsCleanup(ctxt: xmlXPathContextPtr);
}
extern "C" {
    pub fn xmlXPathVariableLookup(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
    ) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathVariableLookupNS(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
    ) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathRegisteredVariablesCleanup(ctxt: xmlXPathContextPtr);
}
extern "C" {
    pub fn xmlXPathNewParserContext(
        str_: *const xmlChar,
        ctxt: xmlXPathContextPtr,
    ) -> xmlXPathParserContextPtr;
}
extern "C" {
    pub fn xmlXPathFreeParserContext(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn valuePop(ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn valuePush(
        ctxt: xmlXPathParserContextPtr,
        value: xmlXPathObjectPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNewString(val: *const xmlChar) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewCString(val: *const ::std::os::raw::c_char) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathWrapString(val: *mut xmlChar) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathWrapCString(val: *mut ::std::os::raw::c_char) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewFloat(val: f64) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewBoolean(val: ::std::os::raw::c_int) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewNodeSet(val: xmlNodePtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNewValueTree(val: xmlNodePtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathNodeSetAdd(cur: xmlNodeSetPtr, val: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNodeSetAddUnique(cur: xmlNodeSetPtr, val: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNodeSetAddNs(
        cur: xmlNodeSetPtr,
        node: xmlNodePtr,
        ns: xmlNsPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNodeSetSort(set: xmlNodeSetPtr);
}
extern "C" {
    pub fn xmlXPathRoot(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathEvalExpr(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathParseName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathParseNCName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlXPathStringEvalNumber(str_: *const xmlChar) -> f64;
}
extern "C" {
    pub fn xmlXPathEvaluatePredicateResult(
        ctxt: xmlXPathParserContextPtr,
        res: xmlXPathObjectPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathRegisterAllFunctions(ctxt: xmlXPathContextPtr);
}
extern "C" {
    pub fn xmlXPathNodeSetMerge(val1: xmlNodeSetPtr, val2: xmlNodeSetPtr) -> xmlNodeSetPtr;
}
extern "C" {
    pub fn xmlXPathNodeSetDel(cur: xmlNodeSetPtr, val: xmlNodePtr);
}
extern "C" {
    pub fn xmlXPathNodeSetRemove(cur: xmlNodeSetPtr, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathNewNodeSetList(val: xmlNodeSetPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathWrapNodeSet(val: xmlNodeSetPtr) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathWrapExternal(val: *mut ::std::os::raw::c_void) -> xmlXPathObjectPtr;
}
extern "C" {
    pub fn xmlXPathEqualValues(ctxt: xmlXPathParserContextPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNotEqualValues(ctxt: xmlXPathParserContextPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathCompareValues(
        ctxt: xmlXPathParserContextPtr,
        inf: ::std::os::raw::c_int,
        strict: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathValueFlipSign(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathAddValues(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathSubValues(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathMultValues(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathDivValues(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathModValues(ctxt: xmlXPathParserContextPtr);
}
extern "C" {
    pub fn xmlXPathIsNodeType(name: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXPathNextSelf(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextChild(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextDescendant(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextDescendantOrSelf(
        ctxt: xmlXPathParserContextPtr,
        cur: xmlNodePtr,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextParent(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextAncestorOrSelf(
        ctxt: xmlXPathParserContextPtr,
        cur: xmlNodePtr,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextFollowingSibling(
        ctxt: xmlXPathParserContextPtr,
        cur: xmlNodePtr,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextFollowing(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextNamespace(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextAttribute(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextPreceding(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextAncestor(ctxt: xmlXPathParserContextPtr, cur: xmlNodePtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathNextPrecedingSibling(
        ctxt: xmlXPathParserContextPtr,
        cur: xmlNodePtr,
    ) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlXPathLastFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathPositionFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathCountFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathIdFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathLocalNameFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathNamespaceURIFunction(
        ctxt: xmlXPathParserContextPtr,
        nargs: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathStringFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathStringLengthFunction(
        ctxt: xmlXPathParserContextPtr,
        nargs: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathConcatFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathContainsFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathStartsWithFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathSubstringFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathSubstringBeforeFunction(
        ctxt: xmlXPathParserContextPtr,
        nargs: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathSubstringAfterFunction(
        ctxt: xmlXPathParserContextPtr,
        nargs: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn xmlXPathNormalizeFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathTranslateFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathNotFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathTrueFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathFalseFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathLangFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathNumberFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathSumFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathFloorFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathCeilingFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathRoundFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathBooleanFunction(ctxt: xmlXPathParserContextPtr, nargs: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlXPathNodeSetFreeNs(ns: xmlNsPtr);
}
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlURI {
    pub scheme: *mut ::std::os::raw::c_char,
    pub opaque: *mut ::std::os::raw::c_char,
    pub authority: *mut ::std::os::raw::c_char,
    pub server: *mut ::std::os::raw::c_char,
    pub user: *mut ::std::os::raw::c_char,
    pub port: ::std::os::raw::c_int,
    pub path: *mut ::std::os::raw::c_char,
    pub query: *mut ::std::os::raw::c_char,
    pub fragment: *mut ::std::os::raw::c_char,
    pub cleanup: ::std::os::raw::c_int,
    pub query_raw: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__xmlURI() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlURI> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlURI>(),
        88usize,
        concat!("Size of: ", stringify!(_xmlURI))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlURI>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlURI))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scheme) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(scheme)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(opaque)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).authority) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(authority)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).server) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(server)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).user) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).port) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).path) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(path)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).query) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(query)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fragment) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(fragment)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cleanup) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(cleanup)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).query_raw) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlURI),
            "::",
            stringify!(query_raw)
        )
    );
}
extern "C" {
    pub fn xmlCreateURI() -> xmlURIPtr;
}
extern "C" {
    pub fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlBuildRelativeURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseURI(str_: *const ::std::os::raw::c_char) -> xmlURIPtr;
}
extern "C" {
    pub fn xmlParseURIRaw(
        str_: *const ::std::os::raw::c_char,
        raw: ::std::os::raw::c_int,
    ) -> xmlURIPtr;
}
extern "C" {
    pub fn xmlParseURIReference(
        uri: xmlURIPtr,
        str_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlPrintURI(stream: *mut FILE, uri: xmlURIPtr);
}
extern "C" {
    pub fn xmlURIEscapeStr(str_: *const xmlChar, list: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlURIUnescapeString(
        str_: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        target: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlNormalizeURIPath(path: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlURIEscape(str_: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlFreeURI(uri: xmlURIPtr);
}
extern "C" {
    pub fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlPathToURI(path: *const xmlChar) -> *mut xmlChar;
}
pub const xmlSchemaValType_XML_SCHEMAS_UNKNOWN: xmlSchemaValType = 0;
pub const xmlSchemaValType_XML_SCHEMAS_STRING: xmlSchemaValType = 1;
pub const xmlSchemaValType_XML_SCHEMAS_NORMSTRING: xmlSchemaValType = 2;
pub const xmlSchemaValType_XML_SCHEMAS_DECIMAL: xmlSchemaValType = 3;
pub const xmlSchemaValType_XML_SCHEMAS_TIME: xmlSchemaValType = 4;
pub const xmlSchemaValType_XML_SCHEMAS_GDAY: xmlSchemaValType = 5;
pub const xmlSchemaValType_XML_SCHEMAS_GMONTH: xmlSchemaValType = 6;
pub const xmlSchemaValType_XML_SCHEMAS_GMONTHDAY: xmlSchemaValType = 7;
pub const xmlSchemaValType_XML_SCHEMAS_GYEAR: xmlSchemaValType = 8;
pub const xmlSchemaValType_XML_SCHEMAS_GYEARMONTH: xmlSchemaValType = 9;
pub const xmlSchemaValType_XML_SCHEMAS_DATE: xmlSchemaValType = 10;
pub const xmlSchemaValType_XML_SCHEMAS_DATETIME: xmlSchemaValType = 11;
pub const xmlSchemaValType_XML_SCHEMAS_DURATION: xmlSchemaValType = 12;
pub const xmlSchemaValType_XML_SCHEMAS_FLOAT: xmlSchemaValType = 13;
pub const xmlSchemaValType_XML_SCHEMAS_DOUBLE: xmlSchemaValType = 14;
pub const xmlSchemaValType_XML_SCHEMAS_BOOLEAN: xmlSchemaValType = 15;
pub const xmlSchemaValType_XML_SCHEMAS_TOKEN: xmlSchemaValType = 16;
pub const xmlSchemaValType_XML_SCHEMAS_LANGUAGE: xmlSchemaValType = 17;
pub const xmlSchemaValType_XML_SCHEMAS_NMTOKEN: xmlSchemaValType = 18;
pub const xmlSchemaValType_XML_SCHEMAS_NMTOKENS: xmlSchemaValType = 19;
pub const xmlSchemaValType_XML_SCHEMAS_NAME: xmlSchemaValType = 20;
pub const xmlSchemaValType_XML_SCHEMAS_QNAME: xmlSchemaValType = 21;
pub const xmlSchemaValType_XML_SCHEMAS_NCNAME: xmlSchemaValType = 22;
pub const xmlSchemaValType_XML_SCHEMAS_ID: xmlSchemaValType = 23;
pub const xmlSchemaValType_XML_SCHEMAS_IDREF: xmlSchemaValType = 24;
pub const xmlSchemaValType_XML_SCHEMAS_IDREFS: xmlSchemaValType = 25;
pub const xmlSchemaValType_XML_SCHEMAS_ENTITY: xmlSchemaValType = 26;
pub const xmlSchemaValType_XML_SCHEMAS_ENTITIES: xmlSchemaValType = 27;
pub const xmlSchemaValType_XML_SCHEMAS_NOTATION: xmlSchemaValType = 28;
pub const xmlSchemaValType_XML_SCHEMAS_ANYURI: xmlSchemaValType = 29;
pub const xmlSchemaValType_XML_SCHEMAS_INTEGER: xmlSchemaValType = 30;
pub const xmlSchemaValType_XML_SCHEMAS_NPINTEGER: xmlSchemaValType = 31;
pub const xmlSchemaValType_XML_SCHEMAS_NINTEGER: xmlSchemaValType = 32;
pub const xmlSchemaValType_XML_SCHEMAS_NNINTEGER: xmlSchemaValType = 33;
pub const xmlSchemaValType_XML_SCHEMAS_PINTEGER: xmlSchemaValType = 34;
pub const xmlSchemaValType_XML_SCHEMAS_INT: xmlSchemaValType = 35;
pub const xmlSchemaValType_XML_SCHEMAS_UINT: xmlSchemaValType = 36;
pub const xmlSchemaValType_XML_SCHEMAS_LONG: xmlSchemaValType = 37;
pub const xmlSchemaValType_XML_SCHEMAS_ULONG: xmlSchemaValType = 38;
pub const xmlSchemaValType_XML_SCHEMAS_SHORT: xmlSchemaValType = 39;
pub const xmlSchemaValType_XML_SCHEMAS_USHORT: xmlSchemaValType = 40;
pub const xmlSchemaValType_XML_SCHEMAS_BYTE: xmlSchemaValType = 41;
pub const xmlSchemaValType_XML_SCHEMAS_UBYTE: xmlSchemaValType = 42;
pub const xmlSchemaValType_XML_SCHEMAS_HEXBINARY: xmlSchemaValType = 43;
pub const xmlSchemaValType_XML_SCHEMAS_BASE64BINARY: xmlSchemaValType = 44;
pub const xmlSchemaValType_XML_SCHEMAS_ANYTYPE: xmlSchemaValType = 45;
pub const xmlSchemaValType_XML_SCHEMAS_ANYSIMPLETYPE: xmlSchemaValType = 46;
pub type xmlSchemaValType = ::std::os::raw::c_uint;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_BASIC: xmlSchemaTypeType = 1;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ANY: xmlSchemaTypeType = 2;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_FACET: xmlSchemaTypeType = 3;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_SIMPLE: xmlSchemaTypeType = 4;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_COMPLEX: xmlSchemaTypeType = 5;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_SEQUENCE: xmlSchemaTypeType = 6;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_CHOICE: xmlSchemaTypeType = 7;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ALL: xmlSchemaTypeType = 8;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_SIMPLE_CONTENT: xmlSchemaTypeType = 9;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_COMPLEX_CONTENT: xmlSchemaTypeType = 10;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_UR: xmlSchemaTypeType = 11;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_RESTRICTION: xmlSchemaTypeType = 12;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_EXTENSION: xmlSchemaTypeType = 13;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ELEMENT: xmlSchemaTypeType = 14;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ATTRIBUTE: xmlSchemaTypeType = 15;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ATTRIBUTEGROUP: xmlSchemaTypeType = 16;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_GROUP: xmlSchemaTypeType = 17;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_NOTATION: xmlSchemaTypeType = 18;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_LIST: xmlSchemaTypeType = 19;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_UNION: xmlSchemaTypeType = 20;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ANY_ATTRIBUTE: xmlSchemaTypeType = 21;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_IDC_UNIQUE: xmlSchemaTypeType = 22;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_IDC_KEY: xmlSchemaTypeType = 23;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_IDC_KEYREF: xmlSchemaTypeType = 24;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_PARTICLE: xmlSchemaTypeType = 25;
pub const xmlSchemaTypeType_XML_SCHEMA_TYPE_ATTRIBUTE_USE: xmlSchemaTypeType = 26;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MININCLUSIVE: xmlSchemaTypeType = 1000;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MINEXCLUSIVE: xmlSchemaTypeType = 1001;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MAXINCLUSIVE: xmlSchemaTypeType = 1002;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MAXEXCLUSIVE: xmlSchemaTypeType = 1003;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_TOTALDIGITS: xmlSchemaTypeType = 1004;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_FRACTIONDIGITS: xmlSchemaTypeType = 1005;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_PATTERN: xmlSchemaTypeType = 1006;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_ENUMERATION: xmlSchemaTypeType = 1007;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_WHITESPACE: xmlSchemaTypeType = 1008;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_LENGTH: xmlSchemaTypeType = 1009;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MAXLENGTH: xmlSchemaTypeType = 1010;
pub const xmlSchemaTypeType_XML_SCHEMA_FACET_MINLENGTH: xmlSchemaTypeType = 1011;
pub const xmlSchemaTypeType_XML_SCHEMA_EXTRA_QNAMEREF: xmlSchemaTypeType = 2000;
pub const xmlSchemaTypeType_XML_SCHEMA_EXTRA_ATTR_USE_PROHIB: xmlSchemaTypeType = 2001;
pub type xmlSchemaTypeType = ::std::os::raw::c_uint;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_UNKNOWN: xmlSchemaContentType = 0;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_EMPTY: xmlSchemaContentType = 1;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_ELEMENTS: xmlSchemaContentType = 2;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_MIXED: xmlSchemaContentType = 3;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_SIMPLE: xmlSchemaContentType = 4;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_MIXED_OR_ELEMENTS: xmlSchemaContentType = 5;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_BASIC: xmlSchemaContentType = 6;
pub const xmlSchemaContentType_XML_SCHEMA_CONTENT_ANY: xmlSchemaContentType = 7;
pub type xmlSchemaContentType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaVal {
    _unused: [u8; 0],
}
pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
pub type xmlSchemaType = _xmlSchemaType;
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaFacet = _xmlSchemaFacet;
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaAnnot = _xmlSchemaAnnot;
pub type xmlSchemaAnnotPtr = *mut xmlSchemaAnnot;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaAnnot {
    pub next: *mut _xmlSchemaAnnot,
    pub content: xmlNodePtr,
}
#[test]
fn bindgen_test_layout__xmlSchemaAnnot() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaAnnot> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaAnnot>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlSchemaAnnot))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaAnnot>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaAnnot))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAnnot),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).content) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAnnot),
            "::",
            stringify!(content)
        )
    );
}
pub type xmlSchemaAttribute = _xmlSchemaAttribute;
pub type xmlSchemaAttributePtr = *mut xmlSchemaAttribute;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaAttribute {
    pub type_: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaAttribute,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_: *const xmlChar,
    pub refNs: *const xmlChar,
    pub typeName: *const xmlChar,
    pub typeNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub base: xmlSchemaTypePtr,
    pub occurs: ::std::os::raw::c_int,
    pub defValue: *const xmlChar,
    pub subtypes: xmlSchemaTypePtr,
    pub node: xmlNodePtr,
    pub targetNamespace: *const xmlChar,
    pub flags: ::std::os::raw::c_int,
    pub refPrefix: *const xmlChar,
    pub defVal: xmlSchemaValPtr,
    pub refDecl: xmlSchemaAttributePtr,
}
#[test]
fn bindgen_test_layout__xmlSchemaAttribute() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaAttribute> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaAttribute>(),
        152usize,
        concat!("Size of: ", stringify!(_xmlSchemaAttribute))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaAttribute>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaAttribute))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ref_) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(ref_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refNs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(refNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typeName) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typeNs) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(typeNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).base) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).occurs) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(occurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).defValue) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(defValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subtypes) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(subtypes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(targetNamespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refPrefix) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(refPrefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).defVal) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(defVal)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refDecl) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttribute),
            "::",
            stringify!(refDecl)
        )
    );
}
pub type xmlSchemaAttributeLink = _xmlSchemaAttributeLink;
pub type xmlSchemaAttributeLinkPtr = *mut xmlSchemaAttributeLink;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaAttributeLink {
    pub next: *mut _xmlSchemaAttributeLink,
    pub attr: *mut _xmlSchemaAttribute,
}
#[test]
fn bindgen_test_layout__xmlSchemaAttributeLink() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaAttributeLink> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaAttributeLink>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlSchemaAttributeLink))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaAttributeLink>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaAttributeLink))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeLink),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeLink),
            "::",
            stringify!(attr)
        )
    );
}
pub type xmlSchemaWildcardNs = _xmlSchemaWildcardNs;
pub type xmlSchemaWildcardNsPtr = *mut xmlSchemaWildcardNs;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaWildcardNs {
    pub next: *mut _xmlSchemaWildcardNs,
    pub value: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlSchemaWildcardNs() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaWildcardNs> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaWildcardNs>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlSchemaWildcardNs))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaWildcardNs>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaWildcardNs))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcardNs),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcardNs),
            "::",
            stringify!(value)
        )
    );
}
pub type xmlSchemaWildcard = _xmlSchemaWildcard;
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaWildcard {
    pub type_: xmlSchemaTypeType,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub minOccurs: ::std::os::raw::c_int,
    pub maxOccurs: ::std::os::raw::c_int,
    pub processContents: ::std::os::raw::c_int,
    pub any: ::std::os::raw::c_int,
    pub nsSet: xmlSchemaWildcardNsPtr,
    pub negNsSet: xmlSchemaWildcardNsPtr,
    pub flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__xmlSchemaWildcard() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaWildcard> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaWildcard>(),
        72usize,
        concat!("Size of: ", stringify!(_xmlSchemaWildcard))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaWildcard>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaWildcard))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minOccurs) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(minOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxOccurs) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(maxOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).processContents) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(processContents)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).any) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(any)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nsSet) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(nsSet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).negNsSet) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(negNsSet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaWildcard),
            "::",
            stringify!(flags)
        )
    );
}
pub type xmlSchemaAttributeGroup = _xmlSchemaAttributeGroup;
pub type xmlSchemaAttributeGroupPtr = *mut xmlSchemaAttributeGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaAttributeGroup {
    pub type_: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaAttribute,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub flags: ::std::os::raw::c_int,
    pub attributeWildcard: xmlSchemaWildcardPtr,
    pub refPrefix: *const xmlChar,
    pub refItem: xmlSchemaAttributeGroupPtr,
    pub targetNamespace: *const xmlChar,
    pub attrUses: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlSchemaAttributeGroup() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaAttributeGroup> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaAttributeGroup>(),
        120usize,
        concat!("Size of: ", stringify!(_xmlSchemaAttributeGroup))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaAttributeGroup>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaAttributeGroup))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ref_) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(ref_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refNs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(refNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(attributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeWildcard) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(attributeWildcard)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refPrefix) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(refPrefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refItem) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(refItem)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(targetNamespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrUses) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaAttributeGroup),
            "::",
            stringify!(attrUses)
        )
    );
}
pub type xmlSchemaTypeLink = _xmlSchemaTypeLink;
pub type xmlSchemaTypeLinkPtr = *mut xmlSchemaTypeLink;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaTypeLink {
    pub next: *mut _xmlSchemaTypeLink,
    pub type_: xmlSchemaTypePtr,
}
#[test]
fn bindgen_test_layout__xmlSchemaTypeLink() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaTypeLink> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaTypeLink>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlSchemaTypeLink))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaTypeLink>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaTypeLink))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaTypeLink),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaTypeLink),
            "::",
            stringify!(type_)
        )
    );
}
pub type xmlSchemaFacetLink = _xmlSchemaFacetLink;
pub type xmlSchemaFacetLinkPtr = *mut xmlSchemaFacetLink;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaFacetLink {
    pub next: *mut _xmlSchemaFacetLink,
    pub facet: xmlSchemaFacetPtr,
}
#[test]
fn bindgen_test_layout__xmlSchemaFacetLink() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaFacetLink> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaFacetLink>(),
        16usize,
        concat!("Size of: ", stringify!(_xmlSchemaFacetLink))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaFacetLink>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaFacetLink))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacetLink),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).facet) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacetLink),
            "::",
            stringify!(facet)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaType {
    pub type_: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaType,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub subtypes: xmlSchemaTypePtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub minOccurs: ::std::os::raw::c_int,
    pub maxOccurs: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub contentType: xmlSchemaContentType,
    pub base: *const xmlChar,
    pub baseNs: *const xmlChar,
    pub baseType: xmlSchemaTypePtr,
    pub facets: xmlSchemaFacetPtr,
    pub redef: *mut _xmlSchemaType,
    pub recurse: ::std::os::raw::c_int,
    pub attributeUses: *mut xmlSchemaAttributeLinkPtr,
    pub attributeWildcard: xmlSchemaWildcardPtr,
    pub builtInType: ::std::os::raw::c_int,
    pub memberTypes: xmlSchemaTypeLinkPtr,
    pub facetSet: xmlSchemaFacetLinkPtr,
    pub refPrefix: *const xmlChar,
    pub contentTypeDef: xmlSchemaTypePtr,
    pub contModel: xmlRegexpPtr,
    pub targetNamespace: *const xmlChar,
    pub attrUses: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlSchemaType() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaType> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaType>(),
        224usize,
        concat!("Size of: ", stringify!(_xmlSchemaType))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaType>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaType))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ref_) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(ref_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refNs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(refNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subtypes) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(subtypes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(attributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minOccurs) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(minOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxOccurs) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(maxOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contentType) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).base) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baseNs) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(baseNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baseType) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(baseType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).facets) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(facets)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).redef) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(redef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).recurse) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(recurse)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeUses) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(attributeUses)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributeWildcard) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(attributeWildcard)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).builtInType) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(builtInType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memberTypes) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(memberTypes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).facetSet) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(facetSet)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refPrefix) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(refPrefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contentTypeDef) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(contentTypeDef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contModel) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(contModel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(targetNamespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrUses) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaType),
            "::",
            stringify!(attrUses)
        )
    );
}
pub type xmlSchemaElement = _xmlSchemaElement;
pub type xmlSchemaElementPtr = *mut xmlSchemaElement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaElement {
    pub type_: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaType,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub subtypes: xmlSchemaTypePtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub minOccurs: ::std::os::raw::c_int,
    pub maxOccurs: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub targetNamespace: *const xmlChar,
    pub namedType: *const xmlChar,
    pub namedTypeNs: *const xmlChar,
    pub substGroup: *const xmlChar,
    pub substGroupNs: *const xmlChar,
    pub scope: *const xmlChar,
    pub value: *const xmlChar,
    pub refDecl: *mut _xmlSchemaElement,
    pub contModel: xmlRegexpPtr,
    pub contentType: xmlSchemaContentType,
    pub refPrefix: *const xmlChar,
    pub defVal: xmlSchemaValPtr,
    pub idcs: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlSchemaElement() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaElement> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaElement>(),
        200usize,
        concat!("Size of: ", stringify!(_xmlSchemaElement))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaElement>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaElement))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ref_) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(ref_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refNs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(refNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subtypes) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(subtypes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(attributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).minOccurs) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(minOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).maxOccurs) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(maxOccurs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(targetNamespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namedType) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(namedType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namedTypeNs) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(namedTypeNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).substGroup) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(substGroup)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).substGroupNs) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(substGroupNs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scope) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(scope)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refDecl) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(refDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contModel) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(contModel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).contentType) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(contentType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refPrefix) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(refPrefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).defVal) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(defVal)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idcs) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaElement),
            "::",
            stringify!(idcs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaFacet {
    pub type_: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaFacet,
    pub value: *const xmlChar,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub fixed: ::std::os::raw::c_int,
    pub whitespace: ::std::os::raw::c_int,
    pub val: xmlSchemaValPtr,
    pub regexp: xmlRegexpPtr,
}
#[test]
fn bindgen_test_layout__xmlSchemaFacet() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaFacet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaFacet>(),
        72usize,
        concat!("Size of: ", stringify!(_xmlSchemaFacet))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaFacet>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaFacet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(next)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).node) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fixed) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(fixed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).whitespace) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(whitespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).regexp) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaFacet),
            "::",
            stringify!(regexp)
        )
    );
}
pub type xmlSchemaNotation = _xmlSchemaNotation;
pub type xmlSchemaNotationPtr = *mut xmlSchemaNotation;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaNotation {
    pub type_: xmlSchemaTypeType,
    pub name: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub identifier: *const xmlChar,
    pub targetNamespace: *const xmlChar,
}
#[test]
fn bindgen_test_layout__xmlSchemaNotation() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchemaNotation> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchemaNotation>(),
        40usize,
        concat!("Size of: ", stringify!(_xmlSchemaNotation))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchemaNotation>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchemaNotation))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaNotation),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaNotation),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaNotation),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).identifier) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaNotation),
            "::",
            stringify!(identifier)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchemaNotation),
            "::",
            stringify!(targetNamespace)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchema {
    pub name: *const xmlChar,
    pub targetNamespace: *const xmlChar,
    pub version: *const xmlChar,
    pub id: *const xmlChar,
    pub doc: xmlDocPtr,
    pub annot: xmlSchemaAnnotPtr,
    pub flags: ::std::os::raw::c_int,
    pub typeDecl: xmlHashTablePtr,
    pub attrDecl: xmlHashTablePtr,
    pub attrgrpDecl: xmlHashTablePtr,
    pub elemDecl: xmlHashTablePtr,
    pub notaDecl: xmlHashTablePtr,
    pub schemasImports: xmlHashTablePtr,
    pub _private: *mut ::std::os::raw::c_void,
    pub groupDecl: xmlHashTablePtr,
    pub dict: xmlDictPtr,
    pub includes: *mut ::std::os::raw::c_void,
    pub preserve: ::std::os::raw::c_int,
    pub counter: ::std::os::raw::c_int,
    pub idcDef: xmlHashTablePtr,
    pub volatiles: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__xmlSchema() {
    const UNINIT: ::std::mem::MaybeUninit<_xmlSchema> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xmlSchema>(),
        160usize,
        concat!("Size of: ", stringify!(_xmlSchema))
    );
    assert_eq!(
        ::std::mem::align_of::<_xmlSchema>(),
        8usize,
        concat!("Alignment of ", stringify!(_xmlSchema))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).targetNamespace) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(targetNamespace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).doc) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(doc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).annot) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(annot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typeDecl) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(typeDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrDecl) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(attrDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrgrpDecl) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(attrgrpDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).elemDecl) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(elemDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).notaDecl) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(notaDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).schemasImports) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(schemasImports)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._private) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(_private)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).groupDecl) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(groupDecl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dict) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(dict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).includes) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(includes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).preserve) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(preserve)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).counter) as usize - ptr as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(counter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idcDef) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(idcDef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).volatiles) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_xmlSchema),
            "::",
            stringify!(volatiles)
        )
    );
}
extern "C" {
    pub fn xmlSchemaFreeType(type_: xmlSchemaTypePtr);
}
extern "C" {
    pub fn xmlSchemaFreeWildcard(wildcard: xmlSchemaWildcardPtr);
}
pub const xmlCatalogPrefer_XML_CATA_PREFER_NONE: xmlCatalogPrefer = 0;
pub const xmlCatalogPrefer_XML_CATA_PREFER_PUBLIC: xmlCatalogPrefer = 1;
pub const xmlCatalogPrefer_XML_CATA_PREFER_SYSTEM: xmlCatalogPrefer = 2;
pub type xmlCatalogPrefer = ::std::os::raw::c_uint;
pub const xmlCatalogAllow_XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
pub const xmlCatalogAllow_XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub const xmlCatalogAllow_XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const xmlCatalogAllow_XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub type xmlCatalogAllow = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlCatalog {
    _unused: [u8; 0],
}
pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;
extern "C" {
    pub fn xmlNewCatalog(sgml: ::std::os::raw::c_int) -> xmlCatalogPtr;
}
extern "C" {
    pub fn xmlLoadACatalog(filename: *const ::std::os::raw::c_char) -> xmlCatalogPtr;
}
extern "C" {
    pub fn xmlLoadSGMLSuperCatalog(filename: *const ::std::os::raw::c_char) -> xmlCatalogPtr;
}
extern "C" {
    pub fn xmlConvertSGMLCatalog(catal: xmlCatalogPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlACatalogAdd(
        catal: xmlCatalogPtr,
        type_: *const xmlChar,
        orig: *const xmlChar,
        replace: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlACatalogRemove(catal: xmlCatalogPtr, value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlACatalogResolve(
        catal: xmlCatalogPtr,
        pubID: *const xmlChar,
        sysID: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlACatalogResolveSystem(catal: xmlCatalogPtr, sysID: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlACatalogResolvePublic(catal: xmlCatalogPtr, pubID: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlACatalogResolveURI(catal: xmlCatalogPtr, URI: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlACatalogDump(catal: xmlCatalogPtr, out: *mut FILE);
}
extern "C" {
    pub fn xmlFreeCatalog(catal: xmlCatalogPtr);
}
extern "C" {
    pub fn xmlCatalogIsEmpty(catal: xmlCatalogPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlInitializeCatalog();
}
extern "C" {
    pub fn xmlLoadCatalog(filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlLoadCatalogs(paths: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn xmlCatalogCleanup();
}
extern "C" {
    pub fn xmlCatalogDump(out: *mut FILE);
}
extern "C" {
    pub fn xmlCatalogResolve(pubID: *const xmlChar, sysID: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogResolveSystem(sysID: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogResolvePublic(pubID: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogResolveURI(URI: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogAdd(
        type_: *const xmlChar,
        orig: *const xmlChar,
        replace: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCatalogRemove(value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseCatalogFile(filename: *const ::std::os::raw::c_char) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlCatalogConvert() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCatalogFreeLocal(catalogs: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlCatalogAddLocal(
        catalogs: *mut ::std::os::raw::c_void,
        URL: *const xmlChar,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlCatalogLocalResolve(
        catalogs: *mut ::std::os::raw::c_void,
        pubID: *const xmlChar,
        sysID: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogLocalResolveURI(
        catalogs: *mut ::std::os::raw::c_void,
        URI: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlCatalogSetDebug(level: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCatalogSetDefaultPrefer(prefer: xmlCatalogPrefer) -> xmlCatalogPrefer;
}
extern "C" {
    pub fn xmlCatalogSetDefaults(allow: xmlCatalogAllow);
}
extern "C" {
    pub fn xmlCatalogGetDefaults() -> xmlCatalogAllow;
}
extern "C" {
    pub fn xmlCatalogGetSystem(sysID: *const xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlCatalogGetPublic(pubID: *const xmlChar) -> *const xmlChar;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlPattern {
    _unused: [u8; 0],
}
pub type xmlPattern = _xmlPattern;
pub type xmlPatternPtr = *mut xmlPattern;
pub const xmlPatternFlags_XML_PATTERN_DEFAULT: xmlPatternFlags = 0;
pub const xmlPatternFlags_XML_PATTERN_XPATH: xmlPatternFlags = 1;
pub const xmlPatternFlags_XML_PATTERN_XSSEL: xmlPatternFlags = 2;
pub const xmlPatternFlags_XML_PATTERN_XSFIELD: xmlPatternFlags = 4;
pub type xmlPatternFlags = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlFreePattern(comp: xmlPatternPtr);
}
extern "C" {
    pub fn xmlFreePatternList(comp: xmlPatternPtr);
}
extern "C" {
    pub fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: ::std::os::raw::c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
}
extern "C" {
    pub fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlStreamCtxt {
    _unused: [u8; 0],
}
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
extern "C" {
    pub fn xmlPatternStreamable(comp: xmlPatternPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlPatternMaxDepth(comp: xmlPatternPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlPatternMinDepth(comp: xmlPatternPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlPatternFromRoot(comp: xmlPatternPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
}
extern "C" {
    pub fn xmlFreeStreamCtxt(stream: xmlStreamCtxtPtr);
}
extern "C" {
    pub fn xmlStreamPushNode(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
        nodeType: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStreamPush(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStreamPushAttr(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStreamPop(stream: xmlStreamCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStreamWantsAnyNode(stream: xmlStreamCtxtPtr) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRelaxNG {
    _unused: [u8; 0],
}
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidityErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlRelaxNGValidityWarningFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRelaxNGParserCtxt {
    _unused: [u8; 0],
}
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlRelaxNGValidCtxt {
    _unused: [u8; 0],
}
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub const xmlRelaxNGValidErr_XML_RELAXNG_OK: xmlRelaxNGValidErr = 0;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_MEMORY: xmlRelaxNGValidErr = 1;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_TYPE: xmlRelaxNGValidErr = 2;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_TYPEVAL: xmlRelaxNGValidErr = 3;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_DUPID: xmlRelaxNGValidErr = 4;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_TYPECMP: xmlRelaxNGValidErr = 5;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_NOSTATE: xmlRelaxNGValidErr = 6;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_NODEFINE: xmlRelaxNGValidErr = 7;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_LISTEXTRA: xmlRelaxNGValidErr = 8;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_LISTEMPTY: xmlRelaxNGValidErr = 9;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_INTERNODATA: xmlRelaxNGValidErr = 10;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_INTERSEQ: xmlRelaxNGValidErr = 11;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_INTEREXTRA: xmlRelaxNGValidErr = 12;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMNAME: xmlRelaxNGValidErr = 13;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ATTRNAME: xmlRelaxNGValidErr = 14;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMNONS: xmlRelaxNGValidErr = 15;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ATTRNONS: xmlRelaxNGValidErr = 16;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMWRONGNS: xmlRelaxNGValidErr = 17;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ATTRWRONGNS: xmlRelaxNGValidErr = 18;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMEXTRANS: xmlRelaxNGValidErr = 19;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ATTREXTRANS: xmlRelaxNGValidErr = 20;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMNOTEMPTY: xmlRelaxNGValidErr = 21;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_NOELEM: xmlRelaxNGValidErr = 22;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_NOTELEM: xmlRelaxNGValidErr = 23;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ATTRVALID: xmlRelaxNGValidErr = 24;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_CONTENTVALID: xmlRelaxNGValidErr = 25;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_EXTRACONTENT: xmlRelaxNGValidErr = 26;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_INVALIDATTR: xmlRelaxNGValidErr = 27;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_DATAELEM: xmlRelaxNGValidErr = 28;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_VALELEM: xmlRelaxNGValidErr = 29;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_LISTELEM: xmlRelaxNGValidErr = 30;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_DATATYPE: xmlRelaxNGValidErr = 31;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_VALUE: xmlRelaxNGValidErr = 32;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_LIST: xmlRelaxNGValidErr = 33;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_NOGRAMMAR: xmlRelaxNGValidErr = 34;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_EXTRADATA: xmlRelaxNGValidErr = 35;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_LACKDATA: xmlRelaxNGValidErr = 36;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_INTERNAL: xmlRelaxNGValidErr = 37;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_ELEMWRONG: xmlRelaxNGValidErr = 38;
pub const xmlRelaxNGValidErr_XML_RELAXNG_ERR_TEXTWRONG: xmlRelaxNGValidErr = 39;
pub type xmlRelaxNGValidErr = ::std::os::raw::c_uint;
pub const xmlRelaxNGParserFlag_XML_RELAXNGP_NONE: xmlRelaxNGParserFlag = 0;
pub const xmlRelaxNGParserFlag_XML_RELAXNGP_FREE_DOC: xmlRelaxNGParserFlag = 1;
pub const xmlRelaxNGParserFlag_XML_RELAXNGP_CRNG: xmlRelaxNGParserFlag = 2;
pub type xmlRelaxNGParserFlag = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlRelaxNGInitTypes() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGCleanupTypes();
}
extern "C" {
    pub fn xmlRelaxNGNewParserCtxt(URL: *const ::std::os::raw::c_char) -> xmlRelaxNGParserCtxtPtr;
}
extern "C" {
    pub fn xmlRelaxNGNewMemParserCtxt(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlRelaxNGParserCtxtPtr;
}
extern "C" {
    pub fn xmlRelaxNGNewDocParserCtxt(doc: xmlDocPtr) -> xmlRelaxNGParserCtxtPtr;
}
extern "C" {
    pub fn xmlRelaxParserSetFlag(
        ctxt: xmlRelaxNGParserCtxtPtr,
        flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
}
extern "C" {
    pub fn xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlRelaxNGGetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGSetParserStructuredErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
}
extern "C" {
    pub fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
}
extern "C" {
    pub fn xmlRelaxNGDump(output: *mut FILE, schema: xmlRelaxNGPtr);
}
extern "C" {
    pub fn xmlRelaxNGDumpTree(output: *mut FILE, schema: xmlRelaxNGPtr);
}
extern "C" {
    pub fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlRelaxNGGetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGSetValidStructuredErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
}
extern "C" {
    pub fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
}
extern "C" {
    pub fn xmlRelaxNGValidateDoc(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGValidatePushCData(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_OK: xmlSchemaValidError = 0;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOROOT: xmlSchemaValidError = 1;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_UNDECLAREDELEM: xmlSchemaValidError = 2;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTTOPLEVEL: xmlSchemaValidError = 3;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_MISSING: xmlSchemaValidError = 4;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_WRONGELEM: xmlSchemaValidError = 5;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTYPE: xmlSchemaValidError = 6;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOROLLBACK: xmlSchemaValidError = 7;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_ISABSTRACT: xmlSchemaValidError = 8;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTEMPTY: xmlSchemaValidError = 9;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_ELEMCONT: xmlSchemaValidError = 10;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_HAVEDEFAULT: xmlSchemaValidError = 11;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTNILLABLE: xmlSchemaValidError = 12;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_EXTRACONTENT: xmlSchemaValidError = 13;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_INVALIDATTR: xmlSchemaValidError = 14;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_INVALIDELEM: xmlSchemaValidError = 15;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTDETERMINIST: xmlSchemaValidError = 16;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_CONSTRUCT: xmlSchemaValidError = 17;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_INTERNAL: xmlSchemaValidError = 18;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_NOTSIMPLE: xmlSchemaValidError = 19;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_ATTRUNKNOWN: xmlSchemaValidError = 20;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_ATTRINVALID: xmlSchemaValidError = 21;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_VALUE: xmlSchemaValidError = 22;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_FACET: xmlSchemaValidError = 23;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_: xmlSchemaValidError = 24;
pub const xmlSchemaValidError_XML_SCHEMAS_ERR_XXX: xmlSchemaValidError = 25;
pub type xmlSchemaValidError = ::std::os::raw::c_uint;
pub const xmlSchemaValidOption_XML_SCHEMA_VAL_VC_I_CREATE: xmlSchemaValidOption = 1;
pub type xmlSchemaValidOption = ::std::os::raw::c_uint;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidityErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
pub type xmlSchemaValidityWarningFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, msg: *const ::std::os::raw::c_char, ...),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaParserCtxt {
    _unused: [u8; 0],
}
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaValidCtxt {
    _unused: [u8; 0],
}
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlSchemaValidityLocatorFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        file: *mut *const ::std::os::raw::c_char,
        line: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn xmlSchemaNewParserCtxt(URL: *const ::std::os::raw::c_char) -> xmlSchemaParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchemaNewMemParserCtxt(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlSchemaParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchemaNewDocParserCtxt(doc: xmlDocPtr) -> xmlSchemaParserCtxtPtr;
}
extern "C" {
    pub fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
}
extern "C" {
    pub fn xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlSchemaSetParserStructuredErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlSchemaGetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
}
extern "C" {
    pub fn xmlSchemaFree(schema: xmlSchemaPtr);
}
extern "C" {
    pub fn xmlSchemaDump(output: *mut FILE, schema: xmlSchemaPtr);
}
extern "C" {
    pub fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlSchemaSetValidStructuredErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlSchemaGetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaSetValidOptions(
        ctxt: xmlSchemaValidCtxtPtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateSetFilename(
        vctxt: xmlSchemaValidCtxtPtr,
        filename: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn xmlSchemaValidCtxtGetOptions(ctxt: xmlSchemaValidCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
}
extern "C" {
    pub fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
}
extern "C" {
    pub fn xmlSchemaValidateDoc(
        ctxt: xmlSchemaValidCtxtPtr,
        instance: xmlDocPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateOneElement(
        ctxt: xmlSchemaValidCtxtPtr,
        elem: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateStream(
        ctxt: xmlSchemaValidCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateFile(
        ctxt: xmlSchemaValidCtxtPtr,
        filename: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidCtxtGetParserCtxt(ctxt: xmlSchemaValidCtxtPtr) -> xmlParserCtxtPtr;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSchemaSAXPlug {
    _unused: [u8; 0],
}
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
extern "C" {
    pub fn xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut ::std::os::raw::c_void,
    ) -> xmlSchemaSAXPlugPtr;
}
extern "C" {
    pub fn xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateSetLocator(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut ::std::os::raw::c_void,
    );
}
pub const xmlParserSeverities_XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub const xmlParserSeverities_XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const xmlParserSeverities_XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const xmlParserSeverities_XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub type xmlParserSeverities = ::std::os::raw::c_uint;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_INITIAL: xmlTextReaderMode = 0;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_INTERACTIVE: xmlTextReaderMode = 1;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_ERROR: xmlTextReaderMode = 2;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_EOF: xmlTextReaderMode = 3;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_CLOSED: xmlTextReaderMode = 4;
pub const xmlTextReaderMode_XML_TEXTREADER_MODE_READING: xmlTextReaderMode = 5;
pub type xmlTextReaderMode = ::std::os::raw::c_uint;
pub const xmlParserProperties_XML_PARSER_LOADDTD: xmlParserProperties = 1;
pub const xmlParserProperties_XML_PARSER_DEFAULTATTRS: xmlParserProperties = 2;
pub const xmlParserProperties_XML_PARSER_VALIDATE: xmlParserProperties = 3;
pub const xmlParserProperties_XML_PARSER_SUBST_ENTITIES: xmlParserProperties = 4;
pub type xmlParserProperties = ::std::os::raw::c_uint;
pub const xmlReaderTypes_XML_READER_TYPE_NONE: xmlReaderTypes = 0;
pub const xmlReaderTypes_XML_READER_TYPE_ELEMENT: xmlReaderTypes = 1;
pub const xmlReaderTypes_XML_READER_TYPE_ATTRIBUTE: xmlReaderTypes = 2;
pub const xmlReaderTypes_XML_READER_TYPE_TEXT: xmlReaderTypes = 3;
pub const xmlReaderTypes_XML_READER_TYPE_CDATA: xmlReaderTypes = 4;
pub const xmlReaderTypes_XML_READER_TYPE_ENTITY_REFERENCE: xmlReaderTypes = 5;
pub const xmlReaderTypes_XML_READER_TYPE_ENTITY: xmlReaderTypes = 6;
pub const xmlReaderTypes_XML_READER_TYPE_PROCESSING_INSTRUCTION: xmlReaderTypes = 7;
pub const xmlReaderTypes_XML_READER_TYPE_COMMENT: xmlReaderTypes = 8;
pub const xmlReaderTypes_XML_READER_TYPE_DOCUMENT: xmlReaderTypes = 9;
pub const xmlReaderTypes_XML_READER_TYPE_DOCUMENT_TYPE: xmlReaderTypes = 10;
pub const xmlReaderTypes_XML_READER_TYPE_DOCUMENT_FRAGMENT: xmlReaderTypes = 11;
pub const xmlReaderTypes_XML_READER_TYPE_NOTATION: xmlReaderTypes = 12;
pub const xmlReaderTypes_XML_READER_TYPE_WHITESPACE: xmlReaderTypes = 13;
pub const xmlReaderTypes_XML_READER_TYPE_SIGNIFICANT_WHITESPACE: xmlReaderTypes = 14;
pub const xmlReaderTypes_XML_READER_TYPE_END_ELEMENT: xmlReaderTypes = 15;
pub const xmlReaderTypes_XML_READER_TYPE_END_ENTITY: xmlReaderTypes = 16;
pub const xmlReaderTypes_XML_READER_TYPE_XML_DECLARATION: xmlReaderTypes = 17;
pub type xmlReaderTypes = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlTextReader {
    _unused: [u8; 0],
}
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
extern "C" {
    pub fn xmlNewTextReader(
        input: xmlParserInputBufferPtr,
        URI: *const ::std::os::raw::c_char,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlNewTextReaderFilename(URI: *const ::std::os::raw::c_char) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlFreeTextReader(reader: xmlTextReaderPtr);
}
extern "C" {
    pub fn xmlTextReaderSetup(
        reader: xmlTextReaderPtr,
        input: xmlParserInputBufferPtr,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderReadInnerXml(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderReadOuterXml(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderReadString(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderReadAttributeValue(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderAttributeCount(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderDepth(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderHasAttributes(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderIsDefault(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderQuoteChar(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderReadState(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderIsNamespaceDecl(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderConstBaseUri(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstLocalName(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstNamespaceUri(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstPrefix(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstXmlLang(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstString(
        reader: xmlTextReaderPtr,
        str_: *const xmlChar,
    ) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderBaseUri(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderLocalName(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderName(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderNamespaceUri(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderPrefix(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderXmlLang(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderValue(reader: xmlTextReaderPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderClose(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderGetAttributeNo(
        reader: xmlTextReaderPtr,
        no: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderGetAttribute(
        reader: xmlTextReaderPtr,
        name: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderGetAttributeNs(
        reader: xmlTextReaderPtr,
        localName: *const xmlChar,
        namespaceURI: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderGetRemainder(reader: xmlTextReaderPtr) -> xmlParserInputBufferPtr;
}
extern "C" {
    pub fn xmlTextReaderLookupNamespace(
        reader: xmlTextReaderPtr,
        prefix: *const xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderMoveToAttributeNo(
        reader: xmlTextReaderPtr,
        no: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderMoveToAttribute(
        reader: xmlTextReaderPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderMoveToAttributeNs(
        reader: xmlTextReaderPtr,
        localName: *const xmlChar,
        namespaceURI: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderMoveToFirstAttribute(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderMoveToNextAttribute(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderMoveToElement(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderNormalization(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderConstEncoding(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderSetParserProp(
        reader: xmlTextReaderPtr,
        prop: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderGetParserProp(
        reader: xmlTextReaderPtr,
        prop: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderCurrentNode(reader: xmlTextReaderPtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlTextReaderGetParserLineNumber(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderGetParserColumnNumber(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderPreserve(reader: xmlTextReaderPtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlTextReaderPreservePattern(
        reader: xmlTextReaderPtr,
        pattern: *const xmlChar,
        namespaces: *mut *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderCurrentDoc(reader: xmlTextReaderPtr) -> xmlDocPtr;
}
extern "C" {
    pub fn xmlTextReaderExpand(reader: xmlTextReaderPtr) -> xmlNodePtr;
}
extern "C" {
    pub fn xmlTextReaderNext(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderNextSibling(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderIsValid(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderRelaxNGValidate(
        reader: xmlTextReaderPtr,
        rng: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderRelaxNGValidateCtxt(
        reader: xmlTextReaderPtr,
        ctxt: xmlRelaxNGValidCtxtPtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderRelaxNGSetSchema(
        reader: xmlTextReaderPtr,
        schema: xmlRelaxNGPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderSchemaValidate(
        reader: xmlTextReaderPtr,
        xsd: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderSchemaValidateCtxt(
        reader: xmlTextReaderPtr,
        ctxt: xmlSchemaValidCtxtPtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderSetSchema(
        reader: xmlTextReaderPtr,
        schema: xmlSchemaPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderConstXmlVersion(reader: xmlTextReaderPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlTextReaderStandalone(reader: xmlTextReaderPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderByteConsumed(reader: xmlTextReaderPtr) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn xmlReaderWalker(doc: xmlDocPtr) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderForDoc(
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderForFile(
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderForMemory(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderForFd(
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderForIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlTextReaderPtr;
}
extern "C" {
    pub fn xmlReaderNewWalker(reader: xmlTextReaderPtr, doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReaderNewDoc(
        reader: xmlTextReaderPtr,
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReaderNewFile(
        reader: xmlTextReaderPtr,
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReaderNewMemory(
        reader: xmlTextReaderPtr,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReaderNewFd(
        reader: xmlTextReaderPtr,
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlReaderNewIO(
        reader: xmlTextReaderPtr,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type xmlTextReaderLocatorPtr = *mut ::std::os::raw::c_void;
pub type xmlTextReaderErrorFunc = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        severity: xmlParserSeverities,
        locator: xmlTextReaderLocatorPtr,
    ),
>;
extern "C" {
    pub fn xmlTextReaderLocatorLineNumber(
        locator: xmlTextReaderLocatorPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextReaderLocatorBaseURI(locator: xmlTextReaderLocatorPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlTextReaderSetErrorHandler(
        reader: xmlTextReaderPtr,
        f: xmlTextReaderErrorFunc,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlTextReaderSetStructuredErrorHandler(
        reader: xmlTextReaderPtr,
        f: xmlStructuredErrorFunc,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn xmlTextReaderGetErrorHandler(
        reader: xmlTextReaderPtr,
        f: *mut xmlTextReaderErrorFunc,
        arg: *mut *mut ::std::os::raw::c_void,
    );
}
pub type xlinkHRef = *mut xmlChar;
pub type xlinkRole = *mut xmlChar;
pub type xlinkTitle = *mut xmlChar;
pub const xlinkType_XLINK_TYPE_NONE: xlinkType = 0;
pub const xlinkType_XLINK_TYPE_SIMPLE: xlinkType = 1;
pub const xlinkType_XLINK_TYPE_EXTENDED: xlinkType = 2;
pub const xlinkType_XLINK_TYPE_EXTENDED_SET: xlinkType = 3;
pub type xlinkType = ::std::os::raw::c_uint;
pub const xlinkShow_XLINK_SHOW_NONE: xlinkShow = 0;
pub const xlinkShow_XLINK_SHOW_NEW: xlinkShow = 1;
pub const xlinkShow_XLINK_SHOW_EMBED: xlinkShow = 2;
pub const xlinkShow_XLINK_SHOW_REPLACE: xlinkShow = 3;
pub type xlinkShow = ::std::os::raw::c_uint;
pub const xlinkActuate_XLINK_ACTUATE_NONE: xlinkActuate = 0;
pub const xlinkActuate_XLINK_ACTUATE_AUTO: xlinkActuate = 1;
pub const xlinkActuate_XLINK_ACTUATE_ONREQUEST: xlinkActuate = 2;
pub type xlinkActuate = ::std::os::raw::c_uint;
pub type xlinkNodeDetectFunc =
    ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, node: xmlNodePtr)>;
pub type xlinkSimpleLinkFunk = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        node: xmlNodePtr,
        href: xlinkHRef,
        role: xlinkRole,
        title: xlinkTitle,
    ),
>;
pub type xlinkExtendedLinkFunk = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        node: xmlNodePtr,
        nbLocators: ::std::os::raw::c_int,
        hrefs: *const xlinkHRef,
        roles: *const xlinkRole,
        nbArcs: ::std::os::raw::c_int,
        from: *const xlinkRole,
        to: *const xlinkRole,
        show: *mut xlinkShow,
        actuate: *mut xlinkActuate,
        nbTitles: ::std::os::raw::c_int,
        titles: *const xlinkTitle,
        langs: *mut *const xmlChar,
    ),
>;
pub type xlinkExtendedLinkSetFunk = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        node: xmlNodePtr,
        nbLocators: ::std::os::raw::c_int,
        hrefs: *const xlinkHRef,
        roles: *const xlinkRole,
        nbTitles: ::std::os::raw::c_int,
        titles: *const xlinkTitle,
        langs: *mut *const xmlChar,
    ),
>;
pub type xlinkHandler = _xlinkHandler;
pub type xlinkHandlerPtr = *mut xlinkHandler;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xlinkHandler {
    pub simple: xlinkSimpleLinkFunk,
    pub extended: xlinkExtendedLinkFunk,
    pub set: xlinkExtendedLinkSetFunk,
}
#[test]
fn bindgen_test_layout__xlinkHandler() {
    const UNINIT: ::std::mem::MaybeUninit<_xlinkHandler> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_xlinkHandler>(),
        24usize,
        concat!("Size of: ", stringify!(_xlinkHandler))
    );
    assert_eq!(
        ::std::mem::align_of::<_xlinkHandler>(),
        8usize,
        concat!("Alignment of ", stringify!(_xlinkHandler))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).simple) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_xlinkHandler),
            "::",
            stringify!(simple)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).extended) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_xlinkHandler),
            "::",
            stringify!(extended)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).set) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_xlinkHandler),
            "::",
            stringify!(set)
        )
    );
}
extern "C" {
    pub fn xlinkGetDefaultDetect() -> xlinkNodeDetectFunc;
}
extern "C" {
    pub fn xlinkSetDefaultDetect(func: xlinkNodeDetectFunc);
}
extern "C" {
    pub fn xlinkGetDefaultHandler() -> xlinkHandlerPtr;
}
extern "C" {
    pub fn xlinkSetDefaultHandler(handler: xlinkHandlerPtr);
}
extern "C" {
    pub fn xlinkIsLink(doc: xmlDocPtr, node: xmlNodePtr) -> xlinkType;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlXIncludeCtxt {
    _unused: [u8; 0],
}
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
extern "C" {
    pub fn xmlXIncludeProcess(doc: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeProcessFlags(
        doc: xmlDocPtr,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeProcessFlagsData(
        doc: xmlDocPtr,
        flags: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeProcessTreeFlagsData(
        tree: xmlNodePtr,
        flags: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeProcessTree(tree: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeProcessTreeFlags(
        tree: xmlNodePtr,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeNewContext(doc: xmlDocPtr) -> xmlXIncludeCtxtPtr;
}
extern "C" {
    pub fn xmlXIncludeSetFlags(
        ctxt: xmlXIncludeCtxtPtr,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlXIncludeFreeContext(ctxt: xmlXIncludeCtxtPtr);
}
extern "C" {
    pub fn xmlXIncludeProcessNode(
        ctxt: xmlXIncludeCtxtPtr,
        tree: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
pub const xmlSaveOption_XML_SAVE_FORMAT: xmlSaveOption = 1;
pub const xmlSaveOption_XML_SAVE_NO_DECL: xmlSaveOption = 2;
pub const xmlSaveOption_XML_SAVE_NO_EMPTY: xmlSaveOption = 4;
pub const xmlSaveOption_XML_SAVE_NO_XHTML: xmlSaveOption = 8;
pub const xmlSaveOption_XML_SAVE_XHTML: xmlSaveOption = 16;
pub const xmlSaveOption_XML_SAVE_AS_XML: xmlSaveOption = 32;
pub const xmlSaveOption_XML_SAVE_AS_HTML: xmlSaveOption = 64;
pub const xmlSaveOption_XML_SAVE_WSNONSIG: xmlSaveOption = 128;
pub type xmlSaveOption = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlSaveCtxt {
    _unused: [u8; 0],
}
pub type xmlSaveCtxt = _xmlSaveCtxt;
pub type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
extern "C" {
    pub fn xmlSaveToFd(
        fd: ::std::os::raw::c_int,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlSaveCtxtPtr;
}
extern "C" {
    pub fn xmlSaveToFilename(
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlSaveCtxtPtr;
}
extern "C" {
    pub fn xmlSaveToBuffer(
        buffer: xmlBufferPtr,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlSaveCtxtPtr;
}
extern "C" {
    pub fn xmlSaveToIO(
        iowrite: xmlOutputWriteCallback,
        ioclose: xmlOutputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlSaveCtxtPtr;
}
extern "C" {
    pub fn xmlSaveDoc(ctxt: xmlSaveCtxtPtr, doc: xmlDocPtr) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn xmlSaveTree(ctxt: xmlSaveCtxtPtr, node: xmlNodePtr) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn xmlSaveFlush(ctxt: xmlSaveCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveClose(ctxt: xmlSaveCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveSetEscape(
        ctxt: xmlSaveCtxtPtr,
        escape: xmlCharEncodingOutputFunc,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSaveSetAttrEscape(
        ctxt: xmlSaveCtxtPtr,
        escape: xmlCharEncodingOutputFunc,
    ) -> ::std::os::raw::c_int;
}
pub type htmlParserCtxt = xmlParserCtxt;
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlParserNodeInfo = xmlParserNodeInfo;
pub type htmlSAXHandler = xmlSAXHandler;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlParserInput = xmlParserInput;
pub type htmlParserInputPtr = xmlParserInputPtr;
pub type htmlDocPtr = xmlDocPtr;
pub type htmlNodePtr = xmlNodePtr;
pub type htmlElemDesc = _htmlElemDesc;
pub type htmlElemDescPtr = *mut htmlElemDesc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _htmlElemDesc {
    pub name: *const ::std::os::raw::c_char,
    pub startTag: ::std::os::raw::c_char,
    pub endTag: ::std::os::raw::c_char,
    pub saveEndTag: ::std::os::raw::c_char,
    pub empty: ::std::os::raw::c_char,
    pub depr: ::std::os::raw::c_char,
    pub dtd: ::std::os::raw::c_char,
    pub isinline: ::std::os::raw::c_char,
    pub desc: *const ::std::os::raw::c_char,
    pub subelts: *mut *const ::std::os::raw::c_char,
    pub defaultsubelt: *const ::std::os::raw::c_char,
    pub attrs_opt: *mut *const ::std::os::raw::c_char,
    pub attrs_depr: *mut *const ::std::os::raw::c_char,
    pub attrs_req: *mut *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__htmlElemDesc() {
    const UNINIT: ::std::mem::MaybeUninit<_htmlElemDesc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_htmlElemDesc>(),
        64usize,
        concat!("Size of: ", stringify!(_htmlElemDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<_htmlElemDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(_htmlElemDesc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).startTag) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(startTag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).endTag) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(endTag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).saveEndTag) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(saveEndTag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).empty) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(empty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).depr) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(depr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dtd) as usize - ptr as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(dtd)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).isinline) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(isinline)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).desc) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(desc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subelts) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(subelts)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).defaultsubelt) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(defaultsubelt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrs_opt) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(attrs_opt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrs_depr) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(attrs_depr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).attrs_req) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlElemDesc),
            "::",
            stringify!(attrs_req)
        )
    );
}
pub type htmlEntityDesc = _htmlEntityDesc;
pub type htmlEntityDescPtr = *mut htmlEntityDesc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _htmlEntityDesc {
    pub value: ::std::os::raw::c_uint,
    pub name: *const ::std::os::raw::c_char,
    pub desc: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout__htmlEntityDesc() {
    const UNINIT: ::std::mem::MaybeUninit<_htmlEntityDesc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_htmlEntityDesc>(),
        24usize,
        concat!("Size of: ", stringify!(_htmlEntityDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<_htmlEntityDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(_htmlEntityDesc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlEntityDesc),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlEntityDesc),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).desc) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_htmlEntityDesc),
            "::",
            stringify!(desc)
        )
    );
}
extern "C" {
    pub fn htmlTagLookup(tag: *const xmlChar) -> *const htmlElemDesc;
}
extern "C" {
    pub fn htmlEntityLookup(name: *const xmlChar) -> *const htmlEntityDesc;
}
extern "C" {
    pub fn htmlEntityValueLookup(value: ::std::os::raw::c_uint) -> *const htmlEntityDesc;
}
extern "C" {
    pub fn htmlIsAutoClosed(doc: htmlDocPtr, elem: htmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlAutoCloseTag(
        doc: htmlDocPtr,
        name: *const xmlChar,
        elem: htmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlParseEntityRef(
        ctxt: htmlParserCtxtPtr,
        str_: *mut *const xmlChar,
    ) -> *const htmlEntityDesc;
}
extern "C" {
    pub fn htmlParseCharRef(ctxt: htmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlParseElement(ctxt: htmlParserCtxtPtr);
}
extern "C" {
    pub fn htmlNewParserCtxt() -> htmlParserCtxtPtr;
}
extern "C" {
    pub fn htmlCreateMemoryParserCtxt(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> htmlParserCtxtPtr;
}
extern "C" {
    pub fn htmlParseDocument(ctxt: htmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlSAXParseDoc(
        cur: *const xmlChar,
        encoding: *const ::std::os::raw::c_char,
        sax: htmlSAXHandlerPtr,
        userData: *mut ::std::os::raw::c_void,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlParseDoc(cur: *const xmlChar, encoding: *const ::std::os::raw::c_char)
        -> htmlDocPtr;
}
extern "C" {
    pub fn htmlSAXParseFile(
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        sax: htmlSAXHandlerPtr,
        userData: *mut ::std::os::raw::c_void,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlParseFile(
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn UTF8ToHtml(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlEncodeEntities(
        out: *mut ::std::os::raw::c_uchar,
        outlen: *mut ::std::os::raw::c_int,
        in_: *const ::std::os::raw::c_uchar,
        inlen: *mut ::std::os::raw::c_int,
        quoteChar: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlIsScriptAttribute(name: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlHandleOmittedElem(val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlCreatePushParserCtxt(
        sax: htmlSAXHandlerPtr,
        user_data: *mut ::std::os::raw::c_void,
        chunk: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
        enc: xmlCharEncoding,
    ) -> htmlParserCtxtPtr;
}
extern "C" {
    pub fn htmlParseChunk(
        ctxt: htmlParserCtxtPtr,
        chunk: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        terminate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlFreeParserCtxt(ctxt: htmlParserCtxtPtr);
}
pub const htmlParserOption_HTML_PARSE_RECOVER: htmlParserOption = 1;
pub const htmlParserOption_HTML_PARSE_NODEFDTD: htmlParserOption = 4;
pub const htmlParserOption_HTML_PARSE_NOERROR: htmlParserOption = 32;
pub const htmlParserOption_HTML_PARSE_NOWARNING: htmlParserOption = 64;
pub const htmlParserOption_HTML_PARSE_PEDANTIC: htmlParserOption = 128;
pub const htmlParserOption_HTML_PARSE_NOBLANKS: htmlParserOption = 256;
pub const htmlParserOption_HTML_PARSE_NONET: htmlParserOption = 2048;
pub const htmlParserOption_HTML_PARSE_NOIMPLIED: htmlParserOption = 8192;
pub const htmlParserOption_HTML_PARSE_COMPACT: htmlParserOption = 65536;
pub const htmlParserOption_HTML_PARSE_IGNORE_ENC: htmlParserOption = 2097152;
pub type htmlParserOption = ::std::os::raw::c_uint;
extern "C" {
    pub fn htmlCtxtReset(ctxt: htmlParserCtxtPtr);
}
extern "C" {
    pub fn htmlCtxtUseOptions(
        ctxt: htmlParserCtxtPtr,
        options: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlReadDoc(
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlReadFile(
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlReadMemory(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlReadFd(
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlReadIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlCtxtReadDoc(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlCtxtReadFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlCtxtReadMemory(
        ctxt: xmlParserCtxtPtr,
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlCtxtReadFd(
        ctxt: xmlParserCtxtPtr,
        fd: ::std::os::raw::c_int,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlCtxtReadIO(
        ctxt: xmlParserCtxtPtr,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::std::os::raw::c_void,
        URL: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> htmlDocPtr;
}
pub const htmlStatus_HTML_NA: htmlStatus = 0;
pub const htmlStatus_HTML_INVALID: htmlStatus = 1;
pub const htmlStatus_HTML_DEPRECATED: htmlStatus = 2;
pub const htmlStatus_HTML_VALID: htmlStatus = 4;
pub const htmlStatus_HTML_REQUIRED: htmlStatus = 12;
pub type htmlStatus = ::std::os::raw::c_uint;
extern "C" {
    pub fn htmlAttrAllowed(
        arg1: *const htmlElemDesc,
        arg2: *const xmlChar,
        arg3: ::std::os::raw::c_int,
    ) -> htmlStatus;
}
extern "C" {
    pub fn htmlElementAllowedHere(
        arg1: *const htmlElemDesc,
        arg2: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlElementStatusHere(
        arg1: *const htmlElemDesc,
        arg2: *const htmlElemDesc,
    ) -> htmlStatus;
}
extern "C" {
    pub fn htmlNodeStatus(arg1: htmlNodePtr, arg2: ::std::os::raw::c_int) -> htmlStatus;
}
extern "C" {
    pub fn htmlNewDoc(URI: *const xmlChar, ExternalID: *const xmlChar) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlNewDocNoDtD(URI: *const xmlChar, ExternalID: *const xmlChar) -> htmlDocPtr;
}
extern "C" {
    pub fn htmlGetMetaEncoding(doc: htmlDocPtr) -> *const xmlChar;
}
extern "C" {
    pub fn htmlSetMetaEncoding(doc: htmlDocPtr, encoding: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlDocDumpMemory(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn htmlDocDumpMemoryFormat(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlSaveFile(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlNodeDump(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlNodeDumpFile(out: *mut FILE, doc: xmlDocPtr, cur: xmlNodePtr);
}
extern "C" {
    pub fn htmlNodeDumpFileFormat(
        out: *mut FILE,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlSaveFileEnc(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlSaveFileFormat(
        filename: *const ::std::os::raw::c_char,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn htmlNodeDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn htmlDocContentDumpOutput(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn htmlDocContentDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const ::std::os::raw::c_char,
        format: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn htmlNodeDumpOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn htmlIsBooleanAttr(name: *const xmlChar) -> ::std::os::raw::c_int;
}
pub const xmlSchemaWhitespaceValueType_XML_SCHEMA_WHITESPACE_UNKNOWN: xmlSchemaWhitespaceValueType =
    0;
pub const xmlSchemaWhitespaceValueType_XML_SCHEMA_WHITESPACE_PRESERVE:
    xmlSchemaWhitespaceValueType = 1;
pub const xmlSchemaWhitespaceValueType_XML_SCHEMA_WHITESPACE_REPLACE: xmlSchemaWhitespaceValueType =
    2;
pub const xmlSchemaWhitespaceValueType_XML_SCHEMA_WHITESPACE_COLLAPSE:
    xmlSchemaWhitespaceValueType = 3;
pub type xmlSchemaWhitespaceValueType = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlSchemaInitTypes();
}
extern "C" {
    pub fn xmlSchemaCleanupTypes();
}
extern "C" {
    pub fn xmlSchemaGetPredefinedType(name: *const xmlChar, ns: *const xmlChar)
        -> xmlSchemaTypePtr;
}
extern "C" {
    pub fn xmlSchemaValidatePredefinedType(
        type_: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValPredefTypeNode(
        type_: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateFacet(
        base: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        fws: xmlSchemaWhitespaceValueType,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        ws: xmlSchemaWhitespaceValueType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaFreeValue(val: xmlSchemaValPtr);
}
extern "C" {
    pub fn xmlSchemaNewFacet() -> xmlSchemaFacetPtr;
}
extern "C" {
    pub fn xmlSchemaCheckFacet(
        facet: xmlSchemaFacetPtr,
        typeDecl: xmlSchemaTypePtr,
        ctxt: xmlSchemaParserCtxtPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaFreeFacet(facet: xmlSchemaFacetPtr);
}
extern "C" {
    pub fn xmlSchemaCompareValues(x: xmlSchemaValPtr, y: xmlSchemaValPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaGetBuiltInListSimpleTypeItemType(type_: xmlSchemaTypePtr) -> xmlSchemaTypePtr;
}
extern "C" {
    pub fn xmlSchemaValidateListSimpleTypeFacet(
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        actualLen: ::std::os::raw::c_ulong,
        expectedLen: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaGetBuiltInType(type_: xmlSchemaValType) -> xmlSchemaTypePtr;
}
extern "C" {
    pub fn xmlSchemaIsBuiltInTypeFacet(
        type_: xmlSchemaTypePtr,
        facetType: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaCollapseString(value: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlSchemaWhiteSpaceReplace(value: *const xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlSchemaGetFacetValueAsULong(facet: xmlSchemaFacetPtr) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn xmlSchemaValidateLengthFacet(
        type_: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValidateLengthFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut ::std::os::raw::c_ulong,
        ws: xmlSchemaWhitespaceValueType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValPredefTypeNodeNoNorm(
        type_: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaGetCanonValue(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaGetCanonValueWhtsp(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
        ws: xmlSchemaWhitespaceValueType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValueAppend(
        prev: xmlSchemaValPtr,
        cur: xmlSchemaValPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaValueGetNext(cur: xmlSchemaValPtr) -> xmlSchemaValPtr;
}
extern "C" {
    pub fn xmlSchemaValueGetAsString(val: xmlSchemaValPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlSchemaValueGetAsBoolean(val: xmlSchemaValPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaNewStringValue(
        type_: xmlSchemaValType,
        value: *const xmlChar,
    ) -> xmlSchemaValPtr;
}
extern "C" {
    pub fn xmlSchemaNewNOTATIONValue(name: *const xmlChar, ns: *const xmlChar) -> xmlSchemaValPtr;
}
extern "C" {
    pub fn xmlSchemaNewQNameValue(
        namespaceName: *const xmlChar,
        localName: *const xmlChar,
    ) -> xmlSchemaValPtr;
}
extern "C" {
    pub fn xmlSchemaCompareValuesWhtsp(
        x: xmlSchemaValPtr,
        xws: xmlSchemaWhitespaceValueType,
        y: xmlSchemaValPtr,
        yws: xmlSchemaWhitespaceValueType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSchemaCopyValue(val: xmlSchemaValPtr) -> xmlSchemaValPtr;
}
extern "C" {
    pub fn xmlSchemaGetValType(val: xmlSchemaValPtr) -> xmlSchemaValType;
}
pub const xmlC14NMode_XML_C14N_1_0: xmlC14NMode = 0;
pub const xmlC14NMode_XML_C14N_EXCLUSIVE_1_0: xmlC14NMode = 1;
pub const xmlC14NMode_XML_C14N_1_1: xmlC14NMode = 2;
pub type xmlC14NMode = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlC14NDocSaveTo(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: ::std::os::raw::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: ::std::os::raw::c_int,
        buf: xmlOutputBufferPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlC14NDocDumpMemory(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: ::std::os::raw::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: ::std::os::raw::c_int,
        doc_txt_ptr: *mut *mut xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlC14NDocSave(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: ::std::os::raw::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
        compression: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type xmlC14NIsVisibleCallback = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        node: xmlNodePtr,
        parent: xmlNodePtr,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn xmlC14NExecute(
        doc: xmlDocPtr,
        is_visible_callback: xmlC14NIsVisibleCallback,
        user_data: *mut ::std::os::raw::c_void,
        mode: ::std::os::raw::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: ::std::os::raw::c_int,
        buf: xmlOutputBufferPtr,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlModule {
    _unused: [u8; 0],
}
pub type xmlModule = _xmlModule;
pub type xmlModulePtr = *mut xmlModule;
pub const xmlModuleOption_XML_MODULE_LAZY: xmlModuleOption = 1;
pub const xmlModuleOption_XML_MODULE_LOCAL: xmlModuleOption = 2;
pub type xmlModuleOption = ::std::os::raw::c_uint;
extern "C" {
    pub fn xmlModuleOpen(
        filename: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlModulePtr;
}
extern "C" {
    pub fn xmlModuleSymbol(
        module: xmlModulePtr,
        name: *const ::std::os::raw::c_char,
        result: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlModuleClose(module: xmlModulePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlModuleFree(module: xmlModulePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPInit();
}
extern "C" {
    pub fn xmlNanoHTTPCleanup();
}
extern "C" {
    pub fn xmlNanoHTTPScanProxy(URL: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn xmlNanoHTTPFetch(
        URL: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
        contentType: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPMethod(
        URL: *const ::std::os::raw::c_char,
        method: *const ::std::os::raw::c_char,
        input: *const ::std::os::raw::c_char,
        contentType: *mut *mut ::std::os::raw::c_char,
        headers: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlNanoHTTPMethodRedir(
        URL: *const ::std::os::raw::c_char,
        method: *const ::std::os::raw::c_char,
        input: *const ::std::os::raw::c_char,
        contentType: *mut *mut ::std::os::raw::c_char,
        redir: *mut *mut ::std::os::raw::c_char,
        headers: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlNanoHTTPOpen(
        URL: *const ::std::os::raw::c_char,
        contentType: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlNanoHTTPOpenRedir(
        URL: *const ::std::os::raw::c_char,
        contentType: *mut *mut ::std::os::raw::c_char,
        redir: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn xmlNanoHTTPReturnCode(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPAuthHeader(ctx: *mut ::std::os::raw::c_void)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlNanoHTTPRedir(ctx: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlNanoHTTPContentLength(ctx: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPEncoding(ctx: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlNanoHTTPMimeType(ctx: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn xmlNanoHTTPRead(
        ctx: *mut ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPSave(
        ctxt: *mut ::std::os::raw::c_void,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNanoHTTPClose(ctx: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn xmlUCSIsAegeanNumbers(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsAlphabeticPresentationForms(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsArabic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsArabicPresentationFormsA(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsArabicPresentationFormsB(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsArmenian(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsArrows(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBasicLatin(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBengali(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBlockElements(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBopomofo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBopomofoExtended(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBoxDrawing(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBraillePatterns(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBuhid(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsByzantineMusicalSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKCompatibility(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKCompatibilityForms(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKCompatibilityIdeographs(code: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKCompatibilityIdeographsSupplement(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKRadicalsSupplement(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKSymbolsandPunctuation(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKUnifiedIdeographs(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKUnifiedIdeographsExtensionA(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCJKUnifiedIdeographsExtensionB(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCherokee(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCombiningDiacriticalMarks(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCombiningDiacriticalMarksforSymbols(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCombiningHalfMarks(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCombiningMarksforSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsControlPictures(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCurrencySymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCypriotSyllabary(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCyrillic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCyrillicSupplement(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsDeseret(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsDevanagari(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsDingbats(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsEnclosedAlphanumerics(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsEnclosedCJKLettersandMonths(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsEthiopic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGeneralPunctuation(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGeometricShapes(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGeorgian(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGothic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGreek(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGreekExtended(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGreekandCoptic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGujarati(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsGurmukhi(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHalfwidthandFullwidthForms(code: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHangulCompatibilityJamo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHangulJamo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHangulSyllables(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHanunoo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHebrew(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHighPrivateUseSurrogates(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHighSurrogates(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsHiragana(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsIPAExtensions(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsIdeographicDescriptionCharacters(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKanbun(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKangxiRadicals(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKannada(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKatakana(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKatakanaPhoneticExtensions(code: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKhmer(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsKhmerSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLao(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLatin1Supplement(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLatinExtendedA(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLatinExtendedB(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLatinExtendedAdditional(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLetterlikeSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLimbu(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLinearBIdeograms(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLinearBSyllabary(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsLowSurrogates(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMalayalam(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMathematicalAlphanumericSymbols(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMathematicalOperators(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMiscellaneousMathematicalSymbolsA(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMiscellaneousMathematicalSymbolsB(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMiscellaneousSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMiscellaneousSymbolsandArrows(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMiscellaneousTechnical(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMongolian(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMusicalSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsMyanmar(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsNumberForms(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsOgham(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsOldItalic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsOpticalCharacterRecognition(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsOriya(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsOsmanya(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsPhoneticExtensions(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsPrivateUse(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsPrivateUseArea(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsRunic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsShavian(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSinhala(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSmallFormVariants(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSpacingModifierLetters(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSpecials(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSuperscriptsandSubscripts(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSupplementalArrowsA(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSupplementalArrowsB(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSupplementalMathematicalOperators(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSupplementaryPrivateUseAreaA(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSupplementaryPrivateUseAreaB(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsSyriac(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTagalog(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTagbanwa(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTags(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTaiLe(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTaiXuanJingSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTamil(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTelugu(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsThaana(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsThai(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsTibetan(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsUgaritic(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsUnifiedCanadianAboriginalSyllabics(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsVariationSelectors(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsVariationSelectorsSupplement(
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsYiRadicals(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsYiSyllables(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsYijingHexagramSymbols(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsBlock(
        code: ::std::os::raw::c_int,
        block: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatC(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatCc(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatCf(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatCo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatCs(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatL(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatLl(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatLm(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatLo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatLt(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatLu(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatM(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatMc(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatMe(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatMn(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatN(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatNd(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatNl(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatNo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatP(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPc(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPd(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPe(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPf(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPi(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatPs(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatS(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatSc(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatSk(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatSm(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatSo(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatZ(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatZl(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatZp(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCatZs(code: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlUCSIsCat(
        code: ::std::os::raw::c_int,
        cat: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xmlTextWriter {
    _unused: [u8; 0],
}
pub type xmlTextWriter = _xmlTextWriter;
pub type xmlTextWriterPtr = *mut xmlTextWriter;
extern "C" {
    pub fn xmlNewTextWriter(out: xmlOutputBufferPtr) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlNewTextWriterFilename(
        uri: *const ::std::os::raw::c_char,
        compression: ::std::os::raw::c_int,
    ) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlNewTextWriterMemory(
        buf: xmlBufferPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlNewTextWriterPushParser(
        ctxt: xmlParserCtxtPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlNewTextWriterDoc(
        doc: *mut xmlDocPtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlNewTextWriterTree(
        doc: xmlDocPtr,
        node: xmlNodePtr,
        compression: ::std::os::raw::c_int,
    ) -> xmlTextWriterPtr;
}
extern "C" {
    pub fn xmlFreeTextWriter(writer: xmlTextWriterPtr);
}
extern "C" {
    pub fn xmlTextWriterStartDocument(
        writer: xmlTextWriterPtr,
        version: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
        standalone: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndDocument(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartComment(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndComment(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatComment(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatComment(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteComment(
        writer: xmlTextWriterPtr,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartElementNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndElement(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterFullEndElement(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatElementNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatElementNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteElementNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatRaw(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatRaw(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteRawLen(
        writer: xmlTextWriterPtr,
        content: *const xmlChar,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteRaw(
        writer: xmlTextWriterPtr,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatString(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatString(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteString(
        writer: xmlTextWriterPtr,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteBase64(
        writer: xmlTextWriterPtr,
        data: *const ::std::os::raw::c_char,
        start: ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteBinHex(
        writer: xmlTextWriterPtr,
        data: *const ::std::os::raw::c_char,
        start: ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartAttributeNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndAttribute(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatAttributeNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatAttributeNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteAttributeNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartPI(
        writer: xmlTextWriterPtr,
        target: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndPI(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatPI(
        writer: xmlTextWriterPtr,
        target: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatPI(
        writer: xmlTextWriterPtr,
        target: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWritePI(
        writer: xmlTextWriterPtr,
        target: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartCDATA(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndCDATA(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatCDATA(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatCDATA(
        writer: xmlTextWriterPtr,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteCDATA(
        writer: xmlTextWriterPtr,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartDTD(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndDTD(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatDTD(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatDTD(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTD(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        subset: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartDTDElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndDTDElement(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatDTDElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatDTDElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartDTDAttlist(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndDTDAttlist(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatDTDAttlist(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatDTDAttlist(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDAttlist(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterStartDTDEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterEndDTDEntity(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteFormatDTDInternalEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteVFormatDTDInternalEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
        format: *const ::std::os::raw::c_char,
        argptr: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDInternalEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDExternalEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        ndataid: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDExternalEntityContents(
        writer: xmlTextWriterPtr,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        ndataid: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDEntity(
        writer: xmlTextWriterPtr,
        pe: ::std::os::raw::c_int,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        ndataid: *const xmlChar,
        content: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterWriteDTDNotation(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterSetIndent(
        writer: xmlTextWriterPtr,
        indent: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterSetIndentString(
        writer: xmlTextWriterPtr,
        str_: *const xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterSetQuoteChar(
        writer: xmlTextWriterPtr,
        quotechar: xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlTextWriterFlush(writer: xmlTextWriterPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut xmlParserMaxDepth: ::std::os::raw::c_uint;
}
extern "C" {
    pub static xmlStringText: [xmlChar; 0usize];
}
extern "C" {
    pub static xmlStringTextNoenc: [xmlChar; 0usize];
}
extern "C" {
    pub static xmlStringComment: [xmlChar; 0usize];
}
extern "C" {
    pub fn xmlIsLetter(c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCreateFileParserCtxt(filename: *const ::std::os::raw::c_char) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlCreateURLParserCtxt(
        filename: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_int,
    ) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlCreateMemoryParserCtxt(
        buffer: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlCreateEntityParserCtxt(
        URL: *const xmlChar,
        ID: *const xmlChar,
        base: *const xmlChar,
    ) -> xmlParserCtxtPtr;
}
extern "C" {
    pub fn xmlSwitchEncoding(ctxt: xmlParserCtxtPtr, enc: xmlCharEncoding)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSwitchToEncoding(
        ctxt: xmlParserCtxtPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSwitchInputEncoding(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNewStringInputStream(
        ctxt: xmlParserCtxtPtr,
        buffer: *const xmlChar,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlNewEntityInputStream(
        ctxt: xmlParserCtxtPtr,
        entity: xmlEntityPtr,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlPushInput(ctxt: xmlParserCtxtPtr, input: xmlParserInputPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlPopInput(ctxt: xmlParserCtxtPtr) -> xmlChar;
}
extern "C" {
    pub fn xmlFreeInputStream(input: xmlParserInputPtr);
}
extern "C" {
    pub fn xmlNewInputFromFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const ::std::os::raw::c_char,
    ) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
}
extern "C" {
    pub fn xmlSplitQName(
        ctxt: xmlParserCtxtPtr,
        name: *const xmlChar,
        prefix: *mut *mut xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseName(ctxt: xmlParserCtxtPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlParseNmtoken(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseEntityValue(ctxt: xmlParserCtxtPtr, orig: *mut *mut xmlChar) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseAttValue(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseSystemLiteral(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParsePubidLiteral(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseCharData(ctxt: xmlParserCtxtPtr, cdata: ::std::os::raw::c_int);
}
extern "C" {
    pub fn xmlParseExternalID(
        ctxt: xmlParserCtxtPtr,
        publicID: *mut *mut xmlChar,
        strict: ::std::os::raw::c_int,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseComment(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParsePITarget(ctxt: xmlParserCtxtPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlParsePI(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseNotationDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseEntityDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseDefaultDecl(
        ctxt: xmlParserCtxtPtr,
        value: *mut *mut xmlChar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseNotationType(ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr;
}
extern "C" {
    pub fn xmlParseEnumerationType(ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr;
}
extern "C" {
    pub fn xmlParseEnumeratedType(
        ctxt: xmlParserCtxtPtr,
        tree: *mut xmlEnumerationPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseAttributeType(
        ctxt: xmlParserCtxtPtr,
        tree: *mut xmlEnumerationPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseAttributeListDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseElementMixedContentDecl(
        ctxt: xmlParserCtxtPtr,
        inputchk: ::std::os::raw::c_int,
    ) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlParseElementChildrenContentDecl(
        ctxt: xmlParserCtxtPtr,
        inputchk: ::std::os::raw::c_int,
    ) -> xmlElementContentPtr;
}
extern "C" {
    pub fn xmlParseElementContentDecl(
        ctxt: xmlParserCtxtPtr,
        name: *const xmlChar,
        result: *mut xmlElementContentPtr,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseElementDecl(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseMarkupDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseCharRef(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseEntityRef(ctxt: xmlParserCtxtPtr) -> xmlEntityPtr;
}
extern "C" {
    pub fn xmlParseReference(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParsePEReference(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseDocTypeDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseAttribute(ctxt: xmlParserCtxtPtr, value: *mut *mut xmlChar) -> *const xmlChar;
}
extern "C" {
    pub fn xmlParseStartTag(ctxt: xmlParserCtxtPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlParseEndTag(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseCDSect(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseContent(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseElement(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseVersionNum(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseVersionInfo(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseEncName(ctxt: xmlParserCtxtPtr) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlParseEncodingDecl(ctxt: xmlParserCtxtPtr) -> *const xmlChar;
}
extern "C" {
    pub fn xmlParseSDDecl(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParseXMLDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseTextDecl(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseMisc(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParseExternalSubset(
        ctxt: xmlParserCtxtPtr,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
}
extern "C" {
    pub fn xmlStringDecodeEntities(
        ctxt: xmlParserCtxtPtr,
        str_: *const xmlChar,
        what: ::std::os::raw::c_int,
        end: xmlChar,
        end2: xmlChar,
        end3: xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn xmlStringLenDecodeEntities(
        ctxt: xmlParserCtxtPtr,
        str_: *const xmlChar,
        len: ::std::os::raw::c_int,
        what: ::std::os::raw::c_int,
        end: xmlChar,
        end2: xmlChar,
        end3: xmlChar,
    ) -> *mut xmlChar;
}
extern "C" {
    pub fn nodePush(ctxt: xmlParserCtxtPtr, value: xmlNodePtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nodePop(ctxt: xmlParserCtxtPtr) -> xmlNodePtr;
}
extern "C" {
    pub fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn inputPop(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
}
extern "C" {
    pub fn namePop(ctxt: xmlParserCtxtPtr) -> *const xmlChar;
}
extern "C" {
    pub fn namePush(ctxt: xmlParserCtxtPtr, value: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlSkipBlankChars(ctxt: xmlParserCtxtPtr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlParserHandlePEReference(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlCheckLanguageID(lang: *const xmlChar) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCurrentChar(
        ctxt: xmlParserCtxtPtr,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCopyCharMultiByte(
        out: *mut xmlChar,
        val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlCopyChar(
        len: ::std::os::raw::c_int,
        out: *mut xmlChar,
        val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xmlNextChar(ctxt: xmlParserCtxtPtr);
}
extern "C" {
    pub fn xmlParserInputShrink(in_: xmlParserInputPtr);
}
extern "C" {
    pub fn htmlInitAutoClose();
}
extern "C" {
    pub fn htmlCreateFileParserCtxt(
        filename: *const ::std::os::raw::c_char,
        encoding: *const ::std::os::raw::c_char,
    ) -> htmlParserCtxtPtr;
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
