#[doc = "Register `DAC0_R12DH` reader"]
pub struct R(crate::R<DAC0_R12DH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0_R12DH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0_R12DH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0_R12DH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0_R12DH` writer"]
pub struct W(crate::W<DAC0_R12DH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0_R12DH_SPEC>;
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
impl From<crate::W<DAC0_R12DH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0_R12DH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit right-aligned data"]
pub type DAC0_DH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit right-aligned data"]
pub type DAC0_DH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0_R12DH_SPEC, u16, u16, 12, O>;
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
    pub fn dac0_dh(&mut self) -> DAC0_DH_W<0> {
        DAC0_DH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0_r12dh](index.html) module"]
pub struct DAC0_R12DH_SPEC;
impl crate::RegisterSpec for DAC0_R12DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0_r12dh::R](R) reader structure"]
impl crate::Readable for DAC0_R12DH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0_r12dh::W](W) writer structure"]
impl crate::Writable for DAC0_R12DH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0_R12DH to value 0"]
impl crate::Resettable for DAC0_R12DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
