#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `LK0` reader - Port Lock bit 0"]
pub type LK0_R = crate::BitReader;
#[doc = "Field `LK0` writer - Port Lock bit 0"]
pub type LK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK1` reader - Port Lock bit 1"]
pub type LK1_R = crate::BitReader;
#[doc = "Field `LK1` writer - Port Lock bit 1"]
pub type LK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK2` reader - Port Lock bit 2"]
pub type LK2_R = crate::BitReader;
#[doc = "Field `LK2` writer - Port Lock bit 2"]
pub type LK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK3` reader - Port Lock bit 3"]
pub type LK3_R = crate::BitReader;
#[doc = "Field `LK3` writer - Port Lock bit 3"]
pub type LK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK4` reader - Port Lock bit 4"]
pub type LK4_R = crate::BitReader;
#[doc = "Field `LK4` writer - Port Lock bit 4"]
pub type LK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK5` reader - Port Lock bit 5"]
pub type LK5_R = crate::BitReader;
#[doc = "Field `LK5` writer - Port Lock bit 5"]
pub type LK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK6` reader - Port Lock bit 6"]
pub type LK6_R = crate::BitReader;
#[doc = "Field `LK6` writer - Port Lock bit 6"]
pub type LK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK7` reader - Port Lock bit 7"]
pub type LK7_R = crate::BitReader;
#[doc = "Field `LK7` writer - Port Lock bit 7"]
pub type LK7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK8` reader - Port Lock bit 8"]
pub type LK8_R = crate::BitReader;
#[doc = "Field `LK8` writer - Port Lock bit 8"]
pub type LK8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK9` reader - Port Lock bit 9"]
pub type LK9_R = crate::BitReader;
#[doc = "Field `LK9` writer - Port Lock bit 9"]
pub type LK9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK10` reader - Port Lock bit 10"]
pub type LK10_R = crate::BitReader;
#[doc = "Field `LK10` writer - Port Lock bit 10"]
pub type LK10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK11` reader - Port Lock bit 11"]
pub type LK11_R = crate::BitReader;
#[doc = "Field `LK11` writer - Port Lock bit 11"]
pub type LK11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK12` reader - Port Lock bit 12"]
pub type LK12_R = crate::BitReader;
#[doc = "Field `LK12` writer - Port Lock bit 12"]
pub type LK12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK13` reader - Port Lock bit 13"]
pub type LK13_R = crate::BitReader;
#[doc = "Field `LK13` writer - Port Lock bit 13"]
pub type LK13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK14` reader - Port Lock bit 14"]
pub type LK14_R = crate::BitReader;
#[doc = "Field `LK14` writer - Port Lock bit 14"]
pub type LK14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK15` reader - Port Lock bit 15"]
pub type LK15_R = crate::BitReader;
#[doc = "Field `LK15` writer - Port Lock bit 15"]
pub type LK15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKK` reader - Lock sequence key"]
pub type LKK_R = crate::BitReader;
#[doc = "Field `LKK` writer - Lock sequence key"]
pub type LKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    pub fn lk0(&self) -> LK0_R {
        LK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    pub fn lk1(&self) -> LK1_R {
        LK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    pub fn lk2(&self) -> LK2_R {
        LK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    pub fn lk3(&self) -> LK3_R {
        LK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    pub fn lk4(&self) -> LK4_R {
        LK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    pub fn lk5(&self) -> LK5_R {
        LK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    pub fn lk6(&self) -> LK6_R {
        LK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    pub fn lk7(&self) -> LK7_R {
        LK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    pub fn lk8(&self) -> LK8_R {
        LK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    pub fn lk9(&self) -> LK9_R {
        LK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    pub fn lk10(&self) -> LK10_R {
        LK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    pub fn lk11(&self) -> LK11_R {
        LK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    pub fn lk12(&self) -> LK12_R {
        LK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    pub fn lk13(&self) -> LK13_R {
        LK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    pub fn lk14(&self) -> LK14_R {
        LK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    pub fn lk15(&self) -> LK15_R {
        LK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    pub fn lkk(&self) -> LKK_R {
        LKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Lock bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lk0(&mut self) -> LK0_W<LOCK_SPEC> {
        LK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Lock bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lk1(&mut self) -> LK1_W<LOCK_SPEC> {
        LK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Lock bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lk2(&mut self) -> LK2_W<LOCK_SPEC> {
        LK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Lock bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn lk3(&mut self) -> LK3_W<LOCK_SPEC> {
        LK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port Lock bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn lk4(&mut self) -> LK4_W<LOCK_SPEC> {
        LK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port Lock bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lk5(&mut self) -> LK5_W<LOCK_SPEC> {
        LK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Lock bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lk6(&mut self) -> LK6_W<LOCK_SPEC> {
        LK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Lock bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lk7(&mut self) -> LK7_W<LOCK_SPEC> {
        LK7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Lock bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lk8(&mut self) -> LK8_W<LOCK_SPEC> {
        LK8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port Lock bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lk9(&mut self) -> LK9_W<LOCK_SPEC> {
        LK9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port Lock bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn lk10(&mut self) -> LK10_W<LOCK_SPEC> {
        LK10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port Lock bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn lk11(&mut self) -> LK11_W<LOCK_SPEC> {
        LK11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port Lock bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn lk12(&mut self) -> LK12_W<LOCK_SPEC> {
        LK12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port Lock bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn lk13(&mut self) -> LK13_W<LOCK_SPEC> {
        LK13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port Lock bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn lk14(&mut self) -> LK14_W<LOCK_SPEC> {
        LK14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port Lock bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn lk15(&mut self) -> LK15_W<LOCK_SPEC> {
        LK15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Lock sequence key"]
    #[inline(always)]
    #[must_use]
    pub fn lkk(&mut self) -> LKK_W<LOCK_SPEC> {
        LKK_W::new(self, 16)
    }
}
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
