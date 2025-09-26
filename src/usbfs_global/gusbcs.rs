#[doc = "Register `GUSBCS` reader"]
pub type R = crate::R<GUSBCS_SPEC>;
#[doc = "Register `GUSBCS` writer"]
pub type W = crate::W<GUSBCS_SPEC>;
#[doc = "Field `TOC` reader - Timeout calibration"]
pub type TOC_R = crate::FieldReader;
#[doc = "Field `TOC` writer - Timeout calibration"]
pub type TOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SRPCEN` reader - SRP capability enable"]
pub type SRPCEN_R = crate::BitReader;
#[doc = "Field `SRPCEN` writer - SRP capability enable"]
pub type SRPCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNPCEN` reader - HNP capability enable"]
pub type HNPCEN_R = crate::BitReader;
#[doc = "Field `HNPCEN` writer - HNP capability enable"]
pub type HNPCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UTT` reader - USB turnaround time"]
pub type UTT_R = crate::FieldReader;
#[doc = "Field `UTT` writer - USB turnaround time"]
pub type UTT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FHM` reader - Force host mode"]
pub type FHM_R = crate::BitReader;
#[doc = "Field `FHM` writer - Force host mode"]
pub type FHM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDM` reader - Force device mode"]
pub type FDM_R = crate::BitReader;
#[doc = "Field `FDM` writer - Force device mode"]
pub type FDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn toc(&mut self) -> TOC_W<GUSBCS_SPEC, 0> {
        TOC_W::new(self)
    }
    #[doc = "Bit 8 - SRP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcen(&mut self) -> SRPCEN_W<GUSBCS_SPEC, 8> {
        SRPCEN_W::new(self)
    }
    #[doc = "Bit 9 - HNP capability enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcen(&mut self) -> HNPCEN_W<GUSBCS_SPEC, 9> {
        HNPCEN_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn utt(&mut self) -> UTT_W<GUSBCS_SPEC, 10> {
        UTT_W::new(self)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhm(&mut self) -> FHM_W<GUSBCS_SPEC, 29> {
        FHM_W::new(self)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdm(&mut self) -> FDM_W<GUSBCS_SPEC, 30> {
        FDM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global USB control and status register (USBFS_GUSBCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCS_SPEC;
impl crate::RegisterSpec for GUSBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcs::R`](R) reader structure"]
impl crate::Readable for GUSBCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcs::W`](W) writer structure"]
impl crate::Writable for GUSBCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCS to value 0x0a80"]
impl crate::Resettable for GUSBCS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a80;
}
