#[doc = "Register `DAEPINT` reader"]
pub struct R(crate::R<DAEPINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAEPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAEPINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAEPINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEPITB` reader - Device all IN endpoint interrupt bits"]
pub type IEPITB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OEPITB` reader - Device all OUT endpoint interrupt bits"]
pub type OEPITB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Device all IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepitb(&self) -> IEPITB_R {
        IEPITB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Device all OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepitb(&self) -> OEPITB_R {
        OEPITB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "device all endpoints interrupt register (DAEPINT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daepint](index.html) module"]
pub struct DAEPINT_SPEC;
impl crate::RegisterSpec for DAEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daepint::R](R) reader structure"]
impl crate::Readable for DAEPINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAEPINT to value 0"]
impl crate::Resettable for DAEPINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
