#[doc = "Register `CKCFG` reader"]
pub type R = crate::R<CKCFG_SPEC>;
#[doc = "Register `CKCFG` writer"]
pub type W = crate::W<CKCFG_SPEC>;
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type CLKC_R = crate::FieldReader<u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type CLKC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DTCY_R = crate::BitReader;
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DTCY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FAST_R = crate::BitReader;
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn clkc(&mut self) -> CLKC_W<CKCFG_SPEC, 0> {
        CLKC_W::new(self)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcy(&mut self) -> DTCY_W<CKCFG_SPEC, 14> {
        DTCY_W::new(self)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<CKCFG_SPEC, 15> {
        FAST_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKCFG_SPEC;
impl crate::RegisterSpec for CKCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ckcfg::R`](R) reader structure"]
impl crate::Readable for CKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckcfg::W`](W) writer structure"]
impl crate::Writable for CKCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
