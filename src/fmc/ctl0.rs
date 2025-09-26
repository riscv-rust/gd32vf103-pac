#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `PG` reader - Main flash program for bank0 command bit"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Main flash program for bank0 command bit"]
pub type PG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PER` reader - Main flash page erase for bank0 command bit"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Main flash page erase for bank0 command bit"]
pub type PER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MER` reader - Main flash mass erase for bank0 command bit"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Main flash mass erase for bank0 command bit"]
pub type MER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBPG` reader - Option bytes program command bit"]
pub type OBPG_R = crate::BitReader;
#[doc = "Field `OBPG` writer - Option bytes program command bit"]
pub type OBPG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBER` reader - Option bytes erase command bit"]
pub type OBER_R = crate::BitReader;
#[doc = "Field `OBER` writer - Option bytes erase command bit"]
pub type OBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LK` reader - FMC_CTL0 lock bit"]
pub type LK_R = crate::BitReader;
#[doc = "Field `LK` writer - FMC_CTL0 lock bit"]
pub type LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBWEN` reader - Option byte erase/program enable bit"]
pub type OBWEN_R = crate::BitReader;
#[doc = "Field `OBWEN` writer - Option byte erase/program enable bit"]
pub type OBWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type ENDIE_R = crate::BitReader;
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type ENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    pub fn obpg(&self) -> OBPG_R {
        OBPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    pub fn ober(&self) -> OBER_R {
        OBER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LK_R {
        LK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    pub fn obwen(&self) -> OBWEN_R {
        OBWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> ENDIE_R {
        ENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main flash program for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CTL0_SPEC, 0> {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Main flash page erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CTL0_SPEC, 1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Main flash mass erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CTL0_SPEC, 2> {
        MER_W::new(self)
    }
    #[doc = "Bit 4 - Option bytes program command bit"]
    #[inline(always)]
    #[must_use]
    pub fn obpg(&mut self) -> OBPG_W<CTL0_SPEC, 4> {
        OBPG_W::new(self)
    }
    #[doc = "Bit 5 - Option bytes erase command bit"]
    #[inline(always)]
    #[must_use]
    pub fn ober(&mut self) -> OBER_W<CTL0_SPEC, 5> {
        OBER_W::new(self)
    }
    #[doc = "Bit 6 - Send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTL0_SPEC, 6> {
        START_W::new(self)
    }
    #[doc = "Bit 7 - FMC_CTL0 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LK_W<CTL0_SPEC, 7> {
        LK_W::new(self)
    }
    #[doc = "Bit 9 - Option byte erase/program enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn obwen(&mut self) -> OBWEN_W<CTL0_SPEC, 9> {
        OBWEN_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL0_SPEC, 10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> ENDIE_W<CTL0_SPEC, 12> {
        ENDIE_W::new(self)
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
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x80"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
