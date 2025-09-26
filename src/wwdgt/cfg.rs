#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `WIN` reader - 7-bit window value"]
pub type WIN_R = crate::FieldReader;
#[doc = "Field `WIN` writer - 7-bit window value"]
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PSC` reader - Prescaler"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWIE` reader - Early wakeup interrupt"]
pub type EWIE_R = crate::BitReader;
#[doc = "Field `EWIE` writer - Early wakeup interrupt"]
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<CFG_SPEC> {
        WIN_W::new(self, 0)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CFG_SPEC> {
        PSC_W::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<CFG_SPEC> {
        EWIE_W::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
