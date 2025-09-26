#[doc = "Register `EVEN` reader"]
pub type R = crate::R<EVEN_SPEC>;
#[doc = "Register `EVEN` writer"]
pub type W = crate::W<EVEN_SPEC>;
#[doc = "Field `EVEN0` reader - Enable Event on line 0"]
pub type EVEN0_R = crate::BitReader;
#[doc = "Field `EVEN0` writer - Enable Event on line 0"]
pub type EVEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN1` reader - Enable Event on line 1"]
pub type EVEN1_R = crate::BitReader;
#[doc = "Field `EVEN1` writer - Enable Event on line 1"]
pub type EVEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN2` reader - Enable Event on line 2"]
pub type EVEN2_R = crate::BitReader;
#[doc = "Field `EVEN2` writer - Enable Event on line 2"]
pub type EVEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN3` reader - Enable Event on line 3"]
pub type EVEN3_R = crate::BitReader;
#[doc = "Field `EVEN3` writer - Enable Event on line 3"]
pub type EVEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN4` reader - Enable Event on line 4"]
pub type EVEN4_R = crate::BitReader;
#[doc = "Field `EVEN4` writer - Enable Event on line 4"]
pub type EVEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN5` reader - Enable Event on line 5"]
pub type EVEN5_R = crate::BitReader;
#[doc = "Field `EVEN5` writer - Enable Event on line 5"]
pub type EVEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN6` reader - Enable Event on line 6"]
pub type EVEN6_R = crate::BitReader;
#[doc = "Field `EVEN6` writer - Enable Event on line 6"]
pub type EVEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN7` reader - Enable Event on line 7"]
pub type EVEN7_R = crate::BitReader;
#[doc = "Field `EVEN7` writer - Enable Event on line 7"]
pub type EVEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN8` reader - Enable Event on line 8"]
pub type EVEN8_R = crate::BitReader;
#[doc = "Field `EVEN8` writer - Enable Event on line 8"]
pub type EVEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN9` reader - Enable Event on line 9"]
pub type EVEN9_R = crate::BitReader;
#[doc = "Field `EVEN9` writer - Enable Event on line 9"]
pub type EVEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN10` reader - Enable Event on line 10"]
pub type EVEN10_R = crate::BitReader;
#[doc = "Field `EVEN10` writer - Enable Event on line 10"]
pub type EVEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN11` reader - Enable Event on line 11"]
pub type EVEN11_R = crate::BitReader;
#[doc = "Field `EVEN11` writer - Enable Event on line 11"]
pub type EVEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN12` reader - Enable Event on line 12"]
pub type EVEN12_R = crate::BitReader;
#[doc = "Field `EVEN12` writer - Enable Event on line 12"]
pub type EVEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN13` reader - Enable Event on line 13"]
pub type EVEN13_R = crate::BitReader;
#[doc = "Field `EVEN13` writer - Enable Event on line 13"]
pub type EVEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN14` reader - Enable Event on line 14"]
pub type EVEN14_R = crate::BitReader;
#[doc = "Field `EVEN14` writer - Enable Event on line 14"]
pub type EVEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN15` reader - Enable Event on line 15"]
pub type EVEN15_R = crate::BitReader;
#[doc = "Field `EVEN15` writer - Enable Event on line 15"]
pub type EVEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN16` reader - Enable Event on line 16"]
pub type EVEN16_R = crate::BitReader;
#[doc = "Field `EVEN16` writer - Enable Event on line 16"]
pub type EVEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN17` reader - Enable Event on line 17"]
pub type EVEN17_R = crate::BitReader;
#[doc = "Field `EVEN17` writer - Enable Event on line 17"]
pub type EVEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVEN18` reader - Enable Event on line 18"]
pub type EVEN18_R = crate::BitReader;
#[doc = "Field `EVEN18` writer - Enable Event on line 18"]
pub type EVEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Event on line 0"]
    #[inline(always)]
    pub fn even0(&self) -> EVEN0_R {
        EVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Event on line 1"]
    #[inline(always)]
    pub fn even1(&self) -> EVEN1_R {
        EVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Event on line 2"]
    #[inline(always)]
    pub fn even2(&self) -> EVEN2_R {
        EVEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Event on line 3"]
    #[inline(always)]
    pub fn even3(&self) -> EVEN3_R {
        EVEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Event on line 4"]
    #[inline(always)]
    pub fn even4(&self) -> EVEN4_R {
        EVEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Event on line 5"]
    #[inline(always)]
    pub fn even5(&self) -> EVEN5_R {
        EVEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Event on line 6"]
    #[inline(always)]
    pub fn even6(&self) -> EVEN6_R {
        EVEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Event on line 7"]
    #[inline(always)]
    pub fn even7(&self) -> EVEN7_R {
        EVEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Event on line 8"]
    #[inline(always)]
    pub fn even8(&self) -> EVEN8_R {
        EVEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Event on line 9"]
    #[inline(always)]
    pub fn even9(&self) -> EVEN9_R {
        EVEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Event on line 10"]
    #[inline(always)]
    pub fn even10(&self) -> EVEN10_R {
        EVEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Event on line 11"]
    #[inline(always)]
    pub fn even11(&self) -> EVEN11_R {
        EVEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Event on line 12"]
    #[inline(always)]
    pub fn even12(&self) -> EVEN12_R {
        EVEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Event on line 13"]
    #[inline(always)]
    pub fn even13(&self) -> EVEN13_R {
        EVEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Event on line 14"]
    #[inline(always)]
    pub fn even14(&self) -> EVEN14_R {
        EVEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Event on line 15"]
    #[inline(always)]
    pub fn even15(&self) -> EVEN15_R {
        EVEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Event on line 16"]
    #[inline(always)]
    pub fn even16(&self) -> EVEN16_R {
        EVEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Event on line 17"]
    #[inline(always)]
    pub fn even17(&self) -> EVEN17_R {
        EVEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Event on line 18"]
    #[inline(always)]
    pub fn even18(&self) -> EVEN18_R {
        EVEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Event on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn even0(&mut self) -> EVEN0_W<EVEN_SPEC> {
        EVEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Event on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn even1(&mut self) -> EVEN1_W<EVEN_SPEC> {
        EVEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Event on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn even2(&mut self) -> EVEN2_W<EVEN_SPEC> {
        EVEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Event on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn even3(&mut self) -> EVEN3_W<EVEN_SPEC> {
        EVEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Event on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn even4(&mut self) -> EVEN4_W<EVEN_SPEC> {
        EVEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Event on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn even5(&mut self) -> EVEN5_W<EVEN_SPEC> {
        EVEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Event on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn even6(&mut self) -> EVEN6_W<EVEN_SPEC> {
        EVEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Event on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn even7(&mut self) -> EVEN7_W<EVEN_SPEC> {
        EVEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Event on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn even8(&mut self) -> EVEN8_W<EVEN_SPEC> {
        EVEN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Event on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn even9(&mut self) -> EVEN9_W<EVEN_SPEC> {
        EVEN9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Event on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn even10(&mut self) -> EVEN10_W<EVEN_SPEC> {
        EVEN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Event on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn even11(&mut self) -> EVEN11_W<EVEN_SPEC> {
        EVEN11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Event on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn even12(&mut self) -> EVEN12_W<EVEN_SPEC> {
        EVEN12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Event on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn even13(&mut self) -> EVEN13_W<EVEN_SPEC> {
        EVEN13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Event on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn even14(&mut self) -> EVEN14_W<EVEN_SPEC> {
        EVEN14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Event on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn even15(&mut self) -> EVEN15_W<EVEN_SPEC> {
        EVEN15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Event on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn even16(&mut self) -> EVEN16_W<EVEN_SPEC> {
        EVEN16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Event on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn even17(&mut self) -> EVEN17_W<EVEN_SPEC> {
        EVEN17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Event on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn even18(&mut self) -> EVEN18_W<EVEN_SPEC> {
        EVEN18_W::new(self, 18)
    }
}
#[doc = "Event enable register (EXTI_EVEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`even::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`even::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVEN_SPEC;
impl crate::RegisterSpec for EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`even::R`](R) reader structure"]
impl crate::Readable for EVEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`even::W`](W) writer structure"]
impl crate::Writable for EVEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EVEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
