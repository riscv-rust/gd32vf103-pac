#[doc = "Register `DOEP2INTF` reader"]
pub type R = crate::R<DOEP2INTF_SPEC>;
#[doc = "Register `DOEP2INTF` writer"]
pub type W = crate::W<DOEP2INTF_SPEC>;
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disabled"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint disabled"]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPF` reader - Setup phase finished"]
pub type STPF_R = crate::BitReader;
#[doc = "Field `STPF` writer - Setup phase finished"]
pub type STPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXFOVR` reader - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_R = crate::BitReader;
#[doc = "Field `EPRXFOVR` writer - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTBSTP` reader - Back-to-back SETUP packets"]
pub type BTBSTP_R = crate::BitReader;
#[doc = "Field `BTBSTP` writer - Back-to-back SETUP packets"]
pub type BTBSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    pub fn stpf(&self) -> STPF_R {
        STPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    pub fn eprxfovr(&self) -> EPRXFOVR_R {
        EPRXFOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    pub fn btbstp(&self) -> BTBSTP_R {
        BTBSTP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<DOEP2INTF_SPEC> {
        TF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DOEP2INTF_SPEC> {
        EPDIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    #[must_use]
    pub fn stpf(&mut self) -> STPF_W<DOEP2INTF_SPEC> {
        STPF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovr(&mut self) -> EPRXFOVR_W<DOEP2INTF_SPEC> {
        EPRXFOVR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    #[must_use]
    pub fn btbstp(&mut self) -> BTBSTP_W<DOEP2INTF_SPEC> {
        BTBSTP_W::new(self, 6)
    }
}
#[doc = "device out endpoint-2 interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEP2INTF_SPEC;
impl crate::RegisterSpec for DOEP2INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2intf::R`](R) reader structure"]
impl crate::Readable for DOEP2INTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doep2intf::W`](W) writer structure"]
impl crate::Writable for DOEP2INTF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEP2INTF to value 0"]
impl crate::Resettable for DOEP2INTF_SPEC {
    const RESET_VALUE: u32 = 0;
}
