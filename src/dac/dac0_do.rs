#[doc = "Register `DAC0_DO` reader"]
pub type R = crate::R<DAC0_DO_SPEC>;
#[doc = "Field `DAC0_DO` reader - DAC0 data output"]
pub type DAC0_DO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 data output"]
    #[inline(always)]
    pub fn dac0_do(&self) -> DAC0_DO_R {
        DAC0_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0_DO_SPEC;
impl crate::RegisterSpec for DAC0_DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_do::R`](R) reader structure"]
impl crate::Readable for DAC0_DO_SPEC {}
#[doc = "`reset()` method sets DAC0_DO to value 0"]
impl crate::Resettable for DAC0_DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
