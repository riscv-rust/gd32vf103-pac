#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `INTEN0` reader - Enable Interrupt on line 0"]
pub type INTEN0_R = crate::BitReader;
#[doc = "Field `INTEN0` writer - Enable Interrupt on line 0"]
pub type INTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN1` reader - Enable Interrupt on line 1"]
pub type INTEN1_R = crate::BitReader;
#[doc = "Field `INTEN1` writer - Enable Interrupt on line 1"]
pub type INTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN2` reader - Enable Interrupt on line 2"]
pub type INTEN2_R = crate::BitReader;
#[doc = "Field `INTEN2` writer - Enable Interrupt on line 2"]
pub type INTEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN3` reader - Enable Interrupt on line 3"]
pub type INTEN3_R = crate::BitReader;
#[doc = "Field `INTEN3` writer - Enable Interrupt on line 3"]
pub type INTEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN4` reader - Enable Interrupt on line 4"]
pub type INTEN4_R = crate::BitReader;
#[doc = "Field `INTEN4` writer - Enable Interrupt on line 4"]
pub type INTEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN5` reader - Enable Interrupt on line 5"]
pub type INTEN5_R = crate::BitReader;
#[doc = "Field `INTEN5` writer - Enable Interrupt on line 5"]
pub type INTEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN6` reader - Enable Interrupt on line 6"]
pub type INTEN6_R = crate::BitReader;
#[doc = "Field `INTEN6` writer - Enable Interrupt on line 6"]
pub type INTEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN7` reader - Enable Interrupt on line 7"]
pub type INTEN7_R = crate::BitReader;
#[doc = "Field `INTEN7` writer - Enable Interrupt on line 7"]
pub type INTEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN8` reader - Enable Interrupt on line 8"]
pub type INTEN8_R = crate::BitReader;
#[doc = "Field `INTEN8` writer - Enable Interrupt on line 8"]
pub type INTEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN9` reader - Enable Interrupt on line 9"]
pub type INTEN9_R = crate::BitReader;
#[doc = "Field `INTEN9` writer - Enable Interrupt on line 9"]
pub type INTEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN10` reader - Enable Interrupt on line 10"]
pub type INTEN10_R = crate::BitReader;
#[doc = "Field `INTEN10` writer - Enable Interrupt on line 10"]
pub type INTEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN11` reader - Enable Interrupt on line 11"]
pub type INTEN11_R = crate::BitReader;
#[doc = "Field `INTEN11` writer - Enable Interrupt on line 11"]
pub type INTEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN12` reader - Enable Interrupt on line 12"]
pub type INTEN12_R = crate::BitReader;
#[doc = "Field `INTEN12` writer - Enable Interrupt on line 12"]
pub type INTEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN13` reader - Enable Interrupt on line 13"]
pub type INTEN13_R = crate::BitReader;
#[doc = "Field `INTEN13` writer - Enable Interrupt on line 13"]
pub type INTEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN14` reader - Enable Interrupt on line 14"]
pub type INTEN14_R = crate::BitReader;
#[doc = "Field `INTEN14` writer - Enable Interrupt on line 14"]
pub type INTEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN15` reader - Enable Interrupt on line 15"]
pub type INTEN15_R = crate::BitReader;
#[doc = "Field `INTEN15` writer - Enable Interrupt on line 15"]
pub type INTEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN16` reader - Enable Interrupt on line 16"]
pub type INTEN16_R = crate::BitReader;
#[doc = "Field `INTEN16` writer - Enable Interrupt on line 16"]
pub type INTEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN17` reader - Enable Interrupt on line 17"]
pub type INTEN17_R = crate::BitReader;
#[doc = "Field `INTEN17` writer - Enable Interrupt on line 17"]
pub type INTEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN18` reader - Enable Interrupt on line 18"]
pub type INTEN18_R = crate::BitReader;
#[doc = "Field `INTEN18` writer - Enable Interrupt on line 18"]
pub type INTEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt on line 0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt on line 1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt on line 2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt on line 3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt on line 4"]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt on line 5"]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt on line 6"]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Interrupt on line 7"]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Interrupt on line 8"]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Interrupt on line 9"]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN9_R {
        INTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Interrupt on line 10"]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN10_R {
        INTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Interrupt on line 11"]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN11_R {
        INTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Interrupt on line 12"]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN12_R {
        INTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Interrupt on line 13"]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN13_R {
        INTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Interrupt on line 14"]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN14_R {
        INTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Interrupt on line 15"]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN15_R {
        INTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Interrupt on line 16"]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN16_R {
        INTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Interrupt on line 17"]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN17_R {
        INTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Interrupt on line 18"]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN18_R {
        INTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<INTEN_SPEC> {
        INTEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<INTEN_SPEC> {
        INTEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Interrupt on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<INTEN_SPEC> {
        INTEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Interrupt on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN3_W<INTEN_SPEC> {
        INTEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Interrupt on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn inten4(&mut self) -> INTEN4_W<INTEN_SPEC> {
        INTEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Interrupt on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn inten5(&mut self) -> INTEN5_W<INTEN_SPEC> {
        INTEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Interrupt on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn inten6(&mut self) -> INTEN6_W<INTEN_SPEC> {
        INTEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Interrupt on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn inten7(&mut self) -> INTEN7_W<INTEN_SPEC> {
        INTEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Interrupt on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> INTEN8_W<INTEN_SPEC> {
        INTEN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Interrupt on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn inten9(&mut self) -> INTEN9_W<INTEN_SPEC> {
        INTEN9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Interrupt on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> INTEN10_W<INTEN_SPEC> {
        INTEN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Interrupt on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn inten11(&mut self) -> INTEN11_W<INTEN_SPEC> {
        INTEN11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Interrupt on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn inten12(&mut self) -> INTEN12_W<INTEN_SPEC> {
        INTEN12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Interrupt on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn inten13(&mut self) -> INTEN13_W<INTEN_SPEC> {
        INTEN13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Interrupt on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn inten14(&mut self) -> INTEN14_W<INTEN_SPEC> {
        INTEN14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Interrupt on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn inten15(&mut self) -> INTEN15_W<INTEN_SPEC> {
        INTEN15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Interrupt on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn inten16(&mut self) -> INTEN16_W<INTEN_SPEC> {
        INTEN16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Interrupt on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn inten17(&mut self) -> INTEN17_W<INTEN_SPEC> {
        INTEN17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Interrupt on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn inten18(&mut self) -> INTEN18_W<INTEN_SPEC> {
        INTEN18_W::new(self, 18)
    }
}
#[doc = "Interrupt enable register (EXTI_INTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
