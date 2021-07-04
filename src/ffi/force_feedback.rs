#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub type BOOL = bool;
pub type BYTE = std::os::raw::c_uchar;
pub type UCHAR = std::os::raw::c_uchar;
pub type WORD = std::os::raw::c_ushort;
pub type DWORD = std::os::raw::c_ulong;
pub type UINT = std::os::raw::c_uint;
pub type ULONG = std::os::raw::c_ulong;
pub type LONG = std::os::raw::c_long;
pub type PVOID = *mut std::os::raw::c_void;

pub const HID_USAGE_CONST: u32 = 38;
pub const HID_USAGE_RAMP: u32 = 39;
pub const HID_USAGE_SQUR: u32 = 48;
pub const HID_USAGE_SINE: u32 = 49;
pub const HID_USAGE_TRNG: u32 = 50;
pub const HID_USAGE_STUP: u32 = 51;
pub const HID_USAGE_STDN: u32 = 52;
pub const HID_USAGE_SPRNG: u32 = 64;
pub const HID_USAGE_DMPR: u32 = 65;
pub const HID_USAGE_INRT: u32 = 66;
pub const HID_USAGE_FRIC: u32 = 67;

pub const HID_ID_STATE: u32 = 2;
pub const HID_ID_EFFREP: u32 = 1;
pub const HID_ID_ENVREP: u32 = 2;
pub const HID_ID_CONDREP: u32 = 3;
pub const HID_ID_PRIDREP: u32 = 4;
pub const HID_ID_CONSTREP: u32 = 5;
pub const HID_ID_RAMPREP: u32 = 6;
pub const HID_ID_CSTMREP: u32 = 7;
pub const HID_ID_SMPLREP: u32 = 8;
pub const HID_ID_EFOPREP: u32 = 10;
pub const HID_ID_BLKFRREP: u32 = 11;
pub const HID_ID_CTRLREP: u32 = 12;
pub const HID_ID_GAINREP: u32 = 13;
pub const HID_ID_SETCREP: u32 = 14;
pub const HID_ID_NEWEFREP: u32 = 1;
pub const HID_ID_BLKLDREP: u32 = 2;
pub const HID_ID_POOLREP: u32 = 3;

pub const F_LOAD_POSITIONS: u32 = 2320;
pub const F_GETATTRIB: u32 = 2321;
pub const F_GET_FFB_DATA: u32 = 2322;
pub const F_SET_FFB_STAT: u32 = 2323;
pub const F_GET_FFB_STAT: u32 = 2326;
pub const F_GET_DEV_INFO: u32 = 2327;
pub const F_IS_DRV_FFB_CAP: u32 = 2328;
pub const F_IS_DRV_FFB_EN: u32 = 2329;
pub const F_GET_DRV_DEV_MAX: u32 = 2330;
pub const F_GET_DRV_DEV_EN: u32 = 2331;
pub const F_IS_DEV_FFB_START: u32 = 2332;
pub const F_GET_DEV_STAT: u32 = 2333;
pub const F_GET_DRV_INFO: u32 = 2334;
pub const F_RESET_DEV: u32 = 2335;
pub const F_GET_POSITIONS: u32 = 2336;

pub type RemovalCB = std::option::Option<unsafe extern "C" fn(arg1: BOOL, arg2: BOOL, arg3: PVOID)>;
pub const FFBEType_ET_NONE: FFBEType = 0;
pub const FFBEType_ET_CONST: FFBEType = 1;
pub const FFBEType_ET_RAMP: FFBEType = 2;
pub const FFBEType_ET_SQR: FFBEType = 3;
pub const FFBEType_ET_SINE: FFBEType = 4;
pub const FFBEType_ET_TRNGL: FFBEType = 5;
pub const FFBEType_ET_STUP: FFBEType = 6;
pub const FFBEType_ET_STDN: FFBEType = 7;
pub const FFBEType_ET_SPRNG: FFBEType = 8;
pub const FFBEType_ET_DMPR: FFBEType = 9;
pub const FFBEType_ET_INRT: FFBEType = 10;
pub const FFBEType_ET_FRCTN: FFBEType = 11;
pub const FFBEType_ET_CSTM: FFBEType = 12;
pub type FFBEType = std::os::raw::c_uint;
pub const FFBPType_PT_EFFREP: FFBPType = 1;
pub const FFBPType_PT_ENVREP: FFBPType = 2;
pub const FFBPType_PT_CONDREP: FFBPType = 3;
pub const FFBPType_PT_PRIDREP: FFBPType = 4;
pub const FFBPType_PT_CONSTREP: FFBPType = 5;
pub const FFBPType_PT_RAMPREP: FFBPType = 6;
pub const FFBPType_PT_CSTMREP: FFBPType = 7;
pub const FFBPType_PT_SMPLREP: FFBPType = 8;
pub const FFBPType_PT_EFOPREP: FFBPType = 10;
pub const FFBPType_PT_BLKFRREP: FFBPType = 11;
pub const FFBPType_PT_CTRLREP: FFBPType = 12;
pub const FFBPType_PT_GAINREP: FFBPType = 13;
pub const FFBPType_PT_SETCREP: FFBPType = 14;
pub const FFBPType_PT_NEWEFREP: FFBPType = 17;
pub const FFBPType_PT_BLKLDREP: FFBPType = 18;
pub const FFBPType_PT_POOLREP: FFBPType = 19;
pub type FFBPType = std::os::raw::c_uint;
pub const FFBOP_EFF_START: FFBOP = 1;
pub const FFBOP_EFF_SOLO: FFBOP = 2;
pub const FFBOP_EFF_STOP: FFBOP = 3;
pub type FFBOP = std::os::raw::c_uint;
pub const FFB_CTRL_CTRL_ENACT: FFB_CTRL = 1;
pub const FFB_CTRL_CTRL_DISACT: FFB_CTRL = 2;
pub const FFB_CTRL_CTRL_STOPALL: FFB_CTRL = 3;
pub const FFB_CTRL_CTRL_DEVRST: FFB_CTRL = 4;
pub const FFB_CTRL_CTRL_DEVPAUSE: FFB_CTRL = 5;
pub const FFB_CTRL_CTRL_DEVCONT: FFB_CTRL = 6;
pub type FFB_CTRL = std::os::raw::c_uint;
pub const FFB_EFFECTS_Constant: FFB_EFFECTS = 1;
pub const FFB_EFFECTS_Ramp: FFB_EFFECTS = 2;
pub const FFB_EFFECTS_Square: FFB_EFFECTS = 4;
pub const FFB_EFFECTS_Sine: FFB_EFFECTS = 8;
pub const FFB_EFFECTS_Triangle: FFB_EFFECTS = 16;
pub const FFB_EFFECTS_Sawtooth_Up: FFB_EFFECTS = 32;
pub const FFB_EFFECTS_Sawtooth_Dn: FFB_EFFECTS = 64;
pub const FFB_EFFECTS_Spring: FFB_EFFECTS = 128;
pub const FFB_EFFECTS_Damper: FFB_EFFECTS = 256;
pub const FFB_EFFECTS_Inertia: FFB_EFFECTS = 512;
pub const FFB_EFFECTS_Friction: FFB_EFFECTS = 1024;
pub const FFB_EFFECTS_Custom: FFB_EFFECTS = 2048;
pub type FFB_EFFECTS = std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_DATA {
    pub size: ULONG,
    pub cmd: ULONG,
    pub data: *mut UCHAR,
}
pub type FFB_DATA = _FFB_DATA;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_CONSTANT {
    pub EffectBlockIndex: BYTE,
    pub Magnitude: LONG,
}
pub type FFB_EFF_CONSTANT = _FFB_EFF_CONSTANT;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_RAMP {
    pub EffectBlockIndex: BYTE,
    pub Start: LONG,
    pub End: LONG,
}
pub type FFB_EFF_RAMP = _FFB_EFF_RAMP;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _FFB_EFF_REPORT {
    pub EffectBlockIndex: BYTE,
    pub EffectType: FFBEType,
    pub Duration: WORD,
    pub TrigerRpt: WORD,
    pub SamplePrd: WORD,
    pub Gain: BYTE,
    pub TrigerBtn: BYTE,
    pub Polar: BOOL,
    pub __bindgen_anon_1: _FFB_EFF_REPORT__bindgen_ty_1,
    pub DirY: BYTE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _FFB_EFF_REPORT__bindgen_ty_1 {
    pub Direction: BYTE,
    pub DirX: BYTE,
}
pub type FFB_EFF_REPORT = _FFB_EFF_REPORT;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_OP {
    pub EffectBlockIndex: BYTE,
    pub EffectOp: FFBOP,
    pub LoopCount: BYTE,
}
pub type FFB_EFF_OP = _FFB_EFF_OP;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_PERIOD {
    pub EffectBlockIndex: BYTE,
    pub Magnitude: DWORD,
    pub Offset: LONG,
    pub Phase: DWORD,
    pub Period: DWORD,
}
pub type FFB_EFF_PERIOD = _FFB_EFF_PERIOD;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_COND {
    pub EffectBlockIndex: BYTE,
    pub isY: BOOL,
    pub CenterPointOffset: LONG,
    pub PosCoeff: LONG,
    pub NegCoeff: LONG,
    pub PosSatur: DWORD,
    pub NegSatur: DWORD,
    pub DeadBand: LONG,
}
pub type FFB_EFF_COND = _FFB_EFF_COND;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _FFB_EFF_ENVLP {
    pub EffectBlockIndex: BYTE,
    pub AttackLevel: DWORD,
    pub FadeLevel: DWORD,
    pub AttackTime: DWORD,
    pub FadeTime: DWORD,
}
pub type FFB_EFF_ENVLP = _FFB_EFF_ENVLP;

pub type FfbGenCB = std::option::Option<unsafe extern "C" fn(arg1: PVOID, arg2: PVOID)>;

extern "C" {
    pub fn RegisterRemovalCB(cb: RemovalCB, data: PVOID);
    pub fn vJoyFfbCap(Supported: *mut BOOL) -> BOOL;
    pub fn FfbGetEffect() -> FFBEType;
    pub fn FfbRegisterGenCB(cb: FfbGenCB, data: PVOID);
    pub fn FfbStart(rID: UINT) -> BOOL;
    pub fn FfbStop(rID: UINT);
    pub fn IsDeviceFfb(rID: UINT) -> BOOL;
    pub fn IsDeviceFfbEffect(rID: UINT, Effect: UINT) -> BOOL;
    pub fn Ffb_h_DeviceID(Packet: *const FFB_DATA, DeviceID: *mut std::os::raw::c_int) -> DWORD;
    pub fn Ffb_h_Type(Packet: *const FFB_DATA, Type: *mut FFBPType) -> DWORD;
    pub fn Ffb_h_Packet(
        Packet: *const FFB_DATA,
        Type: *mut WORD,
        DataSize: *mut std::os::raw::c_int,
        Data: *mut *mut BYTE,
    ) -> DWORD;
    pub fn Ffb_h_EBI(Packet: *const FFB_DATA, Index: *mut std::os::raw::c_int) -> DWORD;
    pub fn Ffb_h_Eff_Report(Packet: *const FFB_DATA, Effect: *mut FFB_EFF_REPORT) -> DWORD;
    pub fn Ffb_h_Eff_Ramp(Packet: *const FFB_DATA, RampEffect: *mut FFB_EFF_RAMP) -> DWORD;
    pub fn Ffb_h_EffOp(Packet: *const FFB_DATA, Operation: *mut FFB_EFF_OP) -> DWORD;
    pub fn Ffb_h_DevCtrl(Packet: *const FFB_DATA, Control: *mut FFB_CTRL) -> DWORD;
    pub fn Ffb_h_Eff_Period(Packet: *const FFB_DATA, Effect: *mut FFB_EFF_PERIOD) -> DWORD;
    pub fn Ffb_h_Eff_Cond(Packet: *const FFB_DATA, Condition: *mut FFB_EFF_COND) -> DWORD;
    pub fn Ffb_h_DevGain(Packet: *const FFB_DATA, Gain: *mut BYTE) -> DWORD;
    pub fn Ffb_h_Eff_Envlp(Packet: *const FFB_DATA, Envelope: *mut FFB_EFF_ENVLP) -> DWORD;
    pub fn Ffb_h_EffNew(Packet: *const FFB_DATA, Effect: *mut FFBEType) -> DWORD;
    pub fn Ffb_h_Eff_Constant(
        Packet: *const FFB_DATA,
        ConstantEffect: *mut FFB_EFF_CONSTANT,
    ) -> DWORD;
}
