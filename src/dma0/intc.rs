#[doc = "Register `INTC` writer"]
pub type W = crate::W<INTC_SPEC>;
#[doc = "Field `GIFC0` writer - Clear global interrupt flag of channel 0"]
pub type GIFC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC0` writer - Clear bit for full transfer finish flag of channel 0"]
pub type FTFIFC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC0` writer - Clear bit for half transfer finish flag of channel 0"]
pub type HTFIFC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC0` writer - Clear bit for error flag of channel 0"]
pub type ERRIFC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC1` writer - Clear global interrupt flag of channel 1"]
pub type GIFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC1` writer - Clear bit for full transfer finish flag of channel 1"]
pub type FTFIFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC1` writer - Clear bit for half transfer finish flag of channel 1"]
pub type HTFIFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC1` writer - Clear bit for error flag of channel 1"]
pub type ERRIFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC2` writer - Clear global interrupt flag of channel 2"]
pub type GIFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC2` writer - Clear bit for full transfer finish flag of channel 2"]
pub type FTFIFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC2` writer - Clear bit for half transfer finish flag of channel 2"]
pub type HTFIFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC2` writer - Clear bit for error flag of channel 2"]
pub type ERRIFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC3` writer - Clear global interrupt flag of channel 3"]
pub type GIFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC3` writer - Clear bit for full transfer finish flag of channel 3"]
pub type FTFIFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC3` writer - Clear bit for half transfer finish flag of channel 3"]
pub type HTFIFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC3` writer - Clear bit for error flag of channel 3"]
pub type ERRIFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC4` writer - Clear global interrupt flag of channel 4"]
pub type GIFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC4` writer - Clear bit for full transfer finish flag of channel 4"]
pub type FTFIFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC4` writer - Clear bit for half transfer finish flag of channel 4"]
pub type HTFIFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC4` writer - Clear bit for error flag of channel 4"]
pub type ERRIFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC5` writer - Clear global interrupt flag of channel 5"]
pub type GIFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC5` writer - Clear bit for full transfer finish flag of channel 5"]
pub type FTFIFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC5` writer - Clear bit for half transfer finish flag of channel 5"]
pub type HTFIFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC5` writer - Clear bit for error flag of channel 5"]
pub type ERRIFC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC6` writer - Clear global interrupt flag of channel 6"]
pub type GIFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC6` writer - Clear bit for full transfer finish flag of channel 6"]
pub type FTFIFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC6` writer - Clear bit for half transfer finish flag of channel 6"]
pub type HTFIFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC6` writer - Clear bit for error flag of channel 6"]
pub type ERRIFC6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear global interrupt flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn gifc0(&mut self) -> GIFC0_W<INTC_SPEC> {
        GIFC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit for full transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc0(&mut self) -> FTFIFC0_W<INTC_SPEC> {
        FTFIFC0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit for half transfer finish flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc0(&mut self) -> HTFIFC0_W<INTC_SPEC> {
        HTFIFC0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for error flag of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn errifc0(&mut self) -> ERRIFC0_W<INTC_SPEC> {
        ERRIFC0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear global interrupt flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gifc1(&mut self) -> GIFC1_W<INTC_SPEC> {
        GIFC1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit for full transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc1(&mut self) -> FTFIFC1_W<INTC_SPEC> {
        FTFIFC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit for half transfer finish flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc1(&mut self) -> HTFIFC1_W<INTC_SPEC> {
        HTFIFC1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit for error flag of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn errifc1(&mut self) -> ERRIFC1_W<INTC_SPEC> {
        ERRIFC1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global interrupt flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gifc2(&mut self) -> GIFC2_W<INTC_SPEC> {
        GIFC2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit for full transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc2(&mut self) -> FTFIFC2_W<INTC_SPEC> {
        FTFIFC2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit for half transfer finish flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc2(&mut self) -> HTFIFC2_W<INTC_SPEC> {
        HTFIFC2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit for error flag of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn errifc2(&mut self) -> ERRIFC2_W<INTC_SPEC> {
        ERRIFC2_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear global interrupt flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gifc3(&mut self) -> GIFC3_W<INTC_SPEC> {
        GIFC3_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit for full transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc3(&mut self) -> FTFIFC3_W<INTC_SPEC> {
        FTFIFC3_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit for half transfer finish flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc3(&mut self) -> HTFIFC3_W<INTC_SPEC> {
        HTFIFC3_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit for error flag of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn errifc3(&mut self) -> ERRIFC3_W<INTC_SPEC> {
        ERRIFC3_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear global interrupt flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gifc4(&mut self) -> GIFC4_W<INTC_SPEC> {
        GIFC4_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bit for full transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc4(&mut self) -> FTFIFC4_W<INTC_SPEC> {
        FTFIFC4_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear bit for half transfer finish flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc4(&mut self) -> HTFIFC4_W<INTC_SPEC> {
        HTFIFC4_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit for error flag of channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn errifc4(&mut self) -> ERRIFC4_W<INTC_SPEC> {
        ERRIFC4_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear global interrupt flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gifc5(&mut self) -> GIFC5_W<INTC_SPEC> {
        GIFC5_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit for full transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc5(&mut self) -> FTFIFC5_W<INTC_SPEC> {
        FTFIFC5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit for half transfer finish flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc5(&mut self) -> HTFIFC5_W<INTC_SPEC> {
        HTFIFC5_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear bit for error flag of channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn errifc5(&mut self) -> ERRIFC5_W<INTC_SPEC> {
        ERRIFC5_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear global interrupt flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gifc6(&mut self) -> GIFC6_W<INTC_SPEC> {
        GIFC6_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit for full transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ftfifc6(&mut self) -> FTFIFC6_W<INTC_SPEC> {
        FTFIFC6_W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit for half transfer finish flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htfifc6(&mut self) -> HTFIFC6_W<INTC_SPEC> {
        HTFIFC6_W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit for error flag of channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn errifc6(&mut self) -> ERRIFC6_W<INTC_SPEC> {
        ERRIFC6_W::new(self, 27)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTC_SPEC;
impl crate::RegisterSpec for INTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for INTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for INTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
