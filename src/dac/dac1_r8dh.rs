#[doc = "Register `DAC1_R8DH` reader"]
pub type R = crate::R<DAC1_R8DH_SPEC>;
#[doc = "Register `DAC1_R8DH` writer"]
pub type W = crate::W<DAC1_R8DH_SPEC>;
#[doc = "Field `DAC1_DH` reader - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_R = crate::FieldReader;
#[doc = "Field `DAC1_DH` writer - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<DAC1_R8DH_SPEC> {
        DAC1_DH_W::new(self, 0)
    }
}
#[doc = "DAC1 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_r8dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_r8dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1_R8DH_SPEC;
impl crate::RegisterSpec for DAC1_R8DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_r8dh::R`](R) reader structure"]
impl crate::Readable for DAC1_R8DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1_r8dh::W`](W) writer structure"]
impl crate::Writable for DAC1_R8DH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1_R8DH to value 0"]
impl crate::Resettable for DAC1_R8DH_SPEC {
    const RESET_VALUE: u32 = 0;
}
