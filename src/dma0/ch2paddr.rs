#[doc = "Register `CH2PADDR` reader"]
pub struct R(crate::R<CH2PADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2PADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2PADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2PADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2PADDR` writer"]
pub struct W(crate::W<CH2PADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2PADDR_SPEC>;
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
impl From<crate::W<CH2PADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2PADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADDR` reader - Peripheral base address"]
pub type PADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PADDR` writer - Peripheral base address"]
pub type PADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2PADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral base address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PADDR_W<0> {
        PADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2paddr](index.html) module"]
pub struct CH2PADDR_SPEC;
impl crate::RegisterSpec for CH2PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2paddr::R](R) reader structure"]
impl crate::Readable for CH2PADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2paddr::W](W) writer structure"]
impl crate::Writable for CH2PADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2PADDR to value 0"]
impl crate::Resettable for CH2PADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
