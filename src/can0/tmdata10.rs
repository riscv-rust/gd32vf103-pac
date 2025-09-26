#[doc = "Register `TMDATA10` reader"]
pub type R = crate::R<TMDATA10_SPEC>;
#[doc = "Register `TMDATA10` writer"]
pub type W = crate::W<TMDATA10_SPEC>;
#[doc = "Field `DB4` reader - Data byte 4"]
pub type DB4_R = crate::FieldReader;
#[doc = "Field `DB4` writer - Data byte 4"]
pub type DB4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB5` reader - Data byte 5"]
pub type DB5_R = crate::FieldReader;
#[doc = "Field `DB5` writer - Data byte 5"]
pub type DB5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB6` reader - Data byte 6"]
pub type DB6_R = crate::FieldReader;
#[doc = "Field `DB6` writer - Data byte 6"]
pub type DB6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB7` reader - Data byte 7"]
pub type DB7_R = crate::FieldReader;
#[doc = "Field `DB7` writer - Data byte 7"]
pub type DB7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn db4(&mut self) -> DB4_W<TMDATA10_SPEC, 0> {
        DB4_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn db5(&mut self) -> DB5_W<TMDATA10_SPEC, 8> {
        DB5_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn db6(&mut self) -> DB6_W<TMDATA10_SPEC, 16> {
        DB6_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    #[must_use]
    pub fn db7(&mut self) -> DB7_W<TMDATA10_SPEC, 24> {
        DB7_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDATA10_SPEC;
impl crate::RegisterSpec for TMDATA10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdata10::R`](R) reader structure"]
impl crate::Readable for TMDATA10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdata10::W`](W) writer structure"]
impl crate::Writable for TMDATA10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDATA10 to value 0"]
impl crate::Resettable for TMDATA10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
