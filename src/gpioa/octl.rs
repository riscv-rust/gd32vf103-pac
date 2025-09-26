#[doc = "Register `OCTL` reader"]
pub type R = crate::R<OCTL_SPEC>;
#[doc = "Register `OCTL` writer"]
pub type W = crate::W<OCTL_SPEC>;
#[doc = "Field `OCTL0` reader - Port output control"]
pub type OCTL0_R = crate::BitReader;
#[doc = "Field `OCTL0` writer - Port output control"]
pub type OCTL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL1` reader - Port output control"]
pub type OCTL1_R = crate::BitReader;
#[doc = "Field `OCTL1` writer - Port output control"]
pub type OCTL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL2` reader - Port output control"]
pub type OCTL2_R = crate::BitReader;
#[doc = "Field `OCTL2` writer - Port output control"]
pub type OCTL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL3` reader - Port output control"]
pub type OCTL3_R = crate::BitReader;
#[doc = "Field `OCTL3` writer - Port output control"]
pub type OCTL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL4` reader - Port output control"]
pub type OCTL4_R = crate::BitReader;
#[doc = "Field `OCTL4` writer - Port output control"]
pub type OCTL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL5` reader - Port output control"]
pub type OCTL5_R = crate::BitReader;
#[doc = "Field `OCTL5` writer - Port output control"]
pub type OCTL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL6` reader - Port output control"]
pub type OCTL6_R = crate::BitReader;
#[doc = "Field `OCTL6` writer - Port output control"]
pub type OCTL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL7` reader - Port output control"]
pub type OCTL7_R = crate::BitReader;
#[doc = "Field `OCTL7` writer - Port output control"]
pub type OCTL7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL8` reader - Port output control"]
pub type OCTL8_R = crate::BitReader;
#[doc = "Field `OCTL8` writer - Port output control"]
pub type OCTL8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL9` reader - Port output control"]
pub type OCTL9_R = crate::BitReader;
#[doc = "Field `OCTL9` writer - Port output control"]
pub type OCTL9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL10` reader - Port output control"]
pub type OCTL10_R = crate::BitReader;
#[doc = "Field `OCTL10` writer - Port output control"]
pub type OCTL10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL11` reader - Port output control"]
pub type OCTL11_R = crate::BitReader;
#[doc = "Field `OCTL11` writer - Port output control"]
pub type OCTL11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL12` reader - Port output control"]
pub type OCTL12_R = crate::BitReader;
#[doc = "Field `OCTL12` writer - Port output control"]
pub type OCTL12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL13` reader - Port output control"]
pub type OCTL13_R = crate::BitReader;
#[doc = "Field `OCTL13` writer - Port output control"]
pub type OCTL13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL14` reader - Port output control"]
pub type OCTL14_R = crate::BitReader;
#[doc = "Field `OCTL14` writer - Port output control"]
pub type OCTL14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL15` reader - Port output control"]
pub type OCTL15_R = crate::BitReader;
#[doc = "Field `OCTL15` writer - Port output control"]
pub type OCTL15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    pub fn octl0(&self) -> OCTL0_R {
        OCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    pub fn octl1(&self) -> OCTL1_R {
        OCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    pub fn octl2(&self) -> OCTL2_R {
        OCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    pub fn octl3(&self) -> OCTL3_R {
        OCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    pub fn octl4(&self) -> OCTL4_R {
        OCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    pub fn octl5(&self) -> OCTL5_R {
        OCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    pub fn octl6(&self) -> OCTL6_R {
        OCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    pub fn octl7(&self) -> OCTL7_R {
        OCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    pub fn octl8(&self) -> OCTL8_R {
        OCTL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    pub fn octl9(&self) -> OCTL9_R {
        OCTL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    pub fn octl10(&self) -> OCTL10_R {
        OCTL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    pub fn octl11(&self) -> OCTL11_R {
        OCTL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    pub fn octl12(&self) -> OCTL12_R {
        OCTL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    pub fn octl13(&self) -> OCTL13_R {
        OCTL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    pub fn octl14(&self) -> OCTL14_R {
        OCTL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    pub fn octl15(&self) -> OCTL15_R {
        OCTL15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl0(&mut self) -> OCTL0_W<OCTL_SPEC> {
        OCTL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl1(&mut self) -> OCTL1_W<OCTL_SPEC> {
        OCTL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl2(&mut self) -> OCTL2_W<OCTL_SPEC> {
        OCTL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl3(&mut self) -> OCTL3_W<OCTL_SPEC> {
        OCTL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl4(&mut self) -> OCTL4_W<OCTL_SPEC> {
        OCTL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl5(&mut self) -> OCTL5_W<OCTL_SPEC> {
        OCTL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl6(&mut self) -> OCTL6_W<OCTL_SPEC> {
        OCTL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl7(&mut self) -> OCTL7_W<OCTL_SPEC> {
        OCTL7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl8(&mut self) -> OCTL8_W<OCTL_SPEC> {
        OCTL8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl9(&mut self) -> OCTL9_W<OCTL_SPEC> {
        OCTL9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl10(&mut self) -> OCTL10_W<OCTL_SPEC> {
        OCTL10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl11(&mut self) -> OCTL11_W<OCTL_SPEC> {
        OCTL11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl12(&mut self) -> OCTL12_W<OCTL_SPEC> {
        OCTL12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl13(&mut self) -> OCTL13_W<OCTL_SPEC> {
        OCTL13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl14(&mut self) -> OCTL14_W<OCTL_SPEC> {
        OCTL14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    #[must_use]
    pub fn octl15(&mut self) -> OCTL15_W<OCTL_SPEC> {
        OCTL15_W::new(self, 15)
    }
}
#[doc = "Port output control register\n\nYou can [`read`](crate::Reg::read) this register and get [`octl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl::R`](R) reader structure"]
impl crate::Readable for OCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`octl::W`](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
