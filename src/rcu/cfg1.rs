#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `PREDV0` reader - PREDV0 division factor"]
pub type PREDV0_R = crate::FieldReader;
#[doc = "Field `PREDV0` writer - PREDV0 division factor"]
pub type PREDV0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PREDV1` reader - PREDV1 division factor"]
pub type PREDV1_R = crate::FieldReader;
#[doc = "Field `PREDV1` writer - PREDV1 division factor"]
pub type PREDV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL1MF` reader - The PLL1 clock multiplication factor"]
pub type PLL1MF_R = crate::FieldReader;
#[doc = "Field `PLL1MF` writer - The PLL1 clock multiplication factor"]
pub type PLL1MF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL2MF` reader - The PLL2 clock multiplication factor"]
pub type PLL2MF_R = crate::FieldReader;
#[doc = "Field `PLL2MF` writer - The PLL2 clock multiplication factor"]
pub type PLL2MF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PREDV0SEL` reader - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_R = crate::BitReader;
#[doc = "Field `PREDV0SEL` writer - PREDV0 input Clock Source Selection"]
pub type PREDV0SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S1SEL` reader - I2S1 Clock Source Selection"]
pub type I2S1SEL_R = crate::BitReader;
#[doc = "Field `I2S1SEL` writer - I2S1 Clock Source Selection"]
pub type I2S1SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S2SEL` reader - I2S2 Clock Source Selection"]
pub type I2S2SEL_R = crate::BitReader;
#[doc = "Field `I2S2SEL` writer - I2S2 Clock Source Selection"]
pub type I2S2SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&self) -> PREDV0_R {
        PREDV0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&self) -> PREDV1_R {
        PREDV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&self) -> PLL1MF_R {
        PLL1MF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&self) -> PLL2MF_R {
        PLL2MF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&self) -> PREDV0SEL_R {
        PREDV0SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn predv0(&mut self) -> PREDV0_W<CFG1_SPEC, 0> {
        PREDV0_W::new(self)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn predv1(&mut self) -> PREDV1_W<CFG1_SPEC, 4> {
        PREDV1_W::new(self)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll1mf(&mut self) -> PLL1MF_W<CFG1_SPEC, 8> {
        PLL1MF_W::new(self)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll2mf(&mut self) -> PLL2MF_W<CFG1_SPEC, 12> {
        PLL2MF_W::new(self)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn predv0sel(&mut self) -> PREDV0SEL_W<CFG1_SPEC, 16> {
        PREDV0SEL_W::new(self)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W<CFG1_SPEC, 17> {
        I2S1SEL_W::new(self)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<CFG1_SPEC, 18> {
        I2S2SEL_W::new(self)
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
#[doc = "Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
