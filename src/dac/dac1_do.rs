#[doc = "Register `DAC1_DO` reader"]
pub type R = crate::R<DAC1_DO_SPEC>;
#[doc = "Field `DAC1_DO` reader - DAC1 data output"]
pub type DAC1_DO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn dac1_do(&self) -> DAC1_DO_R {
        DAC1_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_do::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1_DO_SPEC;
impl crate::RegisterSpec for DAC1_DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_do::R`](R) reader structure"]
impl crate::Readable for DAC1_DO_SPEC {}
#[doc = "`reset()` method sets DAC1_DO to value 0"]
impl crate::Resettable for DAC1_DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
