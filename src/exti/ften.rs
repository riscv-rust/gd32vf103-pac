#[doc = "Register `FTEN` reader"]
pub type R = crate::R<FTEN_SPEC>;
#[doc = "Register `FTEN` writer"]
pub type W = crate::W<FTEN_SPEC>;
#[doc = "Field `FTEN0` reader - Falling edge trigger enable of line 0"]
pub type FTEN0_R = crate::BitReader;
#[doc = "Field `FTEN0` writer - Falling edge trigger enable of line 0"]
pub type FTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN1` reader - Falling edge trigger enable of line 1"]
pub type FTEN1_R = crate::BitReader;
#[doc = "Field `FTEN1` writer - Falling edge trigger enable of line 1"]
pub type FTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN2` reader - Falling edge trigger enable of line 2"]
pub type FTEN2_R = crate::BitReader;
#[doc = "Field `FTEN2` writer - Falling edge trigger enable of line 2"]
pub type FTEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN3` reader - Falling edge trigger enable of line 3"]
pub type FTEN3_R = crate::BitReader;
#[doc = "Field `FTEN3` writer - Falling edge trigger enable of line 3"]
pub type FTEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN4` reader - Falling edge trigger enable of line 4"]
pub type FTEN4_R = crate::BitReader;
#[doc = "Field `FTEN4` writer - Falling edge trigger enable of line 4"]
pub type FTEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN5` reader - Falling edge trigger enable of line 5"]
pub type FTEN5_R = crate::BitReader;
#[doc = "Field `FTEN5` writer - Falling edge trigger enable of line 5"]
pub type FTEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN6` reader - Falling edge trigger enable of line 6"]
pub type FTEN6_R = crate::BitReader;
#[doc = "Field `FTEN6` writer - Falling edge trigger enable of line 6"]
pub type FTEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN7` reader - Falling edge trigger enable of line 7"]
pub type FTEN7_R = crate::BitReader;
#[doc = "Field `FTEN7` writer - Falling edge trigger enable of line 7"]
pub type FTEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN8` reader - Falling edge trigger enable of line 8"]
pub type FTEN8_R = crate::BitReader;
#[doc = "Field `FTEN8` writer - Falling edge trigger enable of line 8"]
pub type FTEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN9` reader - Falling edge trigger enable of line 9"]
pub type FTEN9_R = crate::BitReader;
#[doc = "Field `FTEN9` writer - Falling edge trigger enable of line 9"]
pub type FTEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN10` reader - Falling edge trigger enable of line 10"]
pub type FTEN10_R = crate::BitReader;
#[doc = "Field `FTEN10` writer - Falling edge trigger enable of line 10"]
pub type FTEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN11` reader - Falling edge trigger enable of line 11"]
pub type FTEN11_R = crate::BitReader;
#[doc = "Field `FTEN11` writer - Falling edge trigger enable of line 11"]
pub type FTEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN12` reader - Falling edge trigger enable of line 12"]
pub type FTEN12_R = crate::BitReader;
#[doc = "Field `FTEN12` writer - Falling edge trigger enable of line 12"]
pub type FTEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN13` reader - Falling edge trigger enable of line 13"]
pub type FTEN13_R = crate::BitReader;
#[doc = "Field `FTEN13` writer - Falling edge trigger enable of line 13"]
pub type FTEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN14` reader - Falling edge trigger enable of line 14"]
pub type FTEN14_R = crate::BitReader;
#[doc = "Field `FTEN14` writer - Falling edge trigger enable of line 14"]
pub type FTEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN15` reader - Falling edge trigger enable of line 15"]
pub type FTEN15_R = crate::BitReader;
#[doc = "Field `FTEN15` writer - Falling edge trigger enable of line 15"]
pub type FTEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN16` reader - Falling edge trigger enable of line 16"]
pub type FTEN16_R = crate::BitReader;
#[doc = "Field `FTEN16` writer - Falling edge trigger enable of line 16"]
pub type FTEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN17` reader - Falling edge trigger enable of line 17"]
pub type FTEN17_R = crate::BitReader;
#[doc = "Field `FTEN17` writer - Falling edge trigger enable of line 17"]
pub type FTEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTEN18` reader - Falling edge trigger enable of line 18"]
pub type FTEN18_R = crate::BitReader;
#[doc = "Field `FTEN18` writer - Falling edge trigger enable of line 18"]
pub type FTEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    pub fn ften0(&self) -> FTEN0_R {
        FTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    pub fn ften1(&self) -> FTEN1_R {
        FTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    pub fn ften2(&self) -> FTEN2_R {
        FTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    pub fn ften3(&self) -> FTEN3_R {
        FTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    pub fn ften4(&self) -> FTEN4_R {
        FTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    pub fn ften5(&self) -> FTEN5_R {
        FTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    pub fn ften6(&self) -> FTEN6_R {
        FTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    pub fn ften7(&self) -> FTEN7_R {
        FTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    pub fn ften8(&self) -> FTEN8_R {
        FTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    pub fn ften9(&self) -> FTEN9_R {
        FTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    pub fn ften10(&self) -> FTEN10_R {
        FTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    pub fn ften11(&self) -> FTEN11_R {
        FTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    pub fn ften12(&self) -> FTEN12_R {
        FTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    pub fn ften13(&self) -> FTEN13_R {
        FTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    pub fn ften14(&self) -> FTEN14_R {
        FTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    pub fn ften15(&self) -> FTEN15_R {
        FTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    pub fn ften16(&self) -> FTEN16_R {
        FTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    pub fn ften17(&self) -> FTEN17_R {
        FTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    pub fn ften18(&self) -> FTEN18_R {
        FTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn ften0(&mut self) -> FTEN0_W<FTEN_SPEC> {
        FTEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn ften1(&mut self) -> FTEN1_W<FTEN_SPEC> {
        FTEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn ften2(&mut self) -> FTEN2_W<FTEN_SPEC> {
        FTEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn ften3(&mut self) -> FTEN3_W<FTEN_SPEC> {
        FTEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn ften4(&mut self) -> FTEN4_W<FTEN_SPEC> {
        FTEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn ften5(&mut self) -> FTEN5_W<FTEN_SPEC> {
        FTEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn ften6(&mut self) -> FTEN6_W<FTEN_SPEC> {
        FTEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn ften7(&mut self) -> FTEN7_W<FTEN_SPEC> {
        FTEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn ften8(&mut self) -> FTEN8_W<FTEN_SPEC> {
        FTEN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn ften9(&mut self) -> FTEN9_W<FTEN_SPEC> {
        FTEN9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn ften10(&mut self) -> FTEN10_W<FTEN_SPEC> {
        FTEN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn ften11(&mut self) -> FTEN11_W<FTEN_SPEC> {
        FTEN11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn ften12(&mut self) -> FTEN12_W<FTEN_SPEC> {
        FTEN12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn ften13(&mut self) -> FTEN13_W<FTEN_SPEC> {
        FTEN13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn ften14(&mut self) -> FTEN14_W<FTEN_SPEC> {
        FTEN14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn ften15(&mut self) -> FTEN15_W<FTEN_SPEC> {
        FTEN15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn ften16(&mut self) -> FTEN16_W<FTEN_SPEC> {
        FTEN16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn ften17(&mut self) -> FTEN17_W<FTEN_SPEC> {
        FTEN17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn ften18(&mut self) -> FTEN18_W<FTEN_SPEC> {
        FTEN18_W::new(self, 18)
    }
}
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`ften::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ften::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTEN_SPEC;
impl crate::RegisterSpec for FTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ften::R`](R) reader structure"]
impl crate::Readable for FTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ften::W`](W) writer structure"]
impl crate::Writable for FTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTEN to value 0"]
impl crate::Resettable for FTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
