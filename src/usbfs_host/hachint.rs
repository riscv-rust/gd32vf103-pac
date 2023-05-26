#[doc = "Register `HACHINT` reader"]
pub struct R(crate::R<HACHINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HACHINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HACHINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HACHINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HACHINT` reader - Host all channel interrupts"]
pub type HACHINT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Host all channel interrupts"]
    #[inline(always)]
    pub fn hachint(&self) -> HACHINT_R {
        HACHINT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hachint](index.html) module"]
pub struct HACHINT_SPEC;
impl crate::RegisterSpec for HACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hachint::R](R) reader structure"]
impl crate::Readable for HACHINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HACHINT to value 0"]
impl crate::Resettable for HACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
