#[doc = "Register `HFT` reader"]
pub struct R(crate::R<HFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFT` writer"]
pub struct W(crate::W<HFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFT_SPEC>;
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
impl From<crate::W<HFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRI` reader - Frame interval"]
pub type FRI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRI` writer - Frame interval"]
pub type FRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn fri(&mut self) -> FRI_W<0> {
        FRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hft](index.html) module"]
pub struct HFT_SPEC;
impl crate::RegisterSpec for HFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hft::R](R) reader structure"]
impl crate::Readable for HFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hft::W](W) writer structure"]
impl crate::Writable for HFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFT to value 0xbb80"]
impl crate::Resettable for HFT_SPEC {
    const RESET_VALUE: Self::Ux = 0xbb80;
}
