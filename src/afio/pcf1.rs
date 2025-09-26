#[doc = "Register `PCF1` reader"]
pub type R = crate::R<PCF1_SPEC>;
#[doc = "Register `PCF1` writer"]
pub type W = crate::W<PCF1_SPEC>;
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_R = crate::BitReader;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> EXMC_NADV_R {
        EXMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W<PCF1_SPEC> {
        EXMC_NADV_W::new(self, 10)
    }
}
#[doc = "AFIO port configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCF1_SPEC;
impl crate::RegisterSpec for PCF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf1::R`](R) reader structure"]
impl crate::Readable for PCF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcf1::W`](W) writer structure"]
impl crate::Writable for PCF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for PCF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
