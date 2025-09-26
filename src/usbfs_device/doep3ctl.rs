#[doc = "Register `DOEP3CTL` reader"]
pub type R = crate::R<DOEP3CTL_SPEC>;
#[doc = "Register `DOEP3CTL` writer"]
pub type W = crate::W<DOEP3CTL_SPEC>;
#[doc = "Field `MPL` reader - maximum packet length"]
pub type MPL_R = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - maximum packet length"]
pub type MPL_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPACT` reader - Endpoint active"]
pub type EPACT_R = crate::BitReader;
#[doc = "Field `EPACT` writer - Endpoint active"]
pub type EPACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOFRM_DPID` reader - EOFRM/DPID"]
pub type EOFRM_DPID_R = crate::BitReader;
#[doc = "Field `NAKS` reader - NAK status"]
pub type NAKS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNOOP` reader - Snoop mode"]
pub type SNOOP_R = crate::BitReader;
#[doc = "Field `SNOOP` writer - Snoop mode"]
pub type SNOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID_SEVENFRM` writer - SD0PID/SEVENFRM"]
pub type SD0PID_SEVENFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1PID_SODDFRM` writer - SD1PID/SODDFRM"]
pub type SD1PID_SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPD` reader - Endpoint disable"]
pub type EPD_R = crate::BitReader;
#[doc = "Field `EPD` writer - Endpoint disable"]
pub type EPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEN` reader - Endpoint enable"]
pub type EPEN_R = crate::BitReader;
#[doc = "Field `EPEN` writer - Endpoint enable"]
pub type EPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snoop(&self) -> SNOOP_R {
        SNOOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
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
    pub fn mpl(&mut self) -> MPL_W<DOEP3CTL_SPEC> {
        MPL_W::new(self, 0)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    #[must_use]
    pub fn epact(&mut self) -> EPACT_W<DOEP3CTL_SPEC> {
        EPACT_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DOEP3CTL_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snoop(&mut self) -> SNOOP_W<DOEP3CTL_SPEC> {
        SNOOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEP3CTL_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEP3CTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEP3CTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - SD0PID/SEVENFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd0pid_sevenfrm(&mut self) -> SD0PID_SEVENFRM_W<DOEP3CTL_SPEC> {
        SD0PID_SEVENFRM_W::new(self, 28)
    }
    #[doc = "Bit 29 - SD1PID/SODDFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd1pid_soddfrm(&mut self) -> SD1PID_SODDFRM_W<DOEP3CTL_SPEC> {
        SD1PID_SODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn epd(&mut self) -> EPD_W<DOEP3CTL_SPEC> {
        EPD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<DOEP3CTL_SPEC> {
        EPEN_W::new(self, 31)
    }
}
#[doc = "device endpoint-3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP3CTL_SPEC;
impl crate::RegisterSpec for DOEP3CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep3ctl::R`](R) reader structure"]
impl crate::Readable for DOEP3CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep3ctl::W`](W) writer structure"]
impl crate::Writable for DOEP3CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP3CTL to value 0"]
impl crate::Resettable for DOEP3CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
