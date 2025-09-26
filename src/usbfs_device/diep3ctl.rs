#[doc = "Register `DIEP3CTL` reader"]
pub type R = crate::R<DIEP3CTL_SPEC>;
#[doc = "Register `DIEP3CTL` writer"]
pub type W = crate::W<DIEP3CTL_SPEC>;
#[doc = "Field `MPL` reader - maximum packet length"]
pub type MPL_R = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - maximum packet length"]
pub type MPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `EPACT` reader - Endpoint active"]
pub type EPACT_R = crate::BitReader;
#[doc = "Field `EPACT` writer - Endpoint active"]
pub type EPACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOFRM_DPID` reader - EOFRM/DPID"]
pub type EOFRM_DPID_R = crate::BitReader;
#[doc = "Field `NAKS` reader - NAK status"]
pub type NAKS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFNUM` reader - Tx FIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - Tx FIFO number"]
pub type TXFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SD0PID_SEVENFRM` writer - SD0PID/SEVNFRM"]
pub type SD0PID_SEVENFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SD1PID_SODDFRM` writer - Set DATA1 PID/Set odd frame"]
pub type SD1PID_SODDFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPD` reader - Endpoint disable"]
pub type EPD_R = crate::BitReader;
#[doc = "Field `EPD` writer - Endpoint disable"]
pub type EPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPEN` reader - Endpoint enable"]
pub type EPEN_R = crate::BitReader;
#[doc = "Field `EPEN` writer - Endpoint enable"]
pub type EPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    pub fn mpl(&self) -> MPL_R {
        MPL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    pub fn epact(&self) -> EPACT_R {
        EPACT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EOFRM/DPID"]
    #[inline(always)]
    pub fn eofrm_dpid(&self) -> EOFRM_DPID_R {
        EOFRM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naks(&self) -> NAKS_R {
        NAKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - Tx FIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epd(&self) -> EPD_R {
        EPD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    #[must_use]
    pub fn mpl(&mut self) -> MPL_W<DIEP3CTL_SPEC, 0> {
        MPL_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    #[must_use]
    pub fn epact(&mut self) -> EPACT_W<DIEP3CTL_SPEC, 15> {
        EPACT_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DIEP3CTL_SPEC, 18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEP3CTL_SPEC, 21> {
        STALL_W::new(self)
    }
    #[doc = "Bits 22:25 - Tx FIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEP3CTL_SPEC, 22> {
        TXFNUM_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEP3CTL_SPEC, 26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEP3CTL_SPEC, 27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 28 - SD0PID/SEVNFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd0pid_sevenfrm(&mut self) -> SD0PID_SEVENFRM_W<DIEP3CTL_SPEC, 28> {
        SD0PID_SEVENFRM_W::new(self)
    }
    #[doc = "Bit 29 - Set DATA1 PID/Set odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn sd1pid_soddfrm(&mut self) -> SD1PID_SODDFRM_W<DIEP3CTL_SPEC, 29> {
        SD1PID_SODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn epd(&mut self) -> EPD_W<DIEP3CTL_SPEC, 30> {
        EPD_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<DIEP3CTL_SPEC, 31> {
        EPEN_W::new(self)
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
#[doc = "device endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP3CTL_SPEC;
impl crate::RegisterSpec for DIEP3CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep3ctl::R`](R) reader structure"]
impl crate::Readable for DIEP3CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep3ctl::W`](W) writer structure"]
impl crate::Writable for DIEP3CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP3CTL to value 0"]
impl crate::Resettable for DIEP3CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
