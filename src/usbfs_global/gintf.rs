#[doc = "Register `GINTF` reader"]
pub struct R(crate::R<GINTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTF` writer"]
pub struct W(crate::W<GINTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTF_SPEC>;
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
impl From<crate::W<GINTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COPM` reader - Current operation mode"]
pub type COPM_R = crate::BitReader<bool>;
#[doc = "Field `MFIF` reader - Mode fault interrupt flag"]
pub type MFIF_R = crate::BitReader<bool>;
#[doc = "Field `MFIF` writer - Mode fault interrupt flag"]
pub type MFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `OTGIF` reader - OTG interrupt flag"]
pub type OTGIF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `RXFNEIF` reader - RxFIFO non-empty interrupt flag"]
pub type RXFNEIF_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFEIF` reader - Non-periodic TxFIFO empty interrupt flag"]
pub type NPTXFEIF_R = crate::BitReader<bool>;
#[doc = "Field `GNPINAK` reader - Global Non-Periodic IN NAK effective"]
pub type GNPINAK_R = crate::BitReader<bool>;
#[doc = "Field `GONAK` reader - Global OUT NAK effective"]
pub type GONAK_R = crate::BitReader<bool>;
#[doc = "Field `ESP` reader - Early suspend"]
pub type ESP_R = crate::BitReader<bool>;
#[doc = "Field `ESP` writer - Early suspend"]
pub type ESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `SP` reader - USB suspend"]
pub type SP_R = crate::BitReader<bool>;
#[doc = "Field `SP` writer - USB suspend"]
pub type SP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `RST` reader - USB reset"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - USB reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `ENUMF` reader - Enumeration finished"]
pub type ENUMF_R = crate::BitReader<bool>;
#[doc = "Field `ENUMF` writer - Enumeration finished"]
pub type ENUMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `ISOOPDIF` reader - Isochronous OUT packet dropped interrupt"]
pub type ISOOPDIF_R = crate::BitReader<bool>;
#[doc = "Field `ISOOPDIF` writer - Isochronous OUT packet dropped interrupt"]
pub type ISOOPDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `EOPFIF` reader - End of periodic frame interrupt flag"]
pub type EOPFIF_R = crate::BitReader<bool>;
#[doc = "Field `EOPFIF` writer - End of periodic frame interrupt flag"]
pub type EOPFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `IEPIF` reader - IN endpoint interrupt flag"]
pub type IEPIF_R = crate::BitReader<bool>;
#[doc = "Field `OEPIF` reader - OUT endpoint interrupt flag"]
pub type OEPIF_R = crate::BitReader<bool>;
#[doc = "Field `ISOINCIF` reader - Isochronous IN transfer Not Complete Interrupt Flag"]
pub type ISOINCIF_R = crate::BitReader<bool>;
#[doc = "Field `ISOINCIF` writer - Isochronous IN transfer Not Complete Interrupt Flag"]
pub type ISOINCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `PXNCIF_ISOONCIF` reader - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
pub type PXNCIF_ISOONCIF_R = crate::BitReader<bool>;
#[doc = "Field `PXNCIF_ISOONCIF` writer - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
pub type PXNCIF_ISOONCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `HPIF` reader - Host port interrupt flag"]
pub type HPIF_R = crate::BitReader<bool>;
#[doc = "Field `HCIF` reader - Host channels interrupt flag"]
pub type HCIF_R = crate::BitReader<bool>;
#[doc = "Field `PTXFEIF` reader - Periodic TxFIFO empty interrupt flag"]
pub type PTXFEIF_R = crate::BitReader<bool>;
#[doc = "Field `IDPSC` reader - ID pin status change"]
pub type IDPSC_R = crate::BitReader<bool>;
#[doc = "Field `IDPSC` writer - ID pin status change"]
pub type IDPSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `DISCIF` reader - Disconnect interrupt flag"]
pub type DISCIF_R = crate::BitReader<bool>;
#[doc = "Field `DISCIF` writer - Disconnect interrupt flag"]
pub type DISCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `SESIF` reader - Session interrupt flag"]
pub type SESIF_R = crate::BitReader<bool>;
#[doc = "Field `SESIF` writer - Session interrupt flag"]
pub type SESIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
#[doc = "Field `WKUPIF` reader - Wakeup interrupt flag"]
pub type WKUPIF_R = crate::BitReader<bool>;
#[doc = "Field `WKUPIF` writer - Wakeup interrupt flag"]
pub type WKUPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current operation mode"]
    #[inline(always)]
    pub fn copm(&self) -> COPM_R {
        COPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode fault interrupt flag"]
    #[inline(always)]
    pub fn mfif(&self) -> MFIF_R {
        MFIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt flag"]
    #[inline(always)]
    pub fn otgif(&self) -> OTGIF_R {
        OTGIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO non-empty interrupt flag"]
    #[inline(always)]
    pub fn rxfneif(&self) -> RXFNEIF_R {
        RXFNEIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt flag"]
    #[inline(always)]
    pub fn nptxfeif(&self) -> NPTXFEIF_R {
        NPTXFEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-Periodic IN NAK effective"]
    #[inline(always)]
    pub fn gnpinak(&self) -> GNPINAK_R {
        GNPINAK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn gonak(&self) -> GONAK_R {
        GONAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esp(&self) -> ESP_R {
        ESP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration finished"]
    #[inline(always)]
    pub fn enumf(&self) -> ENUMF_R {
        ENUMF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoopdif(&self) -> ISOOPDIF_R {
        ISOOPDIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt flag"]
    #[inline(always)]
    pub fn eopfif(&self) -> EOPFIF_R {
        EOPFIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt flag"]
    #[inline(always)]
    pub fn iepif(&self) -> IEPIF_R {
        IEPIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt flag"]
    #[inline(always)]
    pub fn oepif(&self) -> OEPIF_R {
        OEPIF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Isochronous IN transfer Not Complete Interrupt Flag"]
    #[inline(always)]
    pub fn isoincif(&self) -> ISOINCIF_R {
        ISOINCIF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
    #[inline(always)]
    pub fn pxncif_isooncif(&self) -> PXNCIF_ISOONCIF_R {
        PXNCIF_ISOONCIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt flag"]
    #[inline(always)]
    pub fn hpif(&self) -> HPIF_R {
        HPIF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt flag"]
    #[inline(always)]
    pub fn hcif(&self) -> HCIF_R {
        HCIF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt flag"]
    #[inline(always)]
    pub fn ptxfeif(&self) -> PTXFEIF_R {
        PTXFEIF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - ID pin status change"]
    #[inline(always)]
    pub fn idpsc(&self) -> IDPSC_R {
        IDPSC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect interrupt flag"]
    #[inline(always)]
    pub fn discif(&self) -> DISCIF_R {
        DISCIF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session interrupt flag"]
    #[inline(always)]
    pub fn sesif(&self) -> SESIF_R {
        SESIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&self) -> WKUPIF_R {
        WKUPIF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode fault interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn mfif(&mut self) -> MFIF_W<1> {
        MFIF_W::new(self)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    #[must_use]
    pub fn esp(&mut self) -> ESP_W<10> {
        ESP_W::new(self)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<11> {
        SP_W::new(self)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<12> {
        RST_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration finished"]
    #[inline(always)]
    #[must_use]
    pub fn enumf(&mut self) -> ENUMF_W<13> {
        ENUMF_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isoopdif(&mut self) -> ISOOPDIF_W<14> {
        ISOOPDIF_W::new(self)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn eopfif(&mut self) -> EOPFIF_W<15> {
        EOPFIF_W::new(self)
    }
    #[doc = "Bit 20 - Isochronous IN transfer Not Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isoincif(&mut self) -> ISOINCIF_W<20> {
        ISOINCIF_W::new(self)
    }
    #[doc = "Bit 21 - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn pxncif_isooncif(&mut self) -> PXNCIF_ISOONCIF_W<21> {
        PXNCIF_ISOONCIF_W::new(self)
    }
    #[doc = "Bit 28 - ID pin status change"]
    #[inline(always)]
    #[must_use]
    pub fn idpsc(&mut self) -> IDPSC_W<28> {
        IDPSC_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn discif(&mut self) -> DISCIF_W<29> {
        DISCIF_W::new(self)
    }
    #[doc = "Bit 30 - Session interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn sesif(&mut self) -> SESIF_W<30> {
        SESIF_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn wkupif(&mut self) -> WKUPIF_W<31> {
        WKUPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global interrupt flag register (USBFS_GINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintf](index.html) module"]
pub struct GINTF_SPEC;
impl crate::RegisterSpec for GINTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintf::R](R) reader structure"]
impl crate::Readable for GINTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintf::W](W) writer structure"]
impl crate::Writable for GINTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTF to value 0x0400_0021"]
impl crate::Resettable for GINTF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0021;
}
