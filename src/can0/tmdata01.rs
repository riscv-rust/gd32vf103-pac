#[doc = "Register `TMDATA01` reader"]
pub type R = crate::R<TMDATA01_SPEC>;
#[doc = "Register `TMDATA01` writer"]
pub type W = crate::W<TMDATA01_SPEC>;
#[doc = "Field `DB0` reader - Data byte 0"]
pub type DB0_R = crate::FieldReader;
#[doc = "Field `DB0` writer - Data byte 0"]
pub type DB0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB1` reader - Data byte 1"]
pub type DB1_R = crate::FieldReader;
#[doc = "Field `DB1` writer - Data byte 1"]
pub type DB1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB2` reader - Data byte 2"]
pub type DB2_R = crate::FieldReader;
#[doc = "Field `DB2` writer - Data byte 2"]
pub type DB2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DB3` reader - Data byte 3"]
pub type DB3_R = crate::FieldReader;
#[doc = "Field `DB3` writer - Data byte 3"]
pub type DB3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn db0(&mut self) -> DB0_W<TMDATA01_SPEC, 0> {
        DB0_W::new(self)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn db1(&mut self) -> DB1_W<TMDATA01_SPEC, 8> {
        DB1_W::new(self)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn db2(&mut self) -> DB2_W<TMDATA01_SPEC, 16> {
        DB2_W::new(self)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn db3(&mut self) -> DB3_W<TMDATA01_SPEC, 24> {
        DB3_W::new(self)
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
#[doc = "Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDATA01_SPEC;
impl crate::RegisterSpec for TMDATA01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdata01::R`](R) reader structure"]
impl crate::Readable for TMDATA01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdata01::W`](W) writer structure"]
impl crate::Writable for TMDATA01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDATA01 to value 0"]
impl crate::Resettable for TMDATA01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
