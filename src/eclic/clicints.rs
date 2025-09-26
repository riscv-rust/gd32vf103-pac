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
#[doc = "CLICINTIP (rw) register accessor: clicintip Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintip`]
module"]
pub type CLICINTIP = crate::Reg<clicintip::CLICINTIP_SPEC>;
#[doc = "clicintip Register"]
pub mod clicintip;
#[doc = "CLICINTIE (rw) register accessor: clicintie Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintie`]
module"]
pub type CLICINTIE = crate::Reg<clicintie::CLICINTIE_SPEC>;
#[doc = "clicintie Register"]
pub mod clicintie;
#[doc = "CLICINTATTR (rw) register accessor: clicintattr Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintattr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintattr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintattr`]
module"]
pub type CLICINTATTR = crate::Reg<clicintattr::CLICINTATTR_SPEC>;
#[doc = "clicintattr Register"]
pub mod clicintattr;
#[doc = "CLICINTCTL (rw) register accessor: clicintctl Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicintctl`]
module"]
pub type CLICINTCTL = crate::Reg<clicintctl::CLICINTCTL_SPEC>;
#[doc = "clicintctl Register"]
pub mod clicintctl;
