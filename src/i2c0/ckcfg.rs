#[doc = "Register `CKCFG` reader"]
pub type R = crate::R<CKCFG_SPEC>;
#[doc = "Register `CKCFG` writer"]
pub type W = crate::W<CKCFG_SPEC>;
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type CLKC_R = crate::FieldReader<u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type CLKC_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DTCY_R = crate::BitReader;
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DTCY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&self) -> DTCY_R {
        DTCY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> CLKC_W<CKCFG_SPEC> {
        CLKC_W::new(self, 0)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcy(&mut self) -> DTCY_W<CKCFG_SPEC> {
        DTCY_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<CKCFG_SPEC> {
        FAST_W::new(self, 15)
    }
}
#[doc = "Clock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKCFG_SPEC;
impl crate::RegisterSpec for CKCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ckcfg::R`](R) reader structure"]
impl crate::Readable for CKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckcfg::W`](W) writer structure"]
impl crate::Writable for CKCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CKCFG_SPEC {
    const RESET_VALUE: u16 = 0;
}
