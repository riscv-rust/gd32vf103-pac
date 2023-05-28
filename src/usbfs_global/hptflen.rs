#[doc = "Register `HPTFLEN` reader"]
pub struct R(crate::R<HPTFLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTFLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTFLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTFLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTFLEN` writer"]
pub struct W(crate::W<HPTFLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTFLEN_SPEC>;
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
impl From<crate::W<HPTFLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTFLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPTXFSAR` reader - Host periodic TxFIFO start address"]
pub type HPTXFSAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPTXFSAR` writer - Host periodic TxFIFO start address"]
pub type HPTXFSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTFLEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `HPTXFD` reader - Host periodic TxFIFO depth"]
pub type HPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPTXFD` writer - Host periodic TxFIFO depth"]
pub type HPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTFLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&self) -> HPTXFSAR_R {
        HPTXFSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&self) -> HPTXFD_R {
        HPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn hptxfsar(&mut self) -> HPTXFSAR_W<0> {
        HPTXFSAR_W::new(self)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn hptxfd(&mut self) -> HPTXFD_W<16> {
        HPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptflen](index.html) module"]
pub struct HPTFLEN_SPEC;
impl crate::RegisterSpec for HPTFLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptflen::R](R) reader structure"]
impl crate::Readable for HPTFLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptflen::W](W) writer structure"]
impl crate::Writable for HPTFLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTFLEN to value 0x0200_0600"]
impl crate::Resettable for HPTFLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0600;
}
