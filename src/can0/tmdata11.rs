#[doc = "Register `TMDATA11` reader"]
pub type R = crate::R<TMDATA11_SPEC>;
#[doc = "Register `TMDATA11` writer"]
pub type W = crate::W<TMDATA11_SPEC>;
#[doc = "Field `DB4` reader - Data byte 4"]
pub type DB4_R = crate::FieldReader;
#[doc = "Field `DB4` writer - Data byte 4"]
pub type DB4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB5` reader - Data byte 5"]
pub type DB5_R = crate::FieldReader;
#[doc = "Field `DB5` writer - Data byte 5"]
pub type DB5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB6` reader - Data byte 6"]
pub type DB6_R = crate::FieldReader;
#[doc = "Field `DB6` writer - Data byte 6"]
pub type DB6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DB7` reader - Data byte 7"]
pub type DB7_R = crate::FieldReader;
#[doc = "Field `DB7` writer - Data byte 7"]
pub type DB7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn db4(&mut self) -> DB4_W<TMDATA11_SPEC> {
        DB4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn db5(&mut self) -> DB5_W<TMDATA11_SPEC> {
        DB5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn db6(&mut self) -> DB6_W<TMDATA11_SPEC> {
        DB6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    #[must_use]
    pub fn db7(&mut self) -> DB7_W<TMDATA11_SPEC> {
        DB7_W::new(self, 24)
    }
}
#[doc = "Transmit mailbox data1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdata11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdata11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDATA11_SPEC;
impl crate::RegisterSpec for TMDATA11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdata11::R`](R) reader structure"]
impl crate::Readable for TMDATA11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdata11::W`](W) writer structure"]
impl crate::Writable for TMDATA11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDATA11 to value 0"]
impl crate::Resettable for TMDATA11_SPEC {
    const RESET_VALUE: u32 = 0;
}
