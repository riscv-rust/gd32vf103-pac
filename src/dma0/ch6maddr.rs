#[doc = "Register `CH6MADDR` reader"]
pub struct R(crate::R<CH6MADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6MADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH6MADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH6MADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH6MADDR` writer"]
pub struct W(crate::W<CH6MADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH6MADDR_SPEC>;
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
impl From<crate::W<CH6MADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH6MADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MADDR` reader - Memory base address"]
pub type MADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MADDR` writer - Memory base address"]
pub type MADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6MADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    pub fn maddr(&self) -> MADDR_R {
        MADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory base address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MADDR_W<0> {
        MADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 6 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6maddr](index.html) module"]
pub struct CH6MADDR_SPEC;
impl crate::RegisterSpec for CH6MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6maddr::R](R) reader structure"]
impl crate::Readable for CH6MADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch6maddr::W](W) writer structure"]
impl crate::Writable for CH6MADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH6MADDR to value 0"]
impl crate::Resettable for CH6MADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
