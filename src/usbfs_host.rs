#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - host configuration register (HCTL)"]
    pub hctl: HCTL,
    #[doc = "0x04 - Host frame interval register"]
    pub hft: HFT,
    #[doc = "0x08 - FS host frame number/frame time remaining register (HFINFR)"]
    pub hfinfr: HFINFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
    pub hptfqstat: HPTFQSTAT,
    #[doc = "0x14 - Host all channels interrupt register"]
    pub hachint: HACHINT,
    #[doc = "0x18 - host all channels interrupt mask register"]
    pub hachinten: HACHINTEN,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - Host port control and status register (USBFS_HPCS)"]
    pub hpcs: HPCS,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - host channel-0 characteristics register (HCH0CTL)"]
    pub hch0ctl: HCH0CTL,
    _reserved8: [u8; 0x04],
    #[doc = "0x108 - host channel-0 interrupt register (USBFS_HCHxINTF)"]
    pub hch0intf: HCH0INTF,
    #[doc = "0x10c - host channel-0 interrupt enable register (HCH0INTEN)"]
    pub hch0inten: HCH0INTEN,
    #[doc = "0x110 - host channel-0 transfer length register"]
    pub hch0len: HCH0LEN,
    _reserved11: [u8; 0x0c],
    #[doc = "0x120 - host channel-1 characteristics register (HCH1CTL)"]
    pub hch1ctl: HCH1CTL,
    _reserved12: [u8; 0x04],
    #[doc = "0x128 - host channel-1 interrupt register (HCH1INTF)"]
    pub hch1intf: HCH1INTF,
    #[doc = "0x12c - host channel-1 interrupt enable register (HCH1INTEN)"]
    pub hch1inten: HCH1INTEN,
    #[doc = "0x130 - host channel-1 transfer length register"]
    pub hch1len: HCH1LEN,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - host channel-2 characteristics register (HCH2CTL)"]
    pub hch2ctl: HCH2CTL,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - host channel-2 interrupt register (HCH2INTF)"]
    pub hch2intf: HCH2INTF,
    #[doc = "0x14c - host channel-2 interrupt enable register (HCH2INTEN)"]
    pub hch2inten: HCH2INTEN,
    #[doc = "0x150 - host channel-2 transfer length register"]
    pub hch2len: HCH2LEN,
    _reserved19: [u8; 0x0c],
    #[doc = "0x160 - host channel-3 characteristics register (HCH3CTL)"]
    pub hch3ctl: HCH3CTL,
    _reserved20: [u8; 0x04],
    #[doc = "0x168 - host channel-3 interrupt register (HCH3INTF)"]
    pub hch3intf: HCH3INTF,
    #[doc = "0x16c - host channel-3 interrupt enable register (HCH3INTEN)"]
    pub hch3inten: HCH3INTEN,
    #[doc = "0x170 - host channel-3 transfer length register"]
    pub hch3len: HCH3LEN,
    _reserved23: [u8; 0x0c],
    #[doc = "0x180 - host channel-4 characteristics register (HCH4CTL)"]
    pub hch4ctl: HCH4CTL,
    _reserved24: [u8; 0x04],
    #[doc = "0x188 - host channel-4 interrupt register (HCH4INTF)"]
    pub hch4intf: HCH4INTF,
    #[doc = "0x18c - host channel-4 interrupt enable register (HCH4INTEN)"]
    pub hch4inten: HCH4INTEN,
    #[doc = "0x190 - host channel-4 transfer length register"]
    pub hch4len: HCH4LEN,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1a0 - host channel-5 characteristics register (HCH5CTL)"]
    pub hch5ctl: HCH5CTL,
    _reserved28: [u8; 0x04],
    #[doc = "0x1a8 - host channel-5 interrupt register (HCH5INTF)"]
    pub hch5intf: HCH5INTF,
    #[doc = "0x1ac - host channel-5 interrupt enable register (HCH5INTEN)"]
    pub hch5inten: HCH5INTEN,
    #[doc = "0x1b0 - host channel-5 transfer length register"]
    pub hch5len: HCH5LEN,
    _reserved31: [u8; 0x0c],
    #[doc = "0x1c0 - host channel-6 characteristics register (HCH6CTL)"]
    pub hch6ctl: HCH6CTL,
    _reserved32: [u8; 0x04],
    #[doc = "0x1c8 - host channel-6 interrupt register (HCH6INTF)"]
    pub hch6intf: HCH6INTF,
    #[doc = "0x1cc - host channel-6 interrupt enable register (HCH6INTEN)"]
    pub hch6inten: HCH6INTEN,
    #[doc = "0x1d0 - host channel-6 transfer length register"]
    pub hch6len: HCH6LEN,
    _reserved35: [u8; 0x0c],
    #[doc = "0x1e0 - host channel-7 characteristics register (HCH7CTL)"]
    pub hch7ctl: HCH7CTL,
    _reserved36: [u8; 0x04],
    #[doc = "0x1e8 - host channel-7 interrupt register (HCH7INTF)"]
    pub hch7intf: HCH7INTF,
    #[doc = "0x1ec - host channel-7 interrupt enable register (HCH7INTEN)"]
    pub hch7inten: HCH7INTEN,
    #[doc = "0x1f0 - host channel-7 transfer length register"]
    pub hch7len: HCH7LEN,
}
#[doc = "HCTL (rw) register accessor: host configuration register (HCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctl`]
module"]
pub type HCTL = crate::Reg<hctl::HCTL_SPEC>;
#[doc = "host configuration register (HCTL)"]
pub mod hctl;
#[doc = "HFT (rw) register accessor: Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hft::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hft::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hft`]
module"]
pub type HFT = crate::Reg<hft::HFT_SPEC>;
#[doc = "Host frame interval register"]
pub mod hft;
#[doc = "HFINFR (r) register accessor: FS host frame number/frame time remaining register (HFINFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfinfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfinfr`]
module"]
pub type HFINFR = crate::Reg<hfinfr::HFINFR_SPEC>;
#[doc = "FS host frame number/frame time remaining register (HFINFR)"]
pub mod hfinfr;
#[doc = "HPTFQSTAT (r) register accessor: Host periodic transmit FIFO/queue status register (HPTFQSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptfqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptfqstat`]
module"]
pub type HPTFQSTAT = crate::Reg<hptfqstat::HPTFQSTAT_SPEC>;
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub mod hptfqstat;
#[doc = "HACHINT (r) register accessor: Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hachint`]
module"]
pub type HACHINT = crate::Reg<hachint::HACHINT_SPEC>;
#[doc = "Host all channels interrupt register"]
pub mod hachint;
#[doc = "HACHINTEN (rw) register accessor: host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hachinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hachinten`]
module"]
pub type HACHINTEN = crate::Reg<hachinten::HACHINTEN_SPEC>;
#[doc = "host all channels interrupt mask register"]
pub mod hachinten;
#[doc = "HPCS (rw) register accessor: Host port control and status register (USBFS_HPCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcs`]
module"]
pub type HPCS = crate::Reg<hpcs::HPCS_SPEC>;
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub mod hpcs;
#[doc = "HCH0CTL (rw) register accessor: host channel-0 characteristics register (HCH0CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0ctl`]
module"]
pub type HCH0CTL = crate::Reg<hch0ctl::HCH0CTL_SPEC>;
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub mod hch0ctl;
#[doc = "HCH1CTL (rw) register accessor: host channel-1 characteristics register (HCH1CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1ctl`]
module"]
pub type HCH1CTL = crate::Reg<hch1ctl::HCH1CTL_SPEC>;
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub mod hch1ctl;
#[doc = "HCH2CTL (rw) register accessor: host channel-2 characteristics register (HCH2CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2ctl`]
module"]
pub type HCH2CTL = crate::Reg<hch2ctl::HCH2CTL_SPEC>;
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub mod hch2ctl;
#[doc = "HCH3CTL (rw) register accessor: host channel-3 characteristics register (HCH3CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3ctl`]
module"]
pub type HCH3CTL = crate::Reg<hch3ctl::HCH3CTL_SPEC>;
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub mod hch3ctl;
#[doc = "HCH4CTL (rw) register accessor: host channel-4 characteristics register (HCH4CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4ctl`]
module"]
pub type HCH4CTL = crate::Reg<hch4ctl::HCH4CTL_SPEC>;
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub mod hch4ctl;
#[doc = "HCH5CTL (rw) register accessor: host channel-5 characteristics register (HCH5CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5ctl`]
module"]
pub type HCH5CTL = crate::Reg<hch5ctl::HCH5CTL_SPEC>;
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub mod hch5ctl;
#[doc = "HCH6CTL (rw) register accessor: host channel-6 characteristics register (HCH6CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6ctl`]
module"]
pub type HCH6CTL = crate::Reg<hch6ctl::HCH6CTL_SPEC>;
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub mod hch6ctl;
#[doc = "HCH7CTL (rw) register accessor: host channel-7 characteristics register (HCH7CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7ctl`]
module"]
pub type HCH7CTL = crate::Reg<hch7ctl::HCH7CTL_SPEC>;
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub mod hch7ctl;
#[doc = "HCH0INTF (rw) register accessor: host channel-0 interrupt register (USBFS_HCHxINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0intf`]
module"]
pub type HCH0INTF = crate::Reg<hch0intf::HCH0INTF_SPEC>;
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)"]
pub mod hch0intf;
#[doc = "HCH1INTF (rw) register accessor: host channel-1 interrupt register (HCH1INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1intf`]
module"]
pub type HCH1INTF = crate::Reg<hch1intf::HCH1INTF_SPEC>;
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub mod hch1intf;
#[doc = "HCH2INTF (rw) register accessor: host channel-2 interrupt register (HCH2INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2intf`]
module"]
pub type HCH2INTF = crate::Reg<hch2intf::HCH2INTF_SPEC>;
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub mod hch2intf;
#[doc = "HCH3INTF (rw) register accessor: host channel-3 interrupt register (HCH3INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3intf`]
module"]
pub type HCH3INTF = crate::Reg<hch3intf::HCH3INTF_SPEC>;
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub mod hch3intf;
#[doc = "HCH4INTF (rw) register accessor: host channel-4 interrupt register (HCH4INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4intf`]
module"]
pub type HCH4INTF = crate::Reg<hch4intf::HCH4INTF_SPEC>;
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub mod hch4intf;
#[doc = "HCH5INTF (rw) register accessor: host channel-5 interrupt register (HCH5INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5intf`]
module"]
pub type HCH5INTF = crate::Reg<hch5intf::HCH5INTF_SPEC>;
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub mod hch5intf;
#[doc = "HCH6INTF (rw) register accessor: host channel-6 interrupt register (HCH6INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6intf`]
module"]
pub type HCH6INTF = crate::Reg<hch6intf::HCH6INTF_SPEC>;
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub mod hch6intf;
#[doc = "HCH7INTF (rw) register accessor: host channel-7 interrupt register (HCH7INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7intf`]
module"]
pub type HCH7INTF = crate::Reg<hch7intf::HCH7INTF_SPEC>;
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub mod hch7intf;
#[doc = "HCH0INTEN (rw) register accessor: host channel-0 interrupt enable register (HCH0INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0inten`]
module"]
pub type HCH0INTEN = crate::Reg<hch0inten::HCH0INTEN_SPEC>;
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub mod hch0inten;
#[doc = "HCH1INTEN (rw) register accessor: host channel-1 interrupt enable register (HCH1INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1inten`]
module"]
pub type HCH1INTEN = crate::Reg<hch1inten::HCH1INTEN_SPEC>;
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub mod hch1inten;
#[doc = "HCH2INTEN (rw) register accessor: host channel-2 interrupt enable register (HCH2INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2inten`]
module"]
pub type HCH2INTEN = crate::Reg<hch2inten::HCH2INTEN_SPEC>;
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub mod hch2inten;
#[doc = "HCH3INTEN (rw) register accessor: host channel-3 interrupt enable register (HCH3INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3inten`]
module"]
pub type HCH3INTEN = crate::Reg<hch3inten::HCH3INTEN_SPEC>;
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub mod hch3inten;
#[doc = "HCH4INTEN (rw) register accessor: host channel-4 interrupt enable register (HCH4INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4inten`]
module"]
pub type HCH4INTEN = crate::Reg<hch4inten::HCH4INTEN_SPEC>;
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub mod hch4inten;
#[doc = "HCH5INTEN (rw) register accessor: host channel-5 interrupt enable register (HCH5INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5inten`]
module"]
pub type HCH5INTEN = crate::Reg<hch5inten::HCH5INTEN_SPEC>;
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub mod hch5inten;
#[doc = "HCH6INTEN (rw) register accessor: host channel-6 interrupt enable register (HCH6INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6inten`]
module"]
pub type HCH6INTEN = crate::Reg<hch6inten::HCH6INTEN_SPEC>;
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub mod hch6inten;
#[doc = "HCH7INTEN (rw) register accessor: host channel-7 interrupt enable register (HCH7INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7inten`]
module"]
pub type HCH7INTEN = crate::Reg<hch7inten::HCH7INTEN_SPEC>;
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub mod hch7inten;
#[doc = "HCH0LEN (rw) register accessor: host channel-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0len`]
module"]
pub type HCH0LEN = crate::Reg<hch0len::HCH0LEN_SPEC>;
#[doc = "host channel-0 transfer length register"]
pub mod hch0len;
#[doc = "HCH1LEN (rw) register accessor: host channel-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1len`]
module"]
pub type HCH1LEN = crate::Reg<hch1len::HCH1LEN_SPEC>;
#[doc = "host channel-1 transfer length register"]
pub mod hch1len;
#[doc = "HCH2LEN (rw) register accessor: host channel-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2len`]
module"]
pub type HCH2LEN = crate::Reg<hch2len::HCH2LEN_SPEC>;
#[doc = "host channel-2 transfer length register"]
pub mod hch2len;
#[doc = "HCH3LEN (rw) register accessor: host channel-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3len`]
module"]
pub type HCH3LEN = crate::Reg<hch3len::HCH3LEN_SPEC>;
#[doc = "host channel-3 transfer length register"]
pub mod hch3len;
#[doc = "HCH4LEN (rw) register accessor: host channel-4 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4len`]
module"]
pub type HCH4LEN = crate::Reg<hch4len::HCH4LEN_SPEC>;
#[doc = "host channel-4 transfer length register"]
pub mod hch4len;
#[doc = "HCH5LEN (rw) register accessor: host channel-5 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5len`]
module"]
pub type HCH5LEN = crate::Reg<hch5len::HCH5LEN_SPEC>;
#[doc = "host channel-5 transfer length register"]
pub mod hch5len;
#[doc = "HCH6LEN (rw) register accessor: host channel-6 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6len`]
module"]
pub type HCH6LEN = crate::Reg<hch6len::HCH6LEN_SPEC>;
#[doc = "host channel-6 transfer length register"]
pub mod hch6len;
#[doc = "HCH7LEN (rw) register accessor: host channel-7 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7len`]
module"]
pub type HCH7LEN = crate::Reg<hch7len::HCH7LEN_SPEC>;
#[doc = "host channel-7 transfer length register"]
pub mod hch7len;
