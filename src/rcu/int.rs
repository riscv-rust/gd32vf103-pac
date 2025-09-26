#[doc = "Register `INT` reader"]
pub type R = crate::R<INT_SPEC>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INT_SPEC>;
#[doc = "Field `IRC40KSTBIF` reader - IRC40K stabilization interrupt flag"]
pub type IRC40KSTBIF_R = crate::BitReader;
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LXTALSTBIF_R = crate::BitReader;
#[doc = "Field `IRC8MSTBIF` reader - IRC8M stabilization interrupt flag"]
pub type IRC8MSTBIF_R = crate::BitReader;
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HXTALSTBIF_R = crate::BitReader;
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PLLSTBIF_R = crate::BitReader;
#[doc = "Field `PLL1STBIF` reader - PLL1 stabilization interrupt flag"]
pub type PLL1STBIF_R = crate::BitReader;
#[doc = "Field `PLL2STBIF` reader - PLL2 stabilization interrupt flag"]
pub type PLL2STBIF_R = crate::BitReader;
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CKMIF_R = crate::BitReader;
#[doc = "Field `IRC40KSTBIE` reader - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_R = crate::BitReader;
#[doc = "Field `IRC40KSTBIE` writer - IRC40K Stabilization interrupt enable"]
pub type IRC40KSTBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_R = crate::BitReader;
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LXTALSTBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC8MSTBIE` reader - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_R = crate::BitReader;
#[doc = "Field `IRC8MSTBIE` writer - IRC8M Stabilization Interrupt Enable"]
pub type IRC8MSTBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_R = crate::BitReader;
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HXTALSTBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_R = crate::BitReader;
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PLLSTBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1STBIE` reader - PLL1 Stabilization Interrupt Enable"]
pub type PLL1STBIE_R = crate::BitReader;
#[doc = "Field `PLL1STBIE` writer - PLL1 Stabilization Interrupt Enable"]
pub type PLL1STBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2STBIE` reader - PLL2 Stabilization Interrupt Enable"]
pub type PLL2STBIE_R = crate::BitReader;
#[doc = "Field `PLL2STBIE` writer - PLL2 Stabilization Interrupt Enable"]
pub type PLL2STBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC40KSTBIC` writer - IRC40K Stabilization Interrupt Clear"]
pub type IRC40KSTBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LXTALSTBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC8MSTBIC` writer - IRC8M Stabilization Interrupt Clear"]
pub type IRC8MSTBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HXTALSTBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PLLSTBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1STBIC` writer - PLL1 stabilization Interrupt Clear"]
pub type PLL1STBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2STBIC` writer - PLL2 stabilization Interrupt Clear"]
pub type PLL2STBIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CKMIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll1stbif(&self) -> PLL1STBIF_R {
        PLL1STBIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL2 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll2stbif(&self) -> PLL2STBIF_R {
        PLL2STBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll1stbie(&self) -> PLL1STBIE_R {
        PLL1STBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll2stbie(&self) -> PLL2STBIE_R {
        PLL2STBIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W<INT_SPEC> {
        IRC40KSTBIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W<INT_SPEC> {
        LXTALSTBIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W<INT_SPEC> {
        IRC8MSTBIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W<INT_SPEC> {
        HXTALSTBIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W<INT_SPEC> {
        PLLSTBIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbie(&mut self) -> PLL1STBIE_W<INT_SPEC> {
        PLL1STBIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbie(&mut self) -> PLL2STBIE_W<INT_SPEC> {
        PLL2STBIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W<INT_SPEC> {
        IRC40KSTBIC_W::new(self, 16)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W<INT_SPEC> {
        LXTALSTBIC_W::new(self, 17)
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W<INT_SPEC> {
        IRC8MSTBIC_W::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W<INT_SPEC> {
        HXTALSTBIC_W::new(self, 19)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W<INT_SPEC> {
        PLLSTBIC_W::new(self, 20)
    }
    #[doc = "Bit 21 - PLL1 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll1stbic(&mut self) -> PLL1STBIC_W<INT_SPEC> {
        PLL1STBIC_W::new(self, 21)
    }
    #[doc = "Bit 22 - PLL2 stabilization Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pll2stbic(&mut self) -> PLL2STBIC_W<INT_SPEC> {
        PLL2STBIC_W::new(self, 22)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ckmic(&mut self) -> CKMIC_W<INT_SPEC> {
        CKMIC_W::new(self, 23)
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
