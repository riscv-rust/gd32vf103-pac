#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `IWMOD` reader - Initial working mode"]
pub type IWMOD_R = crate::BitReader;
#[doc = "Field `IWMOD` writer - Initial working mode"]
pub type IWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPWMOD` reader - Sleep working mode"]
pub type SLPWMOD_R = crate::BitReader;
#[doc = "Field `SLPWMOD` writer - Sleep working mode"]
pub type SLPWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFO` reader - Transmit FIFO order"]
pub type TFO_R = crate::BitReader;
#[doc = "Field `TFO` writer - Transmit FIFO order"]
pub type TFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOD` reader - Receive FIFO overwrite disable"]
pub type RFOD_R = crate::BitReader;
#[doc = "Field `RFOD` writer - Receive FIFO overwrite disable"]
pub type RFOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARD` reader - Automatic retransmission disable"]
pub type ARD_R = crate::BitReader;
#[doc = "Field `ARD` writer - Automatic retransmission disable"]
pub type ARD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWU` reader - Automatic wakeup"]
pub type AWU_R = crate::BitReader;
#[doc = "Field `AWU` writer - Automatic wakeup"]
pub type AWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABOR` reader - Automatic bus-off recovery"]
pub type ABOR_R = crate::BitReader;
#[doc = "Field `ABOR` writer - Automatic bus-off recovery"]
pub type ABOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Time-triggered communication"]
pub type TTC_R = crate::BitReader;
#[doc = "Field `TTC` writer - Time-triggered communication"]
pub type TTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFZ` reader - Debug freeze"]
pub type DFZ_R = crate::BitReader;
#[doc = "Field `DFZ` writer - Debug freeze"]
pub type DFZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    pub fn iwmod(&self) -> IWMOD_R {
        IWMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    pub fn slpwmod(&self) -> SLPWMOD_R {
        SLPWMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    pub fn tfo(&self) -> TFO_R {
        TFO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    pub fn rfod(&self) -> RFOD_R {
        RFOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    pub fn ard(&self) -> ARD_R {
        ARD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    pub fn awu(&self) -> AWU_R {
        AWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    pub fn abor(&self) -> ABOR_R {
        ABOR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dfz(&self) -> DFZ_R {
        DFZ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwmod(&mut self) -> IWMOD_W<CTL_SPEC> {
        IWMOD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn slpwmod(&mut self) -> SLPWMOD_W<CTL_SPEC> {
        SLPWMOD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    #[must_use]
    pub fn tfo(&mut self) -> TFO_W<CTL_SPEC> {
        TFO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    #[must_use]
    pub fn rfod(&mut self) -> RFOD_W<CTL_SPEC> {
        RFOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    #[must_use]
    pub fn ard(&mut self) -> ARD_W<CTL_SPEC> {
        ARD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn awu(&mut self) -> AWU_W<CTL_SPEC> {
        AWU_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    #[must_use]
    pub fn abor(&mut self) -> ABOR_W<CTL_SPEC> {
        ABOR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<CTL_SPEC> {
        TTC_W::new(self, 7)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTL_SPEC> {
        SWRST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    #[must_use]
    pub fn dfz(&mut self) -> DFZ_W<CTL_SPEC> {
        DFZ_W::new(self, 16)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0001_0002"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0x0001_0002;
}
