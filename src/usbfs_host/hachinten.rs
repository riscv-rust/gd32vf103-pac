#[doc = "Register `HACHINTEN` reader"]
pub struct R(crate::R<HACHINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HACHINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HACHINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HACHINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HACHINTEN` writer"]
pub struct W(crate::W<HACHINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HACHINTEN_SPEC>;
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
impl From<crate::W<HACHINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HACHINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CINTEN` reader - Channel interrupt enable"]
pub type CINTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CINTEN` writer - Channel interrupt enable"]
pub type CINTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HACHINTEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&self) -> CINTEN_R {
        CINTEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinten(&mut self) -> CINTEN_W<0> {
        CINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hachinten](index.html) module"]
pub struct HACHINTEN_SPEC;
impl crate::RegisterSpec for HACHINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hachinten::R](R) reader structure"]
impl crate::Readable for HACHINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hachinten::W](W) writer structure"]
impl crate::Writable for HACHINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HACHINTEN to value 0"]
impl crate::Resettable for HACHINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
