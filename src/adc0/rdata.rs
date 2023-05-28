#[doc = "Register `RDATA` reader"]
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Regular channel data"]
pub type RDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC1RDTR` reader - ADC regular channel data"]
pub type ADC1RDTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Regular channel data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC regular channel data"]
    #[inline(always)]
    pub fn adc1rdtr(&self) -> ADC1RDTR_R {
        ADC1RDTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](index.html) module"]
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdata::R](R) reader structure"]
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
