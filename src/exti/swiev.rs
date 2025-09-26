#[doc = "Register `SWIEV` reader"]
pub type R = crate::R<SWIEV_SPEC>;
#[doc = "Register `SWIEV` writer"]
pub type W = crate::W<SWIEV_SPEC>;
#[doc = "Field `SWIEV0` reader - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_R = crate::BitReader;
#[doc = "Field `SWIEV0` writer - Interrupt/Event software trigger on line 0"]
pub type SWIEV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV1` reader - Interrupt/Event software trigger on line 1"]
pub type SWIEV1_R = crate::BitReader;
#[doc = "Field `SWIEV1` writer - Interrupt/Event software trigger on line 1"]
pub type SWIEV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV2` reader - Interrupt/Event software trigger on line 2"]
pub type SWIEV2_R = crate::BitReader;
#[doc = "Field `SWIEV2` writer - Interrupt/Event software trigger on line 2"]
pub type SWIEV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV3` reader - Interrupt/Event software trigger on line 3"]
pub type SWIEV3_R = crate::BitReader;
#[doc = "Field `SWIEV3` writer - Interrupt/Event software trigger on line 3"]
pub type SWIEV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV4` reader - Interrupt/Event software trigger on line 4"]
pub type SWIEV4_R = crate::BitReader;
#[doc = "Field `SWIEV4` writer - Interrupt/Event software trigger on line 4"]
pub type SWIEV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV5` reader - Interrupt/Event software trigger on line 5"]
pub type SWIEV5_R = crate::BitReader;
#[doc = "Field `SWIEV5` writer - Interrupt/Event software trigger on line 5"]
pub type SWIEV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV6` reader - Interrupt/Event software trigger on line 6"]
pub type SWIEV6_R = crate::BitReader;
#[doc = "Field `SWIEV6` writer - Interrupt/Event software trigger on line 6"]
pub type SWIEV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV7` reader - Interrupt/Event software trigger on line 7"]
pub type SWIEV7_R = crate::BitReader;
#[doc = "Field `SWIEV7` writer - Interrupt/Event software trigger on line 7"]
pub type SWIEV7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV8` reader - Interrupt/Event software trigger on line 8"]
pub type SWIEV8_R = crate::BitReader;
#[doc = "Field `SWIEV8` writer - Interrupt/Event software trigger on line 8"]
pub type SWIEV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV9` reader - Interrupt/Event software trigger on line 9"]
pub type SWIEV9_R = crate::BitReader;
#[doc = "Field `SWIEV9` writer - Interrupt/Event software trigger on line 9"]
pub type SWIEV9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV10` reader - Interrupt/Event software trigger on line 10"]
pub type SWIEV10_R = crate::BitReader;
#[doc = "Field `SWIEV10` writer - Interrupt/Event software trigger on line 10"]
pub type SWIEV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV11` reader - Interrupt/Event software trigger on line 11"]
pub type SWIEV11_R = crate::BitReader;
#[doc = "Field `SWIEV11` writer - Interrupt/Event software trigger on line 11"]
pub type SWIEV11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV12` reader - Interrupt/Event software trigger on line 12"]
pub type SWIEV12_R = crate::BitReader;
#[doc = "Field `SWIEV12` writer - Interrupt/Event software trigger on line 12"]
pub type SWIEV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV13` reader - Interrupt/Event software trigger on line 13"]
pub type SWIEV13_R = crate::BitReader;
#[doc = "Field `SWIEV13` writer - Interrupt/Event software trigger on line 13"]
pub type SWIEV13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV14` reader - Interrupt/Event software trigger on line 14"]
pub type SWIEV14_R = crate::BitReader;
#[doc = "Field `SWIEV14` writer - Interrupt/Event software trigger on line 14"]
pub type SWIEV14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV15` reader - Interrupt/Event software trigger on line 15"]
pub type SWIEV15_R = crate::BitReader;
#[doc = "Field `SWIEV15` writer - Interrupt/Event software trigger on line 15"]
pub type SWIEV15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV16` reader - Interrupt/Event software trigger on line 16"]
pub type SWIEV16_R = crate::BitReader;
#[doc = "Field `SWIEV16` writer - Interrupt/Event software trigger on line 16"]
pub type SWIEV16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV17` reader - Interrupt/Event software trigger on line 17"]
pub type SWIEV17_R = crate::BitReader;
#[doc = "Field `SWIEV17` writer - Interrupt/Event software trigger on line 17"]
pub type SWIEV17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIEV18` reader - Interrupt/Event software trigger on line 18"]
pub type SWIEV18_R = crate::BitReader;
#[doc = "Field `SWIEV18` writer - Interrupt/Event software trigger on line 18"]
pub type SWIEV18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    pub fn swiev0(&self) -> SWIEV0_R {
        SWIEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    pub fn swiev1(&self) -> SWIEV1_R {
        SWIEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    pub fn swiev2(&self) -> SWIEV2_R {
        SWIEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    pub fn swiev3(&self) -> SWIEV3_R {
        SWIEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    pub fn swiev4(&self) -> SWIEV4_R {
        SWIEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    pub fn swiev5(&self) -> SWIEV5_R {
        SWIEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    pub fn swiev6(&self) -> SWIEV6_R {
        SWIEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    pub fn swiev7(&self) -> SWIEV7_R {
        SWIEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    pub fn swiev8(&self) -> SWIEV8_R {
        SWIEV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    pub fn swiev9(&self) -> SWIEV9_R {
        SWIEV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    pub fn swiev10(&self) -> SWIEV10_R {
        SWIEV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    pub fn swiev11(&self) -> SWIEV11_R {
        SWIEV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    pub fn swiev12(&self) -> SWIEV12_R {
        SWIEV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    pub fn swiev13(&self) -> SWIEV13_R {
        SWIEV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    pub fn swiev14(&self) -> SWIEV14_R {
        SWIEV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    pub fn swiev15(&self) -> SWIEV15_R {
        SWIEV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    pub fn swiev16(&self) -> SWIEV16_R {
        SWIEV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    pub fn swiev17(&self) -> SWIEV17_R {
        SWIEV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    pub fn swiev18(&self) -> SWIEV18_R {
        SWIEV18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swiev0(&mut self) -> SWIEV0_W<SWIEV_SPEC> {
        SWIEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swiev1(&mut self) -> SWIEV1_W<SWIEV_SPEC> {
        SWIEV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swiev2(&mut self) -> SWIEV2_W<SWIEV_SPEC> {
        SWIEV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swiev3(&mut self) -> SWIEV3_W<SWIEV_SPEC> {
        SWIEV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swiev4(&mut self) -> SWIEV4_W<SWIEV_SPEC> {
        SWIEV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swiev5(&mut self) -> SWIEV5_W<SWIEV_SPEC> {
        SWIEV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swiev6(&mut self) -> SWIEV6_W<SWIEV_SPEC> {
        SWIEV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swiev7(&mut self) -> SWIEV7_W<SWIEV_SPEC> {
        SWIEV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swiev8(&mut self) -> SWIEV8_W<SWIEV_SPEC> {
        SWIEV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swiev9(&mut self) -> SWIEV9_W<SWIEV_SPEC> {
        SWIEV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swiev10(&mut self) -> SWIEV10_W<SWIEV_SPEC> {
        SWIEV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swiev11(&mut self) -> SWIEV11_W<SWIEV_SPEC> {
        SWIEV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swiev12(&mut self) -> SWIEV12_W<SWIEV_SPEC> {
        SWIEV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swiev13(&mut self) -> SWIEV13_W<SWIEV_SPEC> {
        SWIEV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swiev14(&mut self) -> SWIEV14_W<SWIEV_SPEC> {
        SWIEV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swiev15(&mut self) -> SWIEV15_W<SWIEV_SPEC> {
        SWIEV15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swiev16(&mut self) -> SWIEV16_W<SWIEV_SPEC> {
        SWIEV16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swiev17(&mut self) -> SWIEV17_W<SWIEV_SPEC> {
        SWIEV17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn swiev18(&mut self) -> SWIEV18_W<SWIEV_SPEC> {
        SWIEV18_W::new(self, 18)
    }
}
#[doc = "Software interrupt event register (EXTI_SWIEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`swiev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swiev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIEV_SPEC;
impl crate::RegisterSpec for SWIEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swiev::R`](R) reader structure"]
impl crate::Readable for SWIEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swiev::W`](W) writer structure"]
impl crate::Writable for SWIEV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIEV to value 0"]
impl crate::Resettable for SWIEV_SPEC {
    const RESET_VALUE: u32 = 0;
}
