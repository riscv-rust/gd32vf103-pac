#[doc = "Register `DAC0_R12DH` reader"]
pub type R = crate::R<DAC0_R12DH_SPEC>;
#[doc = "Register `DAC0_R12DH` writer"]
pub type W = crate::W<DAC0_R12DH_SPEC>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit right-aligned data"]
pub type DAC0_DH_R = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit right-aligned data"]
pub type DAC0_DH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W<DAC0_R12DH_SPEC> {
        DAC0_DH_W::new(self, 0)
    }
}
#[doc = "DAC0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_r12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_r12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0_R12DH_SPEC;
impl crate::RegisterSpec for DAC0_R12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_r12dh::R`](R) reader structure"]
impl crate::Readable for DAC0_R12DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac0_r12dh::W`](W) writer structure"]
impl crate::Writable for DAC0_R12DH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0_R12DH to value 0"]
impl crate::Resettable for DAC0_R12DH_SPEC {
    const RESET_VALUE: u32 = 0;
}
