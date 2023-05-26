#[doc = "Register `DAEPINTEN` reader"]
pub struct R(crate::R<DAEPINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAEPINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAEPINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAEPINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAEPINTEN` writer"]
pub struct W(crate::W<DAEPINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAEPINTEN_SPEC>;
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
impl From<crate::W<DAEPINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAEPINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEPIE` reader - IN EP interrupt interrupt enable bits"]
pub type IEPIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IEPIE` writer - IN EP interrupt interrupt enable bits"]
pub type IEPIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAEPINTEN_SPEC, u8, u8, 4, O>;
#[doc = "Field `OEPIE` reader - OUT endpoint interrupt enable bits"]
pub type OEPIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OEPIE` writer - OUT endpoint interrupt enable bits"]
pub type OEPIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAEPINTEN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&self) -> IEPIE_R {
        IEPIE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&self) -> OEPIE_R {
        OEPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn iepie(&mut self) -> IEPIE_W<0> {
        IEPIE_W::new(self)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn oepie(&mut self) -> OEPIE_W<16> {
        OEPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daepinten](index.html) module"]
pub struct DAEPINTEN_SPEC;
impl crate::RegisterSpec for DAEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daepinten::R](R) reader structure"]
impl crate::Readable for DAEPINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daepinten::W](W) writer structure"]
impl crate::Writable for DAEPINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAEPINTEN to value 0"]
impl crate::Resettable for DAEPINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
