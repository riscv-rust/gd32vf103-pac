#[doc = "Register `CLICINTIE` reader"]
pub struct R(crate::R<CLICINTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLICINTIE` writer"]
pub struct W(crate::W<CLICINTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICINTIE_SPEC>;
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
impl From<crate::W<CLICINTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICINTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLICINTIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clicintie Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicintie](index.html) module"]
pub struct CLICINTIE_SPEC;
impl crate::RegisterSpec for CLICINTIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clicintie::R](R) reader structure"]
impl crate::Readable for CLICINTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clicintie::W](W) writer structure"]
impl crate::Writable for CLICINTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTIE to value 0"]
impl crate::Resettable for CLICINTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
