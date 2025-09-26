#[doc = "Register `PCF1` reader"]
pub type R = crate::R<PCF1_SPEC>;
#[doc = "Register `PCF1` writer"]
pub type W = crate::W<PCF1_SPEC>;
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_R = crate::BitReader;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W<PCF1_SPEC, 10> {
        EXMC_NADV_W::new(self)
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
#[doc = "AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCF1_SPEC;
impl crate::RegisterSpec for PCF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf1::R`](R) reader structure"]
impl crate::Readable for PCF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcf1::W`](W) writer structure"]
impl crate::Writable for PCF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for PCF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
