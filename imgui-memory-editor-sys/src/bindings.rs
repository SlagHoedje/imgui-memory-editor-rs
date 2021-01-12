/* automatically generated by rust-bindgen 0.56.0 */

pub type ImGuiDataType = ::std::os::raw::c_int;
pub type ImU8 = ::std::os::raw::c_uchar;
pub type ImU32 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MemoryEditor {
    pub Open: bool,
    pub ReadOnly: bool,
    pub Cols: ::std::os::raw::c_int,
    pub OptShowOptions: bool,
    pub OptShowDataPreview: bool,
    pub OptShowHexII: bool,
    pub OptShowAscii: bool,
    pub OptGreyOutZeroes: bool,
    pub OptUpperCaseHex: bool,
    pub OptMidColsCount: ::std::os::raw::c_int,
    pub OptAddrDigitsCount: ::std::os::raw::c_int,
    pub HighlightColor: ImU32,
    pub ReadFn: ::core::option::Option<unsafe extern "C" fn(data: *const ImU8, off: usize) -> ImU8>,
    pub WriteFn: ::core::option::Option<unsafe extern "C" fn(data: *mut ImU8, off: usize, d: ImU8)>,
    pub HighlightFn:
        ::core::option::Option<unsafe extern "C" fn(data: *const ImU8, off: usize) -> bool>,
    pub ContentsWidthChanged: bool,
    pub DataPreviewAddr: usize,
    pub DataEditingAddr: usize,
    pub DataEditingTakeFocus: bool,
    pub DataInputBuf: [::std::os::raw::c_char; 32usize],
    pub AddrInputBuf: [::std::os::raw::c_char; 32usize],
    pub GotoAddr: usize,
    pub HighlightMin: usize,
    pub HighlightMax: usize,
    pub PreviewEndianess: ::std::os::raw::c_int,
    pub PreviewDataType: ImGuiDataType,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MemoryEditor_DataFormat {
    DataFormat_Bin = 0,
    DataFormat_Dec = 1,
    DataFormat_Hex = 2,
    DataFormat_COUNT = 3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct MemoryEditor_Sizes {
    pub AddrDigitsCount: ::std::os::raw::c_int,
    pub LineHeight: f32,
    pub GlyphWidth: f32,
    pub HexCellWidth: f32,
    pub SpacingBetweenMidCols: f32,
    pub PosHexStart: f32,
    pub PosHexEnd: f32,
    pub PosAsciiStart: f32,
    pub PosAsciiEnd: f32,
    pub WindowWidth: f32,
}
extern "C" {
    pub fn Editor_Create(editor: *mut MemoryEditor);
}
extern "C" {
    pub fn Editor_DrawContents(
        editor: *mut MemoryEditor,
        mem_data_void: *mut ::core::ffi::c_void,
        mem_size: usize,
        base_display_addr: usize,
    );
}
extern "C" {
    pub fn Editor_DrawWindow(
        editor: *mut MemoryEditor,
        title: *const ::std::os::raw::c_char,
        mem_data: *mut ::core::ffi::c_void,
        mem_size: usize,
        base_display_addr: usize,
    );
}