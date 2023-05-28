#[doc = "Register `RCRC` reader"]
pub struct R(crate::R<RCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCRC` reader - RX CRC value"]
pub type RCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC value"]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(self.bits)
    }
}
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcrc](index.html) module"]
pub struct RCRC_SPEC;
impl crate::RegisterSpec for RCRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rcrc::R](R) reader structure"]
impl crate::Readable for RCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCRC to value 0"]
impl crate::Resettable for RCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
