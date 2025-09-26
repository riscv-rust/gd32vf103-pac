#[doc = "Register `SWEVG` writer"]
pub type W = crate::W<SWEVG_SPEC>;
#[doc = "Field `UPG` writer - Update event generation"]
pub type UPG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type CH0G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub type CH1G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub type CH2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub type CH3G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CMTG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TRGG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BRKG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update event generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UPG_W<SWEVG_SPEC> {
        UPG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> CH0G_W<SWEVG_SPEC> {
        CH0G_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1g(&mut self) -> CH1G_W<SWEVG_SPEC> {
        CH1G_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2g(&mut self) -> CH2G_W<SWEVG_SPEC> {
        CH2G_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3g(&mut self) -> CH3G_W<SWEVG_SPEC> {
        CH3G_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CMTG_W<SWEVG_SPEC> {
        CMTG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    #[must_use]
    pub fn trgg(&mut self) -> TRGG_W<SWEVG_SPEC> {
        TRGG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BRKG_W<SWEVG_SPEC> {
        BRKG_W::new(self, 7)
    }
}
#[doc = "Software event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`swevg::W`](W) writer structure"]
impl crate::Writable for SWEVG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SWEVG_SPEC {
    const RESET_VALUE: u16 = 0;
}
