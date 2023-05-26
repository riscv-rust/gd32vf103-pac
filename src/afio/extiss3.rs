#[doc = "Register `EXTISS3` reader"]
pub struct R(crate::R<EXTISS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTISS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTISS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTISS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTISS3` writer"]
pub struct W(crate::W<EXTISS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTISS3_SPEC>;
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
impl From<crate::W<EXTISS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTISS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12_SS` reader - EXTI 12 sources selection"]
pub type EXTI12_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI12_SS` writer - EXTI 12 sources selection"]
pub type EXTI12_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI13_SS` reader - EXTI 13 sources selection"]
pub type EXTI13_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI13_SS` writer - EXTI 13 sources selection"]
pub type EXTI13_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI14_SS` reader - EXTI 14 sources selection"]
pub type EXTI14_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI14_SS` writer - EXTI 14 sources selection"]
pub type EXTI14_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXTI15_SS` reader - EXTI 15 sources selection"]
pub type EXTI15_SS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI15_SS` writer - EXTI 15 sources selection"]
pub type EXTI15_SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTISS3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&self) -> EXTI12_SS_R {
        EXTI12_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&self) -> EXTI13_SS_R {
        EXTI13_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&self) -> EXTI14_SS_R {
        EXTI14_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&self) -> EXTI15_SS_R {
        EXTI15_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti12_ss(&mut self) -> EXTI12_SS_W<0> {
        EXTI12_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti13_ss(&mut self) -> EXTI13_SS_W<4> {
        EXTI13_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti14_ss(&mut self) -> EXTI14_SS_W<8> {
        EXTI14_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti15_ss(&mut self) -> EXTI15_SS_W<12> {
        EXTI15_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI sources selection register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss3](index.html) module"]
pub struct EXTISS3_SPEC;
impl crate::RegisterSpec for EXTISS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extiss3::R](R) reader structure"]
impl crate::Readable for EXTISS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extiss3::W](W) writer structure"]
impl crate::Writable for EXTISS3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS3 to value 0"]
impl crate::Resettable for EXTISS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
