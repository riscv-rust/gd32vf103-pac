#[doc = "Register `TPCTL` reader"]
pub type R = crate::R<TPCTL_SPEC>;
#[doc = "Register `TPCTL` writer"]
pub type W = crate::W<TPCTL_SPEC>;
#[doc = "Field `TPEN` reader - TAMPER detection enable"]
pub type TPEN_R = crate::BitReader;
#[doc = "Field `TPEN` writer - TAMPER detection enable"]
pub type TPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL` reader - TAMPER pin active level"]
pub type TPAL_R = crate::BitReader;
#[doc = "Field `TPAL` writer - TAMPER pin active level"]
pub type TPAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen(&mut self) -> TPEN_W<TPCTL_SPEC> {
        TPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TPAL_W<TPCTL_SPEC> {
        TPAL_W::new(self, 1)
    }
}
#[doc = "Tamper pin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCTL_SPEC;
impl crate::RegisterSpec for TPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tpctl::R`](R) reader structure"]
impl crate::Readable for TPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpctl::W`](W) writer structure"]
impl crate::Writable for TPCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TPCTL to value 0"]
impl crate::Resettable for TPCTL_SPEC {
    const RESET_VALUE: u16 = 0;
}
