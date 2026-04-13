#![no_std]

pub const FRAME_MAGIC: u8 = 0xAB;
pub const FRAME_SIZE: usize = 53;
pub const MAX_EVENTS_PER_FRAME: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct WireTriggerEvent {
    pub lane_and_flags: u8, // bits [2:0] = lane, bit [3] = retrigger
    pub velocity_hi: u8,
    pub velocity_lo: u8,
}

impl WireTriggerEvent {
    pub const fn new(lane: u8, velocity: u16, retrigger: bool) -> Self {
        Self {
            lane_and_flags: (lane & 0x07) | ((retrigger as u8) << 3),
            velocity_hi: (velocity >> 8) as u8,
            velocity_lo: velocity as u8,
        }
    }

    pub const fn lane(self) -> u8 {
        self.lane_and_flags & 0x07
    }

    pub const fn velocity(self) -> u16 {
        ((self.velocity_hi as u16) << 8) | self.velocity_lo as u16
    }

    pub const fn is_retrigger(self) -> bool {
        (self.lane_and_flags >> 3) & 1 != 0
    }
}

/// Fixed-size SPI wire frame sent from the G474 (master) to the F429ZI (slave).
///
/// Layout:
/// - byte 0:    magic (0xAB)
/// - byte 1:    count of valid events (0–16)
/// - byte 2:    monotonic sequence number mod 256
/// - byte 3:    flags (reserved, 0x00)
/// - bytes 4–51: up to 16 × 3-byte WireTriggerEvent
/// - byte 52:   CRC-8 (optional; currently written as 0x00)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TriggerFrame {
    pub magic: u8,
    pub count: u8,
    pub seq: u8,
    pub flags: u8,
    pub events: [WireTriggerEvent; MAX_EVENTS_PER_FRAME],
    pub crc8: u8,
}

impl TriggerFrame {
    pub const WIRE_SIZE: usize = FRAME_SIZE;

    /// Reinterpret this frame as a fixed-size byte slice for SPI transmission.
    ///
    /// # Safety
    /// `TriggerFrame` is `#[repr(C)]`, all fields are plain integers, and
    /// `core::mem::size_of::<TriggerFrame>() == FRAME_SIZE` is asserted below.
    pub fn as_bytes(&self) -> &[u8; FRAME_SIZE] {
        // SAFETY: see above.
        unsafe { &*(self as *const Self as *const [u8; FRAME_SIZE]) }
    }

    /// Reinterpret a received byte buffer as a `TriggerFrame`.
    ///
    /// # Safety
    /// Same repr guarantees as `as_bytes`. Caller must call `is_valid()` before use.
    pub fn from_bytes(bytes: &[u8; FRAME_SIZE]) -> &Self {
        // SAFETY: see above.
        unsafe { &*(bytes as *const [u8; FRAME_SIZE] as *const Self) }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == FRAME_MAGIC && self.count as usize <= MAX_EVENTS_PER_FRAME
    }
}

const _: () = assert!(core::mem::size_of::<TriggerFrame>() == FRAME_SIZE);
