#[doc = "Register `DACC_L12DH` reader"]
pub type R = crate::R<DACC_L12DH_SPEC>;
#[doc = "Register `DACC_L12DH` writer"]
pub type W = crate::W<DACC_L12DH_SPEC>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit left-aligned data"]
pub type DAC0_DH_R = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit left-aligned data"]
pub type DAC0_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DAC1_DH` reader - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_R = crate::FieldReader<u16>;
#[doc = "Field `DAC1_DH` writer - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W<DACC_L12DH_SPEC, 4> {
        DAC0_DH_W::new(self)
    }
    #[doc = "Bits 20:31 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<DACC_L12DH_SPEC, 20> {
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
#[doc = "DAC concurrent mode 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_l12dh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_l12dh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACC_L12DH_SPEC;
impl crate::RegisterSpec for DACC_L12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacc_l12dh::R`](R) reader structure"]
impl crate::Readable for DACC_L12DH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dacc_l12dh::W`](W) writer structure"]
impl crate::Writable for DACC_L12DH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACC_L12DH to value 0"]
impl crate::Resettable for DACC_L12DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
