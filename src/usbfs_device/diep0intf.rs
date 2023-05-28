#[doc = "Register `DIEP0INTF` reader"]
pub struct R(crate::R<DIEP0INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0INTF` writer"]
pub struct W(crate::W<DIEP0INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0INTF_SPEC>;
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
impl From<crate::W<DIEP0INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0INTF_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - Endpoint finished"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint finished"]
pub type EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0INTF_SPEC, bool, O>;
#[doc = "Field `CITO` reader - Control in timeout interrupt"]
pub type CITO_R = crate::BitReader<bool>;
#[doc = "Field `CITO` writer - Control in timeout interrupt"]
pub type CITO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0INTF_SPEC, bool, O>;
#[doc = "Field `EPTXFUD` reader - Endpoint Tx FIFO underrun"]
pub type EPTXFUD_R = crate::BitReader<bool>;
#[doc = "Field `EPTXFUD` writer - Endpoint Tx FIFO underrun"]
pub type EPTXFUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0INTF_SPEC, bool, O>;
#[doc = "Field `IEPNE` reader - IN endpoint NAK effective"]
pub type IEPNE_R = crate::BitReader<bool>;
#[doc = "Field `IEPNE` writer - IN endpoint NAK effective"]
pub type IEPNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP0INTF_SPEC, bool, O>;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TXFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&self) -> CITO_R {
        CITO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&self) -> EPTXFUD_R {
        EPTXFUD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&self) -> IEPNE_R {
        IEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<0> {
        TF_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<1> {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cito(&mut self) -> CITO_W<3> {
        CITO_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    #[must_use]
    pub fn eptxfud(&mut self) -> EPTXFUD_W<4> {
        EPTXFUD_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn iepne(&mut self) -> IEPNE_W<6> {
        IEPNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0intf](index.html) module"]
pub struct DIEP0INTF_SPEC;
impl crate::RegisterSpec for DIEP0INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0intf::R](R) reader structure"]
impl crate::Readable for DIEP0INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0intf::W](W) writer structure"]
impl crate::Writable for DIEP0INTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP0INTF to value 0x80"]
impl crate::Resettable for DIEP0INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
