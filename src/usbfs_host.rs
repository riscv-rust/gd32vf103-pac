#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - host configuration register (HCTL)"]
    pub hctl: HCTL,
    #[doc = "0x04 - Host frame interval register"]
    pub hft: HFT,
    #[doc = "0x08 - FS host frame number/frame time remaining register (HFINFR)"]
    pub hfinfr: HFINFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
    pub hptfqstat: HPTFQSTAT,
    #[doc = "0x14 - Host all channels interrupt register"]
    pub hachint: HACHINT,
    #[doc = "0x18 - host all channels interrupt mask register"]
    pub hachinten: HACHINTEN,
    _reserved1: [u8; 36usize],
    #[doc = "0x40 - Host port control and status register (USBFS_HPCS)"]
    pub hpcs: HPCS,
    _reserved2: [u8; 188usize],
    #[doc = "0x100 - host channel-0 characteristics register (HCH0CTL)"]
    pub hch0ctl: HCH0CTL,
    _reserved3: [u8; 4usize],
    #[doc = "0x108 - host channel-0 interrupt register (USBFS_HCHxINTF)"]
    pub hch0intf: HCH0INTF,
    #[doc = "0x10c - host channel-0 interrupt enable register (HCH0INTEN)"]
    pub hch0inten: HCH0INTEN,
    #[doc = "0x110 - host channel-0 transfer length register"]
    pub hch0len: HCH0LEN,
    _reserved4: [u8; 12usize],
    #[doc = "0x120 - host channel-1 characteristics register (HCH1CTL)"]
    pub hch1ctl: HCH1CTL,
    _reserved5: [u8; 4usize],
    #[doc = "0x128 - host channel-1 interrupt register (HCH1INTF)"]
    pub hch1intf: HCH1INTF,
    #[doc = "0x12c - host channel-1 interrupt enable register (HCH1INTEN)"]
    pub hch1inten: HCH1INTEN,
    #[doc = "0x130 - host channel-1 transfer length register"]
    pub hch1len: HCH1LEN,
    _reserved6: [u8; 12usize],
    #[doc = "0x140 - host channel-2 characteristics register (HCH2CTL)"]
    pub hch2ctl: HCH2CTL,
    _reserved7: [u8; 4usize],
    #[doc = "0x148 - host channel-2 interrupt register (HCH2INTF)"]
    pub hch2intf: HCH2INTF,
    #[doc = "0x14c - host channel-2 interrupt enable register (HCH2INTEN)"]
    pub hch2inten: HCH2INTEN,
    #[doc = "0x150 - host channel-2 transfer length register"]
    pub hch2len: HCH2LEN,
    _reserved8: [u8; 12usize],
    #[doc = "0x160 - host channel-3 characteristics register (HCH3CTL)"]
    pub hch3ctl: HCH3CTL,
    _reserved9: [u8; 4usize],
    #[doc = "0x168 - host channel-3 interrupt register (HCH3INTF)"]
    pub hch3intf: HCH3INTF,
    #[doc = "0x16c - host channel-3 interrupt enable register (HCH3INTEN)"]
    pub hch3inten: HCH3INTEN,
    #[doc = "0x170 - host channel-3 transfer length register"]
    pub hch3len: HCH3LEN,
    _reserved10: [u8; 12usize],
    #[doc = "0x180 - host channel-4 characteristics register (HCH4CTL)"]
    pub hch4ctl: HCH4CTL,
    _reserved11: [u8; 4usize],
    #[doc = "0x188 - host channel-4 interrupt register (HCH4INTF)"]
    pub hch4intf: HCH4INTF,
    #[doc = "0x18c - host channel-4 interrupt enable register (HCH4INTEN)"]
    pub hch4inten: HCH4INTEN,
    #[doc = "0x190 - host channel-4 transfer length register"]
    pub hch4len: HCH4LEN,
    _reserved12: [u8; 12usize],
    #[doc = "0x1a0 - host channel-5 characteristics register (HCH5CTL)"]
    pub hch5ctl: HCH5CTL,
    _reserved13: [u8; 4usize],
    #[doc = "0x1a8 - host channel-5 interrupt register (HCH5INTF)"]
    pub hch5intf: HCH5INTF,
    #[doc = "0x1ac - host channel-5 interrupt enable register (HCH5INTEN)"]
    pub hch5inten: HCH5INTEN,
    #[doc = "0x1b0 - host channel-5 transfer length register"]
    pub hch5len: HCH5LEN,
    _reserved14: [u8; 12usize],
    #[doc = "0x1c0 - host channel-6 characteristics register (HCH6CTL)"]
    pub hch6ctl: HCH6CTL,
    _reserved15: [u8; 4usize],
    #[doc = "0x1c8 - host channel-6 interrupt register (HCH6INTF)"]
    pub hch6intf: HCH6INTF,
    #[doc = "0x1cc - host channel-6 interrupt enable register (HCH6INTEN)"]
    pub hch6inten: HCH6INTEN,
    #[doc = "0x1d0 - host channel-6 transfer length register"]
    pub hch6len: HCH6LEN,
    _reserved16: [u8; 12usize],
    #[doc = "0x1e0 - host channel-7 characteristics register (HCH7CTL)"]
    pub hch7ctl: HCH7CTL,
    _reserved17: [u8; 4usize],
    #[doc = "0x1e8 - host channel-7 interrupt register (HCH7INTF)"]
    pub hch7intf: HCH7INTF,
    #[doc = "0x1ec - host channel-7 interrupt enable register (HCH7INTEN)"]
    pub hch7inten: HCH7INTEN,
    #[doc = "0x1f0 - host channel-7 transfer length register"]
    pub hch7len: HCH7LEN,
}
#[doc = "host configuration register (HCTL)"]
pub struct HCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host configuration register (HCTL)"]
pub mod hctl;
#[doc = "Host frame interval register"]
pub struct HFT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host frame interval register"]
pub mod hft;
#[doc = "FS host frame number/frame time remaining register (HFINFR)"]
pub struct HFINFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FS host frame number/frame time remaining register (HFINFR)"]
pub mod hfinfr;
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub struct HPTFQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub mod hptfqstat;
#[doc = "Host all channels interrupt register"]
pub struct HACHINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host all channels interrupt register"]
pub mod hachint;
#[doc = "host all channels interrupt mask register"]
pub struct HACHINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host all channels interrupt mask register"]
pub mod hachinten;
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub struct HPCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub mod hpcs;
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub struct HCH0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub mod hch0ctl;
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub struct HCH1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub mod hch1ctl;
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub struct HCH2CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub mod hch2ctl;
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub struct HCH3CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub mod hch3ctl;
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub struct HCH4CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub mod hch4ctl;
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub struct HCH5CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub mod hch5ctl;
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub struct HCH6CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub mod hch6ctl;
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub struct HCH7CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub mod hch7ctl;
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)"]
pub struct HCH0INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)"]
pub mod hch0intf;
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub struct HCH1INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub mod hch1intf;
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub struct HCH2INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub mod hch2intf;
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub struct HCH3INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub mod hch3intf;
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub struct HCH4INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub mod hch4intf;
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub struct HCH5INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub mod hch5intf;
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub struct HCH6INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub mod hch6intf;
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub struct HCH7INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub mod hch7intf;
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub struct HCH0INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub mod hch0inten;
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub struct HCH1INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub mod hch1inten;
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub struct HCH2INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub mod hch2inten;
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub struct HCH3INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub mod hch3inten;
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub struct HCH4INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub mod hch4inten;
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub struct HCH5INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub mod hch5inten;
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub struct HCH6INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub mod hch6inten;
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub struct HCH7INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub mod hch7inten;
#[doc = "host channel-0 transfer length register"]
pub struct HCH0LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-0 transfer length register"]
pub mod hch0len;
#[doc = "host channel-1 transfer length register"]
pub struct HCH1LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-1 transfer length register"]
pub mod hch1len;
#[doc = "host channel-2 transfer length register"]
pub struct HCH2LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-2 transfer length register"]
pub mod hch2len;
#[doc = "host channel-3 transfer length register"]
pub struct HCH3LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-3 transfer length register"]
pub mod hch3len;
#[doc = "host channel-4 transfer length register"]
pub struct HCH4LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-4 transfer length register"]
pub mod hch4len;
#[doc = "host channel-5 transfer length register"]
pub struct HCH5LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-5 transfer length register"]
pub mod hch5len;
#[doc = "host channel-6 transfer length register"]
pub struct HCH6LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-6 transfer length register"]
pub mod hch6len;
#[doc = "host channel-7 transfer length register"]
pub struct HCH7LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "host channel-7 transfer length register"]
pub mod hch7len;
