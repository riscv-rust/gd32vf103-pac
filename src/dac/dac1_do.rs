#[doc = "Register `DAC1_DO` reader"]
pub struct R(crate::R<DAC1_DO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1_DO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1_DO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1_DO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAC1_DO` reader - DAC1 data output"]
pub type DAC1_DO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn dac1_do(&self) -> DAC1_DO_R {
        DAC1_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_do](index.html) module"]
pub struct DAC1_DO_SPEC;
impl crate::RegisterSpec for DAC1_DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1_do::R](R) reader structure"]
impl crate::Readable for DAC1_DO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAC1_DO to value 0"]
impl crate::Resettable for DAC1_DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
