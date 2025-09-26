#[doc = "Register `DIVH` reader"]
pub type R = crate::R<DIVH_SPEC>;
#[doc = "Field `DIV` reader - RTC divider value high"]
pub type DIV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC divider value high"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC divider high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divh::R`](R) reader structure"]
impl crate::Readable for DIVH_SPEC {}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
