#[doc = "Register `GINTEN` reader"]
pub type R = crate::R<GINTEN_SPEC>;
#[doc = "Register `GINTEN` writer"]
pub type W = crate::W<GINTEN_SPEC>;
#[doc = "Field `MFIE` reader - Mode fault interrupt enable"]
pub type MFIE_R = crate::BitReader;
#[doc = "Field `MFIE` writer - Mode fault interrupt enable"]
pub type MFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGIE` reader - OTG interrupt enable"]
pub type OTGIE_R = crate::BitReader;
#[doc = "Field `OTGIE` writer - OTG interrupt enable"]
pub type OTGIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFNEIE` reader - Receive FIFO non-empty interrupt enable"]
pub type RXFNEIE_R = crate::BitReader;
#[doc = "Field `RXFNEIE` writer - Receive FIFO non-empty interrupt enable"]
pub type RXFNEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NPTXFEIE` reader - Non-periodic TxFIFO empty interrupt enable"]
pub type NPTXFEIE_R = crate::BitReader;
#[doc = "Field `NPTXFEIE` writer - Non-periodic TxFIFO empty interrupt enable"]
pub type NPTXFEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GNPINAKIE` reader - Global non-periodic IN NAK effective interrupt enable"]
pub type GNPINAKIE_R = crate::BitReader;
#[doc = "Field `GNPINAKIE` writer - Global non-periodic IN NAK effective interrupt enable"]
pub type GNPINAKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GONAKIE` reader - Global OUT NAK effective interrupt enable"]
pub type GONAKIE_R = crate::BitReader;
#[doc = "Field `GONAKIE` writer - Global OUT NAK effective interrupt enable"]
pub type GONAKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESPIE` reader - Early suspend interrupt enable"]
pub type ESPIE_R = crate::BitReader;
#[doc = "Field `ESPIE` writer - Early suspend interrupt enable"]
pub type ESPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPIE` reader - USB suspend interrupt enable"]
pub type SPIE_R = crate::BitReader;
#[doc = "Field `SPIE` writer - USB suspend interrupt enable"]
pub type SPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIE` reader - USB reset interrupt enable"]
pub type RSTIE_R = crate::BitReader;
#[doc = "Field `RSTIE` writer - USB reset interrupt enable"]
pub type RSTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENUMFIE` reader - Enumeration finish interrupt enable"]
pub type ENUMFIE_R = crate::BitReader;
#[doc = "Field `ENUMFIE` writer - Enumeration finish interrupt enable"]
pub type ENUMFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISOOPDIE` reader - Isochronous OUT packet dropped interrupt enable"]
pub type ISOOPDIE_R = crate::BitReader;
#[doc = "Field `ISOOPDIE` writer - Isochronous OUT packet dropped interrupt enable"]
pub type ISOOPDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPFIE` reader - End of periodic frame interrupt enable"]
pub type EOPFIE_R = crate::BitReader;
#[doc = "Field `EOPFIE` writer - End of periodic frame interrupt enable"]
pub type EOPFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IEPIE` reader - IN endpoints interrupt enable"]
pub type IEPIE_R = crate::BitReader;
#[doc = "Field `IEPIE` writer - IN endpoints interrupt enable"]
pub type IEPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEPIE` reader - OUT endpoints interrupt enable"]
pub type OEPIE_R = crate::BitReader;
#[doc = "Field `OEPIE` writer - OUT endpoints interrupt enable"]
pub type OEPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISOINCIE` reader - isochronous IN transfer not complete interrupt enable"]
pub type ISOINCIE_R = crate::BitReader;
#[doc = "Field `ISOINCIE` writer - isochronous IN transfer not complete interrupt enable"]
pub type ISOINCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PXNCIE_ISOONCIE` reader - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
pub type PXNCIE_ISOONCIE_R = crate::BitReader;
#[doc = "Field `PXNCIE_ISOONCIE` writer - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
pub type PXNCIE_ISOONCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPIE` reader - Host port interrupt enable"]
pub type HPIE_R = crate::BitReader;
#[doc = "Field `HCIE` reader - Host channels interrupt enable"]
pub type HCIE_R = crate::BitReader;
#[doc = "Field `HCIE` writer - Host channels interrupt enable"]
pub type HCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTXFEIE` reader - Periodic TxFIFO empty interrupt enable"]
pub type PTXFEIE_R = crate::BitReader;
#[doc = "Field `PTXFEIE` writer - Periodic TxFIFO empty interrupt enable"]
pub type PTXFEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDPSCIE` reader - ID pin status change interrupt enable"]
pub type IDPSCIE_R = crate::BitReader;
#[doc = "Field `IDPSCIE` writer - ID pin status change interrupt enable"]
pub type IDPSCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCIE` reader - Disconnect interrupt enable"]
pub type DISCIE_R = crate::BitReader;
#[doc = "Field `DISCIE` writer - Disconnect interrupt enable"]
pub type DISCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SESIE` reader - Session interrupt enable"]
pub type SESIE_R = crate::BitReader;
#[doc = "Field `SESIE` writer - Session interrupt enable"]
pub type SESIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUPIE` reader - Wakeup interrupt enable"]
pub type WKUPIE_R = crate::BitReader;
#[doc = "Field `WKUPIE` writer - Wakeup interrupt enable"]
pub type WKUPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Mode fault interrupt enable"]
    #[inline(always)]
    pub fn mfie(&self) -> MFIE_R {
        MFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt enable"]
    #[inline(always)]
    pub fn otgie(&self) -> OTGIE_R {
        OTGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty interrupt enable"]
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn nptxfeie(&self) -> NPTXFEIE_R {
        NPTXFEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gnpinakie(&self) -> GNPINAKIE_R {
        GNPINAKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gonakie(&self) -> GONAKIE_R {
        GONAKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend interrupt enable"]
    #[inline(always)]
    pub fn espie(&self) -> ESPIE_R {
        ESPIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend interrupt enable"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration finish interrupt enable"]
    #[inline(always)]
    pub fn enumfie(&self) -> ENUMFIE_R {
        ENUMFIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt enable"]
    #[inline(always)]
    pub fn isoopdie(&self) -> ISOOPDIE_R {
        ISOOPDIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt enable"]
    #[inline(always)]
    pub fn eopfie(&self) -> EOPFIE_R {
        EOPFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt enable"]
    #[inline(always)]
    pub fn iepie(&self) -> IEPIE_R {
        IEPIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt enable"]
    #[inline(always)]
    pub fn oepie(&self) -> OEPIE_R {
        OEPIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - isochronous IN transfer not complete interrupt enable"]
    #[inline(always)]
    pub fn isoincie(&self) -> ISOINCIE_R {
        ISOINCIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
    #[inline(always)]
    pub fn pxncie_isooncie(&self) -> PXNCIE_ISOONCIE_R {
        PXNCIE_ISOONCIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt enable"]
    #[inline(always)]
    pub fn hpie(&self) -> HPIE_R {
        HPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt enable"]
    #[inline(always)]
    pub fn hcie(&self) -> HCIE_R {
        HCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn ptxfeie(&self) -> PTXFEIE_R {
        PTXFEIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - ID pin status change interrupt enable"]
    #[inline(always)]
    pub fn idpscie(&self) -> IDPSCIE_R {
        IDPSCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect interrupt enable"]
    #[inline(always)]
    pub fn discie(&self) -> DISCIE_R {
        DISCIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session interrupt enable"]
    #[inline(always)]
    pub fn sesie(&self) -> SESIE_R {
        SESIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mfie(&mut self) -> MFIE_W<GINTEN_SPEC, 1> {
        MFIE_W::new(self)
    }
    #[doc = "Bit 2 - OTG interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgie(&mut self) -> OTGIE_W<GINTEN_SPEC, 2> {
        OTGIE_W::new(self)
    }
    #[doc = "Bit 3 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<GINTEN_SPEC, 3> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfneie(&mut self) -> RXFNEIE_W<GINTEN_SPEC, 4> {
        RXFNEIE_W::new(self)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfeie(&mut self) -> NPTXFEIE_W<GINTEN_SPEC, 5> {
        NPTXFEIE_W::new(self)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gnpinakie(&mut self) -> GNPINAKIE_W<GINTEN_SPEC, 6> {
        GNPINAKIE_W::new(self)
    }
    #[doc = "Bit 7 - Global OUT NAK effective interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gonakie(&mut self) -> GONAKIE_W<GINTEN_SPEC, 7> {
        GONAKIE_W::new(self)
    }
    #[doc = "Bit 10 - Early suspend interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn espie(&mut self) -> ESPIE_W<GINTEN_SPEC, 10> {
        ESPIE_W::new(self)
    }
    #[doc = "Bit 11 - USB suspend interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SPIE_W<GINTEN_SPEC, 11> {
        SPIE_W::new(self)
    }
    #[doc = "Bit 12 - USB reset interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<GINTEN_SPEC, 12> {
        RSTIE_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration finish interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn enumfie(&mut self) -> ENUMFIE_W<GINTEN_SPEC, 13> {
        ENUMFIE_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn isoopdie(&mut self) -> ISOOPDIE_W<GINTEN_SPEC, 14> {
        ISOOPDIE_W::new(self)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopfie(&mut self) -> EOPFIE_W<GINTEN_SPEC, 15> {
        EOPFIE_W::new(self)
    }
    #[doc = "Bit 18 - IN endpoints interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iepie(&mut self) -> IEPIE_W<GINTEN_SPEC, 18> {
        IEPIE_W::new(self)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oepie(&mut self) -> OEPIE_W<GINTEN_SPEC, 19> {
        OEPIE_W::new(self)
    }
    #[doc = "Bit 20 - isochronous IN transfer not complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn isoincie(&mut self) -> ISOINCIE_W<GINTEN_SPEC, 20> {
        ISOINCIE_W::new(self)
    }
    #[doc = "Bit 21 - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn pxncie_isooncie(&mut self) -> PXNCIE_ISOONCIE_W<GINTEN_SPEC, 21> {
        PXNCIE_ISOONCIE_W::new(self)
    }
    #[doc = "Bit 25 - Host channels interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcie(&mut self) -> HCIE_W<GINTEN_SPEC, 25> {
        HCIE_W::new(self)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfeie(&mut self) -> PTXFEIE_W<GINTEN_SPEC, 26> {
        PTXFEIE_W::new(self)
    }
    #[doc = "Bit 28 - ID pin status change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idpscie(&mut self) -> IDPSCIE_W<GINTEN_SPEC, 28> {
        IDPSCIE_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn discie(&mut self) -> DISCIE_W<GINTEN_SPEC, 29> {
        DISCIE_W::new(self)
    }
    #[doc = "Bit 30 - Session interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sesie(&mut self) -> SESIE_W<GINTEN_SPEC, 30> {
        SESIE_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkupie(&mut self) -> WKUPIE_W<GINTEN_SPEC, 31> {
        WKUPIE_W::new(self)
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
#[doc = "Global interrupt enable register (USBFS_GINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ginten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ginten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTEN_SPEC;
impl crate::RegisterSpec for GINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ginten::R`](R) reader structure"]
impl crate::Readable for GINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ginten::W`](W) writer structure"]
impl crate::Writable for GINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTEN to value 0"]
impl crate::Resettable for GINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
