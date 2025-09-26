#[doc = "Register `HFINFR` reader"]
pub type R = crate::R<HFINFR_SPEC>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FRT` reader - Frame remaining time"]
pub type FRT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame remaining time"]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FS host frame number/frame time remaining register (HFINFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfinfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFINFR_SPEC;
impl crate::RegisterSpec for HFINFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfinfr::R`](R) reader structure"]
impl crate::Readable for HFINFR_SPEC {}
#[doc = "`reset()` method sets HFINFR to value 0xbb80_0000"]
impl crate::Resettable for HFINFR_SPEC {
    const RESET_VALUE: u32 = 0xbb80_0000;
}
