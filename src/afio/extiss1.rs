#[doc = "Register `EXTISS1` reader"]
pub struct R(crate::R<EXTISS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS1` writer"]
pub struct W(crate::W<EXTISS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS1_SPEC>;
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
impl From<crate::W<EXTISS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4_SS` reader - EXTI 4 sources selection"]
pub type EXTI4_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI4_SS` writer - EXTI 4 sources selection"]
pub type EXTI4_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI5_SS` reader - EXTI 5 sources selection"]
pub type EXTI5_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI5_SS` writer - EXTI 5 sources selection"]
pub type EXTI5_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI6_SS` reader - EXTI 6 sources selection"]
pub type EXTI6_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI6_SS` writer - EXTI 6 sources selection"]
pub type EXTI6_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI7_SS` reader - EXTI 7 sources selection"]
pub type EXTI7_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI7_SS` writer - EXTI 7 sources selection"]
pub type EXTI7_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> EXTI4_SS_R {
        EXTI4_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> EXTI5_SS_R {
        EXTI5_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> EXTI6_SS_R {
        EXTI6_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> EXTI7_SS_R {
        EXTI7_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti4_ss(&mut self) -> EXTI4_SS_W<0> {
        EXTI4_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti5_ss(&mut self) -> EXTI5_SS_W<4> {
        EXTI5_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti6_ss(&mut self) -> EXTI6_SS_W<8> {
        EXTI6_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti7_ss(&mut self) -> EXTI7_SS_W<12> {
        EXTI7_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss1](index.html) module"]
pub struct EXTISS1_SPEC;
impl crate::RegisterSpec for EXTISS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss1::R](R) reader structure"]
impl crate::Readable for EXTISS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss1::W](W) writer structure"]
impl crate::Writable for EXTISS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS1 to value 0"]
impl crate::Resettable for EXTISS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
