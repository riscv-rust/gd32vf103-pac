#[doc = "Register `GUSBCS` reader"]
pub struct R(crate::R<GUSBCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCS` writer"]
pub struct W(crate::W<GUSBCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCS_SPEC>;
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
impl From<crate::W<GUSBCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC` reader - Timeout calibration"]
pub type TOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOC` writer - Timeout calibration"]
pub type TOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCS_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRPCEN` reader - SRP capability enable"]
pub type SRPCEN_R = crate::BitReader<bool>;
#[doc = "Field `SRPCEN` writer - SRP capability enable"]
pub type SRPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCS_SPEC, bool, O>;
#[doc = "Field `HNPCEN` reader - HNP capability enable"]
pub type HNPCEN_R = crate::BitReader<bool>;
#[doc = "Field `HNPCEN` writer - HNP capability enable"]
pub type HNPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCS_SPEC, bool, O>;
#[doc = "Field `UTT` reader - USB turnaround time"]
pub type UTT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTT` writer - USB turnaround time"]
pub type UTT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCS_SPEC, u8, u8, 4, O>;
#[doc = "Field `FHM` reader - Force host mode"]
pub type FHM_R = crate::BitReader<bool>;
#[doc = "Field `FHM` writer - Force host mode"]
pub type FHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCS_SPEC, bool, O>;
#[doc = "Field `FDM` reader - Force device mode"]
pub type FDM_R = crate::BitReader<bool>;
#[doc = "Field `FDM` writer - Force device mode"]
pub type FDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Timeout calibration"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    pub fn srpcen(&self) -> SRPCEN_R {
        SRPCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    pub fn hnpcen(&self) -> HNPCEN_R {
        HNPCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn utt(&self) -> UTT_R {
        UTT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhm(&self) -> FHM_R {
        FHM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdm(&self) -> FDM_R {
        FDM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<0> {
        TOC_W::new(self)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcen(&mut self) -> SRPCEN_W<8> {
        SRPCEN_W::new(self)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcen(&mut self) -> HNPCEN_W<9> {
        HNPCEN_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn utt(&mut self) -> UTT_W<10> {
        UTT_W::new(self)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhm(&mut self) -> FHM_W<29> {
        FHM_W::new(self)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdm(&mut self) -> FDM_W<30> {
        FDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global USB control and status register (USBFS_GUSBCSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcs](index.html) module"]
pub struct GUSBCS_SPEC;
impl crate::RegisterSpec for GUSBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcs::R](R) reader structure"]
impl crate::Readable for GUSBCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcs::W](W) writer structure"]
impl crate::Writable for GUSBCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCS to value 0x0a80"]
impl crate::Resettable for GUSBCS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a80;
}
