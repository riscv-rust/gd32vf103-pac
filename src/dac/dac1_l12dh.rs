#[doc = "Register `DAC1_L12DH` reader"]
pub type R = crate::R<DAC1_L12DH_SPEC>;
#[doc = "Register `DAC1_L12DH` writer"]
pub type W = crate::W<DAC1_L12DH_SPEC>;
#[doc = "Field `DAC1_DH` reader - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_R = crate::FieldReader<u16>;
#[doc = "Field `DAC1_DH` writer - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<DAC1_L12DH_SPEC, 4> {
        DAC1_DH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_l12dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_l12dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC1_L12DH_SPEC;
impl crate::RegisterSpec for DAC1_L12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_l12dh::R`](R) reader structure"]
impl crate::Readable for DAC1_L12DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac1_l12dh::W`](W) writer structure"]
impl crate::Writable for DAC1_L12DH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC1_L12DH to value 0"]
impl crate::Resettable for DAC1_L12DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
