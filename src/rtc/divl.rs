#[doc = "Register `DIVL` reader"]
pub type R = crate::R<DIVL_SPEC>;
#[doc = "Field `DIV` reader - RTC divider value low"]
pub type DIV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RTC divider value low"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC divider low register\n\nYou can [`read`](crate::Reg::read) this register and get [`divl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVL_SPEC;
impl crate::RegisterSpec for DIVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divl::R`](R) reader structure"]
impl crate::Readable for DIVL_SPEC {}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DIVL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
