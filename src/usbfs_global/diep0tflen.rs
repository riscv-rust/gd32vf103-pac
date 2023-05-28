#[doc = "Register `DIEP0TFLEN` reader"]
pub struct R(crate::R<DIEP0TFLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0TFLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0TFLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0TFLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0TFLEN` writer"]
pub struct W(crate::W<DIEP0TFLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0TFLEN_SPEC>;
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
impl From<crate::W<DIEP0TFLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0TFLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEP0TXRSAR` reader - in endpoint 0 Tx RAM start address"]
pub type IEP0TXRSAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEP0TXRSAR` writer - in endpoint 0 Tx RAM start address"]
pub type IEP0TXRSAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIEP0TFLEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `IEP0TXFD` reader - in endpoint 0 Tx FIFO depth"]
pub type IEP0TXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEP0TXFD` writer - in endpoint 0 Tx FIFO depth"]
pub type IEP0TXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIEP0TFLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    pub fn iep0txrsar(&self) -> IEP0TXRSAR_R {
        IEP0TXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    pub fn iep0txfd(&self) -> IEP0TXFD_R {
        IEP0TXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn iep0txrsar(&mut self) -> IEP0TXRSAR_W<0> {
        IEP0TXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn iep0txfd(&mut self) -> IEP0TXFD_W<16> {
        IEP0TXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tflen](index.html) module"]
pub struct DIEP0TFLEN_SPEC;
impl crate::RegisterSpec for DIEP0TFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0tflen::R](R) reader structure"]
impl crate::Readable for DIEP0TFLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0tflen::W](W) writer structure"]
impl crate::Writable for DIEP0TFLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0TFLEN to value 0x0200_0200"]
impl crate::Resettable for DIEP0TFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0200;
}
