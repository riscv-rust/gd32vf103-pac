#[doc = "Register `RFIFOMI1` reader"]
pub struct R(crate::R<RFIFOMI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOMI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOMI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOMI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FT` reader - Frame type"]
pub type FT_R = crate::BitReader<bool>;
#[doc = "Field `FF` reader - Frame format"]
pub type FF_R = crate::BitReader<bool>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EFID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SFID_EFID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Receive FIFO1 mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomi1](index.html) module"]
pub struct RFIFOMI1_SPEC;
impl crate::RegisterSpec for RFIFOMI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifomi1::R](R) reader structure"]
impl crate::Readable for RFIFOMI1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFOMI1 to value 0"]
impl crate::Resettable for RFIFOMI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
