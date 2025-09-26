#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `IWS` reader - Initial working state"]
pub type IWS_R = crate::BitReader;
#[doc = "Field `SLPWS` reader - Sleep working state"]
pub type SLPWS_R = crate::BitReader;
#[doc = "Field `ERRIF` reader - Error interrupt flag"]
pub type ERRIF_R = crate::BitReader;
#[doc = "Field `ERRIF` writer - Error interrupt flag"]
pub type ERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIF` reader - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_R = crate::BitReader;
#[doc = "Field `WUIF` writer - Status change interrupt flag of wakeup from sleep working mode"]
pub type WUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPIF` reader - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_R = crate::BitReader;
#[doc = "Field `SLPIF` writer - Status change interrupt flag of sleep working mode entering"]
pub type SLPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - Transmitting state"]
pub type TS_R = crate::BitReader;
#[doc = "Field `RS` reader - Receiving state"]
pub type RS_R = crate::BitReader;
#[doc = "Field `LASTRX` reader - Last sample value of RX pin"]
pub type LASTRX_R = crate::BitReader;
#[doc = "Field `RXL` reader - RX level"]
pub type RXL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Initial working state"]
    #[inline(always)]
    pub fn iws(&self) -> IWS_R {
        IWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working state"]
    #[inline(always)]
    pub fn slpws(&self) -> SLPWS_R {
        SLPWS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn errif(&self) -> ERRIF_R {
        ERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    pub fn slpif(&self) -> SLPIF_R {
        SLPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitting state"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiving state"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample value of RX pin"]
    #[inline(always)]
    pub fn lastrx(&self) -> LASTRX_R {
        LASTRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX level"]
    #[inline(always)]
    pub fn rxl(&self) -> RXL_R {
        RXL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn errif(&mut self) -> ERRIF_W<STAT_SPEC> {
        ERRIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Status change interrupt flag of wakeup from sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn wuif(&mut self) -> WUIF_W<STAT_SPEC> {
        WUIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Status change interrupt flag of sleep working mode entering"]
    #[inline(always)]
    #[must_use]
    pub fn slpif(&mut self) -> SLPIF_W<STAT_SPEC> {
        SLPIF_W::new(self, 4)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0c02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0x0c02;
}
