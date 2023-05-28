#[doc = "Register `RSQ0` reader"]
pub struct R(crate::R<RSQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQ0` writer"]
pub struct W(crate::W<RSQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQ0_SPEC>;
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
impl From<crate::W<RSQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSQ12` reader - 13th conversion in regular sequence"]
pub type RSQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ12` writer - 13th conversion in regular sequence"]
pub type RSQ12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ0_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ13` reader - 14th conversion in regular sequence"]
pub type RSQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ13` writer - 14th conversion in regular sequence"]
pub type RSQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ0_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ14` reader - 15th conversion in regular sequence"]
pub type RSQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ14` writer - 15th conversion in regular sequence"]
pub type RSQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ0_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSQ15` reader - 16th conversion in regular sequence"]
pub type RSQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ15` writer - 16th conversion in regular sequence"]
pub type RSQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ0_SPEC, u8, u8, 5, O>;
#[doc = "Field `RL` reader - Regular channel group length"]
pub type RL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RL` writer - Regular channel group length"]
pub type RL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSQ0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq12(&self) -> RSQ12_R {
        RSQ12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&self) -> RSQ13_R {
        RSQ13_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&self) -> RSQ14_R {
        RSQ14_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&self) -> RSQ15_R {
        RSQ15_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq12(&mut self) -> RSQ12_W<0> {
        RSQ12_W::new(self)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq13(&mut self) -> RSQ13_W<5> {
        RSQ13_W::new(self)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq14(&mut self) -> RSQ14_W<10> {
        RSQ14_W::new(self)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq15(&mut self) -> RSQ15_W<15> {
        RSQ15_W::new(self)
    }
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<20> {
        RL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq0](index.html) module"]
pub struct RSQ0_SPEC;
impl crate::RegisterSpec for RSQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsq0::R](R) reader structure"]
impl crate::Readable for RSQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsq0::W](W) writer structure"]
impl crate::Writable for RSQ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQ0 to value 0"]
impl crate::Resettable for RSQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
