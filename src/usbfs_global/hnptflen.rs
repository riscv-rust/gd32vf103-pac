#[doc = "Register `HNPTFLEN` reader"]
pub struct R(crate::R<HNPTFLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HNPTFLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HNPTFLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HNPTFLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HNPTFLEN` writer"]
pub struct W(crate::W<HNPTFLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HNPTFLEN_SPEC>;
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
impl From<crate::W<HNPTFLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HNPTFLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HNPTXRSAR` reader - host non-periodic transmit Tx RAM start address"]
pub type HNPTXRSAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HNPTXRSAR` writer - host non-periodic transmit Tx RAM start address"]
pub type HNPTXRSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HNPTFLEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `HNPTXFD` reader - host non-periodic TxFIFO depth"]
pub type HNPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HNPTXFD` writer - host non-periodic TxFIFO depth"]
pub type HNPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HNPTFLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    pub fn hnptxrsar(&self) -> HNPTXRSAR_R {
        HNPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hnptxfd(&self) -> HNPTXFD_R {
        HNPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn hnptxrsar(&mut self) -> HNPTXRSAR_W<0> {
        HNPTXRSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn hnptxfd(&mut self) -> HNPTXFD_W<16> {
        HNPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host non-periodic transmit FIFO length register (Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptflen](index.html) module"]
pub struct HNPTFLEN_SPEC;
impl crate::RegisterSpec for HNPTFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hnptflen::R](R) reader structure"]
impl crate::Readable for HNPTFLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hnptflen::W](W) writer structure"]
impl crate::Writable for HNPTFLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HNPTFLEN to value 0x0200_0200"]
impl crate::Resettable for HNPTFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0200;
}
