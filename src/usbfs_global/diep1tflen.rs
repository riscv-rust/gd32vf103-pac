#[doc = "Register `DIEP1TFLEN` reader"]
pub struct R(crate::R<DIEP1TFLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP1TFLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP1TFLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP1TFLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP1TFLEN` writer"]
pub struct W(crate::W<DIEP1TFLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP1TFLEN_SPEC>;
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
impl From<crate::W<DIEP1TFLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP1TFLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEPTXRSAR` reader - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEPTXRSAR` writer - IN endpoint FIFO transmit RAM start address"]
pub type IEPTXRSAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIEP1TFLEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `IEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type IEPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type IEPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1TFLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO transmit RAM start address"]
    #[inline(always)]
    pub fn ieptxrsar(&self) -> IEPTXRSAR_R {
        IEPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ieptxfd(&self) -> IEPTXFD_R {
        IEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxrsar(&mut self) -> IEPTXRSAR_W<0> {
        IEPTXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxfd(&mut self) -> IEPTXFD_W<16> {
        IEPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1tflen](index.html) module"]
pub struct DIEP1TFLEN_SPEC;
impl crate::RegisterSpec for DIEP1TFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep1tflen::R](R) reader structure"]
impl crate::Readable for DIEP1TFLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep1tflen::W](W) writer structure"]
impl crate::Writable for DIEP1TFLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP1TFLEN to value 0x0200_0400"]
impl crate::Resettable for DIEP1TFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
