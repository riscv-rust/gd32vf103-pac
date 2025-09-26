#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `IRC8MEN` reader - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_R = crate::BitReader;
#[doc = "Field `IRC8MEN` writer - Internal 8MHz RC oscillator Enable"]
pub type IRC8MEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC8MSTB` reader - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
pub type IRC8MSTB_R = crate::BitReader;
#[doc = "Field `IRC8MADJ` reader - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_R = crate::FieldReader;
#[doc = "Field `IRC8MADJ` writer - Internal 8MHz RC Oscillator clock trim adjust value"]
pub type IRC8MADJ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IRC8MCALIB` reader - Internal 8MHz RC Oscillator calibration value register"]
pub type IRC8MCALIB_R = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub type HXTALEN_R = crate::BitReader;
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub type HXTALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HXTALSTB_R = crate::BitReader;
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_R = crate::BitReader;
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HXTALBPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CKMEN_R = crate::BitReader;
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CKMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PLLSTB_R = crate::BitReader;
#[doc = "Field `PLL1EN` reader - PLL1 enable"]
pub type PLL1EN_R = crate::BitReader;
#[doc = "Field `PLL1EN` writer - PLL1 enable"]
pub type PLL1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1STB` reader - PLL1 Clock Stabilization Flag"]
pub type PLL1STB_R = crate::BitReader;
#[doc = "Field `PLL2EN` reader - PLL2 enable"]
pub type PLL2EN_R = crate::BitReader;
#[doc = "Field `PLL2EN` writer - PLL2 enable"]
pub type PLL2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2STB` reader - PLL2 Clock Stabilization Flag"]
pub type PLL2STB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc8men(&self) -> IRC8MEN_R {
        IRC8MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc8mstb(&self) -> IRC8MSTB_R {
        IRC8MSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc8madj(&self) -> IRC8MADJ_R {
        IRC8MADJ_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 8MHz RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc8mcalib(&self) -> IRC8MCALIB_R {
        IRC8MCALIB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HXTALEN_R {
        HXTALEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HXTALSTB_R {
        HXTALSTB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HXTALBPS_R {
        HXTALBPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CKMEN_R {
        CKMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PLLSTB_R {
        PLLSTB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL1 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll1stb(&self) -> PLL1STB_R {
        PLL1STB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2en(&self) -> PLL2EN_R {
        PLL2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL2 Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pll2stb(&self) -> PLL2STB_R {
        PLL2STB_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 8MHz RC oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8men(&mut self) -> IRC8MEN_W<CTL_SPEC> {
        IRC8MEN_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 8MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    #[must_use]
    pub fn irc8madj(&mut self) -> IRC8MADJ_W<CTL_SPEC> {
        IRC8MADJ_W::new(self, 3)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalen(&mut self) -> HXTALEN_W<CTL_SPEC> {
        HXTALEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalbps(&mut self) -> HXTALBPS_W<CTL_SPEC> {
        HXTALBPS_W::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CKMEN_W<CTL_SPEC> {
        CKMEN_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CTL_SPEC> {
        PLLEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - PLL1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1en(&mut self) -> PLL1EN_W<CTL_SPEC> {
        PLL1EN_W::new(self, 26)
    }
    #[doc = "Bit 28 - PLL2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2en(&mut self) -> PLL2EN_W<CTL_SPEC> {
        PLL2EN_W::new(self, 28)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x83"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
