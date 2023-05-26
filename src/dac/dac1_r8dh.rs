#[doc = "Register `DAC1_R8DH` reader"]
pub struct R(crate::R<DAC1_R8DH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1_R8DH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1_R8DH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1_R8DH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC1_R8DH` writer"]
pub struct W(crate::W<DAC1_R8DH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC1_R8DH_SPEC>;
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
impl From<crate::W<DAC1_R8DH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC1_R8DH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC1_DH` reader - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC1_DH` writer - DAC1 8-bit right-aligned data"]
pub type DAC1_DH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC1_R8DH_SPEC, u8, u8, 8, O>;
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
    pub fn dac1_dh(&mut self) -> DAC1_DH_W<0> {
        DAC1_DH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC1 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_r8dh](index.html) module"]
pub struct DAC1_R8DH_SPEC;
impl crate::RegisterSpec for DAC1_R8DH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1_r8dh::R](R) reader structure"]
impl crate::Readable for DAC1_R8DH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac1_r8dh::W](W) writer structure"]
impl crate::Writable for DAC1_R8DH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC1_R8DH to value 0"]
impl crate::Resettable for DAC1_R8DH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
