#[doc = "Register `EXTISS2` reader"]
pub struct R(crate::R<EXTISS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS2` writer"]
pub struct W(crate::W<EXTISS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS2_SPEC>;
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
impl From<crate::W<EXTISS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8_SS` reader - EXTI 8 sources selection"]
pub type EXTI8_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI8_SS` writer - EXTI 8 sources selection"]
pub type EXTI8_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI9_SS` reader - EXTI 9 sources selection"]
pub type EXTI9_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI9_SS` writer - EXTI 9 sources selection"]
pub type EXTI9_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI10_SS` reader - EXTI 10 sources selection"]
pub type EXTI10_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI10_SS` writer - EXTI 10 sources selection"]
pub type EXTI10_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI11_SS` reader - EXTI 11 sources selection"]
pub type EXTI11_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI11_SS` writer - EXTI 11 sources selection"]
pub type EXTI11_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&self) -> EXTI8_SS_R {
        EXTI8_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&self) -> EXTI9_SS_R {
        EXTI9_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&self) -> EXTI10_SS_R {
        EXTI10_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&self) -> EXTI11_SS_R {
        EXTI11_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_ss(&mut self) -> EXTI8_SS_W<0> {
        EXTI8_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti9_ss(&mut self) -> EXTI9_SS_W<4> {
        EXTI9_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti10_ss(&mut self) -> EXTI10_SS_W<8> {
        EXTI10_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti11_ss(&mut self) -> EXTI11_SS_W<12> {
        EXTI11_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss2](index.html) module"]
pub struct EXTISS2_SPEC;
impl crate::RegisterSpec for EXTISS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss2::R](R) reader structure"]
impl crate::Readable for EXTISS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss2::W](W) writer structure"]
impl crate::Writable for EXTISS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS2 to value 0"]
impl crate::Resettable for EXTISS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
