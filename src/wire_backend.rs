use crate::wire_shim::WireInstanceHandle;

#[derive(Debug, Clone, Copy)]
pub struct WireHandle {
    id: u32,
    generation: u32,
}

impl WireHandle {
    pub fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    pub fn generation(self) -> u32 {
        self.generation
    }
}

impl From<WireInstanceHandle> for WireHandle {
    fn from(value: WireInstanceHandle) -> Self {
        Self {
            id: value.id,
            generation: value.generation,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WireTextureReservation {
    texture_handle: WireHandle,
    device_handle: WireHandle,
    width: u32,
    height: u32,
    format: u32,
    usage: u64,
}

impl WireTextureReservation {
    pub fn new(
        texture_handle: WireHandle,
        device_handle: WireHandle,
        width: u32,
        height: u32,
        format: u32,
        usage: u64,
    ) -> Self {
        Self {
            texture_handle,
            device_handle,
            width,
            height,
            format,
            usage,
        }
    }

    pub fn texture_handle(self) -> WireHandle {
        self.texture_handle
    }

    pub fn device_handle(self) -> WireHandle {
        self.device_handle
    }

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }

    pub fn format_raw(self) -> u32 {
        self.format
    }

    pub fn usage_bits(self) -> u64 {
        self.usage
    }
}
