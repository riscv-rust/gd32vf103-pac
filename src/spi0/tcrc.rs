#[doc = "Register `TCRC` reader"]
pub struct R(crate::R<TCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TCRC` reader - Tx CRC value"]
pub type TCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC value"]
    #[inline(always)]
    pub fn tcrc(&self) -> TCRC_R {
        TCRC_R::new(self.bits)
    }
}
#[doc = "TX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcrc](index.html) module"]
pub struct TCRC_SPEC;
impl crate::RegisterSpec for TCRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcrc::R](R) reader structure"]
impl crate::Readable for TCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCRC to value 0"]
impl crate::Resettable for TCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
