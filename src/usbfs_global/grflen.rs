#[doc = "Register `GRFLEN` reader"]
pub struct R(crate::R<GRFLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRFLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRFLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRFLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRFLEN` writer"]
pub struct W(crate::W<GRFLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRFLEN_SPEC>;
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
impl From<crate::W<GRFLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRFLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFD` reader - Rx FIFO depth"]
pub type RXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXFD` writer - Rx FIFO depth"]
pub type RXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GRFLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Rx FIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<0> {
        RXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grflen](index.html) module"]
pub struct GRFLEN_SPEC;
impl crate::RegisterSpec for GRFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grflen::R](R) reader structure"]
impl crate::Readable for GRFLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grflen::W](W) writer structure"]
impl crate::Writable for GRFLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRFLEN to value 0x0200"]
impl crate::Resettable for GRFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
