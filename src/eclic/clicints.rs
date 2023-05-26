#[doc = r"Register block"]
#[repr(C)]
pub struct CLICINTS {
    #[doc = "0x00 - clicintip Register"]
    pub clicintip: CLICINTIP,
    #[doc = "0x01 - clicintie Register"]
    pub clicintie: CLICINTIE,
    #[doc = "0x02 - clicintattr Register"]
    pub clicintattr: CLICINTATTR,
    #[doc = "0x03 - clicintctl Register"]
    pub clicintctl: CLICINTCTL,
}
#[doc = "CLICINTIP (rw) register accessor: an alias for `Reg<CLICINTIP_SPEC>`"]
pub type CLICINTIP = crate::Reg<clicintip::CLICINTIP_SPEC>;
#[doc = "clicintip Register"]
pub mod clicintip;
#[doc = "CLICINTIE (rw) register accessor: an alias for `Reg<CLICINTIE_SPEC>`"]
pub type CLICINTIE = crate::Reg<clicintie::CLICINTIE_SPEC>;
#[doc = "clicintie Register"]
pub mod clicintie;
#[doc = "CLICINTATTR (rw) register accessor: an alias for `Reg<CLICINTATTR_SPEC>`"]
pub type CLICINTATTR = crate::Reg<clicintattr::CLICINTATTR_SPEC>;
#[doc = "clicintattr Register"]
pub mod clicintattr;
#[doc = "CLICINTCTL (rw) register accessor: an alias for `Reg<CLICINTCTL_SPEC>`"]
pub type CLICINTCTL = crate::Reg<clicintctl::CLICINTCTL_SPEC>;
#[doc = "clicintctl Register"]
pub mod clicintctl;
