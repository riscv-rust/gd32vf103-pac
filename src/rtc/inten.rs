#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `SCIE` reader - Second interrupt"]
pub type SCIE_R = crate::BitReader;
#[doc = "Field `SCIE` writer - Second interrupt"]
pub type SCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRMIE` reader - Alarm interrupt enable"]
pub type ALRMIE_R = crate::BitReader;
#[doc = "Field `ALRMIE` writer - Alarm interrupt enable"]
pub type ALRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIE` reader - Overflow interrupt enable"]
pub type OVIE_R = crate::BitReader;
#[doc = "Field `OVIE` writer - Overflow interrupt enable"]
pub type OVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrmie(&self) -> ALRMIE_R {
        ALRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<INTEN_SPEC> {
        SCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrmie(&mut self) -> ALRMIE_W<INTEN_SPEC> {
        ALRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OVIE_W<INTEN_SPEC> {
        OVIE_W::new(self, 2)
    }
}
#[doc = "RTC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
