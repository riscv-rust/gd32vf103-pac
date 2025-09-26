#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `WDE` reader - Analog watchdog event flag"]
pub type WDE_R = crate::BitReader;
#[doc = "Field `WDE` writer - Analog watchdog event flag"]
pub type WDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - End of group conversion flag"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - End of group conversion flag"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIC` reader - End of inserted group conversion flag"]
pub type EOIC_R = crate::BitReader;
#[doc = "Field `EOIC` writer - End of inserted group conversion flag"]
pub type EOIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIC` reader - Start flag of inserted channel group"]
pub type STIC_R = crate::BitReader;
#[doc = "Field `STIC` writer - Start flag of inserted channel group"]
pub type STIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRC` reader - Start flag of regular channel group"]
pub type STRC_R = crate::BitReader;
#[doc = "Field `STRC` writer - Start flag of regular channel group"]
pub type STRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    pub fn eoic(&self) -> EOIC_R {
        EOIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    pub fn stic(&self) -> STIC_R {
        STIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    pub fn strc(&self) -> STRC_R {
        STRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog event flag"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<STAT_SPEC> {
        WDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<STAT_SPEC> {
        EOC_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of inserted group conversion flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoic(&mut self) -> EOIC_W<STAT_SPEC> {
        EOIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start flag of inserted channel group"]
    #[inline(always)]
    #[must_use]
    pub fn stic(&mut self) -> STIC_W<STAT_SPEC> {
        STIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Start flag of regular channel group"]
    #[inline(always)]
    #[must_use]
    pub fn strc(&mut self) -> STRC_W<STAT_SPEC> {
        STRC_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
