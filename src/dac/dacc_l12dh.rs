#[doc = "Register `DACC_L12DH` reader"]
pub struct R(crate::R<DACC_L12DH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACC_L12DH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACC_L12DH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACC_L12DH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACC_L12DH` writer"]
pub struct W(crate::W<DACC_L12DH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACC_L12DH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DACC_L12DH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACC_L12DH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit left-aligned data"]
pub type DAC0_DH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit left-aligned data"]
pub type DAC0_DH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACC_L12DH_SPEC, u16, u16, 12, O>;
#[doc = "Field `DAC1_DH` reader - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAC1_DH` writer - DAC1 12-bit left-aligned data"]
pub type DAC1_DH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACC_L12DH_SPEC, u16, u16, 12, O>;
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
    pub fn dac0_dh(&mut self) -> DAC0_DH_W<4> {
        DAC0_DH_W::new(self)
    }
    #[doc = "Bits 20:31 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<20> {
        DAC1_DH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC concurrent mode 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_l12dh](index.html) module"]
pub struct DACC_L12DH_SPEC;
impl crate::RegisterSpec for DACC_L12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacc_l12dh::R](R) reader structure"]
impl crate::Readable for DACC_L12DH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacc_l12dh::W](W) writer structure"]
impl crate::Writable for DACC_L12DH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACC_L12DH to value 0"]
impl crate::Resettable for DACC_L12DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
