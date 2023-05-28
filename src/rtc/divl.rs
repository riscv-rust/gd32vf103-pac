#[doc = "Register `DIVL` reader"]
pub struct R(crate::R<DIVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIV` reader - RTC divider value low"]
pub type DIV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC divider value low"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC divider low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divl](index.html) module"]
pub struct DIVL_SPEC;
impl crate::RegisterSpec for DIVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divl::R](R) reader structure"]
impl crate::Readable for DIVL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DIVL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
