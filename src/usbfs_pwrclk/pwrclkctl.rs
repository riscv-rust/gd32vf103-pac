#[doc = "Register `PWRCLKCTL` reader"]
pub type R = crate::R<PWRCLKCTL_SPEC>;
#[doc = "Register `PWRCLKCTL` writer"]
pub type W = crate::W<PWRCLKCTL_SPEC>;
#[doc = "Field `SUCLK` reader - Stop the USB clock"]
pub type SUCLK_R = crate::BitReader;
#[doc = "Field `SUCLK` writer - Stop the USB clock"]
pub type SUCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHCLK` reader - Stop HCLK"]
pub type SHCLK_R = crate::BitReader;
#[doc = "Field `SHCLK` writer - Stop HCLK"]
pub type SHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&self) -> SUCLK_R {
        SUCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&self) -> SHCLK_R {
        SHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    #[must_use]
    pub fn suclk(&mut self) -> SUCLK_W<PWRCLKCTL_SPEC> {
        SUCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn shclk(&mut self) -> SHCLK_W<PWRCLKCTL_SPEC> {
        SHCLK_W::new(self, 1)
    }
}
#[doc = "power and clock gating control register (PWRCLKCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCLKCTL_SPEC;
impl crate::RegisterSpec for PWRCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrclkctl::R`](R) reader structure"]
impl crate::Readable for PWRCLKCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrclkctl::W`](W) writer structure"]
impl crate::Writable for PWRCLKCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCLKCTL to value 0"]
impl crate::Resettable for PWRCLKCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
