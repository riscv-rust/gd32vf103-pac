#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<STAT0_SPEC>;
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PGERR_R = crate::BitReader;
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WPERR_R = crate::BitReader;
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type ENDF_R = crate::BitReader;
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type ENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WPERR_R {
        WPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> ENDF_R {
        ENDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PGERR_W<STAT0_SPEC> {
        PGERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wperr(&mut self) -> WPERR_W<STAT0_SPEC> {
        WPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn endf(&mut self) -> ENDF_W<STAT0_SPEC> {
        ENDF_W::new(self, 5)
    }
}
#[doc = "Status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
