#[doc = "Register `DIVH` reader"]
pub struct R(crate::R<DIVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIV` reader - RTC divider value high"]
pub type DIV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RTC divider value high"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "RTC divider high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divh](index.html) module"]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divh::R](R) reader structure"]
impl crate::Readable for DIVH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
